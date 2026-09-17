# Wickra Time Machine examples — Go

Runnable Go examples for the [Wickra Time Machine Go binding](../../bindings/go). The binding links against the
prebuilt C ABI library, so build and stage it once before running anything:

```bash
cargo build -p wickra-timemachine-c --release
mkdir -p bindings/go/lib/linux_amd64
cp target/release/libwickra_timemachine.so bindings/go/lib/linux_amd64/
```

## Run

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

## The examples

| Example | What it does |
|---------|--------------|
| `seek.go` | A runnable Go example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |
