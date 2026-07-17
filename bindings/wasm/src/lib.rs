//! WebAssembly bindings for `wickra-timemachine` (wasm-bindgen).
//!
//! Reconstruct market microstructure in the browser: create a `TimeMachine` from a spec JSON,
//! drive it with a command JSON (`load`, `seek`, `play`, `state_at`, `version`) and
//! read back the response JSON. The same command protocol crosses every
//! binding, so a browser front-end runs against the exact same core as the
//! native CLI.
//!
//! The re-fold runs single-threaded here (no rayon thread pool in a browser
//! sandbox), which is byte-identical to the native run — the exact
//! cross-language golden check.

use wasm_bindgen::prelude::*;

use timemachine_core::TimeMachine as CoreTimeMachine;

/// A recorded-market time machine driven by JSON commands.
#[wasm_bindgen]
pub struct TimeMachine {
    inner: CoreTimeMachine,
}

#[wasm_bindgen]
impl TimeMachine {
    /// Construct a handle from a spec JSON (`"{}"` uses the default spec).
    #[wasm_bindgen(constructor)]
    pub fn new(spec_json: &str) -> Result<TimeMachine, JsError> {
        CoreTimeMachine::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Apply a command JSON (`{"cmd":"...", ...}`) and return the response JSON.
    pub fn command(&mut self, cmd_json: &str) -> Result<String, JsError> {
        self.inner
            .command_json(cmd_json)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// The library version.
    #[wasm_bindgen(js_name = version)]
    pub fn instance_version(&self) -> String {
        timemachine_core::version().to_string()
    }
}

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    timemachine_core::version().to_string()
}
