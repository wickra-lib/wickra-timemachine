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

Each `IndicatorRef` resolves through the same registry the `wickra-backtest`
engine uses — the names and parameter order are identical, so an indicator
behaves the same here as in a backtest. An unknown `name`, or a parameter set the
registry rejects, is refused at construction with `UnknownIndicator`.

The registry validates the name and its parameters together, so multi-parameter
indicators work as they do everywhere else:

```json
{"name": "Macd", "params": [12, 26, 9]}
```

### What the re-fold cannot drive

The fold sees trade prints, not OHLC candles, and hands the indicators the print
alone: each is widened onto a flat bar — open, high, low and close all the trade
price, the volume its size — which is what a single trade's bar is.

Everything the bar alone drives therefore works. The families that read something
else do not, and are **refused by name** rather than accepted and left returning
nothing for ever:

| Family | What it would need | Example |
| --- | --- | --- |
| pairwise | a reference series | `Beta`, `PearsonCorrelation` |
| order book | an order-book snapshot | `Microprice` |
| trade flow | the individual trades of a bar | `CumulativeVolumeDelta` |
| quote-relative flow | trades quoted against the book | `EffectiveSpread` |
| derivatives | a derivatives tick | `FundingRate` |
| breadth | the market cross-section | `AdvanceDecline` |

The Time Machine does reconstruct an order book and a trade tape — they are
snapshot fields of their own — but it does not feed them to the indicators, so
the refusal is honest rather than a limitation of the data:

```
Microprice reads an order-book snapshot, which the re-fold does not hand the
indicators; it would return nothing on every event
```

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
