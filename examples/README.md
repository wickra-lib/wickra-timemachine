# Wickra Time Machine examples

Runnable examples in every Wickra Time Machine language. Each one loads the same
small recorded feed (two trades on `SYM`) and reconstructs the market snapshot at
`ts = 20` by seeking, then prints the same summary — the cross-language guarantee:

## Rust — `examples/rust/`

As the CI examples job runs it, from the repository root:

```bash
cargo run -q --manifest-path examples/rust/Cargo.toml
```

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: load a small recorded two-trade feed and reconstruct the market snapshot at a past timestamp. |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-timemachine-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `seek.c` | A runnable C example: load a small recorded feed and reconstruct the market |
| `seek.cpp` | A runnable C++ example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |

## C# — `examples/csharp/`

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Seek
```

| Example | What it does |
| --- | --- |
| `Seek/Program.cs` | A runnable C# example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |

## Go — `examples/go/`

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

| Example | What it does |
| --- | --- |
| `seek.go` | A runnable Go example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |

## R — `examples/r/`

As the CI examples job runs it, from the repository root:

```bash
R CMD INSTALL bindings/r
Rscript examples/r/seek.R
```

| Example | What it does |
| --- | --- |
| `seek.R` | A runnable R example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |

## Java — `examples/java/`

As the CI examples job runs it, from the repository root:

```bash
mvn -f bindings/java/pom.xml -q install -DskipTests
mvn -f examples/java/pom.xml -q compile exec:exec  -Dnative.lib.dir="$PWD/target/release"
```

| Example | What it does |
| --- | --- |
| `src/main/java/org/wickra/timemachine/examples/Seek.java` | A runnable example against this binding. |

## Python — `examples/python/`

As the CI examples job runs it, from the repository root:

```bash
python -m pip install --require-hashes -r .github/requirements/ci-dev-py3.txt
( cd bindings/python && maturin build --release --out dist )
python -m pip install --no-index --find-links bindings/python/dist wickra-timemachine
python examples/python/seek.py
```

| Example | What it does |
| --- | --- |
| `seek.py` | A runnable Python example: load a small recorded feed and reconstruct the |

## Node.js — `examples/node/`

As the CI examples job runs it, from the repository root:

```bash
( cd bindings/node && npm install --no-audit --no-fund && npx napi build --platform --release )
( cd examples/node && npm install --no-audit --no-fund )
node examples/node/seek.js
```

| Example | What it does |
| --- | --- |
| `seek.js` | A runnable Node.js example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

| Example | What it does |
| --- | --- |
| `scrub.html` | A runnable example against this binding. |

## Example datasets

The examples read from [`examples/data/`](data/): . The
cross-language golden fixtures, which every binding is checked against byte for
byte, live in [`../golden/`](../golden).

## Per language

- **Rust** — [`rust/`](rust/): `cargo run --manifest-path examples/rust/Cargo.toml`
- **Python** — [`python/seek.py`](python/seek.py): `pip install wickra-timemachine && python examples/python/seek.py`
- **Node.js** — [`node/`](node/): `cd examples/node && npm install && node seek.js`
- **Go** — [`go/`](go/): `go run examples/go/seek.go` (with the C ABI library staged, see the Go binding README)
- **C#** — [`csharp/Seek/`](csharp/Seek/): `dotnet run --project examples/csharp/Seek`
- **Java** — [`java/`](java/): `mvn -q compile exec:java -Dexec.mainClass=org.wickra.timemachine.examples.Seek`
- **R** — [`r/seek.R`](r/seek.R): `R CMD INSTALL bindings/r && Rscript examples/r/seek.R`
- **WASM** — [`wasm/scrub.html`](wasm/scrub.html): `wasm-pack build bindings/wasm --target web`, serve the repository root, then open `examples/wasm/scrub.html`
- **C / C++** — [`c/`](c/): build the C ABI, then CMake + ctest:

  ```bash
  cargo build --release -p wickra-timemachine-c
  cmake -S examples/c -B examples/c/build
  cmake --build examples/c/build --config Release
  ctest --test-dir examples/c/build -C Release --output-on-failure
  ```

The binding examples install the published `wickra-timemachine` package for their
language; the Rust and C/C++ examples build against the in-repo core.
