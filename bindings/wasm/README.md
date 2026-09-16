<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/ci.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-timemachine)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/npm.svg)](https://www.npmjs.com/package/wickra-timemachine-wasm)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/license.svg)](https://github.com/wickra-lib/wickra-timemachine#license)

# Wickra Time Machine — WASM

---

**Part of the [Wickra ecosystem](#ecosystem): — for WASM. `npm install wickra-timemachine-wasm` — pure WebAssembly, runs anywhere a modern JS engine does.**

WebAssembly bindings for the Wickra Time Machine, compiled from Rust with
[wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A `TimeMachine` is
built from a spec JSON and driven by command JSONs over a JSON boundary, so a
browser front-end runs against the exact same core as every other Wickra Time
Machine binding.

## Install

```bash
npm install wickra-timemachine-wasm
```

### Building from this repository (contributors)

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Quick start

```js
import init, { TimeMachine } from "./pkg/wickra_timemachine_wasm.js";

await init();

const feed = [
  { ts: 10, symbol: "BTC-USDT", feed: { kind: "market", type: "trade",
    symbol: { base: "BTC", quote: "USDT" }, price: "100", quantity: "1",
    aggressor: "Buy", timestamp: 10 } },
  { ts: 20, symbol: "BTC-USDT", feed: { kind: "market", type: "trade",
    symbol: { base: "BTC", quote: "USDT" }, price: "110", quantity: "2",
    aggressor: "Sell", timestamp: 20 } },
].map((l) => JSON.stringify(l)).join("\n");

const tm = new TimeMachine("{}");
tm.command(JSON.stringify({ cmd: "load", data: feed }));
const snapshot = JSON.parse(tm.command(JSON.stringify({ cmd: "seek", ts: 20 })));
console.log(snapshot.symbols["BTC-USDT"].last); // 110
```

`command` mirrors `TimeMachine::command_json`: the commands are `load`, `seek`,
`state_at`, `play` and `version`. An invalid spec throws; a command failure
throws too.

### Determinism

The re-fold runs single-threaded here — no rayon thread pool in a browser
sandbox — which is byte-identical to the native, parallel run. Seeking to a given
timestamp produces the byte-identical snapshot here and in every other binding:
the exact cross-language golden invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of wasm-bindgen, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-timemachine/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-timemachine>
- **Docs** (guides, spec reference, cookbook): <https://timemachine.wickra.org>
- **Runnable example:** [`examples/wasm/`](https://github.com/wickra-lib/wickra-timemachine/tree/main/examples/wasm)

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
