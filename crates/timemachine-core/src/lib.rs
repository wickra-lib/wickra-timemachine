//! # timemachine-core
//!
//! The deterministic core of Wickra Time Machine: reconstruct the full
//! microstructure state — orderbook, trades and funding for every symbol — of a
//! whole crypto universe at any past moment, by O(1) deterministic **re-fold**
//! over recorded event feeds.
//!
//! This is the scaffold surface. The re-fold engine (`seek(t)` over a
//! multi-symbol universe, built on the `wickra-backtest` replay engine and
//! `wickra-exchange` feeds) lands in the core phase; every language binding will
//! reach it through one JSON-over-C-ABI seam (`command_json`).

/// Returns the crate version string (`CARGO_PKG_VERSION`).
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::version;

    #[test]
    fn version_is_semver_triple() {
        assert_eq!(version().split('.').count(), 3);
    }
}
