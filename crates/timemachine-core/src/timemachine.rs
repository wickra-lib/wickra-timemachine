//! The [`TimeMachine`] handle and its JSON command surface — the one seam every
//! language binding calls.

use serde_json::{json, Value};

use crate::error::{Error, Result};
use crate::event::{parse_records_jsonl, Record};
use crate::seek::seek_snapshot;
use crate::snapshot::MarketSnapshot;
use crate::spec::TimelineSpec;
use crate::version;

/// A loaded recorded universe that can be seeked to any past timestamp.
pub struct TimeMachine {
    spec: TimelineSpec,
    records: Vec<Record>,
    first_ts: Option<i64>,
    last_ts: Option<i64>,
}

impl TimeMachine {
    /// Create a time machine from a [`TimelineSpec`] JSON. No data is loaded yet.
    ///
    /// # Errors
    /// Propagates spec parse/validation failures.
    pub fn new(spec_json: &str) -> Result<Self> {
        Ok(Self {
            spec: TimelineSpec::from_json(spec_json)?,
            records: Vec::new(),
            first_ts: None,
            last_ts: None,
        })
    }

    /// Load a recorded universe (JSONL). Returns the number of records loaded.
    ///
    /// # Errors
    /// Propagates [`parse_records_jsonl`] failures.
    pub fn load(&mut self, jsonl: &str) -> Result<usize> {
        let records = parse_records_jsonl(jsonl)?;
        self.first_ts = records.first().map(|r| r.ts);
        self.last_ts = records.last().map(|r| r.ts);
        let count = records.len();
        self.records = records;
        Ok(count)
    }

    /// Reconstruct the universe snapshot at `ts`.
    ///
    /// # Errors
    /// [`Error::Data`] if no data has been loaded, else propagates the re-fold.
    pub fn seek(&self, ts: i64) -> Result<MarketSnapshot> {
        if self.records.is_empty() {
            return Err(Error::Data("no data loaded; call load first".into()));
        }
        seek_snapshot(&self.records, &self.spec, ts)
    }

    /// Reconstruct a sequence of snapshots from `from` to `to` inclusive, one
    /// every `step` timestamp units.
    ///
    /// # Errors
    /// [`Error::BadSpec`] if `step <= 0`, [`Error::Data`] if no data is loaded,
    /// else propagates the re-fold.
    pub fn play(&self, from: i64, to: i64, step: i64) -> Result<Vec<MarketSnapshot>> {
        if step <= 0 {
            return Err(Error::BadSpec("step must be > 0".into()));
        }
        if self.records.is_empty() {
            return Err(Error::Data("no data loaded; call load first".into()));
        }
        let mut out = Vec::new();
        let mut t = from;
        while t <= to {
            out.push(seek_snapshot(&self.records, &self.spec, t)?);
            t = t.saturating_add(step);
        }
        Ok(out)
    }

    /// Dispatch a JSON command. Recognised commands: `load`, `seek`, `state_at`
    /// (an alias for `seek`), `play`, and `version`.
    ///
    /// # Errors
    /// [`Error::Parse`] for malformed JSON, [`Error::BadSpec`] for a missing or
    /// unknown command / argument, else propagates the underlying operation.
    pub fn command_json(&mut self, command: &str) -> Result<String> {
        let value: Value =
            serde_json::from_str(command).map_err(|e| Error::Parse(e.to_string()))?;
        let name = value
            .get("cmd")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::BadSpec("missing \"cmd\"".into()))?;
        match name {
            "version" => Ok(json!({ "version": version() }).to_string()),
            "load" => {
                let data = value.get("data").and_then(Value::as_str).ok_or_else(|| {
                    Error::BadSpec("load requires a \"data\" JSONL string".into())
                })?;
                let count = self.load(data)?;
                Ok(json!({
                    "ok": true,
                    "loaded": count,
                    "first_ts": self.first_ts,
                    "last_ts": self.last_ts,
                })
                .to_string())
            }
            "seek" | "state_at" => {
                let ts = value
                    .get("ts")
                    .and_then(Value::as_i64)
                    .ok_or_else(|| Error::BadSpec("seek requires an integer \"ts\"".into()))?;
                let snapshot = self.seek(ts)?;
                serde_json::to_string(&snapshot).map_err(|e| Error::Data(e.to_string()))
            }
            "play" => {
                let arg = |k: &str| {
                    value
                        .get(k)
                        .and_then(Value::as_i64)
                        .ok_or_else(|| Error::BadSpec(format!("play requires an integer \"{k}\"")))
                };
                let snapshots = self.play(arg("from")?, arg("to")?, arg("step")?)?;
                serde_json::to_string(&snapshots).map_err(|e| Error::Data(e.to_string()))
            }
            other => Err(Error::BadSpec(format!("unknown command: {other}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TimeMachine;

    const FEED: &str = r#"{"ts":10,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}
{"ts":20,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"110","quantity":"1","aggressor":"Sell","timestamp":20}}"#;

    fn load_cmd() -> String {
        serde_json::json!({ "cmd": "load", "data": FEED }).to_string()
    }

    #[test]
    fn command_roundtrip_all() {
        let mut tm = TimeMachine::new("{}").unwrap();

        let loaded = tm.command_json(&load_cmd()).unwrap();
        assert!(loaded.contains("\"loaded\":2"));

        let seek = tm.command_json(r#"{"cmd":"seek","ts":20}"#).unwrap();
        assert!(seek.contains("BTC-USDT"));

        let state_at = tm.command_json(r#"{"cmd":"state_at","ts":20}"#).unwrap();
        assert_eq!(seek, state_at);

        let play = tm
            .command_json(r#"{"cmd":"play","from":10,"to":20,"step":10}"#)
            .unwrap();
        assert!(play.starts_with('['));

        let version = tm.command_json(r#"{"cmd":"version"}"#).unwrap();
        assert!(version.contains("version"));
    }

    #[test]
    fn seek_before_load_errors() {
        let mut tm = TimeMachine::new("{}").unwrap();
        assert!(tm.command_json(r#"{"cmd":"seek","ts":1}"#).is_err());
    }

    #[test]
    fn unknown_command_errors() {
        let mut tm = TimeMachine::new("{}").unwrap();
        assert!(tm.command_json(r#"{"cmd":"nope"}"#).is_err());
    }

    #[test]
    fn bad_play_step_errors() {
        let mut tm = TimeMachine::new("{}").unwrap();
        tm.command_json(&load_cmd()).unwrap();
        assert!(tm
            .command_json(r#"{"cmd":"play","from":0,"to":10,"step":0}"#)
            .is_err());
    }
}
