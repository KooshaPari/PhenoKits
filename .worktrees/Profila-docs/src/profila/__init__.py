"""Profila - Profiling and analysis toolkit."""

__version__ = "1.0.0"

from .telemetry import (
    MetricsCollector,
    timed_execution,
    trace_function,
    HealthCheck,
    get_collector,
)

__all__ = [
    "MetricsCollector",
    "timed_execution",
    "trace_function",
    "HealthCheck",
    "get_collector",
]
