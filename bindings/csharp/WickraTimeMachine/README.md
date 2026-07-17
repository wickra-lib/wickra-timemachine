# Wickra TimeMachine — C#

.NET bindings for the Wickra Time Machine over its C ABI hub. A `TimeMachine` is
built from a spec JSON and driven over a JSON boundary, so seeking to a timestamp
reconstructs the byte-identical market snapshot as every other Wickra Time
Machine binding.

## Install

```bash
dotnet add package Wickra.TimeMachine
```

The package ships the native C ABI library per runtime identifier under
`runtimes/<rid>/native/`. For a local build, `cargo build -p wickra-timemachine-c --release`
places the library in `target/release/`; the bundled `DllImportResolver` probes
the Cargo `target/` tree, so tests and apps in the repo find it without extra
steps.

## Usage

```csharp
using System.Text.Json;
using Wickra.TimeMachine;

const string feed =
    """{"ts":10,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}""" + "\n" +
    """{"ts":20,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"110","quantity":"2","aggressor":"Sell","timestamp":20}}""";

using var tm = new TimeMachine("{}");
tm.Command($$"""{"cmd":"load","data":{{JsonSerializer.Serialize(feed)}}}""");
string snapshot = tm.Command("""{"cmd":"seek","ts":20}""");
Console.WriteLine(snapshot); // the market snapshot reconstructed at ts=20
```

## Surface

- **`new TimeMachine(specJson)`** — build a time-machine handle (`"{}"` uses the
  default spec). Throws `ArgumentException` on an invalid spec.
- **`Command(cmdJson)`** — apply a command envelope (`{"cmd":"...", ...}`) and
  return the response JSON. Commands: `load`, `seek`, `state_at`, `play`,
  `version`.
- **`TimeMachine.Version()`** — the library version.
- **`Dispose()`** — free the native handle (`using` recommended).

## Determinism

The re-fold lives only in the Rust core; this binding forwards the command
string verbatim, so seeking to a given timestamp produces the byte-identical
snapshot here and in every other binding — the exact cross-language golden
invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-timemachine>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either MIT or Apache-2.0, at your option.
