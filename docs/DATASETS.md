# Recorded datasets

The Time Machine reconstructs state from a **recorded universe**: a JSONL stream
where each line is one `Record`. The format is the wire contract the loader
(`parse_records_jsonl`) and every binding agree on.

## Record

```json
{"ts": 1700000600, "symbol": "BTC-USDT", "feed": { ... }}
```

- `ts: i64` — the venue timestamp the event is ordered and sought by.
- `symbol: String` — the universe key the event folds into.
- `feed: Feed` — the payload, internally tagged by `kind` (snake_case).

## Feed

```json
{"kind": "market", "type": "trade", ...}          // a market event
{"kind": "funding", "rate": 0.0001, "mark_price": "30000"}   // a funding print
```

A `market` feed wraps a `wickra_exchange_core::Event`, internally tagged by
`type`:

| `type`          | Event               | Reconstructs |
|-----------------|---------------------|--------------|
| `trade`         | `TradePrint`        | last price, tape, footprint |
| `book_snapshot` | `OrderBookSnapshot` | the full ladder (resets it) |
| `book_delta`    | `BookDelta`         | ladder updates (qty 0 removes a level) |

`TradePrint` is `{symbol, price, quantity, aggressor, timestamp}`; the book events
carry `bids`/`asks` as `{price, quantity}` levels. Prices and quantities are
`Decimal`, serialised **as strings**, so no precision is lost on the wire.
`Symbol` serialises as `{base, quote}` and `OrderSide` as PascalCase `Buy`/`Sell`.

## Ordering

Records may arrive in any order; `load` sorts them stably by `(ts, sequence)`
before folding. Two events with the same `ts` keep their input order, which fixes
the fold sequence and therefore the reconstructed state.

## Datasets on disk

A CLI `--dataset <dir>` is a directory holding `events.jsonl` (and optionally a
`spec.json`). The [`golden/`](../golden/) corpus is the reference: small,
hand-verified feeds whose blessed snapshots pin the engine's output. See
[golden/README.md](../golden/README.md) for the feed formulas and the bless
command.
