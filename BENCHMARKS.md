# Benchmarks

The headline figure is **snapshots per second** — the rate at which the Time
Machine re-folds a recorded multi-symbol universe to reconstruct the full
microstructure state at a seek target.

Reproduce with:

```bash
cargo bench -p timemachine-bench
```

## Measured throughput

Criterion `seek` group, single machine (Windows, release build, default
`parallel` feature). Each `seek` reconstructs the full-universe `MarketSnapshot`
by re-folding every recorded event in the prefix; throughput counts events folded
(`symbols × events/symbol`) over wall-clock:

| symbols | events/symbol | events folded | seek latency | fold throughput |
|--------:|--------------:|--------------:|-------------:|----------------:|
| 10      | 1,000         | 10,000        | ~1.6 ms      | ~6.1 M events/s |
| 10      | 10,000        | 100,000       | ~16.5 ms     | ~6.1 M events/s |
| 100     | 1,000         | 100,000       | ~16.5 ms     | ~6.0 M events/s |
| 100     | 10,000        | 1,000,000     | ~182 ms      | ~5.5 M events/s |

The fold rate holds at **~5.5 M–6.1 M events/second** whether the universe is 10
symbols or 100, because each symbol folds independently and the cost is governed
by total events in the prefix, not by symbol count. A `seek` to the head of a
one-million-event universe therefore reconstructs the whole market in well under a
quarter second; a `play` sweep amortises this by anchoring, reusing the folded
prefix between steps (`play_10sym_1000ev_step100`: 11 full-universe snapshots in
~150 ms).

## Method notes

- Numbers are indicative of relative scaling, not a hardware datasheet — absolute
  values depend on CPU, event count and the indicator set in the spec.
- The `parallel` (rayon) and single-threaded (`--no-default-features`, WASM) paths
  produce **byte-identical** snapshots; parallelism affects wall-clock only.
- `snapshot_interval` bounds backward-seek cost: a `seek` never re-folds more than
  one anchor window, so seeking near a recent anchor is far cheaper than the
  head-of-universe figures above.
- Re-bless these figures when `wickra-exchange` / `wickra-backtest` are bumped,
  since engine changes move the per-event fold cost.
