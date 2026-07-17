# Support

Thanks for using `wickra-timemachine`. Here is where to go for help.

## Questions and usage help

- Read the [README](README.md), the [architecture guide](ARCHITECTURE.md),
  the [panels guide](docs/PANELS.md), the [sources guide](docs/SOURCES.md) and the
  [renderers guide](docs/RENDERERS.md).
- Browse the runnable [`examples/`](examples/).
- Open a [GitHub Discussion](https://github.com/wickra-lib/wickra-timemachine/discussions)
  for questions and ideas.

## Bugs and feature requests

Open a [GitHub issue](https://github.com/wickra-lib/wickra-timemachine/issues) using
the bug-report or feature-request template. Please include the version, the
renderer (`tui` or `web`), the data source, and the expected vs actual result.
**Never paste API keys, secrets or signed request payloads into an issue.**

## Security

Do **not** open a public issue for security problems. Report privately to
**support@wickra.org** or via GitHub private vulnerability reporting — see
[SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md).

## Note

The Time Machine reads recorded market data only — it reconstructs past
microstructure state and never connects to a live exchange, holds keys, or places
orders. It is a research and engineering tool, not financial advice, and comes
with no warranty — review the code before relying on its output.
