#![no_main]
//! Driving the JSON command surface with arbitrary input must never panic; every
//! failure comes back as an `Err` or an in-band `{"ok":false,...}` response.

use libfuzzer_sys::fuzz_target;
use timemachine_core::TimeMachine;

fuzz_target!(|data: &[u8]| {
    if let Ok(cmd) = std::str::from_utf8(data) {
        if let Ok(mut tm) = TimeMachine::new("{}") {
            let _ = tm.command_json(cmd);
        }
    }
});
