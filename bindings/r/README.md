<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/ci.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-timemachine)
[![r-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/license.svg)](https://github.com/wickra-lib/wickra-timemachine#license)

# Wickra Time Machine — R

---

**Part of the [Wickra ecosystem](#ecosystem): — for R. `install.packages("wickratimemachine", repos = "https://wickra-lib.r-universe.dev")` — over the C ABI via `.Call`, prebuilt library fetched on install.**

R bindings for the Wickra Time Machine over its C ABI hub, via `.Call`. A time
machine is built from a spec JSON and driven over a JSON boundary, so seeking to
a timestamp reconstructs the byte-identical market snapshot as every other Wickra
Time Machine binding.

## Install

From r-universe:

```r
install.packages("wickratimemachine", repos = "https://wickra-lib.r-universe.dev")
```

The package's `configure` downloads the prebuilt C ABI library for this exact
version from the GitHub release and bundles it, so an ordinary install needs
nothing but a C toolchain (Rtools on Windows) for the thin `.Call` glue layer. To
build against a local checkout instead, point it at the header and library with
the environment variables below.

### Building from this repository (contributors)

The C ABI header and shared library are provided out-of-tree through two
environment variables (set by CI / the installer):

```bash
export WKTIMEMACHINE_INC=/path/to/bindings/c/include   # the header dir
export WKTIMEMACHINE_LIB=/path/to/target/release       # the library dir
R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

At run time the loader must find the shared library on `LD_LIBRARY_PATH`
(Linux), `DYLD_LIBRARY_PATH` (macOS) or `PATH` (Windows).

## Quick start

```r
library(wickratimemachine)

# The feed is a JSONL string embedded in the load command; records are joined
# by an escaped newline (\n) inside the JSON string literal.
load_cmd <- paste0(
  '{"cmd":"load","data":"',
  '{\\"ts\\":10,\\"symbol\\":\\"BTC-USDT\\",\\"feed\\":{\\"kind\\":\\"market\\",\\"type\\":\\"trade\\",',
  '\\"symbol\\":{\\"base\\":\\"BTC\\",\\"quote\\":\\"USDT\\"},\\"price\\":\\"100\\",\\"quantity\\":\\"1\\",',
  '\\"aggressor\\":\\"Buy\\",\\"timestamp\\":10}}\\n',
  '{\\"ts\\":20,\\"symbol\\":\\"BTC-USDT\\",\\"feed\\":{\\"kind\\":\\"market\\",\\"type\\":\\"trade\\",',
  '\\"symbol\\":{\\"base\\":\\"BTC\\",\\"quote\\":\\"USDT\\"},\\"price\\":\\"110\\",\\"quantity\\":\\"2\\",',
  '\\"aggressor\\":\\"Sell\\",\\"timestamp\\":20}}',
  '"}'
)

tm <- wktimemachine_new("{}")
invisible(wktimemachine_command(tm, load_cmd))
snapshot <- wktimemachine_command(tm, '{"cmd":"seek","ts":20}')
cat(snapshot) # the market snapshot reconstructed at ts=20
```

### Surface

- **`wktimemachine_new(spec_json)`** — build a time-machine handle from a spec
  JSON (an external pointer; `"{}"` uses the default spec).
- **`wktimemachine_command(tm, cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `load`, `seek`,
  `state_at`, `play`, `version`.
- **`wktimemachine_version()`** — the library version.

### Determinism

The re-fold lives only in the Rust core; this binding forwards the command
string verbatim, so seeking to a given timestamp produces the byte-identical
snapshot here and in every other binding — the exact cross-language golden
invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of R's native `.Call` interface over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-timemachine/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-timemachine>
- **Docs** (guides, spec reference, cookbook): <https://timemachine.wickra.org>
- **Runnable example:** [`examples/r/`](https://github.com/wickra-lib/wickra-timemachine/tree/main/examples/r)

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
