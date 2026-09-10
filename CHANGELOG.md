# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **Every C++ hull used the include guard `WICKRA_SCREENER_HPP`.** The C headers
  beside them are guarded correctly; only the `.hpp` files shared one name, so
  including two of the family's headers in the same translation unit dropped the
  second silently. Proven by compiling a file that includes two of them and
  names a class from each: `'Env' is not a member of 'wickra'`. All seven now
  compile standalone and together.

- **The hull's usage example could not run.** It showed a spec shaped
  `{"universe":[...]}` and `{"cmd":"scan"}`, the screener's, which this core
  rejects twice over. It now shows this repository's own spec fields and one of
  its own commands.

- **The release notes named the wrong package.** They told a reader `dotnet add
  package Wickra.Darwin` and `install.packages("wickrafeaturestore")`, where
  this package is `Wickra.TimeMachine` and `wickratimemachine`. These notes go
  out with the GitHub release: a reader following them installs a different
  library.

- **The issue and pull-request templates asked for a `ScanSpec`**, a type this
  repository does not have, so a contributor was asked to attach something that
  does not exist. `GOVERNANCE.md`, `SUPPORT.md` and `CONTRIBUTING.md` carried
  the same substitution, along with the screener's "condition schema" for a core
  that has no conditions.

- **The R `configure` scripts still defined `wkscreen_download`**, the last
  trace of the screener's prefix — the CI-visible half of which already had to
  be fixed once.

- **Six SHA-pinned actions sat on two lines across the family**, and two of the
  splits were inside this repository. `actions/setup-node` is pinned at the same
  commit everywhere, but some call sites annotated it `# v6.4.0`; GitHub's tag
  list says that commit is **v7.0.0** and v6.4.0 is a different one. Dependabot
  reads that comment to decide what to bump, so a wrong one misdirects the tool
  meant to keep the pin current. `Swatinem/rust-cache` ran at two commits at
  once, the older behind a floating `# v2`. Every pin now matches what the
  sibling repositories run, each target checked against the upstream tag list.

- **The CI Java example step compiled a file that is not there.** The `examples`
  job was ported from the screener, whose Java example is a single
  `examples/java/Scan.java` built with `javac`. This repository ships a Maven
  project instead, so the step compiled a missing file and then asserted on
  output the example never prints. It now builds the binding into the local
  repository and runs the example through `mvn exec:exec`, the way the example's
  own javadoc documents -- verified by running it.

- **Dependabot watched directories that do not exist**, so it reported nothing
  and the silence read as calm. An `npm` entry watched a `/web` Vue/Vite
  renderer this repository does not have; `pip` did not cover
  `/.github/requirements` and `npm` did not cover `/examples/node`.

- **The workspace's own core was pinned as a range.** `timemachine-core` was
  named six times as `version = "0.1"` -- a caret range -- and the root manifest
  carried no `[workspace.dependencies]` entry for it at all. A published
  `timemachine-cli` 0.1.0 would have accepted `timemachine-core` 0.1.99, a crate
  resolving against a core it was never built against, in a workspace whose
  whole point is that the pieces move together. It also hid the line from
  `bump_version.py` and `check_version_sync.py`, both of which look for the
  exact version.

- **`release.yml` overwrote the binding READMEs before packing.** Three steps
  copied the root README over `bindings/python/README.md` (wheel and sdist) and
  `bindings/node/README.md`. They date from when the bindings had no README of
  their own; they do now, one per registry, and `check_readme_links.py` exists to
  keep their links absolute because a relative link is dead on PyPI and npm. The
  copy threw that away and shipped the root README, whose links are relative by
  design. The remaining relative links in the C, C#, Go and WASM READMEs are
  absolute now.

- **The Python wheel would have shipped without its licence texts.**
  `bindings/python/` carried neither `LICENSE-MIT` nor `LICENSE-APACHE`, so
  maturin had nothing to include, while every crate and the release archive
  carry both.

- **`SECURITY.md` named a support policy for releases that do not exist yet.**
  It promised fixes for "the latest `0.x` release line" where there is no
  released line; it now says plainly that nothing is published and names `0.1.0`
  as the first version that will be.

- **The bench could not measure the sequential path it advertises.** It took the
  core with default features on -- and `default = ["parallel"]` -- so its own
  `parallel` feature was a no-op and `--no-default-features` changed nothing. In
  the feature store the manifest comment even said "default features off
  (inherited from the workspace edge)", which the workspace edge did not do.

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
