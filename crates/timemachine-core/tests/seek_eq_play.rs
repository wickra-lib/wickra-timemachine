//! The determinism core: a `seek(t)` is byte-identical to the `play` frame that
//! lands on `t`, and re-seeking the same timestamp is idempotent. Because the
//! same assertion runs under both feature sets in CI, it also pins that the
//! parallel and single-threaded re-folds agree.

use std::fs;
use std::path::PathBuf;

use timemachine_core::TimeMachine;

fn loaded() -> TimeMachine {
    let g = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden");
    let spec = fs::read_to_string(g.join("specs/mini.json")).unwrap();
    let feed = fs::read_to_string(g.join("data/mini/events.jsonl")).unwrap();
    let mut tm = TimeMachine::new(&spec).unwrap();
    tm.load(&feed).unwrap();
    tm
}

#[test]
fn seek_equals_the_play_frame_at_the_same_ts() {
    let tm = loaded();
    let from = 1_700_000_000;
    let to = 1_700_000_700;
    let step = 100;
    let frames = tm.play(from, to, step).unwrap();
    let mut ts = from;
    for frame in &frames {
        let direct = tm.seek(ts).unwrap();
        assert_eq!(
            serde_json::to_string(&direct).unwrap(),
            serde_json::to_string(frame).unwrap(),
            "seek({ts}) != play frame"
        );
        ts += step;
    }
}

#[test]
fn re_seeking_the_same_ts_is_idempotent() {
    let tm = loaded();
    let a = serde_json::to_string(&tm.seek(1_700_000_600).unwrap()).unwrap();
    let b = serde_json::to_string(&tm.seek(1_700_000_600).unwrap()).unwrap();
    assert_eq!(a, b);
}
