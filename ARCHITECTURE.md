# Architecture

Wickra Time Machine is a hybrid product: one data-driven Rust core,
`timemachine-core`, wrapped by a reference CLI, ten language bindings, and a web
scrubber frontend. Everything the Time Machine does is reachable through one
seam — a handle plus a `command_json` string.

## Workspace

```
crates/timemachine-core    the library: recorded universe → seek(t) → re-folded snapshot
crates/timemachine-cli     the wickra-timemachine CLI
crates/timemachine-bench   criterion micro-benchmarks (snapshots/second)
bindings/*                 ten language surfaces (added in the bindings phase)
web/                       the timeline-scrubber frontend (core → WASM + Vue/Vite)
```

## The re-fold core

The Time Machine does not store snapshots. It holds a recorded event feed per
symbol (orderbook deltas, trades, funding — from `wickra-exchange`) and a folding
state built on the `wickra-backtest` replay engine. Seeking to timestamp `t`
**re-folds** the state deterministically from a base point up to `t`: because the
engine advances O(1) per event, the reconstruction scales to a whole
multi-symbol universe.

```
recorded universe (per-symbol event feeds)
        │  seek(t)
        ▼
re-fold each symbol from base → t   (O(1) per event; fanned out across symbols)
        │
        ▼
snapshot = { per symbol: orderbook + tape + funding state }   (BTreeMap, sorted)
```

## Determinism

The reconstructed snapshot is the golden moat: `BTreeMap` everywhere in the
output path, per-symbol maps sorted by symbol key, reductions serial in key order,
and `seek(t)` returning the same snapshot for the same `t`. The result is
byte-identical across all ten language bindings and between the parallel and
single-threaded (WASM) fold paths. Any divergence is a bug.

## The `command_json` seam

Every binding calls the same core surface — a handle plus `command_json` — and
forwards the command string verbatim. Bindings never re-implement the fold; they
marshal strings. That is what makes a seek byte-identical across languages.
