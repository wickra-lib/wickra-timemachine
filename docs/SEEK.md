# Seek and re-fold

`seek(t) -> MarketSnapshot` is the whole product. This document walks the re-fold
pipeline; the output it materialises is described in [SNAPSHOTS.md](SNAPSHOTS.md),
the wire format it consumes in [DATASETS.md](DATASETS.md), and the guarantee it
upholds in [DETERMINISM.md](DETERMINISM.md).

## Inputs

- The **loaded universe** — a time-ordered `Vec<(String, Event)>` built by
  `load(jsonl)` from a JSONL stream of `Record`s. Records are sorted stably by
  `(ts, sequence)`, so the fold order is fixed regardless of input order.
- The **`TimelineSpec`** — the reconstruction request:

  | Field               | Meaning |
  |---------------------|---------|
  | `book_depth`        | levels kept per side of the reconstructed ladder. |
  | `tape_cap`          | most-recent trades retained per symbol. |
  | `indicators`        | the `IndicatorRef` set folded on each symbol's trades. |
  | `snapshot_interval` | events between re-fold anchors (bounds backward seek). |

## The pipeline

To `seek(t)`:

1. **Locate.** Binary-search the sorted events for the last index whose `ts <= t`.
   Ties on `ts` are inclusive — a seek that lands exactly on an event sees it.
   A `t` before the first event yields an empty snapshot; a `t` past the last
   clamps to the end.
2. **Anchor.** Jump to the nearest anchor at or before that index. Anchors are
   dropped every `snapshot_interval` events at load time, so a backward seek never
   re-folds more than one anchor window.
3. **Re-fold.** Replay the event prefix `[anchor, target]` per symbol: apply book
   snapshots and deltas (a zero-quantity level removes it), push trades onto the
   bounded tape and footprint, set the latest funding, and advance the indicator
   set. Symbols fold independently, so the `parallel` feature fans the per-symbol
   folds across a `rayon` pool.
4. **Materialise.** Collect each symbol's state into a `SymbolSnapshot` and gather
   them into a `MarketSnapshot { ts, symbols }` in `BTreeMap` order.

## Forward stepping and `play`

`play(from, to, step)` returns one snapshot per step. Rather than re-folding from
an anchor every step, it keeps a cursor `(index, Universe)` and folds *forward*
from the previous step when the next target is ahead — the common case in a sweep.
A seek that moves backward re-folds from the anchor. Either path produces the same
state: a `seek(t)` is byte-identical to the `play` frame that lands on `t` — the
core determinism assertion, pinned by `tests/seek_eq_play.rs`.

## Complexity

Folding one event is O(1) — a book delta touches one level, a trade pushes one
bounded ring entry, an indicator advances one step. A seek therefore costs
O(events in the anchor window) backward and O(events since the cursor) forward,
never O(whole history). The `snapshot_interval` is the knob that trades memory
(more anchors) for bounded seek latency.
