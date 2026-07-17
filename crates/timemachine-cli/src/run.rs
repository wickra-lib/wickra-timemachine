//! Load a recorded dataset and render a seek or a play.

use std::fmt::Write as _;
use std::fs;

use timemachine_core::{MarketSnapshot, SymbolSnapshot, TimeMachine};

use crate::args::{Cli, Format};

/// Run the CLI: load the dataset, perform the requested action, print the result.
///
/// # Errors
/// Returns a human-readable message on any I/O, parse, or seek failure.
pub fn run(cli: &Cli) -> Result<(), String> {
    let events_path = cli.dataset.join("events.jsonl");
    let jsonl = fs::read_to_string(&events_path)
        .map_err(|e| format!("read {}: {e}", events_path.display()))?;

    let spec_json = if let Some(path) = &cli.spec {
        fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?
    } else {
        let embedded = cli.dataset.join("spec.json");
        if embedded.exists() {
            fs::read_to_string(&embedded)
                .map_err(|e| format!("read {}: {e}", embedded.display()))?
        } else {
            "{}".to_string()
        }
    };

    let mut tm = TimeMachine::new(&spec_json).map_err(|e| e.to_string())?;
    tm.load(&jsonl).map_err(|e| e.to_string())?;

    if let Some(ts) = cli.seek {
        let snapshot = tm.seek(ts).map_err(|e| e.to_string())?;
        print!("{}", render_one(&snapshot, cli)?);
    } else if let Some(range) = &cli.play {
        let snapshots = tm
            .play(range[0], range[1], range[2])
            .map_err(|e| e.to_string())?;
        print!("{}", render_many(&snapshots, cli)?);
    } else {
        return Err("one of --seek or --play is required".into());
    }
    Ok(())
}

fn render_one(snapshot: &MarketSnapshot, cli: &Cli) -> Result<String, String> {
    match cli.format {
        Format::Json => serde_json::to_string(snapshot).map_err(|e| e.to_string()),
        Format::Text => Ok(render_text(snapshot, cli.symbol.as_deref())),
    }
}

fn render_many(snapshots: &[MarketSnapshot], cli: &Cli) -> Result<String, String> {
    match cli.format {
        Format::Json => serde_json::to_string(snapshots).map_err(|e| e.to_string()),
        Format::Text => {
            let mut out = String::new();
            for snapshot in snapshots {
                out.push_str(&render_text(snapshot, cli.symbol.as_deref()));
                out.push('\n');
            }
            Ok(out)
        }
    }
}

fn render_text(snapshot: &MarketSnapshot, only: Option<&str>) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "== snapshot @ {} ==", snapshot.ts);
    let mut any = false;
    for (symbol, state) in &snapshot.symbols {
        if only.is_some_and(|s| s != symbol) {
            continue;
        }
        any = true;
        render_symbol(&mut out, symbol, state);
    }
    if !any {
        let _ = writeln!(out, "(no symbols)");
    }
    out
}

fn render_symbol(out: &mut String, symbol: &str, state: &SymbolSnapshot) {
    let _ = writeln!(out, "\n[{symbol}] last={}", state.last);
    // Asks (lowest last so the ladder reads top-down toward the bid).
    for level in state.book.asks.iter().rev() {
        let _ = writeln!(out, "  ask {:>14} x {}", level.price, level.qty);
    }
    if let Some(spread) = state.book.spread {
        let _ = writeln!(out, "  -- spread {spread} --");
    }
    for level in &state.book.bids {
        let _ = writeln!(out, "  bid {:>14} x {}", level.price, level.qty);
    }
    if let Some(funding) = &state.funding {
        let _ = writeln!(
            out,
            "  funding rate={} mark={}",
            funding.rate, funding.mark_price
        );
    }
    for (name, value) in &state.indicators {
        match value {
            Some(v) => {
                let _ = writeln!(out, "  {name} = {v}");
            }
            None => {
                let _ = writeln!(out, "  {name} = (warming up)");
            }
        }
    }
    let shown = state.tape.len().min(5);
    if shown > 0 {
        let _ = writeln!(out, "  tape (newest {shown}):");
        for trade in state.tape.iter().take(shown) {
            let _ = writeln!(out, "    {} {} x {}", trade.side, trade.price, trade.qty);
        }
    }
}
