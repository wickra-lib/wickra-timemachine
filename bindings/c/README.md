<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Time Machine — scrub the whole crypto market like a video" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/ci.svg)](https://github.com/wickra-lib/wickra-timemachine/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-timemachine)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/release.svg)](https://github.com/wickra-lib/wickra-timemachine/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-timemachine/license.svg)](https://github.com/wickra-lib/wickra-timemachine#license)

# Wickra Time Machine — C / C++

---

**Part of the [Wickra ecosystem](#ecosystem): — for C / C++. `cargo build -p wickra-timemachine-c --release` — a prebuilt shared/static library plus a generated `wickra_timemachine.h`, no system dependencies.**

The C ABI hub for Wickra TimeMachine. It builds as a `cdylib` and a `staticlib` and
exposes a tiny JSON-over-C surface that every C-capable language (C, C++, C#, Go,
Java, R) links against. The whole evolutionary search lives in the Rust core;
this layer only marshals JSON strings across the boundary, so a fixed seed yields
the byte-identical search in every language.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-timemachine/releases) — each archive
has `wickra_timemachine.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-timemachine-c --release
# -> target/release/libwickra_timemachine.{so,dylib} or wickra_timemachine.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

### Building from this repository (contributors)

```bash
cargo build -p wickra-timemachine-c --release
```

This produces `wickra_timemachine.{dll,so,dylib}` (and a static library) under
`target/release/`. The header is committed at
[`include/wickra_timemachine.h`](https://github.com/wickra-lib/wickra-timemachine/blob/main/bindings/c/include/wickra_timemachine.h) and regenerated with:

```bash
cbindgen --config cbindgen.toml --crate wickra-timemachine-c --output include/wickra_timemachine.h
```

## Quick start

[`examples/c/seek.c`](https://github.com/wickra-lib/wickra-timemachine/blob/main/examples/c/seek.c) is the runnable example the CI smoke job executes; in full:

```c
/* A runnable C example: load a small recorded feed and reconstruct the market
 * snapshot at a past timestamp through the wickra-timemachine C ABI. Every
 * language example loads the same feed and prints the same summary. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_timemachine.h"

/* Two trades on SYM; records separated by a newline. */
static const char *FEED =
    "{\"ts\":10,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\",\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"100\",\"quantity\":\"1\",\"aggressor\":\"Buy\",\"timestamp\":10}}\n"
    "{\"ts\":20,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\",\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"110\",\"quantity\":\"2\",\"aggressor\":\"Sell\",\"timestamp\":20}}";

/* Build {"cmd":"load","data":"<JSON-escaped feed>"} into out. */
static void build_load(char *out, size_t cap) {
    size_t n = 0;
    n += (size_t)snprintf(out + n, cap - n, "{\"cmd\":\"load\",\"data\":\"");
    for (const char *p = FEED; *p && n + 2 < cap; p++) {
        if (*p == '"') {
            out[n++] = '\\';
            out[n++] = '"';
        } else if (*p == '\n') {
            out[n++] = '\\';
            out[n++] = 'n';
        } else {
            out[n++] = *p;
        }
    }
    snprintf(out + n, cap - n, "\"}");
}

static char *run(WickraTimeMachine *tm, const char *cmd) {
    int len = wickra_timemachine_command(tm, cmd, NULL, 0);
    if (len < 0) {
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (buf) {
        wickra_timemachine_command(tm, cmd, buf, (size_t)len + 1);
    }
    return buf;
}

int main(void) {
    WickraTimeMachine *tm = wickra_timemachine_new("{}");
    if (!tm) {
        fprintf(stderr, "failed to build time machine\n");
        return 1;
    }
    char load[2048];
    build_load(load, sizeof(load));

    char *loaded = run(tm, load);
    char *snapshot = run(tm, "{\"cmd\":\"seek\",\"ts\":20}");
    if (!loaded || !snapshot) {
        fprintf(stderr, "command failed\n");
        free(loaded);
        free(snapshot);
        wickra_timemachine_free(tm);
        return 1;
    }

    printf("wickra-timemachine %s\n", wickra_timemachine_version());
    printf("snapshot bytes: %zu\n", strlen(snapshot));

    free(loaded);
    free(snapshot);
    wickra_timemachine_free(tm);
    return 0;
}
```

### Surface

```c
typedef struct WickraTimeMachine WickraTimeMachine;

WickraTimeMachine *wickra_timemachine_new(const char *spec_json);   /* NULL on an invalid spec */
void          wickra_timemachine_free(WickraTimeMachine *handle);   /* NULL-safe */
int32_t       wickra_timemachine_command(WickraTimeMachine *handle, const char *cmd_json,
                                    char *out, uintptr_t cap);
const char   *wickra_timemachine_version(void);                /* static NUL string */
```

- `wickra_timemachine_new` takes a spec JSON (`"{}"` defers configuration to a later
  `set_spec` command); it returns `NULL` on a null / non-UTF-8 / invalid spec.
- `wickra_timemachine_command` applies a command envelope (`{"cmd":"...", ...}` —
  `set_spec`, `evolve`, `best`, `version`) and uses the classic two-call
  length-out protocol: call with `out = NULL`, `cap = 0` to learn the response
  length, then allocate `len + 1` and call again. A negative return is an
  unusable argument (`-1` null, `-2` non-UTF-8) or a caught panic (`-3`); a
  non-negative return is the response length. Domain errors come back **in-band**
  as `{"ok":false,"error":...}` JSON.
- `wickra_timemachine_version` returns a static version string (do not free).

### Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so an `evolve` with a fixed seed produces the byte-identical
report here and in every other Wickra TimeMachine binding.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-timemachine/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-timemachine>
- **Docs** (guides, spec reference, cookbook): <https://timemachine.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-timemachine/tree/main/examples/c)

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
