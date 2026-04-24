"""W3C TraceContext support for distributed tracing.

Implements W3C TraceContext specification for trace propagation.

Reference: https://www.w3.org/TR/trace-context/
"""

from __future__ import annotations

import re
from dataclasses import dataclass

__all__ = [
    "TraceContext",
    "TraceContextFormat",
    "format_traceparent",
    "parse_traceparent",
]


@dataclass(frozen=True)
class TraceContext:
    """W3C TraceContext data.

    Attributes:
        version: TraceContext version (default: 00)
        trace_id: 32-hex-character trace ID
        parent_id: 16-hex-character parent/span ID
        trace_flags: 8-bit flags (0x01 = sampled)
        tracestate: Optional vendor-specific trace state
    """

    version: str = "00"
    trace_id: str = ""
    parent_id: str = ""
    trace_flags: int = 0
    tracestate: str | None = None

    @property
    def sampled(self) -> bool:
        """Check if the sampled flag is set."""
        return (self.trace_flags & 0x01) != 0

    @property
    def is_valid(self) -> bool:
        """Check if this is a valid TraceContext."""
        if not self.trace_id or not self.parent_id:
            return False
        return len(self.trace_id) == 32 and len(self.parent_id) == 16

    def to_headers(self) -> dict[str, str]:
        """Convert to HTTP headers dict."""
        headers = {"traceparent": self.format_traceparent()}
        if self.tracestate:
            headers["tracestate"] = self.tracestate
        return headers

    def format_traceparent(self) -> str:
        """Format as traceparent header value."""
        flags_hex = f"{self.trace_flags:02x}"
        return f"{self.version}-{self.trace_id}-{self.parent_id}-{flags_hex}"

    @classmethod
    def from_headers(cls, headers: dict[str, str]) -> TraceContext | None:
        """Parse TraceContext from HTTP headers."""
        traceparent = headers.get("traceparent") or headers.get("Traceparent")
        if not traceparent:
            return None

        return parse_traceparent(traceparent, headers.get("tracestate"))

    def with_new_parent_id(self, new_parent_id: str) -> TraceContext:
        """Create new context with updated parent ID (for child spans)."""
        return TraceContext(
            version=self.version,
            trace_id=self.trace_id,
            parent_id=new_parent_id,
            trace_flags=self.trace_flags,
            tracestate=self.tracestate,
        )


def parse_traceparent(
    traceparent: str, tracestate: str | None = None
) -> TraceContext | None:
    """Parse traceparent header value.

    Format: version-trace_id-parent_id-flags
    Example: 00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01

    Args:
        traceparent: The traceparent header value
        tracestate: Optional tracestate header value

    Returns:
        TraceContext if valid, None otherwise
    """
    # Basic validation
    if not traceparent or len(traceparent) < 55:  # Minimum valid length
        return None

    parts = traceparent.split("-")
    if len(parts) != 4:
        return None

    version, trace_id, parent_id, flags_hex = parts

    # Validate version (00 only for now, but 01+ accepted per spec)
    if not re.match(r"^[0-9a-f]{2}$", version, re.IGNORECASE):
        return None

    # Validate trace_id: 32 hex chars, not all zeros
    if not re.match(r"^[0-9a-f]{32}$", trace_id, re.IGNORECASE):
        return None
    if trace_id == "0" * 32:
        return None

    # Validate parent_id: 16 hex chars, not all zeros
    if not re.match(r"^[0-9a-f]{16}$", parent_id, re.IGNORECASE):
        return None
    if parent_id == "0" * 16:
        return None

    # Validate flags: 2 hex chars
    if not re.match(r"^[0-9a-f]{2}$", flags_hex, re.IGNORECASE):
        return None

    try:
        trace_flags = int(flags_hex, 16)
    except ValueError:
        return None

    return TraceContext(
        version=version.lower(),
        trace_id=trace_id.lower(),
        parent_id=parent_id.lower(),
        trace_flags=trace_flags,
        tracestate=tracestate,
    )


def format_traceparent(
    trace_id: str,
    parent_id: str,
    version: str = "00",
    trace_flags: int = 0,
) -> str:
    """Format traceparent header value.

    Args:
        trace_id: 32-character hex string
        parent_id: 16-character hex string
        version: 2-character hex version (default: 00)
        trace_flags: 8-bit flags as integer

    Returns:
        Formatted traceparent string
    """
    flags_hex = f"{trace_flags:02x}"
    return f"{version}-{trace_id}-{parent_id}-{flags_hex}"


class TraceContextFormat:
    """Trace context format configuration."""

    SIMPLE = "simple"  # X-Correlation-ID header
    W3C = "w3c"  # traceparent/tracestate headers

    @classmethod
    def is_valid(cls, format_value: str) -> bool:
        """Check if format is valid."""
        return format_value in (cls.SIMPLE, cls.W3C)
