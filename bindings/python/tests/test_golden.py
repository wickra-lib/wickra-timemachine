"""Determinism: seeking the same recorded feed to the same timestamp yields the
byte-identical snapshot string.

The full cross-language golden (asserting the response equals a blessed
golden/expected file) lands with the golden corpus in P-TM-5; here we pin the
core invariant that a seek is byte-reproducible, which every binding must
preserve by forwarding the command string verbatim.
"""

import json

from wickra_timemachine import TimeMachine

FEED = "\n".join(
    json.dumps(line)
    for line in [
        {
            "ts": 10,
            "symbol": "SYM",
            "feed": {
                "kind": "market",
                "type": "trade",
                "symbol": {"base": "AAA", "quote": "USDT"},
                "price": "100",
                "quantity": "1",
                "aggressor": "Buy",
                "timestamp": 10,
            },
        },
        {
            "ts": 20,
            "symbol": "SYM",
            "feed": {
                "kind": "market",
                "type": "trade",
                "symbol": {"base": "AAA", "quote": "USDT"},
                "price": "105",
                "quantity": "1",
                "aggressor": "Sell",
                "timestamp": 20,
            },
        },
    ]
)


def _seek(ts: int) -> str:
    tm = TimeMachine("{}")
    tm.command(json.dumps({"cmd": "load", "data": FEED}))
    return tm.command(json.dumps({"cmd": "seek", "ts": ts}))


def test_same_seek_same_snapshot_string() -> None:
    assert _seek(20) == _seek(20)


def test_seek_is_ts_inclusive() -> None:
    early = json.loads(_seek(10))
    late = json.loads(_seek(20))
    assert abs(early["symbols"]["SYM"]["last"] - 100.0) < 1e-9
    assert abs(late["symbols"]["SYM"]["last"] - 105.0) < 1e-9
