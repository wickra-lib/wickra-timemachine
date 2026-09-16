# Wickra Time Machine examples — R

Runnable R examples for the [Wickra Time Machine R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-timemachine-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/seek.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `seek.R` | A runnable R example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |
