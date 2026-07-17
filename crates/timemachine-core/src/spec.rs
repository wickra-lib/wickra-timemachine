//! The timeline spec: what to reconstruct and how much of it.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::indicator_set::is_known_indicator;

fn default_book_depth() -> usize {
    10
}
fn default_tape_cap() -> usize {
    64
}
fn default_snapshot_interval() -> usize {
    256
}

/// A reference to one streaming indicator: a registry name plus its parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndicatorRef {
    /// The indicator name (e.g. `"Sma"`, `"Ema"`, `"Rsi"`).
    pub name: String,
    /// The indicator parameters (e.g. `[20]` for a period).
    #[serde(default)]
    pub params: Vec<f64>,
}

impl IndicatorRef {
    /// The stable map key for this indicator, e.g. `Sma(20)`.
    #[must_use]
    pub fn key(&self) -> String {
        let params = self
            .params
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        format!("{}({params})", self.name)
    }
}

/// How to reconstruct a universe: book depth, tape depth, indicators, and how
/// often to drop a re-fold anchor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineSpec {
    /// How many book levels per side to materialize in a snapshot.
    #[serde(default = "default_book_depth")]
    pub book_depth: usize,
    /// How many recent trade prints to keep per symbol.
    #[serde(default = "default_tape_cap")]
    pub tape_cap: usize,
    /// The indicators to fold on each symbol's trade price.
    #[serde(default)]
    pub indicators: Vec<IndicatorRef>,
    /// Drop a re-fold anchor every `snapshot_interval` events (bounds backward-seek cost).
    #[serde(default = "default_snapshot_interval")]
    pub snapshot_interval: usize,
}

impl Default for TimelineSpec {
    fn default() -> Self {
        Self {
            book_depth: default_book_depth(),
            tape_cap: default_tape_cap(),
            indicators: Vec::new(),
            snapshot_interval: default_snapshot_interval(),
        }
    }
}

impl TimelineSpec {
    /// Parse a spec from JSON.
    ///
    /// # Errors
    /// [`Error::Parse`] on malformed JSON, or [`Error::BadSpec`] /
    /// [`Error::UnknownIndicator`] if validation fails.
    pub fn from_json(input: &str) -> Result<Self> {
        let spec: Self = serde_json::from_str(input).map_err(|e| Error::Parse(e.to_string()))?;
        spec.validate()?;
        Ok(spec)
    }

    /// Validate the spec: positive depths, a positive snapshot interval, and only
    /// known indicators with a valid parameter arity.
    pub(crate) fn validate(&self) -> Result<()> {
        if self.book_depth == 0 {
            return Err(Error::BadSpec("book_depth must be > 0".into()));
        }
        if self.tape_cap == 0 {
            return Err(Error::BadSpec("tape_cap must be > 0".into()));
        }
        if self.snapshot_interval == 0 {
            return Err(Error::BadSpec("snapshot_interval must be > 0".into()));
        }
        for ind in &self.indicators {
            if !is_known_indicator(&ind.name) {
                return Err(Error::UnknownIndicator(ind.name.clone()));
            }
            if ind.params.len() != 1 {
                return Err(Error::BadSpec(format!(
                    "{} takes exactly one parameter (period)",
                    ind.name
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{IndicatorRef, TimelineSpec};

    #[test]
    fn indicator_ref_key_format() {
        let r = IndicatorRef {
            name: "Sma".into(),
            params: vec![20.0],
        };
        assert_eq!(r.key(), "Sma(20)");
    }

    #[test]
    fn defaults_apply_and_validate() {
        let spec = TimelineSpec::from_json("{}").unwrap();
        assert_eq!(spec.book_depth, 10);
        assert_eq!(spec.tape_cap, 64);
        assert_eq!(spec.snapshot_interval, 256);
    }

    #[test]
    fn zero_book_depth_rejected() {
        assert!(TimelineSpec::from_json(r#"{"book_depth":0}"#).is_err());
    }

    #[test]
    fn unknown_indicator_rejected() {
        let err = TimelineSpec::from_json(r#"{"indicators":[{"name":"Nope","params":[5]}]}"#)
            .unwrap_err();
        assert!(err.to_string().contains("unknown indicator"));
    }

    #[test]
    fn wrong_arity_rejected() {
        assert!(TimelineSpec::from_json(r#"{"indicators":[{"name":"Sma","params":[]}]}"#).is_err());
    }
}
