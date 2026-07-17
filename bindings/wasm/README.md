# Wickra Time Machine — WASM

WebAssembly bindings for the Wickra Time Machine, compiled from Rust with
[wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A `TimeMachine` is
built from a spec JSON and driven by command JSONs over a JSON boundary, so a
browser front-end runs against the exact same core as every other Wickra Time
Machine binding.

## Build

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Usage

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

## Determinism

The re-fold runs single-threaded here — no rayon thread pool in a browser
sandbox — which is byte-identical to the native, parallel run. Seeking to a given
timestamp produces the byte-identical snapshot here and in every other binding:
the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-timemachine>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
