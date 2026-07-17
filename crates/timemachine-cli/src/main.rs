//! The `wickra-timemachine` command-line front end: seek a recorded market
//! universe to any past moment and print its reconstructed microstructure.

mod args;
mod run;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let cli = args::Cli::parse();
    match run::run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
