# The market snapshot

`seek` and each `play` frame return a `MarketSnapshot` — the full reconstructed
microstructure state of the universe at one instant. It is the only output type,
and it is **serialise-only**: bindings forward its canonical JSON verbatim, so
there is nothing to deserialise and no per-language reformatting.

## Shape

```
MarketSnapshot
├── ts: i64                          the reconstructed instant
└── symbols: BTreeMap<String, SymbolSnapshot>   ordered by symbol key
        ├── last: f64                last trade price (0.0 before the first trade)
        ├── book: BookSnapshot
        │     ├── bids: [BookLevel]  depth-capped, best first
        │     ├── asks: [BookLevel]  depth-capped, best first
        │     └── spread: f64        best ask − best bid
        ├── tape: [TradeRecord]      most-recent trades, ≤ tape_cap
        ├── footprint: [(price, buy_vol, sell_vol)]   volume-at-price
        ├── funding: FundingSnapshot | null          latest funding, if any
        └── indicators: BTreeMap<String, f64 | null> one entry per IndicatorRef
```

`BookLevel` is `{ price, quantity }`; `TradeRecord` carries price, quantity, side
and timestamp. Indicator keys are the `IndicatorRef::key()` strings (e.g.
`"Sma(20)"`); a value is `null` while the indicator is still in warmup.

## Canonical form

Determinism (see [DETERMINISM.md](DETERMINISM.md)) is enforced at
materialisation:

- **Ordered maps.** `symbols` and `indicators` are `BTreeMap`s, so key order in
  the JSON is fixed.
- **Rounded floats.** Prices, quantities and indicator values pass through
  `round_to` onto a fixed decimal grid, so tiny IEEE-754 differences between the
  parallel and single-threaded fold paths cannot change a byte.
- **Non-finite collapse.** A `NaN` or infinity collapses to `0.0` before
  serialisation, so no platform's float edge case leaks into the output.

The compact `serde_json::to_string` form — no trailing newline — is what the CLI
prints, what `command_json` returns, and what every binding emits. The golden
corpus asserts this byte-for-byte across all ten languages.
