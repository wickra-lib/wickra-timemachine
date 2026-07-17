"""Smoke test: construct a time machine, load a small feed, seek, read a snapshot."""

import json

from wickra_timemachine import TimeMachine, __version__


def _feed() -> str:
    lines = [
        {
            "ts": 10,
            "symbol": "BTC-USDT",
            "feed": {
                "kind": "market",
                "type": "trade",
                "symbol": {"base": "BTC", "quote": "USDT"},
                "price": "100",
                "quantity": "1",
                "aggressor": "Buy",
                "timestamp": 10,
            },
        },
        {
            "ts": 20,
            "symbol": "BTC-USDT",
            "feed": {
                "kind": "market",
                "type": "trade",
                "symbol": {"base": "BTC", "quote": "USDT"},
                "price": "110",
                "quantity": "2",
                "aggressor": "Sell",
                "timestamp": 20,
            },
        },
        {"ts": 20, "symbol": "ETH-USDT", "feed": {"kind": "funding", "rate": 0.0002, "mark_price": 50.0}},
    ]
    return "\n".join(json.dumps(line) for line in lines)


def _loaded() -> TimeMachine:
    tm = TimeMachine("{}")
    tm.command(json.dumps({"cmd": "load", "data": _feed()}))
    return tm


def test_seek_reconstructs_snapshot() -> None:
    tm = _loaded()
    snap = json.loads(tm.command(json.dumps({"cmd": "seek", "ts": 20})))
    assert snap["ts"] == 20
    assert abs(snap["symbols"]["BTC-USDT"]["last"] - 110.0) < 1e-9
    assert snap["symbols"]["ETH-USDT"]["funding"] is not None


def test_seek_is_deterministic() -> None:
    a = _loaded().command(json.dumps({"cmd": "seek", "ts": 20}))
    b = _loaded().command(json.dumps({"cmd": "seek", "ts": 20}))
    assert a == b


def test_version() -> None:
    assert TimeMachine.version() == __version__
