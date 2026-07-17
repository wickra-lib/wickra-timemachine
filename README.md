<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![CI](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml/badge.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![CodeQL](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/codeql.yml/badge.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/codeql.yml)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-timemachine)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-timemachine)
[![Deterministic across 10 languages](https://img.shields.io/badge/deterministic%20across-10%20languages-3b82f6)](#use-in-any-language)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

---

# Wickra Time Machine

**Scrub the whole crypto market like a video — every symbol, full orderbook +
trades + funding, rewound to any moment, reconstructed in O(1) via deterministic
re-fold.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the Time
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

- [ARCHITECTURE.md](ARCHITECTURE.md) — the crates and the re-fold pipeline.
- [BENCHMARKS.md](BENCHMARKS.md) — snapshots per second, and how to reproduce them.
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

## Building from source

```bash
cargo build
cargo test
```

## Benchmarks

The headline figure is **snapshots per second** — the rate at which the Time
Machine re-folds a multi-symbol universe to a target instant. See
[BENCHMARKS.md](BENCHMARKS.md); reproduce with `cargo bench -p timemachine-bench`.

## Requirements

- Rust 1.86+ (MSRV). The Time Machine depends on `wickra-core` (crates.io) and,
  as git dependencies, the `wickra-exchange` feeds and the `wickra-backtest`
  replay engine.

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). The Time
Machine reads recorded market data only — no keys, no order placement — and
folds untrusted feeds under explicit depth/tape/anchor bounds.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Disclaimer

Wickra Time Machine is a research tool, provided "as is" without warranty of any
kind. It reconstructs recorded market microstructure for analysis; nothing here is
financial advice, and trading carries risk of loss.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
