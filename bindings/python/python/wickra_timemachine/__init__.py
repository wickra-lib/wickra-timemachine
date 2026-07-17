"""Wickra Time Machine — reconstruct a recorded crypto market's microstructure
state at any past moment.

Construct a :class:`TimeMachine` from a spec JSON, drive it with command JSONs
(``load``, ``seek``, ``play``, ``state_at``, ``version``), and read back the
response JSON. The same command protocol crosses every language binding, so this
Python front-end drives the exact same core — and returns the byte-identical
snapshot — as the native CLI.
"""

from ._wickra_timemachine import TimeMachine, __version__

__all__ = ["TimeMachine", "__version__"]
