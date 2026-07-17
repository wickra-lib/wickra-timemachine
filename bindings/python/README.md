# Wickra Time Machine — Python

Python bindings for the Wickra Time Machine, built with PyO3 and maturin. A
`TimeMachine` handle is driven over a JSON boundary, so seeking a recorded feed to
a timestamp yields the byte-identical snapshot as every other Wickra Time Machine
binding.

## Install

```bash
pip install wickra-timemachine
```

## Usage

```python
import json
from wickra_timemachine import TimeMachine

feed = "\n".join(json.dumps(line) for line in [
    {"ts": 10, "symbol": "BTC-USDT", "feed": {"kind": "market", "type": "trade",
     "symbol": {"base": "BTC", "quote": "USDT"}, "price": "100", "quantity": "1",
     "aggressor": "Buy", "timestamp": 10}},
    {"ts": 20, "symbol": "BTC-USDT", "feed": {"kind": "market", "type": "trade",
     "symbol": {"base": "BTC", "quote": "USDT"}, "price": "110", "quantity": "2",
     "aggressor": "Sell", "timestamp": 20}},
])

tm = TimeMachine("{}")
tm.command(json.dumps({"cmd": "load", "data": feed}))
snapshot = json.loads(tm.command(json.dumps({"cmd": "seek", "ts": 20})))
print(snapshot["symbols"]["BTC-USDT"]["last"])  # 110.0
```

## Surface

- **`TimeMachine(spec_json)`** — construct a handle from a `TimelineSpec` JSON
  (`"{}"` uses the default spec). Raises `ValueError` on an invalid spec.
- **`TimeMachine.command(cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `load`, `seek`,
  `state_at`, `play`, `version`. Raises `RuntimeError` on a command failure.
- **`TimeMachine.version()`** — the library version.

## Determinism

The re-fold lives only in the Rust core; this binding forwards the command string
verbatim, so seeking to a given timestamp produces the byte-identical snapshot
here and in every other binding — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-timemachine>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
