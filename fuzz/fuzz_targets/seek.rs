#![no_main]
//! Loading an arbitrary feed and seeking to an arbitrary timestamp must never
//! panic (f64 overflow, huge ts, empty feed are all handled). The first eight
//! bytes carry the seek timestamp; the remainder is the JSONL feed.

use libfuzzer_sys::fuzz_target;
use timemachine_core::TimeMachine;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let ts = i64::from_le_bytes(data[..8].try_into().unwrap());
    let Ok(feed) = std::str::from_utf8(&data[8..]) else {
        return;
    };
    let Ok(mut tm) = TimeMachine::new("{}") else {
        return;
    };
    if tm.load(feed).is_ok() {
        let _ = tm.seek(ts);
    }
});
