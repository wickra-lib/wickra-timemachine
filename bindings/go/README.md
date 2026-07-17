# Wickra TimeMachine — Go

Go bindings for the Wickra Time Machine over its C ABI hub via cgo. A
`TimeMachine` is built from a spec JSON and driven over a JSON boundary, so a
seek reconstructs the byte-identical market snapshot as every other Wickra Time
Machine binding.

## Install

```bash
go get github.com/wickra-lib/wickra-timemachine/bindings/go
```

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p wickra-timemachine-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

## Usage

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

## Surface

- **`New(specJSON)`** — build a time-machine handle (`"{}"` uses the default
  spec). Returns an error on an invalid spec.
- **`(*TimeMachine).Command(cmdJSON)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `load`, `seek`,
  `state_at`, `play`, `version`.
- **`(*TimeMachine).Close()`** — free the handle (a finalizer also frees it).
- **`Version()`** — the library version.

## Determinism

The re-fold lives only in the Rust core; this binding forwards the command
string verbatim, so seeking to a given timestamp produces the byte-identical
snapshot here and in every other binding — the exact cross-language golden
invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-timemachine>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
