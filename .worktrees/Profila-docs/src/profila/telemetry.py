"""Telemetry and observability module for Profila.

Provides metrics collection, tracing, and logging infrastructure.
"""

from __future__ import annotations

import functools
import json
import logging
import time
from contextlib import contextmanager
from dataclasses import dataclass, asdict
from datetime import datetime
from pathlib import Path
from typing import Any, Callable, Optional


logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger("profila")


@dataclass
class Metric:
    """A single metric datapoint."""
    name: str
    value: float
    timestamp: str
    tags: dict[str, str]
    unit: str = "count"


class MetricsCollector:
    """Collects and exports metrics for observability."""

    def __init__(self, output_dir: Optional[str] = None):
        self.metrics: list[Metric] = []
        self.output_dir = Path(output_dir) if output_dir else Path("./output")
        self.output_dir.mkdir(parents=True, exist_ok=True)

    def gauge(self, name: str, value: float, tags: Optional[dict] = None, unit: str = "count") -> None:
        """Record a gauge metric."""
        metric = Metric(
            name=name,
            value=value,
            timestamp=datetime.utcnow().isoformat(),
            tags=tags or {},
            unit=unit
        )
        self.metrics.append(metric)
        logger.debug(f"Metric: {name}={value} {unit}")

    def counter(self, name: str, increment: float = 1, tags: Optional[dict] = None) -> None:
        """Record a counter metric."""
        self.gauge(name, increment, tags, "count")

    def histogram(self, name: str, value: float, tags: Optional[dict] = None, unit: str = "ms") -> None:
        """Record a histogram metric."""
        self.gauge(name, value, tags, unit)

    def export_json(self, filename: str = "metrics.json") -> Path:
        """Export all metrics to JSON file."""
        output_file = self.output_dir / filename
        data = {
            "timestamp": datetime.utcnow().isoformat(),
            "metrics": [asdict(m) for m in self.metrics]
        }
        output_file.write_text(json.dumps(data, indent=2))
        logger.info(f"Exported {len(self.metrics)} metrics to {output_file}")
        return output_file

    def export_prometheus(self, filename: str = "metrics.prom") -> Path:
        """Export metrics in Prometheus exposition format."""
        output_file = self.output_dir / filename
        lines = []

        for metric in self.metrics:
            tags_str = ",".join(f'{k}="{v}"' for k, v in metric.tags.items())
            if tags_str:
                lines.append(f'{metric.name}{{{tags_str}}} {metric.value}')
            else:
                lines.append(f'{metric.name} {metric.value}')

        output_file.write_text("\n".join(lines))
        return output_file

    def clear(self) -> None:
        """Clear all collected metrics."""
        self.metrics.clear()


@contextmanager
def timed_execution(name: str, collector: Optional[MetricsCollector] = None):
    """Context manager for timing code execution."""
    start = time.perf_counter()
    duration = 0.0

    try:
        yield duration
    finally:
        duration = (time.perf_counter() - start) * 1000
        logger.info(f"{name} completed in {duration:.2f}ms")

        if collector:
            collector.histogram(f"{name}_duration_ms", duration)


def trace_function(collector: Optional[MetricsCollector] = None):
    """Decorator for tracing function execution."""
    def decorator(func: Callable) -> Callable:
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            start = time.perf_counter()

            try:
                result = func(*args, **kwargs)
                status = "success"
                return result
            except Exception as e:
                status = "error"
                logger.error(f"Error in {func.__name__}: {e}")
                raise
            finally:
                duration = (time.perf_counter() - start) * 1000

                if collector:
                    collector.histogram(
                        f"function_duration_ms",
                        duration,
                        tags={"function": func.__name__, "status": status}
                    )
                    collector.counter(
                        f"function_calls_total",
                        tags={"function": func.__name__, "status": status}
                    )

                logger.debug(f"{func.__name__} [{status}] took {duration:.2f}ms")

        return wrapper
    return decorator


class HealthCheck:
    """Simple health check system."""

    def __init__(self):
        self.checks: dict[str, Callable[[], tuple[bool, str]]] = {}

    def register(self, name: str, check_func: Callable[[], tuple[bool, str]]) -> None:
        """Register a health check."""
        self.checks[name] = check_func

    def run_all(self) -> dict[str, dict]:
        """Run all health checks and return results."""
        results = {}

        for name, check in self.checks.items():
            try:
                healthy, message = check()
                results[name] = {
                    "status": "healthy" if healthy else "unhealthy",
                    "message": message
                }
            except Exception as e:
                results[name] = {
                    "status": "error",
                    "message": str(e)
                }

        return results

    def export_json(self, output_file: str = "health.json") -> Path:
        """Export health check results to JSON."""
        results = self.run_all()
        output = {
            "timestamp": datetime.utcnow().isoformat(),
            "overall": "healthy" if all(
                r["status"] == "healthy" for r in results.values()
            ) else "unhealthy",
            "checks": results
        }

        path = Path(output_file)
        path.write_text(json.dumps(output, indent=2))
        return path


_global_collector: Optional[MetricsCollector] = None


def get_collector(output_dir: Optional[str] = None) -> MetricsCollector:
    """Get or create global metrics collector."""
    global _global_collector

    if _global_collector is None:
        _global_collector = MetricsCollector(output_dir)

    return _global_collector


def reset_collector() -> None:
    """Reset global metrics collector."""
    global _global_collector
    _global_collector = None
