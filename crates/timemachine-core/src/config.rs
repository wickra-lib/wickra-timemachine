//! CLI-facing configuration: a thin wrapper around a [`TimelineSpec`].

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::spec::TimelineSpec;

/// The configuration a CLI or host loads: the timeline spec to reconstruct with.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// The timeline spec.
    #[serde(default)]
    pub spec: TimelineSpec,
}

impl Config {
    /// Parse a config from JSON.
    ///
    /// # Errors
    /// [`Error::Parse`] on malformed JSON, or a spec-validation error.
    pub fn from_json(input: &str) -> Result<Self> {
        let config: Self = serde_json::from_str(input).map_err(|e| Error::Parse(e.to_string()))?;
        config.spec.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_and_parse() {
        let c = Config::from_json(r#"{"spec":{"book_depth":5}}"#).unwrap();
        assert_eq!(c.spec.book_depth, 5);
        assert!(Config::from_json("{}").is_ok());
    }
}
