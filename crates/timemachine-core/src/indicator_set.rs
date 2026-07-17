//! The per-symbol indicator set.
//!
//! The Time Machine folds a small allowlist of **scalar** wickra-core indicators
//! (`Sma`, `Ema`, `Rsi`) on each symbol's trade price. These are the
//! `Indicator<Input = f64, Output = f64>` types; candle-input indicators such as
//! `Atr` are out of scope here because the re-fold sees trade prints, not OHLC
//! candles.

use wickra_core::{Ema, Indicator, Rsi, Sma};

use crate::error::{Error, Result};
use crate::spec::IndicatorRef;

/// Whether `name` is a scalar indicator the Time Machine can fold on trade price.
#[must_use]
pub(crate) fn is_known_indicator(name: &str) -> bool {
    matches!(name, "Sma" | "Ema" | "Rsi")
}

/// One named streaming indicator plus its latest output.
struct Entry {
    key: String,
    indicator: Box<dyn Indicator<Input = f64, Output = f64>>,
    last: Option<f64>,
}

/// The set of streaming indicators tracked for a symbol.
pub struct IndicatorSet {
    entries: Vec<Entry>,
}

impl IndicatorSet {
    /// Build the set from spec references. Each reference must name a known
    /// scalar indicator and carry a single positive integer period.
    ///
    /// # Errors
    /// [`Error::UnknownIndicator`] for an unknown name, or [`Error::BadSpec`] if a
    /// period is invalid for the indicator.
    pub fn new(refs: &[IndicatorRef]) -> Result<Self> {
        let mut entries = Vec::with_capacity(refs.len());
        for r in refs {
            let period = r.params.first().copied().unwrap_or_default();
            let period = period as usize;
            let indicator: Box<dyn Indicator<Input = f64, Output = f64>> = match r.name.as_str() {
                "Sma" => Box::new(Sma::new(period).map_err(|e| Error::BadSpec(format!("{e:?}")))?),
                "Ema" => Box::new(Ema::new(period).map_err(|e| Error::BadSpec(format!("{e:?}")))?),
                "Rsi" => Box::new(Rsi::new(period).map_err(|e| Error::BadSpec(format!("{e:?}")))?),
                other => return Err(Error::UnknownIndicator(other.to_string())),
            };
            entries.push(Entry {
                key: r.key(),
                indicator,
                last: None,
            });
        }
        Ok(Self { entries })
    }

    /// Feed one price into every indicator, advancing each by one input.
    pub fn update(&mut self, price: f64) {
        for entry in &mut self.entries {
            entry.last = entry.indicator.update(price);
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
    use super::{is_known_indicator, IndicatorSet};
    use crate::spec::IndicatorRef;

    fn sma20() -> IndicatorRef {
        IndicatorRef {
            name: "Sma".into(),
            params: vec![20.0],
        }
    }

    #[test]
    fn known_names() {
        assert!(is_known_indicator("Sma"));
        assert!(is_known_indicator("Ema"));
        assert!(is_known_indicator("Rsi"));
        assert!(!is_known_indicator("Atr"));
    }

    #[test]
    fn warms_up_then_reports() {
        let mut set = IndicatorSet::new(&[sma20()]).unwrap();
        for _ in 0..19 {
            set.update(100.0);
        }
        assert_eq!(set.values()[0].1, None);
        set.update(100.0);
        assert_eq!(set.values()[0].1, Some(100.0));
    }

    #[test]
    fn zero_period_rejected() {
        let bad = IndicatorRef {
            name: "Sma".into(),
            params: vec![0.0],
        };
        assert!(IndicatorSet::new(&[bad]).is_err());
    }
}
