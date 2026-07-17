# Examples

Runnable examples in every Wickra Time Machine language. Each one loads the same
small recorded feed (two trades on `SYM`) and reconstructs the market snapshot at
`ts = 20` by seeking, then prints the same summary — the cross-language guarantee:

```
wickra-timemachine 0.1.0
snapshot ts: 20
symbols: 1
SYM last: 110
```

The snapshot JSON a seek returns is byte-identical across every language, so all
the examples reconstruct the same `SYM` last price of `110`. See
[`golden/README.md`](../golden/README.md) for the blessed corpus that pins this.

A recorded dataset and a spec are also in [`data/`](data/) for use with the CLI:

```bash
cargo run -p timemachine-cli -- --dataset examples/data/mini --spec examples/data/specs/mini.json --seek 1700000600 --format json
```

## Per language

- **Rust** — [`rust/`](rust/): `cargo run --manifest-path examples/rust/Cargo.toml`
- **Python** — [`python/seek.py`](python/seek.py): `pip install wickra-timemachine && python examples/python/seek.py`
- **Node.js** — [`node/`](node/): `cd examples/node && npm install && node seek.js`
- **Go** — [`go/`](go/): `go run examples/go/seek.go` (with the C ABI library staged, see the Go binding README)
- **C#** — [`csharp/Seek/`](csharp/Seek/): `dotnet run --project examples/csharp/Seek`
- **Java** — [`java/`](java/): `mvn -q compile exec:java -Dexec.mainClass=org.wickra.timemachine.examples.Seek`
- **R** — [`r/seek.R`](r/seek.R): `R CMD INSTALL bindings/r && Rscript examples/r/seek.R`
- **C / C++** — [`c/`](c/): build the C ABI, then CMake + ctest:

  ```bash
  cargo build --release -p wickra-timemachine-c
  cmake -S examples/c -B examples/c/build
  cmake --build examples/c/build --config Release
  ctest --test-dir examples/c/build -C Release --output-on-failure
  ```

The binding examples install the published `wickra-timemachine` package for their
language; the Rust and C/C++ examples build against the in-repo core.
