# Wickra Time Machine examples — C#

Runnable C# examples for the [Wickra Time Machine C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-timemachine-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Seek
```

## The examples

| Example | What it does |
|---------|--------------|
| `Seek/Program.cs` | A runnable C# example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |
