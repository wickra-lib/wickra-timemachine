# wickra-timemachine WASM examples

Browser demos for the `wickra-timemachine-wasm` binding.

The WASM build carries the whole re-fold engine with `--no-default-features`: no
rayon, so the per-symbol fold is sequential rather than parallel — and
byte-for-byte identical to the parallel one, which is what the golden snapshots
pin down. A spec is data, not code, so the bytes on this page are the same ones
`examples/node/seek.js` sends.

## Build

The module ships as a `wasm-pack` `--target web` bundle. Build it once from the
repository root:

```bash
wasm-pack build bindings/wasm --target web --release
```

That writes `bindings/wasm/pkg/` with the `.wasm` binary, the JS loader and the
type declarations the page imports.

## Run

The page loads its module over `http://`, not `file://`, because ES module
imports and `WebAssembly.instantiateStreaming` both need a real origin. Serve the
repository root:

```bash
python -m http.server 8000
```

Then open `http://localhost:8000/examples/wasm/scrub.html`.

## Pages

| Page | What it does |
|------|--------------|
| `scrub.html` | Loads a three-trade feed and puts a slider over it: every position re-folds the whole market from the start, with no interpolation and no cached frame. Dragging backwards costs the same as dragging forwards and lands on the identical state — which is the claim the golden corpus pins. |

## See also

- [examples/README.md](../README.md) — the same seek in every other language.
- [bindings/wasm/README.md](../../bindings/wasm/README.md) — the binding itself.
