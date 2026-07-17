#![no_main]
//! Parsing an arbitrary timeline spec must never panic — only return `Ok`/`Err`.

use libfuzzer_sys::fuzz_target;
use timemachine_core::TimelineSpec;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = TimelineSpec::from_json(s);
    }
});
