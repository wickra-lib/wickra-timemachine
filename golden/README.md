# Golden corpus

Fixed, deterministic recorded feeds plus the byte-exact `MarketSnapshot` JSON
that seeking (or playing) them must reproduce. The corpus is the cross-language
contract: the Rust core, the CLI and every language binding must all emit these
exact bytes, so `tests/golden.rs` and each binding's golden test assert equality
against these files.

## Layout

```
golden/
├── data/
│   ├── mini/events.jsonl    # one symbol (AAA-USDT), 8 records
│   └── multi/events.jsonl   # two symbols (AAA-USDT, BBB-USDT), interleaved
├── specs/
│   ├── mini.json            # Sma(3), snapshot_interval 4
│   ├── funding.json         # no indicators — exercises a funding print
│   ├── play.json            # Ema(3) — used with --play
│   ├── multi_symbol.json    # Sma(2) over the multi dataset
│   └── anchor_reseek.json   # Rsi(2), snapshot_interval 1 (dense anchors)
└── expected/
    └── <spec>.json          # blessed MarketSnapshot (or frames array for play)
```

## Feed format

Each line of `events.jsonl` is one JSON `Record`:

```json
{ "ts": <i64 ms>, "symbol": "<key>", "feed": <Feed> }
```

`feed` is either a market event (`"kind":"market"`, wrapping the exchange
crate's `type`-tagged `Event` — `book_snapshot`, `book_delta`, `trade`, …) or a
funding print (`"kind":"funding"`, `rate` + `mark_price`). Records are sorted by
`ts` with a stable sort, so equal timestamps keep their line order.

The `mini` feed (single symbol, `ts` 1700000000 … 1700000700): an opening book
snapshot, a buy trade, a delta that removes the 100.5 ask and adds a 100.1 bid, a
sell trade, a delta adding the 100.6 ask, a buy trade, a delta adding the 100.3
bid, and a closing funding print. Seeking to `1700000600` folds the first seven
records: the book has bids `[100.3, 100.1, 100.0]` and ask `[100.6]`, the last
trade is `100.3`, and `Sma(3)` over the three trade prints is `100.3`.

## Blessing (regenerating expected/)

The expected files are produced by the CLI in its default (compact) JSON
rendering — byte-identical to the core's `command_json` and to every binding —
then committed verbatim:

```bash
cargo build -p timemachine-cli --release
BIN=target/release/wickra-timemachine

$BIN --dataset golden/data/mini  --spec golden/specs/mini.json         --seek 1700000600 --format json > golden/expected/mini.json
$BIN --dataset golden/data/mini  --spec golden/specs/funding.json      --seek 1700000700 --format json > golden/expected/funding.json
$BIN --dataset golden/data/mini  --spec golden/specs/play.json         --play 1700000000 1700000700 100 --format json > golden/expected/play.json
$BIN --dataset golden/data/multi --spec golden/specs/multi_symbol.json --seek 1700000600 --format json > golden/expected/multi_symbol.json
$BIN --dataset golden/data/mini  --spec golden/specs/anchor_reseek.json --seek 1700000500 --format json > golden/expected/anchor_reseek.json
```

**Never edit the `expected/` files by hand.** They are machine-generated and
have no trailing newline (they match `serde_json::to_string` exactly). If the
core's snapshot shape changes intentionally, re-bless with the commands above and
review the diff.
