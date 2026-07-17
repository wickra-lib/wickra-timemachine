# Determinism

The whole value of the Time Machine is that a reconstruction is **reproducible**:
the same recorded universe and the same `TimelineSpec` yield a byte-identical
`MarketSnapshot` on every run, on every machine, and in every language binding.
This document explains why.

## The moat

1. **State is a pure fold.** A snapshot is a deterministic fold of the event
   prefix — no wall-clock, thread id, address or RNG ever feeds a decision. The
   only inputs are the recorded events and the spec.
2. **Fixed event order.** `load` sorts records stably by `(ts, sequence)`, so the
   fold sequence is fixed no matter how the JSONL was ordered on disk.
3. **Ordered collections.** The universe is a `BTreeMap<String, SymbolState>` and
   the snapshot's `symbols` and `indicators` maps are `BTreeMap`s, so iteration
   and serialisation order are fixed.
4. **Rounded, finite floats.** Prices, quantities and indicator values are rounded
   onto a fixed grid by `round_to`, and any `NaN`/infinity collapses to `0.0`
   before serialisation. IEEE-754 edge cases can never change a byte.
5. **Canonical serialisation.** The compact `serde_json::to_string` form (no
   trailing newline) is the single output shape the CLI, `command_json` and every
   binding emit.

## Parallel ≡ sequential

The `parallel` feature (default) folds symbols across a `rayon` pool. Because each
symbol folds **independently** — one symbol's state never depends on another —
the set of per-symbol states is identical whether computed on one thread or many.
They are then gathered into the `BTreeMap` in key order. So:

```
seek(t)  with --features parallel   ≡   seek(t)  with --no-default-features
```

byte-for-byte. The WASM binding builds `--no-default-features` (single-threaded)
and produces the same bytes as the parallel native build — pinned by
`tests/seek_eq_play.rs`, which runs under both feature sets.

## Cross-language equivalence

Every binding forwards the core's `command_json` string **verbatim** — no binding
parses and re-emits the JSON. So the golden corpus asserts one blessed snapshot
byte-for-byte against Rust, Python, Node.js, WASM, C, C++, C#, Go, Java and R.
There is no per-language float formatting to drift.

## Seek ≡ play

A `seek(t)` and the `play` frame that lands on `t` reconstruct the same state by
two paths — anchored re-fold versus forward stepping (see [SEEK.md](SEEK.md)) —
and the test suite asserts they are byte-identical. That equivalence is what makes
a timeline scrub trustworthy: dragging to an instant shows exactly what stepping
to it would.
