<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-timemachine)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/ci.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codeql.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-timemachine)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/release.svg)](https://github.com/wickra-lib/wickra-timemachine/releases/latest)
[![crates.io](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/crates.svg)](https://crates.io/crates/wickra-timemachine)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/pypi.svg)](https://pypi.org/project/wickra-timemachine/)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/npm.svg)](https://www.npmjs.com/package/wickra-timemachine)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/nuget.svg)](https://www.nuget.org/packages/Wickra.TimeMachine)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-timemachine)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-timemachine-go)
[![R-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-timemachine)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/provenance.svg)](https://github.com/wickra-lib/wickra-timemachine/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/docs.svg)](https://wickra.org)
[![Verified across 10 languages](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/verified.svg)](golden/)

---

# Wickra Time Machine

**Scrub the whole crypto market like a video — every symbol, full orderbook +
trades + funding, rewound to any moment, reconstructed in O(1) via deterministic
re-fold.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the same data-driven core and ten-language binding surface also power [wickra-exchange](https://github.com/wickra-lib/wickra-exchange), [wickra-backtest](https://github.com/wickra-lib/wickra-backtest), [wickra-terminal](https://github.com/wickra-lib/wickra-terminal) and 20 more — see [the full list](https://github.com/wickra-lib).
> Machine folds recorded market feeds from
> [`wickra-exchange`](https://github.com/wickra-lib/wickra-exchange) through the
> [`wickra-backtest`](https://github.com/wickra-lib/wickra-backtest) replay
> engine, so seeking to any past timestamp reconstructs the exact microstructure
> state — no snapshots, no interpolation.

Wickra Time Machine is one data-driven core, `timemachine-core`: point it at a
recorded universe, `seek(t)`, and it re-folds every symbol's orderbook, tape and
funding state deterministically to that instant. Because the engine is O(1) per
event, seeking scales to the whole market. The core is exposed as a
**JSON-over-C-ABI data API** (`command_json`) in **Rust, Python, Node.js, WASM,
C, C++, C#, Go, Java and R**, plus a reference CLI.

```rust
use timemachine_core::TimeMachine;

// A timeline over a recorded universe: ten book levels, a bounded tape, and
// any registry indicator folded on each symbol's trade price.
let mut tm = TimeMachine::new(r#"{
    "book_depth": 10,
    "tape_cap": 64,
    "indicators": [{"name": "Rsi", "params": [14]},
                   {"name": "Macd", "params": [12, 26, 9]}],
    "snapshot_interval": 256
}"#)?;

tm.load(&events_jsonl)?;
let snapshot = tm.seek(1_700_000_000)?;   // the whole market at that instant
```

## Status

Early development (0.1.0, unreleased). The re-fold core, the reference CLI, the
ten-language binding surface, the golden corpus and the full CI matrix are in
place; the first published release is still pending, and the web scrubber
front-end is a later phase.

## How it works

A recorded universe is a JSONL stream of `Record`s: one line per event, each with
a venue timestamp, a symbol key, and a `Feed` payload — a market event (a trade
or an order-book snapshot/delta, re-exported from `wickra-exchange-core`) or a
funding print. A `TimelineSpec` names the book depth, the tape cap, the
indicators to fold on each symbol's trade price, and how often to drop a re-fold
anchor. To `seek(t)`, the core:

1. binary-searches the sorted records for the last event with `ts <= t`;
2. jumps to the nearest anchor at or before that index (bounding backward-seek
   cost to `snapshot_interval`);
3. re-folds the event prefix per symbol — applying book deltas, pushing trades
   onto a bounded tape, advancing the indicator set, tracking funding — fanning
   out across symbols with `rayon`;
4. materialises a `MarketSnapshot`: per symbol, the depth-capped book ladder, the
   recent tape, the footprint, the latest funding, and the indicator values.

`play(from, to, step)` returns one snapshot per step; a `seek(t)` is byte-identical
to the `play` frame that lands on `t`.

## Determinism

Reconstruction is the golden moat: records are held in ordered collections, the
per-symbol maps are `BTreeMap`s, floats are rounded to a fixed grid and non-finite
values collapse to `0.0`, and the snapshot serialises canonically. The same feed +
spec yields a **byte-identical `MarketSnapshot`** on every run, and — because each
binding forwards the command string verbatim — in every language. The
`rayon`-parallel re-fold and the single-threaded (`--no-default-features`, WASM)
path are byte-identical by construction, since each symbol folds independently.

## Quickstart

```bash
# Reconstruct the recorded mini universe at a past timestamp (compact JSON).
wickra-timemachine --dataset golden/data/mini --spec golden/specs/mini.json --seek 1700000600 --format json

# The same seek as a human-readable book ladder, tape and funding per symbol.
wickra-timemachine --dataset golden/data/mini --spec golden/specs/mini.json --seek 1700000600 --format text

# Play a range: one snapshot every step, from an anchor sweep.
wickra-timemachine --dataset golden/data/mini --spec golden/specs/play.json --play 1700000000 1700000700 100 --format json
```

## Use in any language

The same handle + `command_json` + `version` surface ships for Rust, Python,
Node.js, WASM, and — over a C ABI hub — C, C++, C#, Go, Java and R. Each binding
passes the command string through verbatim, so the `MarketSnapshot` they return
is identical.

```python
import json
from wickra_timemachine import TimeMachine

feed = "\n".join(json.dumps(r) for r in records)  # JSONL of {"ts","symbol","feed"}
tm = TimeMachine("{}")
tm.command(json.dumps({"cmd": "load", "data": feed}))
snap = json.loads(tm.command(json.dumps({"cmd": "seek", "ts": 1700000600})))
print(snap["symbols"]["BTC-USDT"]["last"])
```

See [`examples/`](examples/) for the same program in all ten languages.

## Documentation

- [docs/SEEK.md](docs/SEEK.md) — the seek / re-fold pipeline in depth.
- [docs/SNAPSHOTS.md](docs/SNAPSHOTS.md) — the `MarketSnapshot` output shape.
- [docs/DATASETS.md](docs/DATASETS.md) — the recorded-universe wire format.
- [docs/INDICATORS.md](docs/INDICATORS.md) — declaring and folding indicators.
- [docs/DETERMINISM.md](docs/DETERMINISM.md) — why reconstruction is byte-identical.
- [docs/Cookbook.md](docs/Cookbook.md) — task-oriented recipes.
- [ARCHITECTURE.md](ARCHITECTURE.md) — the crates and how they fit together.
- [BENCHMARKS.md](BENCHMARKS.md) — measured throughput and how to reproduce it.
- [THREAT_MODEL.md](THREAT_MODEL.md) — the trust boundary and resource limits.
- [golden/README.md](golden/README.md) — the blessed cross-language corpus.
- Full documentation: [wickra.org](https://wickra.org).

## Project layout

```
crates/timemachine-core   the library: events, spec, per-symbol fold, seek, snapshot
crates/timemachine-cli    the wickra-timemachine CLI
crates/timemachine-bench  criterion micro-benchmarks (snapshots/second)
bindings/*                ten language surfaces (c, python, node, wasm, csharp, go, java, r)
golden/                   feeds + specs + blessed snapshots (the cross-language corpus)
examples/                 one runnable example per language
fuzz/                     libFuzzer targets (spec parse, event fold, seek, command)
```

## Building everything from source

```bash
cargo build --workspace --all-features                 # Rust core + CLI + C ABI
(cd bindings/python && maturin develop --release)      # Python
(cd bindings/node   && npm ci && npm run build)        # Node
(cd bindings/wasm   && wasm-pack build --target web)   # WASM
(cd bindings/csharp && dotnet build)                   # C#
(cd bindings/go     && go build ./...)                 # Go
(cd bindings/java   && mvn -q package)                 # Java
R CMD INSTALL bindings/r                               # R
```

The C-ABI consumers (C/C++, C#, Go, Java, R) need the C ABI library first —
`cargo build --release -p wickra-timemachine-c` — on the loader path.

## Testing

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

Every binding replays the same golden seeks from [`golden/`](golden/) and must
produce the identical bytes; that corpus is the cross-language contract, not a
per-language approximation. `python scripts/check_binding_surface.py` asserts
the ten surfaces stayed in step.

## Benchmarks

The headline figure is **snapshots per second** — the rate at which the Time
Machine re-folds a multi-symbol universe to a target instant. See
[BENCHMARKS.md](BENCHMARKS.md); reproduce with `cargo bench -p timemachine-bench`.

## Requirements

- **Rust 1.86+** — the workspace MSRV; the Node binding needs **Rust 1.88**.
- **Python 3.9+** — the Python binding.
- **Node 22+** — the Node binding.
- **Go 1.23+** — the Go binding.
- **Java 22+** — the Java binding.
- **R 2.10+** — the R package.
- **.NET 8+** — the C# binding.
- A **C11 / C++17** compiler with CMake for the C and C++ examples.

The Time Machine depends on `wickra-core` for the indicator types,
`wickra-exchange` for the event types and `wickra-backtest` for the name ->
indicator registry. All three come from crates.io.

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). The Time
Machine reads recorded market data only — no keys, no order placement — and
folds untrusted feeds under explicit depth/tape/anchor bounds.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — main library (Rust core + Python / Node.js / WASM bindings + a C ABI for C / C++ / C# / Go / Java / R)
- [**wickra-playground**](https://github.com/wickra-lib/wickra-playground) — a polyglot strategy playground: one StrategySpec live side by side in Python, Rust, JS and Go, entirely in the browser
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- [**wickra-radar**](https://github.com/wickra-lib/wickra-radar) — perp-universe alert radar: OI delta, funding flip, book imbalance, liquidation clusters, OI/price divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history
- [**wickra-benchmark**](https://github.com/wickra-lib/wickra-benchmark) — reproducible, golden-verified benchmark suite — recompute any (strategy, dataset, report) in ten languages and confirm it byte-for-byte
- [**wickra-strategy-ci**](https://github.com/wickra-lib/wickra-strategy-ci) — Jest for trading strategies: golden-pin the report, catch regressions in CI, property-test against fuzzed data
- [**wickra-verify**](https://github.com/wickra-lib/wickra-verify) — confirm or refute a claimed backtest report against its strategy and data, in ten languages
- [**wickra-proof**](https://github.com/wickra-lib/wickra-proof) — Proof-of-Backtest: deterministic (spec, data) → report + blake3 hash, recomputable byte-for-byte in ten languages
- [**wickra-zk**](https://github.com/wickra-lib/wickra-zk) — prove a backtest zero-knowledge — on-chain-verifiable performance without revealing the data or the strategy
- [**wickra-impact**](https://github.com/wickra-lib/wickra-impact) — the backtester that knows you would have moved the market: agent-based fills on the real historical L2 order book
- [**wickra-darwin**](https://github.com/wickra-lib/wickra-darwin) — evolutionary strategy search at millions of backtests per second, mutating and crossing JSON specs across the 514-indicator space
- [**wickra-gym**](https://github.com/wickra-lib/wickra-gym) — a Gymnasium-compatible, microstructure-aware backtest environment with O(1) steps for deterministic RL rollouts
- [**wickra-feature-store**](https://github.com/wickra-lib/wickra-feature-store) — OHLCV and microstructure streams into ML-ready feature matrices over 514 O(1) streaming indicators
- [**wickra-genome**](https://github.com/wickra-lib/wickra-genome) — a vector database of the whole market: every asset a 514-dim live vector, for similarity search, clustering and anomaly detection
- [**wickra-synth**](https://github.com/wickra-lib/wickra-synth) — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed
- [**wickra-compile**](https://github.com/wickra-lib/wickra-compile) — compile a strategy spec into a standalone deployable: a WASM module, a self-contained binary, or a `no_std` artifact
- [**wickra-embed**](https://github.com/wickra-lib/wickra-embed) — allocation-free, `no_std` streaming indicators for bare-metal and HFT, byte-for-byte identical to the core
- [**wickra-pico**](https://github.com/wickra-lib/wickra-pico) — the O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross

The screener's own guides live in [`docs/`](docs/) beside the code; its site,
with the in-browser demo and the benchmark figures, is at
[screener.wickra.org](https://screener.wickra.org). The indicator library's
reference is at [docs.wickra.org](https://docs.wickra.org) and the org landing
page at [wickra.org](https://wickra.org).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-timemachine">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-timemachine/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-timemachine/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-timemachine star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/star-history.svg">
</p>

## Disclaimer

Wickra Time Machine is a research tool, provided "as is" without warranty of any
kind. It reconstructs recorded market microstructure for analysis; nothing here is
financial advice, and trading carries risk of loss.
