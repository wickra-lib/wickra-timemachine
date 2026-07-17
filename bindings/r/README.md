# Wickra Time Machine — R

R bindings for the Wickra Time Machine over its C ABI hub, via `.Call`. A time
machine is built from a spec JSON and driven over a JSON boundary, so seeking to
a timestamp reconstructs the byte-identical market snapshot as every other Wickra
Time Machine binding.

## Build & test

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

## Usage

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

## Surface

- **`wktimemachine_new(spec_json)`** — build a time-machine handle from a spec
  JSON (an external pointer; `"{}"` uses the default spec).
- **`wktimemachine_command(tm, cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `load`, `seek`,
  `state_at`, `play`, `version`.
- **`wktimemachine_version()`** — the library version.

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
