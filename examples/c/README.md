# Wickra Time Machine — C / C++ examples

The Wickra Time Machine C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_timemachine.h`](../../bindings/c/include/wickra_timemachine.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_timemachine.hpp`](../../bindings/c/include/wickra_timemachine.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-timemachine-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_timemachine.so`     | `-lwickra_timemachine` |
| macOS    | `libwickra_timemachine.dylib`  | `-lwickra_timemachine` |
| Windows (MSVC) | `wickra_timemachine.dll` | `wickra_timemachine.dll.lib` (import lib) |

A static library (`libwickra_timemachine.a` / `wickra_timemachine.lib`) is emitted alongside.

## Build and run the examples

### With CMake (portable, used by CI)

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

### Directly with a compiler

```sh
# Linux / macOS
cc examples/c/seek.c -I bindings/c/include -L target/release -lwickra_timemachine -lm -o seek
LD_LIBRARY_PATH=target/release ./seek        # macOS: DYLD_LIBRARY_PATH

# Windows (MinGW gcc, linking the DLL directly)
gcc examples/c/seek.c -I bindings/c/include target/release/wickra_timemachine.dll -lm -o seek.exe
```

## The examples

| Example | What it does |
|---------|--------------|
| `seek.c` | A runnable C example: load a small recorded feed and reconstruct the market |
| `seek.cpp` | A runnable C++ example: load a small recorded feed and reconstruct the market snapshot at a past timestamp. |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_timemachine.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
