# Cookbook

Short recipes against the Time Machine surface. Every binding exposes the same
handle — construct with a spec, `load` a recorded feed, then `seek` / `play` — so
these translate directly across languages. See [`examples/`](../examples/) for a
runnable program per language.

## Reconstruct one instant (CLI)

```bash
wickra-timemachine \
  --dataset golden/data/mini --spec golden/specs/mini.json \
  --seek 1700000600 --format json
```

`--format text` prints the same seek as an aligned book ladder, tape and funding
per symbol.

## Sweep a range

```bash
wickra-timemachine \
  --dataset golden/data/mini --spec golden/specs/play.json \
  --play 1700000000 1700000700 100 --format json
```

One `MarketSnapshot` per step. Because `play` steps a cursor forward, a full sweep
costs little more than a single seek to the end.

## Drive it from code (Python)

```python
import json
from wickra_timemachine import TimeMachine

spec = {"book_depth": 10, "tape_cap": 64,
        "indicators": [{"name": "Sma", "params": [20]}],
        "snapshot_interval": 256}
tm = TimeMachine(json.dumps(spec))
tm.command(json.dumps({"cmd": "load", "data": jsonl_feed}))

snap = json.loads(tm.command(json.dumps({"cmd": "seek", "ts": 1700000600})))
btc = snap["symbols"]["BTC-USDT"]
print(btc["last"], btc["book"]["spread"], btc["indicators"]["Sma(20)"])
```

## Choose a `snapshot_interval`

The interval trades memory for backward-seek latency (see [SEEK.md](SEEK.md)):

- **Scrubbing a timeline UI** — many small backward jumps — favour a *small*
  interval, so each drag re-folds only a short window.
- **One-shot analysis** — a single seek to a known instant — a *large* interval
  saves anchor memory; the one re-fold cost is paid once.

Forward `play` is unaffected: it steps the cursor and never re-folds from an
anchor.

## Verify a reconstruction

Every seek is a pure fold, so the same feed + spec + `ts` always yields the same
bytes. To confirm a build is deterministic, compare a `seek` against the blessed
[`golden/expected`](../golden/) snapshot — the test suite does exactly this under
both feature sets and across all ten bindings (see
[DETERMINISM.md](DETERMINISM.md)).
