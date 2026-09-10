# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **The README said seeking reconstructs state with "no snapshots"** while
  `TimelineSpec::snapshot_interval` drops a re-fold anchor every 256 events by
  default. The anchors are real; what is true is that none is ever handed back
  as the answer and nothing is interpolated — they only bound how far a backward
  seek has to replay.

- **`CITATION.cff` described the wrong project.** The abstract and keywords were
  the terminal's, describing a TUI and a web front-end for a replay engine that
  has neither. `CITATION.cff` is what GitHub's citation box and Zenodo quote
  back at a reader as the project's own words, so it is the one file where a
  wrong description is the project saying it.

- **The Ecosystem section repeated two claims their own repositories had already
  corrected**: DARWIN at "millions of backtests per second" across "the
  514-indicator space", where its benchmark says hundreds of thousands over the
  registry, and GENOME as "a 514-dim live vector", where the dimension is
  whatever the spec's feature list names.

- **Indicator names were a three-name allowlist while the docs promised the
  registry.** `docs/INDICATORS.md` said each `IndicatorRef` "resolves ...
  through the same registry the `wickra-backtest` engine uses"; the code was
  `matches!(name, "Sma" | "Ema" | "Rsi")`. `Wma`, `Dema`, `Roc`, `Macd`, `Atr`
  and every other registry name were rejected as unknown. The spec validation
  carried a second artefact of the same list — "exactly one parameter (period)"
  — so a multi-parameter name was unreachable twice over.

- **Three documentation links pointed at pages that never existed here.**
  `docs/PANELS.md`, `docs/SOURCES.md` and `docs/RENDERERS.md`, along with a
  CONTRIBUTING section describing a `Panel` trait, a `PanelView` enum, a TUI and
  a Web front-end — all of it belongs to wickra-terminal. This is what has been
  failing the link check on `main`.

- **`CMAKE_CXX_STANDARD` asked for C++14** while the C++ hull requires C++17.
  Nothing compiled it, so nothing found out.

### Added

- **Registry resolution.** Any name the backtester can resolve works here, with
  the same parameters and the same semantics. The re-fold sees trade prints, so
  each is widened onto a flat bar — open, high, low and close the trade price,
  the volume its size, which is what a single trade's bar is.

- **A refusal for the names the re-fold cannot drive.** The pairwise,
  order-book, trade-flow, quote-relative and breadth families are refused with
  the feed they would have needed, rather than accepted and left returning
  nothing for ever.

- **Play-equals-repeated-seek tests in every binding.** Python, Node, Go, Java,
  C#, R, WASM and C each ask for the same instants both ways and compare. That
  equality is the whole claim of the engine, and no binding checked it.

- **A golden test for the C binding**, which had none: all five committed cases,
  byte-identical, with a guard that fails if the corpus grows a case the test
  does not cover.

- The blueprint scaffold: `LICENSES/`, `docs/README.md`, the five long-form
  issue templates, the CodeQL config, the actionlint and CodSpeed workflows, the
  five check scripts, a C++ hull, licence copies in every published crate and
  npm package, a WASM example, hash-locked pip requirements for both Python
  rows, and dependabot coverage for the manifests nothing reached.

- CI gains `osv`, `links`, `binding-surface`, `semver`, `fuzz-smoke`,
  `examples` and `python-wheel-container-smoke`; the release pipeline gains the
  `gate` and `guard` jobs, provenance over the nupkg, jar and C ABI archives, a
  Maven artifact on the release page, and a Go mirror that builds before it
  publishes.

### Changed

- **The family pins move to the published releases.** Everything comes from
  crates.io rather than git revs, and `wickra-core` rises from 0.9 to 1.0 so the
  tree carries one set of indicator types rather than two that share none.

- **The event wire format gains a venue timestamp on book snapshots and
  deltas**, which `wickra-exchange-core` 0.1.3 carries and the pinned rev did
  not. The fixtures gain the field; the blessed snapshots are unchanged, which
  is what says the reconstruction itself did not move.

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
