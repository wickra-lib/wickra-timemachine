//! The crate error type.

/// An error produced while loading a recorded universe, parsing a spec, or
/// seeking the time machine.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A JSON or TOML payload failed to parse.
    #[error("parse: {0}")]
    Parse(String),
    /// A spec referenced an indicator the registry does not know.
    #[error("unknown indicator: {0}")]
    UnknownIndicator(String),
    /// A spec was structurally valid but semantically rejected.
    #[error("bad spec: {0}")]
    BadSpec(String),
    /// Recorded data was missing or inconsistent (e.g. a seek before any load).
    #[error("data: {0}")]
    Data(String),
}

/// The crate result alias.
pub type Result<T> = core::result::Result<T, Error>;
