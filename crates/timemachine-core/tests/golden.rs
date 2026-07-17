//! The golden contract: loading a recorded feed under a spec and seeking (or
//! playing) it must reproduce the byte-exact `MarketSnapshot` JSON committed in
//! `golden/expected/`. This runs under both feature sets (default parallel and
//! `--no-default-features`), which is how the parallel and single-threaded
//! re-folds are pinned byte-identical.

use std::fs;
use std::path::PathBuf;

use timemachine_core::TimeMachine;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

fn load(dataset: &str, spec: &str) -> TimeMachine {
    let g = root();
    let spec_json = fs::read_to_string(g.join("specs").join(spec)).unwrap();
    let feed = fs::read_to_string(g.join("data").join(dataset).join("events.jsonl")).unwrap();
    let mut tm = TimeMachine::new(&spec_json).unwrap();
    tm.load(&feed).unwrap();
    tm
}

fn expected(name: &str) -> String {
    fs::read_to_string(root().join("expected").join(name)).unwrap()
}

#[test]
fn seek_matches_golden_snapshot() {
    let cases = [
        ("mini", "mini.json", 1_700_000_600_i64),
        ("mini", "funding.json", 1_700_000_700),
        ("multi", "multi_symbol.json", 1_700_000_600),
        ("mini", "anchor_reseek.json", 1_700_000_500),
    ];
    for (dataset, spec, ts) in cases {
        let tm = load(dataset, spec);
        let snap = tm.seek(ts).unwrap();
        let got = serde_json::to_string(&snap).unwrap();
        assert_eq!(got, expected(spec), "seek {spec} @ {ts}");
    }
}

#[test]
fn play_matches_golden_frames() {
    let tm = load("mini", "play.json");
    let frames = tm.play(1_700_000_000, 1_700_000_700, 100).unwrap();
    let got = serde_json::to_string(&frames).unwrap();
    assert_eq!(got, expected("play.json"));
}
