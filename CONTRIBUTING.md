# Contributing to wickra-timemachine

Thanks for your interest. Issues, bug reports, ideas and pull requests are all
welcome at <https://github.com/wickra-lib/wickra-timemachine>. For larger changes,
open an issue first so we can agree on the approach.

## Orientation

- The core — the recorded-universe model and the deterministic `seek(t)`
  re-fold, built on the `wickra-backtest` replay engine and `wickra-exchange`
  feeds — lives in `crates/timemachine-core`. It is renderer-agnostic: it emits
  reconstructed snapshots, never renderer commands.
- The reference consumers are the `wickra-timemachine` CLI and the Web scrubber
  front-end in `web/` (a timeline slider + canvas over the WASM binding).
- Every language binding lives under `bindings/<lang>/` and exposes the same
  data-driven surface: a handle plus `command_json(json) -> json` and `version`.
  Bindings must preserve the **golden-parity invariant**: seeking the recorded
  universe in `golden/data/` to a given timestamp produces the byte-identical
  snapshot in `golden/expected/`.

## The dev loop

Every change runs green locally before a commit:

```bash
cargo fmt --all
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

`cargo fmt --all` and the `clippy -D warnings` gate are enforced in CI on three
operating systems. Tests that hit a live exchange run only against **testnets**,
are gated behind environment variables and are `#[ignore]` by default — never
add a test that uses mainnet or real keys.

## Conventions

- **Commits are signed** and follow Conventional Commits (`feat:`, `fix:`,
  `chore:`, `docs:`…). One logical change per commit. Open a PR against `main`;
  do not push to `main` directly.
- **All public artifacts are in English** — code, comments, commit messages, PR
  titles and bodies, issues and docs.
- **No secrets, ever** — not in code, tests, fixtures, logs, issues or PRs.
  Price/quantity values use `Decimal`, not `f64`.
- **Production code only** — no mocks outside `#[cfg(test)]`, no TODO stubs, and
  no defensive branches that can never run (they fail coverage).

## Adding an event kind or an indicator

A new **event kind** is a variant of `Feed` in
`crates/timemachine-core/src/event.rs`, is applied in `SymbolState::apply`, and
appears in the snapshot that `seek` reconstructs — the core stays the single
source of truth, and every binding sees it without changing. It ships with a
replay fixture in `golden/`, because the whole claim of this repository is that
a re-fold to an instant is byte-identical.

A new **indicator** needs no code here at all: names resolve through the
`wickra-backtest` registry, which is the ecosystem's only name -> indicator
factory. See [`docs/INDICATORS.md`](docs/INDICATORS.md) for what a name and its
parameters mean, and [`docs/DETERMINISM.md`](docs/DETERMINISM.md) for why the
fold order is fixed.

## Developer Certificate of Origin

Contributions are accepted under the [DCO](DCO); sign off your commits with
`git commit -s`. By contributing you agree your work is dual-licensed under
`MIT OR Apache-2.0`.
