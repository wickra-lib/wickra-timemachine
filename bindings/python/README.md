<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/ci.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-timemachine)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/pypi.svg)](https://pypi.org/project/wickra-timemachine/)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/license.svg)](https://github.com/wickra-lib/wickra-timemachine#license)

# Wickra Time Machine — Python

---

**Part of the [Wickra ecosystem](#ecosystem): — for Python. `pip install wickra-timemachine` — prebuilt wheels for Linux, macOS and Windows, nothing to compile.**

Python bindings for the Wickra Time Machine, built with PyO3 and maturin. A
`TimeMachine` handle is driven over a JSON boundary, so seeking a recorded feed to
a timestamp yields the byte-identical snapshot as every other Wickra Time Machine
binding.

## Install

```bash
pip install wickra-timemachine
```

Pre-built wheels ship for Linux, macOS and Windows — there is nothing to
compile and no C library to track down.

## Quick start

```python
import json
from wickra_timemachine import TimeMachine

feed = "\n".join(json.dumps(line) for line in [
    {"ts": 10, "symbol": "BTC-USDT", "feed": {"kind": "market", "type": "trade",
     "symbol": {"base": "BTC", "quote": "USDT"}, "price": "100", "quantity": "1",
     "aggressor": "Buy", "timestamp": 10}},
    {"ts": 20, "symbol": "BTC-USDT", "feed": {"kind": "market", "type": "trade",
     "symbol": {"base": "BTC", "quote": "USDT"}, "price": "110", "quantity": "2",
     "aggressor": "Sell", "timestamp": 20}},
])

tm = TimeMachine("{}")
tm.command(json.dumps({"cmd": "load", "data": feed}))
snapshot = json.loads(tm.command(json.dumps({"cmd": "seek", "ts": 20})))
print(snapshot["symbols"]["BTC-USDT"]["last"])  # 110.0
```

### Surface

- **`TimeMachine(spec_json)`** — construct a handle from a `TimelineSpec` JSON
  (`"{}"` uses the default spec). Raises `ValueError` on an invalid spec.
- **`TimeMachine.command(cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `load`, `seek`,
  `state_at`, `play`, `version`. Raises `RuntimeError` on a command failure.
- **`TimeMachine.version()`** — the library version.

### Determinism

The re-fold lives only in the Rust core; this binding forwards the command string
verbatim, so seeking to a given timestamp produces the byte-identical snapshot
here and in every other binding — the exact cross-language golden invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of PyO3, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-timemachine/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-timemachine>
- **Docs** (guides, spec reference, cookbook): <https://timemachine.wickra.org>
- **Runnable example:** [`examples/python/`](https://github.com/wickra-lib/wickra-timemachine/tree/main/examples/python)

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
