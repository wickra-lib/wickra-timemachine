//! The reconstructed microstructure snapshot — the time machine's output.
//!
//! Every field is a plain `f64` (or a small record of them), every map is a
//! [`BTreeMap`] sorted by key, and every float is rounded through [`round_to`]
//! before it leaves the core. That is what makes a snapshot byte-identical across
//! every language binding and between the parallel and single-threaded fold paths.

use std::collections::BTreeMap;

use serde::Serialize;

/// Round a value to 8 decimal places, mapping non-finite inputs to `0.0` so a
/// serialized snapshot never carries `NaN`/`Infinity` (which are not valid JSON).
#[must_use]
pub fn round_to(value: f64) -> f64 {
    if value.is_finite() {
        (value * 1e8).round() / 1e8
    } else {
        0.0
    }
}

/// One price level of a reconstructed order book.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BookLevel {
    /// The price of the level.
    pub price: f64,
    /// The resting quantity at the level.
    pub qty: f64,
}

/// A reconstructed order-book top-of-book view.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BookSnapshot {
    /// The best bid levels, highest price first.
    pub bids: Vec<BookLevel>,
    /// The best ask levels, lowest price first.
    pub asks: Vec<BookLevel>,
    /// The bid/ask spread, or `None` if either side is empty.
    pub spread: Option<f64>,
}

/// A single recent trade print.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TradeRecord {
    /// The trade price.
    pub price: f64,
    /// The traded quantity.
    pub qty: f64,
    /// The aggressor side, `"buy"` or `"sell"`.
    pub side: String,
    /// The venue timestamp (milliseconds since the Unix epoch).
    pub ts: i64,
}

/// The latest funding print for a perpetual market.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FundingSnapshot {
    /// The funding rate for the interval (may be negative).
    pub rate: f64,
    /// The mark price at the funding print.
    pub mark_price: f64,
}

/// The reconstructed state of one symbol at the seek target.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SymbolSnapshot {
    /// The last traded price seen, or `0.0` if no trade has printed.
    pub last: f64,
    /// The reconstructed order book.
    pub book: BookSnapshot,
    /// The most recent trade prints, newest first.
    pub tape: Vec<TradeRecord>,
    /// The per-price volume footprint as `(price, buy, sell)`, highest first.
    pub footprint: Vec<(f64, f64, f64)>,
    /// The latest funding print, if any.
    pub funding: Option<FundingSnapshot>,
    /// The latest value of each configured indicator (`None` while warming up).
    pub indicators: BTreeMap<String, Option<f64>>,
}

/// The reconstructed state of the whole universe at the seek target.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MarketSnapshot {
    /// The seek target timestamp the snapshot was reconstructed at.
    pub ts: i64,
    /// Per-symbol reconstructed state, keyed and ordered by symbol.
    pub symbols: BTreeMap<String, SymbolSnapshot>,
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::round_to;

    #[test]
    fn round_to_clamps_non_finite_to_zero() {
        assert_eq!(round_to(f64::NAN), 0.0);
        assert_eq!(round_to(f64::INFINITY), 0.0);
        assert_eq!(round_to(f64::NEG_INFINITY), 0.0);
    }

    #[test]
    fn round_to_keeps_eight_places() {
        assert!((round_to(1.234_567_894_2) - 1.234_567_89).abs() < 1e-12);
    }
}
