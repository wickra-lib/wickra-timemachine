//! Node.js bindings for `wickra-timemachine` via napi-rs.
//!
//! A `TimeMachine` is built from a spec JSON; `command` takes a request JSON and
//! returns the response JSON, so Node drives the exact same byte-identical
//! surface — and gets the byte-identical snapshot — as every other binding.

use napi_derive::napi;

/// A recorded-market time machine driven by JSON commands.
#[napi]
pub struct TimeMachine(timemachine_core::TimeMachine);

#[napi]
impl TimeMachine {
    /// Construct a handle from a spec JSON (`"{}"` uses the default spec).
    #[napi(constructor)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(spec_json: String) -> napi::Result<Self> {
        timemachine_core::TimeMachine::new(&spec_json)
            .map(TimeMachine)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Apply a command envelope (`{"cmd":"...", ...}`) and return the response
    /// JSON. Commands: `load`, `seek`, `play`, `state_at`, `version`.
    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn command(&mut self, cmd_json: String) -> napi::Result<String> {
        self.0
            .command_json(&cmd_json)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// The crate version.
    #[napi]
    pub fn version(&self) -> &'static str {
        timemachine_core::version()
    }
}
