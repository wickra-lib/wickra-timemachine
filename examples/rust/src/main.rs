//! A runnable Rust example: load a small recorded two-trade feed and reconstruct
//! the market snapshot at a past timestamp. Every language example loads the same
//! feed and seeks the same timestamp, so they all print the same summary — that
//! is the cross-language guarantee.
//!
//! ```bash
//! cargo run --manifest-path examples/rust/Cargo.toml
//! ```

use timemachine_core::TimeMachine;

const FEED: &str = concat!(
    r#"{"ts":10,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}"#,
    "\n",
    r#"{"ts":20,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"110","quantity":"2","aggressor":"Sell","timestamp":20}}"#,
);

fn main() {
    let mut tm = TimeMachine::new("{}").expect("valid spec");
    tm.load(FEED).expect("load");
    let snapshot: serde_json::Value =
        serde_json::from_str(&tm.command_json(r#"{"cmd":"seek","ts":20}"#).expect("seek")).unwrap();

    println!("wickra-timemachine {}", timemachine_core::version());
    println!("snapshot ts: {}", snapshot["ts"]);
    println!("symbols: {}", snapshot["symbols"].as_object().unwrap().len());
    println!("SYM last: {}", snapshot["symbols"]["SYM"]["last"]);
}
