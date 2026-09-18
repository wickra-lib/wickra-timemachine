<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/ci.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-timemachine)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-timemachine)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/license.svg)](https://github.com/wickra-lib/wickra-timemachine#license)

# Wickra Time Machine — Java

---

**Part of the [Wickra ecosystem](#ecosystem): — for Java. `org.wickra:wickra-timemachine` — prebuilt native library inside the jar, no JNI, no system dependencies.**

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

## Install

Maven:

```xml
<dependency>
  <groupId>org.wickra</groupId>
  <artifactId>wickra-timemachine</artifactId>
  <version>0.1.2</version>
</dependency>
```

Gradle:

```kotlin
implementation("org.wickra:wickra-timemachine:0.1.2")
```

The native library ships prebuilt per platform inside the jar and is
extracted automatically on first use. There is nothing to compile.

## Quick start

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

### Surface

- **`new TimeMachine(specJson)`** — build a time-machine handle (`"{}"` uses the
  default spec). Throws `IllegalArgumentException` on an invalid spec.
- **`command(cmdJson)`** — apply a command envelope (`{"cmd":"...", ...}`) and
  return the response JSON. Commands: `load`, `seek`, `state_at`, `play`,
  `version`.
- **`TimeMachine.version()`** — the library version.
- **`close()`** — free the native handle (try-with-resources recommended).

### Determinism

The re-fold lives only in the Rust core; this binding forwards the command
string verbatim, so seeking to a given timestamp produces the byte-identical
snapshot here and in every other binding — the exact cross-language golden
invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the Java Foreign Function & Memory API over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-timemachine/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-timemachine>
- **Docs** (guides, spec reference, cookbook): <https://timemachine.wickra.org>
- **Runnable example:** [`examples/java/`](https://github.com/wickra-lib/wickra-timemachine/tree/main/examples/java)

- The main project: <https://github.com/wickra-lib/wickra-timemachine>
- Documentation: <https://wickra.org>

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
