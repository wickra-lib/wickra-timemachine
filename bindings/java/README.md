# Wickra TimeMachine — Java

JVM bindings for the Wickra Time Machine over its C ABI hub, using the Foreign
Function & Memory API (FFM / Panama). A `TimeMachine` is built from a spec JSON
and driven over a JSON boundary, so seeking to a timestamp reconstructs the
byte-identical market snapshot as every other Wickra Time Machine binding.

## Requirements

- JDK 22+ (the FFM API is stable since Java 22). Run with
  `--enable-native-access=ALL-UNNAMED`.
- The native C ABI library, built by `cargo build -p wickra-timemachine-c`.
  The binding loads it from the directory named by the `native.lib.dir` system
  property (the Maven build points it at the workspace `target/debug`).

## Usage

```java
import org.wickra.timemachine.TimeMachine;

String feed = "{\"ts\":10,\"symbol\":\"BTC-USDT\",\"feed\":{\"kind\":\"market\","
    + "\"type\":\"trade\",\"symbol\":{\"base\":\"BTC\",\"quote\":\"USDT\"},"
    + "\"price\":\"100\",\"quantity\":\"1\",\"aggressor\":\"Buy\",\"timestamp\":10}}\\n"
    + "{\"ts\":20,\"symbol\":\"BTC-USDT\",\"feed\":{\"kind\":\"market\","
    + "\"type\":\"trade\",\"symbol\":{\"base\":\"BTC\",\"quote\":\"USDT\"},"
    + "\"price\":\"110\",\"quantity\":\"2\",\"aggressor\":\"Sell\",\"timestamp\":20}}";

try (TimeMachine tm = new TimeMachine("{}")) {
    tm.command("{\"cmd\":\"load\",\"data\":\"" + feed + "\"}");
    String snapshot = tm.command("{\"cmd\":\"seek\",\"ts\":20}");
    System.out.println(snapshot); // the market snapshot reconstructed at ts=20
}
```

## Surface

- **`new TimeMachine(specJson)`** — build a time-machine handle (`"{}"` uses the
  default spec). Throws `IllegalArgumentException` on an invalid spec.
- **`command(cmdJson)`** — apply a command envelope (`{"cmd":"...", ...}`) and
  return the response JSON. Commands: `load`, `seek`, `state_at`, `play`,
  `version`.
- **`TimeMachine.version()`** — the library version.
- **`close()`** — free the native handle (try-with-resources recommended).

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
