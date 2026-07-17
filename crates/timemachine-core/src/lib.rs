//! # timemachine-core
//!
//! The deterministic core of Wickra Time Machine: reconstruct the full
//! microstructure state — order book, trade tape, footprint, funding and scalar
//! indicators for every symbol — of a whole crypto universe at any past moment,
//! by O(1) deterministic **re-fold** over a recorded event stream.
//!
//! A recorded universe is a JSONL stream of [`Record`]s. [`TimeMachine`] loads it
//! and [`TimeMachine::seek`]s to any timestamp; every language binding reaches the
//! same surface through one JSON seam ([`TimeMachine::command_json`]). Seeking to
//! the same timestamp yields a **byte-identical** [`MarketSnapshot`] on every run,
//! in every binding, and between the parallel and single-threaded fold paths.

pub mod config;
pub mod error;
pub mod event;
mod indicator_set;
pub mod seek;
pub mod snapshot;
pub mod spec;
mod symbol_state;
pub mod timemachine;

pub use config::Config;
pub use error::{Error, Result};
pub use event::{parse_records_jsonl, Event, Feed, Record, Symbol};
pub use seek::seek_snapshot;
pub use snapshot::{
    BookLevel, BookSnapshot, FundingSnapshot, MarketSnapshot, SymbolSnapshot, TradeRecord,
};
pub use spec::{IndicatorRef, TimelineSpec};
pub use timemachine::TimeMachine;

/// Returns the crate version string (`CARGO_PKG_VERSION`).
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_semver_triple() {
        assert_eq!(super::version().split('.').count(), 3);
    }
}
