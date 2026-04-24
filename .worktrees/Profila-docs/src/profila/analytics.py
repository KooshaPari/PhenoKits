"""Analytics module for Profila.

Provides anonymized usage analytics and metrics collection.
"""

from __future__ import annotations

import hashlib
import platform
import sys
from dataclasses import dataclass, asdict
from datetime import datetime
from typing import Optional


@dataclass
class AnalyticsEvent:
    """A single analytics event."""
    event_type: str
    timestamp: str
    properties: dict
    anonymous_id: str


class AnalyticsCollector:
    """Collects anonymized analytics data."""

    def __init__(self, user_id: Optional[str] = None):
        # Create anonymous user ID from machine characteristics
        self.anonymous_id = self._generate_anonymous_id(user_id)
        self.events: list[AnalyticsEvent] = []

    def _generate_anonymous_id(self, user_id: Optional[str] = None) -> str:
        """Generate anonymous user ID from non-identifying info."""
        if user_id:
            return hashlib.sha256(user_id.encode()).hexdigest()[:16]

        # Use non-identifying characteristics
        characteristics = f"{platform.system()}-{platform.machine()}-{sys.version_info[:2]}"
        return hashlib.sha256(characteristics.encode()).hexdigest()[:16]

    def track(self, event_type: str, properties: Optional[dict] = None) -> None:
        """Track an analytics event.

        Args:
            event_type: Type of event (e.g., 'analyze_complete')
            properties: Optional event properties
        """
        event = AnalyticsEvent(
            event_type=event_type,
            timestamp=datetime.utcnow().isoformat(),
            properties=properties or {},
            anonymous_id=self.anonymous_id
        )
        self.events.append(event)

    def track_analyze_start(self, file_count: int, directory: str) -> None:
        """Track when analysis starts."""
        self.track("analyze_start", {
            "file_count": file_count,
            "directory_type": "local" if directory.startswith("/") else "remote"
        })

    def track_analyze_complete(self, duration_ms: float, function_count: int,
                              avg_complexity: float) -> None:
        """Track when analysis completes."""
        self.track("analyze_complete", {
            "duration_ms": duration_ms,
            "function_count": function_count,
            "avg_complexity": round(avg_complexity, 2)
        })

    def track_error(self, error_type: str, message: str) -> None:
        """Track an error."""
        self.track("error", {
            "error_type": error_type,
            "message_hash": hashlib.sha256(message.encode()).hexdigest()[:8]
        })

    def export(self) -> list[dict]:
        """Export all events as dictionaries."""
        return [asdict(e) for e in self.events]

    def clear(self) -> None:
        """Clear all collected events."""
        self.events.clear()


# Global collector
_analytics: Optional[AnalyticsCollector] = None


def get_analytics() -> AnalyticsCollector:
    """Get or create global analytics collector."""
    global _analytics
    if _analytics is None:
        _analytics = AnalyticsCollector()
    return _analytics
