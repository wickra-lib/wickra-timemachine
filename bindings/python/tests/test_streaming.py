"""`play` streams frames; `state_at` re-folds each instant independently.

`wickra-timemachine-core` proves the two agree in Rust, but that says nothing about the
boundary this binding crosses: a binding that dropped a frame from the played
array, or mis-serialised one, would hand back a sequence that looks plausible and
is not the engine's.

So the same instants are asked for both ways through this binding and the two
must be identical. That equality is the whole claim of the engine -- a seek is a
deterministic re-fold, not an interpolation -- so a binding that breaks it breaks
the product, not a detail.
"""

import json
import pathlib

import pytest

from wickra_timemachine import TimeMachine

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"

FROM, TO, STEP = 1_700_000_000, 1_700_000_400, 100


def _machine() -> TimeMachine:
    spec = (GOLDEN / "specs" / "mini.json").read_text(encoding="utf-8")
    feed = (GOLDEN / "data" / "mini" / "events.jsonl").read_text(encoding="utf-8")
    tm = TimeMachine(spec)
    tm.command(json.dumps({"cmd": "load", "data": feed}))
    return tm


@pytest.mark.skipif(not (GOLDEN / "specs" / "mini.json").exists(),
                    reason="golden fixtures absent")
def test_play_equals_repeated_state_at() -> None:
    tm = _machine()
    played = json.loads(
        tm.command(json.dumps({"cmd": "play", "from": FROM, "to": TO, "step": STEP}))
    )
    one_by_one = [
        json.loads(tm.command(json.dumps({"cmd": "state_at", "ts": ts})))
        for ts in range(FROM, TO + 1, STEP)
    ]
    assert played == one_by_one
    assert len(played) == 5, "the mini corpus spans five frames at this step"


@pytest.mark.skipif(not (GOLDEN / "specs" / "mini.json").exists(),
                    reason="golden fixtures absent")
def test_a_re_seek_to_the_same_instant_is_byte_identical() -> None:
    tm = _machine()
    cmd = json.dumps({"cmd": "state_at", "ts": TO})
    first = tm.command(cmd)
    # Seek somewhere else and back: a re-fold must not depend on where it came
    # from, which is what makes scrubbing a timeline meaningful.
    tm.command(json.dumps({"cmd": "state_at", "ts": FROM}))
    assert tm.command(cmd) == first
