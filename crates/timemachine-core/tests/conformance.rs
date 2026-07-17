//! Serde round-trips of every wire type, the indicator key format, and the
//! spec-validation error paths — the contract the language bindings depend on.

use timemachine_core::{parse_records_jsonl, IndicatorRef, Record, TimeMachine, TimelineSpec};

const FEED: &str = concat!(
    r#"{"ts":10,"symbol":"AAA-USDT","feed":{"kind":"market","type":"book_snapshot","symbol":{"base":"AAA","quote":"USDT"},"last_update_id":1,"bids":[{"price":"100.0","quantity":"2.0"}],"asks":[{"price":"100.5","quantity":"1.0"}]}}"#,
    "\n",
    r#"{"ts":20,"symbol":"AAA-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"100.4","quantity":"0.5","aggressor":"Buy","timestamp":20}}"#,
    "\n",
    r#"{"ts":30,"symbol":"AAA-USDT","feed":{"kind":"funding","rate":0.0001,"mark_price":100.5}}"#,
);

#[test]
fn record_round_trips_through_json() {
    let records = parse_records_jsonl(FEED).unwrap();
    assert_eq!(records.len(), 3);
    for record in &records {
        let json = serde_json::to_string(record).unwrap();
        let back: Record = serde_json::from_str(&json).unwrap();
        assert_eq!(&back, record);
    }
}

#[test]
fn timeline_spec_round_trips_and_defaults() {
    let spec: TimelineSpec =
        serde_json::from_str(r#"{"indicators":[{"name":"Sma","params":[20]}]}"#).unwrap();
    assert_eq!(spec.book_depth, 10);
    assert_eq!(spec.tape_cap, 64);
    assert_eq!(spec.snapshot_interval, 256);
    let json = serde_json::to_string(&spec).unwrap();
    let back: TimelineSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(back, spec);
}

#[test]
fn indicator_key_is_name_and_params() {
    assert_eq!(
        IndicatorRef {
            name: "Sma".into(),
            params: vec![20.0]
        }
        .key(),
        "Sma(20)"
    );
    assert_eq!(
        IndicatorRef {
            name: "Foo".into(),
            params: vec![]
        }
        .key(),
        "Foo()"
    );
}

#[test]
fn market_snapshot_serializes_to_stable_json() {
    // `MarketSnapshot` is a serialize-only output type. Assert its JSON is
    // deterministic and well-formed (parses back into a generic value with the
    // expected shape).
    let mut tm = TimeMachine::new(r#"{"indicators":[{"name":"Sma","params":[2]}]}"#).unwrap();
    tm.load(FEED).unwrap();
    let snap = tm.seek(30).unwrap();
    let json = serde_json::to_string(&snap).unwrap();
    assert_eq!(serde_json::to_string(&snap).unwrap(), json);
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["ts"], 30);
    assert!(value["symbols"]["AAA-USDT"].is_object());
}

#[test]
fn unknown_indicator_is_rejected() {
    let err = TimeMachine::new(r#"{"indicators":[{"name":"NoSuch","params":[5]}]}"#)
        .err()
        .expect("an unknown indicator must be rejected");
    assert!(!err.to_string().is_empty());
}

#[test]
fn malformed_feed_line_reports_the_line_number() {
    let mut tm = TimeMachine::new("{}").unwrap();
    let err = tm.load("{not json}").unwrap_err();
    assert!(err.to_string().contains("line 1"));
}
