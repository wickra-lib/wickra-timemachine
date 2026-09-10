//! The per-symbol indicator set.
//!
//! Names resolve through the `wickra-backtest` registry — the ecosystem's only
//! name -> indicator factory — so a `TimelineSpec` can name any indicator the
//! backtester and the screener can, with the same parameter order and the same
//! semantics. Nothing about indicator behaviour is re-implemented here.
//!
//! This module used to carry a three-name allowlist (`Sma`, `Ema`, `Rsi`) while
//! `docs/INDICATORS.md` described exactly the resolution above. `Macd`, `Atr`,
//! `Roc` and every other registry name were rejected as unknown, so the promise
//! and the code disagreed by several hundred names.
//!
//! The re-fold sees trade prints rather than OHLC candles, so each print is
//! widened onto a flat bar — open, high, low and close all the trade price, the
//! volume its size. That is what the price is: a bar that opened, peaked, bottomed
//! and closed at one print. An indicator reading the range therefore reads zero
//! range, which is the truth about a single trade rather than an approximation of
//! one.
//!
//! Side feeds are a different matter. The Time Machine reconstructs an order book
//! and a trade tape, but hands the indicators neither: the book is a snapshot
//! field of its own, and the tape is bounded. An indicator needing one would tick
//! and return nothing for ever, so [`feed_kind`] classifies the name and
//! [`IndicatorSet::new`] refuses it, naming the feed rather than accepting a
//! value that never arrives.

use wickra_backtest::core::registry::{build, feed_of, BarInput, EvalIndicator};
use wickra_backtest::core::spec::Feed;
use wickra_backtest::core::Candle as BtCandle;

use crate::error::{Error, Result};
use crate::spec::IndicatorRef;

/// The pairwise indicators, which read a reference series' close alongside the
/// bar close.
///
/// This list exists because `registry::feed_of` cannot express the family:
/// upstream classifies every pairwise indicator as `Feed::Kline`, since the
/// candle is indeed one of its two inputs. The `every_pairwise_name_is_refused`
/// test probes each name against the live registry, so a registry that grows a
/// new pairwise indicator cannot slip past silently.
const PAIRWISE: [&str; 24] = [
    "Alpha",
    "Beta",
    "BetaNeutralSpread",
    "Cointegration",
    "DistanceSsd",
    "GrangerCausality",
    "HasbrouckInformationShare",
    "InformationRatio",
    "KalmanHedgeRatio",
    "KendallTau",
    "LeadLagCrossCorrelation",
    "OuHalfLife",
    "PairSpreadZScore",
    "PairwiseBeta",
    "PearsonCorrelation",
    "RelativeStrengthAB",
    "RollingCorrelation",
    "RollingCovariance",
    "SpearmanCorrelation",
    "SpreadAr1Coefficient",
    "SpreadBollingerBands",
    "SpreadHurst",
    "TreynorRatio",
    "VarianceRatio",
];

/// Which side feed an indicator needs beyond the bar, or `None` for the ones a
/// bar alone drives.
///
/// A `None` here means the Time Machine can fold the indicator; anything else
/// names a feed it does not hand the indicators, and [`check`] refuses the name.
///
/// `feed_of` does not classify every buildable name — the multi-output family
/// (`Macd` and its kin) is absent from its table — and those are all candle
/// driven, so an unclassified name is `None` rather than an error. Whether the
/// name exists at all is the registry's answer, given by `build`.
#[must_use]
pub fn feed_kind(name: &str) -> Option<&'static str> {
    match feed_of(name)? {
        Feed::Kline if PAIRWISE.contains(&name) => Some("a reference series"),
        Feed::Kline => None,
        Feed::Trade => Some("the individual trades of a bar"),
        Feed::Orderbook => Some("an order-book snapshot"),
        Feed::TradeQuote => Some("trades quoted against the book"),
        Feed::Derivatives => Some("a derivatives tick"),
        Feed::CrossSection => Some("the market cross-section"),
    }
}

/// Whether the Time Machine can fold `name` with `params`, without building the
/// indicator to keep.
///
/// One check, not two: the registry validates the name and its parameters
/// together, which is why an arity rule here would only ever disagree with it.
/// The old spec validation carried "exactly one parameter (period)", true of the
/// three names it allowed and false of `Macd(12, 26, 9)`.
///
/// # Errors
/// [`Error::BadSpec`] for a name whose side feed the re-fold does not supply,
/// and [`Error::UnknownIndicator`] for a name or parameter set the registry
/// rejects.
pub(crate) fn check(name: &str, params: &[f64]) -> Result<()> {
    if let Some(feed) = feed_kind(name) {
        return Err(Error::BadSpec(format!(
            "{name} reads {feed}, which the re-fold does not hand the indicators;              it would return nothing on every event"
        )));
    }
    build(name, params).map_err(|_| Error::UnknownIndicator(name.to_string()))?;
    Ok(())
}

/// One named streaming indicator plus its latest output.
struct Entry {
    key: String,
    indicator: Box<dyn EvalIndicator>,
    last: Option<f64>,
}

/// The set of streaming indicators tracked for a symbol.
pub struct IndicatorSet {
    entries: Vec<Entry>,
}

impl IndicatorSet {
    /// Build the set from spec references, resolving each through the registry.
    ///
    /// # Errors
    /// [`Error::UnknownIndicator`] for a name the registry does not know or one
    /// whose parameters it rejects, and [`Error::BadSpec`] for a name whose side
    /// feed the Time Machine does not hand the indicators.
    pub fn new(refs: &[IndicatorRef]) -> Result<Self> {
        let mut entries = Vec::with_capacity(refs.len());
        for r in refs {
            check(&r.name, &r.params)?;
            let indicator =
                build(&r.name, &r.params).map_err(|_| Error::UnknownIndicator(r.name.clone()))?;
            entries.push(Entry {
                key: r.key(),
                indicator,
                last: None,
            });
        }
        Ok(Self { entries })
    }

    /// Feed one trade price into every indicator, advancing each by one input.
    ///
    /// The print is widened onto a flat bar: a single trade opened, peaked,
    /// bottomed and closed at one price, and `size` is the volume that changed
    /// hands. An indicator reading the range reads zero, which is what a single
    /// print's range is.
    pub fn update(&mut self, price: f64, size: f64) {
        let candle = BtCandle {
            time: 0,
            open: price,
            high: price,
            low: price,
            close: price,
            volume: size,
        };
        let bar = BarInput {
            candle: &candle,
            reference: None,
            deriv: None,
            orderbook: None,
            trades: &[],
            cross_section: None,
        };
        for entry in &mut self.entries {
            entry.last = entry.indicator.update(&bar);
        }
    }

    /// The latest `(key, value)` of each indicator (`value` is `None` while
    /// warming up), in configuration order.
    #[must_use]
    pub fn values(&self) -> Vec<(String, Option<f64>)> {
        self.entries
            .iter()
            .map(|e| (e.key.clone(), e.last))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{check, feed_kind, IndicatorSet};
    use crate::spec::IndicatorRef;

    fn indicator(name: &str, params: Vec<f64>) -> IndicatorRef {
        IndicatorRef {
            name: name.into(),
            params,
        }
    }

    #[test]
    fn the_registry_is_the_allowlist() {
        // The three the old hardcoded list carried...
        for name in ["Sma", "Ema", "Rsi"] {
            assert!(check(name, &[14.0]).is_ok(), "{name} must resolve");
        }
        // ...and the ones it rejected, which the registry knows perfectly well.
        for name in ["Wma", "Dema", "Roc", "Atr"] {
            assert!(check(name, &[14.0]).is_ok(), "{name} must resolve");
        }
        // Multi-parameter names were unreachable twice over: the allowlist did
        // not carry them, and the spec demanded exactly one parameter.
        assert!(check("Macd", &[12.0, 26.0, 9.0]).is_ok());
        assert!(check("NotAnIndicator", &[14.0]).is_err());
    }

    #[test]
    fn a_name_needing_a_side_feed_is_refused_by_name() {
        for (name, feed) in [
            ("Microprice", "an order-book snapshot"),
            ("FundingRate", "a derivatives tick"),
            ("CumulativeVolumeDelta", "the individual trades of a bar"),
            ("AdvanceDecline", "the market cross-section"),
            ("RollingCorrelation", "a reference series"),
        ] {
            assert_eq!(feed_kind(name), Some(feed), "{name}");
            assert!(check(name, &[20.0]).is_err(), "{name} must not resolve");
            let Err(err) = check(name, &[20.0]) else {
                panic!("{name} needs a side feed and must be refused");
            };
            assert!(err.to_string().contains(feed), "{name}: {err}");
        }
    }

    #[test]
    fn every_pairwise_name_is_refused() {
        for name in super::PAIRWISE {
            assert_eq!(feed_kind(name), Some("a reference series"), "{name}");
        }
    }

    #[test]
    fn warms_up_then_reports() {
        let mut set = IndicatorSet::new(&[indicator("Sma", vec![20.0])]).unwrap();
        for _ in 0..19 {
            set.update(100.0, 1.0);
        }
        assert_eq!(set.values()[0].1, None);
        set.update(100.0, 1.0);
        assert_eq!(set.values()[0].1, Some(100.0));
    }

    #[test]
    fn a_multi_parameter_indicator_resolves_and_folds() {
        // Macd(12, 26, 9) was rejected outright by the old allowlist.
        let mut set = IndicatorSet::new(&[indicator("Macd", vec![12.0, 26.0, 9.0])]).unwrap();
        for i in 0..60 {
            set.update(100.0 + f64::from(i), 1.0);
        }
        assert!(set.values()[0].1.is_some(), "Macd produced no value");
    }

    #[test]
    fn zero_period_rejected() {
        assert!(IndicatorSet::new(&[indicator("Sma", vec![0.0])]).is_err());
    }

    #[test]
    fn unknown_name_rejected() {
        assert!(IndicatorSet::new(&[indicator("NotAnIndicator", vec![14.0])]).is_err());
    }
}
