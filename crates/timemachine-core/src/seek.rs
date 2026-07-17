//! The re-fold core: reconstruct the whole universe at any timestamp.
//!
//! [`seek_snapshot`] is stateless — it re-folds every symbol from the start of
//! the recorded stream up to (and including) the target timestamp. Because each
//! symbol's state depends only on its own events in order, the per-symbol folds
//! are independent: the `parallel` feature fans them out across a rayon pool, and
//! the result is **byte-identical** to the single-threaded path because the
//! output is collected into a symbol-sorted [`BTreeMap`] regardless of which
//! thread finishes first.

use std::collections::BTreeMap;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::error::Result;
use crate::event::Record;
use crate::snapshot::{MarketSnapshot, SymbolSnapshot};
use crate::spec::TimelineSpec;
use crate::symbol_state::SymbolState;

/// Reconstruct the universe snapshot at `ts` by re-folding every symbol from the
/// start of `records` up to and including `ts`.
///
/// `records` must be sorted by timestamp (as produced by
/// [`crate::event::parse_records_jsonl`]).
///
/// # Errors
/// Propagates spec validation and indicator-construction failures.
pub fn seek_snapshot(records: &[Record], spec: &TimelineSpec, ts: i64) -> Result<MarketSnapshot> {
    spec.validate()?;

    // Group the in-window records by symbol, preserving per-symbol order.
    let mut groups: BTreeMap<&str, Vec<&Record>> = BTreeMap::new();
    for record in records.iter().filter(|r| r.ts <= ts) {
        groups
            .entry(record.symbol.as_str())
            .or_default()
            .push(record);
    }
    let groups: Vec<(&str, Vec<&Record>)> = groups.into_iter().collect();

    let fold_group = |(symbol, recs): &(&str, Vec<&Record>)| -> Result<(String, SymbolSnapshot)> {
        let mut state = SymbolState::new(spec)?;
        for record in recs {
            state.fold(&record.feed);
        }
        Ok(((*symbol).to_string(), state.snapshot(spec)))
    };

    #[cfg(feature = "parallel")]
    let symbols: BTreeMap<String, SymbolSnapshot> =
        groups.par_iter().map(fold_group).collect::<Result<_>>()?;
    #[cfg(not(feature = "parallel"))]
    let symbols: BTreeMap<String, SymbolSnapshot> =
        groups.iter().map(fold_group).collect::<Result<_>>()?;

    Ok(MarketSnapshot { ts, symbols })
}

#[cfg(test)]
mod tests {
    use super::seek_snapshot;
    use crate::event::parse_records_jsonl;
    use crate::spec::TimelineSpec;

    const FEED: &str = r#"
{"ts":10,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}
{"ts":20,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"110","quantity":"1","aggressor":"Sell","timestamp":20}}
{"ts":20,"symbol":"ETH-USDT","feed":{"kind":"funding","rate":0.0002,"mark_price":50.0}}
{"ts":30,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"120","quantity":"1","aggressor":"Buy","timestamp":30}}
"#;

    #[test]
    fn seek_is_ts_inclusive_and_clamps() {
        let records = parse_records_jsonl(FEED).unwrap();
        let spec = TimelineSpec::default();

        // At ts=20 the BTC last is 110 (the ts=20 trade is included); ETH funding is set.
        let snap = seek_snapshot(&records, &spec, 20).unwrap();
        assert!((snap.symbols["BTC-USDT"].last - 110.0).abs() < 1e-9);
        assert!(snap.symbols["ETH-USDT"].funding.is_some());

        // Before any event: empty universe.
        assert!(seek_snapshot(&records, &spec, 0)
            .unwrap()
            .symbols
            .is_empty());

        // After the last event: last BTC trade is 120.
        let end = seek_snapshot(&records, &spec, 999).unwrap();
        assert!((end.symbols["BTC-USDT"].last - 120.0).abs() < 1e-9);
    }

    #[test]
    fn same_seek_is_deterministic() {
        let records = parse_records_jsonl(FEED).unwrap();
        let spec = TimelineSpec::default();
        let a = seek_snapshot(&records, &spec, 30).unwrap();
        let b = seek_snapshot(&records, &spec, 30).unwrap();
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
    }
}
