# Roadmap

`wickra-timemachine` is built out in phases, mirroring the proven structure of the
Wickra exchange and backtester repos. Each phase lands as reviewed, CI-green pull
requests. Status below is updated as phases complete.

## Phases

0. **Scaffold** — workspace, governance, supply-chain config, `.github`
   scaffolding. *In progress.*
1. **`timemachine-core`** — the recorded-universe model and the deterministic
   `seek(t)` re-fold over `wickra-exchange` feeds and the `wickra-backtest`
   replay engine, exposed through the `command_json` boundary. Near-total
   coverage via inline tests.
2. **CLI** — `crates/timemachine-cli`: seek a recorded universe to any timestamp
   and print the reconstructed snapshot (`--dataset` / `--seek` / `--format`).
3. **Bindings** — native Python, Node and WASM, plus the C ABI hub reaching C,
   C++, C#, Go, Java and R; each exposes the handle + `command_json` + `version`,
   with a completeness guard.
4. **Golden harness** — a fixed recorded universe with blessed, byte-exact,
   cross-language snapshots.
5. **Hardening** — conformance suite, property tests, fuzz targets, benchmarks
   (snapshots per second).
6. **Examples** — one runnable example per language.
7. **CI/CD** — the full workflow matrix (all languages), OpenSSF Scorecard,
   Best Practices, link check, release.
8. **Docs** — the banner + badge treatment and the docs guides.
9. **Web scrubber** — a Vue/Vite timeline-slider + canvas front-end over the
   WASM binding, sharing the core's snapshots; view state in `localStorage`.

## Non-goals

- **Live trading.** The Time Machine reads recorded market data only — no keys,
  no exchange connection, no order placement.
- **Secrets in the browser.** The web scrubber holds no credentials.
- **Renderer-specific logic in the core.** The core emits reconstructed
  snapshots, never renderer commands, so every front-end stays a thin view.
