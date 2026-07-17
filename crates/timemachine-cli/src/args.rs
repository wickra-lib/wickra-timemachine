//! Command-line arguments for `wickra-timemachine`.

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Seek a recorded crypto-market universe to any past moment and print its
/// reconstructed microstructure snapshot.
#[derive(Parser)]
#[command(name = "wickra-timemachine", version, about)]
pub struct Cli {
    /// Dataset directory containing `events.jsonl` (and an optional `spec.json`).
    #[arg(long)]
    pub dataset: PathBuf,

    /// Override the timeline spec with this JSON file (else `<dataset>/spec.json`,
    /// else the default spec).
    #[arg(long)]
    pub spec: Option<PathBuf>,

    /// Seek to a single timestamp and print one snapshot.
    #[arg(long, conflicts_with = "play")]
    pub seek: Option<i64>,

    /// Play a range: `--play FROM TO STEP` prints one snapshot every STEP units.
    #[arg(long, num_args = 3, value_names = ["FROM", "TO", "STEP"])]
    pub play: Option<Vec<i64>>,

    /// Restrict the text output to a single symbol.
    #[arg(long)]
    pub symbol: Option<String>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
}

/// The output format.
#[derive(ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// A human-readable book ladder, tape and funding per symbol.
    Text,
    /// The raw `MarketSnapshot` JSON (byte-identical to the core's `command_json`).
    Json,
}
