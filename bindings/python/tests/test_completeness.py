"""The Python surface exposes exactly the documented API."""

import wickra_timemachine
from wickra_timemachine import TimeMachine


def test_module_exports() -> None:
    assert set(wickra_timemachine.__all__) == {"TimeMachine", "__version__"}


def test_time_machine_methods() -> None:
    for name in ("command", "version"):
        assert hasattr(TimeMachine, name)


def test_version_is_a_string() -> None:
    assert isinstance(wickra_timemachine.__version__, str)
    assert wickra_timemachine.__version__
