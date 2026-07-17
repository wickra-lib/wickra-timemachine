//! The recorded-event wire format.
//!
//! A recorded universe is a JSONL stream: one [`Record`] per line, each carrying
//! an explicit timestamp, a symbol key, and a [`Feed`] payload. The market feed
//! re-exports [`wickra_exchange_core::Event`] verbatim (its own `type`-tagged
//! serde representation), so trades and order-book updates round-trip exactly.
//!
//! `Event` itself carries neither a uniform top-level timestamp nor funding, so
//! the Time Machine wraps it: `ts` gives every record a stable sort key, and the
//! [`Feed::Funding`] variant carries perpetual funding, which is not an `Event`
//! variant in the exchange crate.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

pub use wickra_exchange_core::{Event, Symbol};

/// A recorded feed payload for one symbol at one instant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Feed {
    /// A market event (trade, order-book snapshot/delta, …) from the exchange crate.
    Market(Event),
    /// A perpetual funding print (not an exchange `Event` variant).
    Funding {
        /// The funding rate for the interval (may be negative).
        rate: f64,
        /// The mark price at the funding print.
        mark_price: f64,
    },
}

/// One recorded line: a timestamped, symbol-tagged feed payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// The venue timestamp (milliseconds since the Unix epoch).
    pub ts: i64,
    /// The symbol key (e.g. `"BTC-USDT"`).
    pub symbol: String,
    /// The feed payload.
    pub feed: Feed,
}

/// Parse a JSONL recorded universe into records, sorted by `ts` with the
/// original line order preserved for equal timestamps (a stable sort, so the
/// per-instant sequence is deterministic).
///
/// # Errors
/// Returns [`Error::Parse`] if any non-blank line is not a valid [`Record`].
pub fn parse_records_jsonl(input: &str) -> Result<Vec<Record>> {
    let mut records = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let record: Record =
            serde_json::from_str(line).map_err(|e| Error::Parse(format!("line {}: {e}", i + 1)))?;
        records.push(record);
    }
    records.sort_by_key(|r| r.ts);
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::{parse_records_jsonl, Feed};

    #[test]
    fn parse_sorts_stably_by_ts() {
        let jsonl = r#"
{"ts":30,"symbol":"BTC-USDT","feed":{"kind":"funding","rate":0.0001,"mark_price":100.0}}
{"ts":10,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}
{"ts":10,"symbol":"ETH-USDT","feed":{"kind":"funding","rate":0.0,"mark_price":50.0}}
"#;
        let records = parse_records_jsonl(jsonl).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].ts, 10);
        // Equal ts keeps original order: the BTC trade line preceded the ETH one.
        assert_eq!(records[0].symbol, "BTC-USDT");
        assert_eq!(records[1].symbol, "ETH-USDT");
        assert_eq!(records[2].ts, 30);
        assert!(matches!(records[2].feed, Feed::Funding { .. }));
    }

    #[test]
    fn blank_lines_are_skipped() {
        assert!(parse_records_jsonl("\n\n  \n").unwrap().is_empty());
    }

    #[test]
    fn bad_line_errors_with_line_number() {
        let err = parse_records_jsonl("{not json}").unwrap_err();
        assert!(err.to_string().contains("line 1"));
    }
}
