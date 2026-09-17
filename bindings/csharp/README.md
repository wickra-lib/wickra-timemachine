<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/ci.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-timemachine)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/nuget.svg)](https://www.nuget.org/packages/Wickra.TimeMachine)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/license.svg)](https://github.com/wickra-lib/wickra-timemachine#license)

# Wickra Time Machine — C#

---

**Part of the [Wickra ecosystem](#ecosystem): — for C#. `dotnet add package Wickra.TimeMachine` — prebuilt native library, no system dependencies.**

.NET bindings for
[`wickra-timemachine`](https://github.com/wickra-lib/wickra-timemachine) over the
C ABI hub, via source-generated P/Invoke. Build a `TimeMachine` from a spec JSON,
load a recorded feed, and seek to any instant — the same protocol the CLI and
every other binding speak, returning the same bytes.

## Install

```bash
dotnet add package Wickra.TimeMachine
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

Requires .NET 8+. The native library (`wickra_timemachine`) must be resolvable on
the loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux,
`DYLD_LIBRARY_PATH` on macOS — or beside the assembly, where the bundled
resolver finds it.

## Quick start

```csharp
using Wickra.TimeMachine;

const string spec = """
{"book_depth":10,"tape_cap":64,
 "indicators":[{"name":"Rsi","params":[14]},
               {"name":"Macd","params":[12,26,9]}],
 "snapshot_interval":256}
""";

using var tm = new TimeMachine(spec);
tm.Command("""{"cmd":"load","data":"<JSONL feed>"}""");
string snapshot = tm.Command("""{"cmd":"seek","ts":1700000600}""");
```

A seek is a deterministic re-fold, not an interpolation: seeking backwards costs
what seeking forwards costs, and lands on the identical state. `play` streams a
range of instants and is exactly the sequence of individual seeks over the same
range — the tests in every binding hold those two equal.

Indicator names resolve through the `wickra-backtest` registry, so anything the
backtester can name works here with the same parameters. Names that read
something beyond the bar — an order book, a funding tick, a reference series, the
market cross-section — are refused with the feed they would have needed, because
the re-fold hands the indicators the trade print alone.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-timemachine/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-timemachine>
- **Docs** (guides, spec reference, cookbook): <https://timemachine.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-timemachine/tree/main/examples/csharp)

Wickra Time Machine ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-timemachine/blob/main/SECURITY.md>.

## Disclaimer

Wickra Time Machine is a research tool, provided "as is" without warranty of any
kind. It reconstructs recorded market microstructure for analysis; nothing here is
financial advice, and trading carries risk of loss.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-timemachine/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-timemachine/blob/main/LICENSE-MIT) at your option.
