# Documentation

These pages are the guides that live beside the code, because they describe how
this repository behaves and have to change in the same commit the behaviour does.

| Page | What it answers |
|------|-----------------|
| [SEEK.md](SEEK.md) | What a seek does: the anchor window, the re-fold, and why it is O(1) per event |
| [SNAPSHOTS.md](SNAPSHOTS.md) | What a `MarketSnapshot` carries — book, tape, footprint, funding, indicators |
| [DATASETS.md](DATASETS.md) | The JSONL record format and how a recorded universe is assembled |
| [INDICATORS.md](INDICATORS.md) | Naming an indicator, its parameters, and which names the re-fold cannot drive |
| [DETERMINISM.md](DETERMINISM.md) | Why the same seek yields the same bytes, in every language |
| [Cookbook.md](Cookbook.md) | Worked timelines |

The API reference for each language is generated from the source rather than
committed here — `cargo doc` for Rust, the `.d.ts` beside the Node binding, the
docstrings in the Python module, the C header. Keeping a second copy in this
repository would drift from the code that generates it, and a reader opening
`docs/` would have no way to tell which copy was current.

The indicator library the Time Machine resolves names through documents itself
at <https://docs.wickra.org>.

What stays here is what a generator cannot produce: the meaning of a field, the
reason a case is refused rather than answered, and the worked examples.

Elsewhere in the repository:

- [`../ARCHITECTURE.md`](../ARCHITECTURE.md) — the crate and binding layout
- [`../BENCHMARKS.md`](../BENCHMARKS.md) — what is measured and how
- [`../golden/README.md`](../golden/README.md) — the cross-language corpus and how to re-bless it
- [`../CONTRIBUTING.md`](../CONTRIBUTING.md) — how to build, test and propose a change
- [`../THREAT_MODEL.md`](../THREAT_MODEL.md) — what the Time Machine does and does not touch
