"""A runnable Python example: load a small recorded feed and reconstruct the
market snapshot at a past timestamp.

    pip install wickra-timemachine
    python examples/python/seek.py

Every language example loads the same feed and prints the same summary.
"""

import json

from wickra_timemachine import TimeMachine

FEED = "\n".join(
    json.dumps(line)
    for line in [
        {"ts": 10, "symbol": "SYM", "feed": {"kind": "market", "type": "trade", "symbol": {"base": "AAA", "quote": "USDT"}, "price": "100", "quantity": "1", "aggressor": "Buy", "timestamp": 10}},
        {"ts": 20, "symbol": "SYM", "feed": {"kind": "market", "type": "trade", "symbol": {"base": "AAA", "quote": "USDT"}, "price": "110", "quantity": "2", "aggressor": "Sell", "timestamp": 20}},
    ]
)


def main() -> None:
    tm = TimeMachine("{}")
    tm.command(json.dumps({"cmd": "load", "data": FEED}))
    snapshot = json.loads(tm.command(json.dumps({"cmd": "seek", "ts": 20})))
    print(f"wickra-timemachine {TimeMachine.version()}")
    print(f"snapshot ts: {snapshot['ts']}")
    print(f"symbols: {len(snapshot['symbols'])}")
    print(f"SYM last: {snapshot['symbols']['SYM']['last']}")


if __name__ == "__main__":
    main()
