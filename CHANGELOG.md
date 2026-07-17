# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the Rust workspace
  (`timemachine-core`, `timemachine-cli`, `timemachine-bench`), and the
  `wickra-core` / `wickra-exchange` / `wickra-backtest` dependencies (state,
  recorded feeds and the O(1) replay engine the Time Machine re-folds over).
- `timemachine-core`: the deterministic re-fold engine — the `Record` / `Feed`
  wire format, the `TimelineSpec` (book depth, tape cap, indicator set, anchor
  interval), per-symbol event folding with bounded tape and footprint, the
  `seek` / `play` re-fold (anchored binary search, `rayon` symbol fan-out), and
  the canonical `MarketSnapshot` output.
- `wickra-timemachine` CLI over the core (`--dataset`, `--spec`, `--seek`,
  `--play`, `--format json|text`).
- Ten language bindings (Rust, Python, Node.js, WASM natively; C, C++, C#, Go,
  Java, R over a C ABI hub), each forwarding `command_json` verbatim for a
  byte-identical snapshot.
- Golden corpus (recorded feeds + specs + blessed snapshots) and the test suite
  (conformance, golden replay, `seek`/`play` equivalence, proptest invariants),
  fuzz targets and the criterion benchmark crate.
- Runnable examples in every language and the full CI/CD matrix (fmt, clippy,
  tests on 3 OS × 2 feature sets, MSRV, coverage, cargo-deny, the ten-language
  jobs, CodeQL, Scorecard, zizmor, link and metadata checks) plus a USER-GO-gated
  release pipeline.
- Documentation: `README`, per-binding READMEs, and the top-level design docs
  (`ARCHITECTURE.md`, `THREAT_MODEL.md`, `BENCHMARKS.md`).

[Unreleased]: https://github.com/wickra-lib/wickra-timemachine/commits/main
