#![no_main]
//! Parsing an arbitrary JSONL recorded feed must never panic.

use libfuzzer_sys::fuzz_target;
use timemachine_core::parse_records_jsonl;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_records_jsonl(s);
    }
});
