//! Criterion harness for the Wickra Time Machine.
//!
//! Scaffold surface: benchmarks the version lookup so the harness compiles and
//! runs. The re-fold / seek throughput benchmarks (snapshots reconstructed per
//! second over a multi-symbol universe) land in the test-and-bench phase.

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_version(c: &mut Criterion) {
    c.bench_function("version", |b| {
        b.iter(|| black_box(timemachine_core::version()));
    });
}

criterion_group!(benches, bench_version);
criterion_main!(benches);
