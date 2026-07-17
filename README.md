<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-timemachine)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-timemachine)
[![Deterministic across 10 languages](https://img.shields.io/badge/deterministic%20across-10%20languages-3b82f6)](#use-in-any-language)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

---

# Wickra Time Machine

**Scrub the whole crypto market like a video — every symbol, full orderbook +
trades + funding, rewound to any moment, reconstructed in O(1) via deterministic
re-fold.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the Time
> Machine folds recorded market feeds from
> [`wickra-exchange`](https://github.com/wickra-lib/wickra-exchange) through the
> [`wickra-backtest`](https://github.com/wickra-lib/wickra-backtest) replay
> engine, so seeking to any past timestamp reconstructs the exact microstructure
> state — no snapshots, no interpolation.

Wickra Time Machine is one data-driven core, `timemachine-core`: point it at a
recorded universe, `seek(t)`, and it re-folds every symbol's orderbook, tape and
funding state deterministically to that instant. Because the engine is O(1) per
event, seeking scales to the whole market. The core is exposed as a
**JSON-over-C-ABI data API** (`command_json`) in **Rust, Python, Node.js, WASM,
C, C++, C#, Go, Java and R**, plus a reference CLI and a web scrubber frontend.

## Status

Early development (0.1.0, unreleased). This scaffold pins the repository,
governance and supply-chain configuration ahead of the re-fold core, the CLI, the
ten language bindings, the golden harness and the web scrubber.

## Use in any language

The same handle + `command_json` + `version` surface ships for every supported
language; each binding forwards the command string verbatim, so seeking to the
same timestamp yields a byte-identical snapshot in all of them.

## Building from source

```bash
cargo build
cargo test
```

## Requirements

- Rust 1.86+ (MSRV). The Time Machine depends on `wickra-core` (crates.io) and,
  as git dependencies, the `wickra-exchange` feeds and the `wickra-backtest`
  replay engine.

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). The Time
Machine reads recorded market data only — no keys, no order placement.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
