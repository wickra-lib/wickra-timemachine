//! Criterion micro-benchmarks for the Time Machine re-fold engine: snapshots
//! reconstructed per second over a multi-symbol recorded universe, for both a
//! single `seek` and a full `play` sweep.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use timemachine_core::TimeMachine;

const SPEC: &str = r#"{"book_depth":10,"tape_cap":64,"indicators":[{"name":"Sma","params":[14]}],"snapshot_interval":256}"#;

/// Build a deterministic feed: `symbols` markets, each with an opening snapshot
/// followed by `events_per_symbol` trades stepping the price with a sine walk.
fn build_feed(symbols: usize, events_per_symbol: usize) -> String {
    let mut lines = Vec::with_capacity(symbols * (events_per_symbol + 1));
    for s in 0..symbols {
        let sym = format!("S{s:03}-USDT");
        lines.push(format!(
            r#"{{"ts":0,"symbol":"{sym}","feed":{{"kind":"market","type":"book_snapshot","symbol":{{"base":"S{s:03}","quote":"USDT"}},"last_update_id":1,"bids":[{{"price":"100","quantity":"5"}}],"asks":[{{"price":"101","quantity":"5"}}]}}}}"#,
        ));
        for i in 0..events_per_symbol {
            let ts = (i64::try_from(i).unwrap() + 1) * 10;
            let price = 100.0 + 5.0 * ((i as f64) * 0.05).sin();
            let side = if i % 2 == 0 { "Buy" } else { "Sell" };
            lines.push(format!(
                r#"{{"ts":{ts},"symbol":"{sym}","feed":{{"kind":"market","type":"trade","symbol":{{"base":"S{s:03}","quote":"USDT"}},"price":"{price:.4}","quantity":"1","aggressor":"{side}","timestamp":{ts}}}}}"#,
            ));
        }
    }
    lines.join("\n")
}

fn loaded(symbols: usize, events: usize) -> (TimeMachine, i64) {
    let feed = build_feed(symbols, events);
    let mut tm = TimeMachine::new(SPEC).unwrap();
    tm.load(&feed).unwrap();
    (tm, i64::try_from(events).unwrap() * 10)
}

fn bench_seek(c: &mut Criterion) {
    let mut group = c.benchmark_group("seek");
    for &symbols in &[10usize, 100] {
        for &events in &[1_000usize, 10_000] {
            let (tm, last_ts) = loaded(symbols, events);
            group.throughput(Throughput::Elements((symbols * events) as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{symbols}sym_{events}ev")),
                &(tm, last_ts),
                |b, (tm, ts)| b.iter(|| black_box(tm.seek(*ts).unwrap())),
            );
        }
    }
    group.finish();
}

fn bench_play(c: &mut Criterion) {
    let (tm, last_ts) = loaded(10, 1_000);
    c.bench_function("play_10sym_1000ev_step100", |b| {
        b.iter(|| black_box(tm.play(0, last_ts, 100).unwrap()));
    });
}

criterion_group!(benches, bench_seek, bench_play);
criterion_main!(benches);
