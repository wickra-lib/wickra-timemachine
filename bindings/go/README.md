<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/ci.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-timemachine)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-timemachine-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/license.svg)](https://github.com/wickra-lib/wickra-timemachine#license)

# Wickra Time Machine — Go

---

**Part of the [Wickra ecosystem](#ecosystem): — for Go. `go get github.com/wickra-lib/wickra-timemachine-go` — over the C ABI via cgo, prebuilt library bundled in the module.**

Go bindings for the Wickra Time Machine over its C ABI hub via cgo. A
`TimeMachine` is built from a spec JSON and driven over a JSON boundary, so a
seek reconstructs the byte-identical market snapshot as every other Wickra Time
Machine binding.

## Install

Use the published **`wickra-timemachine-go`** module, which bundles the prebuilt C ABI
library for every platform, so `go get` + `go build` works with no extra steps
(a C compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-timemachine-go
```

`wickra-timemachine-go` is generated from this directory by the release pipeline: it mirrors
the Go sources, the vendored C ABI header (`include/wickra_timemachine.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Linux/macOS the library path is baked
in via rpath; on Windows the DLL must be discoverable at run time (next to the
executable or on `PATH`).

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p wickra-timemachine-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

### Building from this repository (contributors)

This `bindings/go` directory is the development source. To build it directly,
compile the C ABI and stage the library into the per-platform directory cgo
links against:

```bash
cargo build -p wickra-timemachine-c --release
mkdir -p bindings/go/lib/linux_amd64
cp target/release/libwickra_timemachine.so bindings/go/lib/linux_amd64/
```

Then, with the library on the loader path, run `go test ./...` from this directory.

## Quick start

```go
package main

import (
	"encoding/json"
	"fmt"

	wickra "github.com/wickra-lib/wickra-timemachine/bindings/go"
)

func main() {
	tm, err := wickra.New("{}")
	if err != nil {
		panic(err)
	}
	defer tm.Close()

	feed := `{"ts":10,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}` + "\n" +
		`{"ts":20,"symbol":"BTC-USDT","feed":{"kind":"market","type":"trade","symbol":{"base":"BTC","quote":"USDT"},"price":"110","quantity":"2","aggressor":"Sell","timestamp":20}}`

	data, _ := json.Marshal(feed)
	if _, err := tm.Command(`{"cmd":"load","data":` + string(data) + `}`); err != nil {
		panic(err)
	}
	resp, err := tm.Command(`{"cmd":"seek","ts":20}`)
	if err != nil {
		panic(err)
	}
	fmt.Println(resp) // the market snapshot reconstructed at ts=20
}
```

### Surface

- **`New(specJSON)`** — build a time-machine handle (`"{}"` uses the default
  spec). Returns an error on an invalid spec.
- **`(*TimeMachine).Command(cmdJSON)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `load`, `seek`,
  `state_at`, `play`, `version`.
- **`(*TimeMachine).Close()`** — free the handle (a finalizer also frees it).
- **`Version()`** — the library version.

### Determinism

The re-fold lives only in the Rust core; this binding forwards the command
string verbatim, so seeking to a given timestamp produces the byte-identical
snapshot here and in every other binding — the exact cross-language golden
invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of cgo over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-timemachine/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-timemachine>
- **Docs** (guides, spec reference, cookbook): <https://timemachine.wickra.org>
- **Runnable example:** [`examples/go/`](https://github.com/wickra-lib/wickra-timemachine/tree/main/examples/go)

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
