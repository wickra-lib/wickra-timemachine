//! The `wickra-timemachine` command-line front end.
//!
//! Scaffold surface: prints the core version. The `--dataset` / `--seek` /
//! `--format` seek interface over a recorded market universe lands in the CLI
//! phase, forwarding to `timemachine_core`'s `command_json` seam.

use clap::Parser;

/// Seek a recorded crypto-market universe to any past moment and print its
/// reconstructed microstructure snapshot.
#[derive(Parser)]
#[command(name = "wickra-timemachine", version, about)]
struct Cli;

fn main() {
    let _ = Cli::parse();
    println!("wickra-timemachine {}", timemachine_core::version());
}
