"""
Infrastructure adapters for external services.

These adapters implement the ports defined in the application layer.

xDD Principles:
- Dependency inversion: adapters depend on ports (abstractions)
- Adapter pattern: wraps external services
- Single responsibility: one adapter per external service
"""

import sys
import json
from datetime import datetime, timezone
from typing import Optional
from phenotype_middleware.domain import Request, Response, MiddlewareResult
from phenotype_middleware.ports import LoggingPort, MetricsPort, MiddlewarePort
from phenotype_middleware.infrastructure.builtin import (
    AuthMiddleware,
    CompressionMiddleware,
    ConditionalMiddleware,
    RateLimitMiddleware,
    RetryMiddleware,
    SyncMiddlewareAdapter,
    TracingMiddleware,
)

__all__ = [
    "AuthMiddleware",
    "CompressionMiddleware",
    "ConditionalMiddleware",
    "LoggingMiddleware",
    "MetricsMiddleware",
    "PrometheusMetricsAdapter",
    "RateLimitMiddleware",
    "RetryMiddleware",
    "StdoutLoggingAdapter",
    "SyncMiddlewareAdapter",
    "TracingMiddleware",
]


class StdoutLoggingAdapter(LoggingPort):
    """
    Logging adapter that writes to stdout.

    Simple for development; production should use structured logging.
    """

    def __init__(self, min_level: str = "INFO"):
        self.min_level = min_level
        self.levels = {"DEBUG": 0, "INFO": 1, "WARNING": 2, "ERROR": 3}

    async def log(self, level: str, message: str, context: dict | None = None) -> None:
        if self.levels.get(level, 0) >= self.levels.get(self.min_level, 1):
            timestamp = datetime.now(timezone.utc).isoformat()
            entry = {
                "timestamp": timestamp,
                "level": level,
                "message": message,
                "context": context or {},
            }
            print(json.dumps(entry), file=sys.stdout)


class PrometheusMetricsAdapter(MetricsPort):
    """
    Metrics adapter that collects in-memory metrics.

    For production, integrate with Prometheus client library.
    """

    def __init__(self) -> None:
        self._counters: dict[str, float] = {}
        self._gauges: dict[str, float] = {}
        self._histograms: dict[str, list[float]] = {}

    async def record(self, name: str, value: float, labels: dict | None = None) -> None:
        # Simple in-memory implementation
        key = self._make_key(name, labels)
        self._counters[key] = self._counters.get(key, 0) + value

    def _make_key(self, name: str, labels: dict | None) -> str:
        if not labels:
            return name
        label_str = ",".join(f"{k}={v}" for k, v in sorted(labels.items()))
        return f"{name}{{{label_str}}}"

    def get_counter(self, name: str, labels: dict | None = None) -> float:
        key = self._make_key(name, labels)
        return self._counters.get(key, 0)


class LoggingMiddleware(MiddlewarePort):
    """
    Middleware that logs requests and responses.

    Example of implementing MiddlewarePort.
    """

    def __init__(self, logger: LoggingPort):
        self.logger = logger

    async def process(self, request: Request) -> MiddlewareResult:
        await self.logger.log(
            "INFO",
            f"Request: {request.method} {request.path}",
            {"method": request.method, "path": request.path}
        )
        return MiddlewareResult.ok(request=request)


class MetricsMiddleware(MiddlewarePort):
    """
    Middleware that records request metrics.

    Records timing and counts for observability.
    """

    def __init__(self, metrics: MetricsPort):
        self.metrics = metrics

    async def process(self, request: Request) -> MiddlewareResult:
        await self.metrics.record(
            "requests_total",
            1,
            {"method": request.method, "path": request.path}
        )
        return MiddlewareResult.ok(request=request)
