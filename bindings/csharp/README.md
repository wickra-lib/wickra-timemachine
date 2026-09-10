# wickra-timemachine (C#)

.NET bindings for
[`wickra-timemachine`](https://github.com/wickra-lib/wickra-timemachine) over the
C ABI hub, via source-generated P/Invoke. Build a `TimeMachine` from a spec JSON,
load a recorded feed, and seek to any instant — the same protocol the CLI and
every other binding speak, returning the same bytes.

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

Requires .NET 8+. The native library (`wickra_timemachine`) must be resolvable on
the loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux,
`DYLD_LIBRARY_PATH` on macOS — or beside the assembly, where the bundled
resolver finds it.

Licensed under either of [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE) at your option.
