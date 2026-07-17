# Indicators

Alongside the reconstructed book and tape, the Time Machine folds a configurable
**indicator set** on each symbol's trade stream, so a snapshot carries the same
indicator values a live consumer would have seen at that instant.

## Declaring indicators

The `TimelineSpec` lists them as `IndicatorRef`s:

```json
{
  "book_depth": 10,
  "tape_cap": 64,
  "indicators": [
    {"name": "Sma", "params": [20]},
    {"name": "Ema", "params": [12]},
    {"name": "Rsi", "params": [14]}
  ],
  "snapshot_interval": 256
}
```

Each `IndicatorRef` resolves to a `wickra_core::Indicator` through the same
registry the `wickra-backtest` engine uses — the names and parameter order are
identical, so an indicator behaves the same here as in a backtest. An unknown
`name` is rejected at construction with `UnknownIndicator`.

## Keys

`IndicatorRef::key()` renders a stable label from the name and params —
`Sma(20)`, `Ema(12)`, `Rsi(14)` — and that key is how the value appears in the
snapshot's `indicators` map. Keys are unique within a spec; declaring the same
indicator twice is a `BadSpec`.

## Folding and warmup

On every `trade` event the fold advances each indicator by one step with the trade
price. Until an indicator has seen enough samples to produce a value it is in
**warmup**, and its entry in the snapshot is `null` (JSON `null`, `None` in Rust,
`NaN`/`None`/`null` in the bindings). Once warm, the entry is the rounded value.

Because the fold is deterministic and O(1) per event, adding indicators changes
the snapshot contents but not the re-fold complexity — a seek still costs only the
events in the anchor window (see [SEEK.md](SEEK.md)).
