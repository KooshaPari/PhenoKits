"""Tests for W3C TraceContext support.

Traces to: FR-BUILTIN-003
"""


from phenotype_middleware.infrastructure.trace_context import (
    TraceContext,
    TraceContextFormat,
    format_traceparent,
    parse_traceparent,
)


class TestTraceContextParse:
    """Tests for parsing traceparent headers."""

    def test_parse_valid_traceparent(self) -> None:
        """Parse valid W3C traceparent header."""
        header = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"

        result = parse_traceparent(header)

        assert result is not None
        assert result.version == "00"
        assert result.trace_id == "0af7651916cd43dd8448eb211c80319c"
        assert result.parent_id == "b7ad6b7169203331"
        assert result.trace_flags == 1
        assert result.sampled is True

    def test_parse_traceparent_not_sampled(self) -> None:
        """Parse traceparent with sampled=0."""
        header = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-00"

        result = parse_traceparent(header)

        assert result is not None
        assert result.sampled is False
        assert result.trace_flags == 0

    def test_parse_with_tracestate(self) -> None:
        """Parse traceparent with tracestate."""
        header = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"
        tracestate = "vendor=value1,other=value2"

        result = parse_traceparent(header, tracestate)

        assert result is not None
        assert result.tracestate == tracestate

    def test_parse_invalid_format_missing_parts(self) -> None:
        """Reject traceparent with missing parts."""
        header = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331"

        result = parse_traceparent(header)

        assert result is None

    def test_parse_invalid_trace_id_all_zeros(self) -> None:
        """Reject traceparent with all-zero trace ID."""
        header = "00-00000000000000000000000000000000-b7ad6b7169203331-01"

        result = parse_traceparent(header)

        assert result is None

    def test_parse_invalid_parent_id_all_zeros(self) -> None:
        """Reject traceparent with all-zero parent ID."""
        header = "00-0af7651916cd43dd8448eb211c80319c-0000000000000000-01"

        result = parse_traceparent(header)

        assert result is None

    def test_parse_invalid_version(self) -> None:
        """Reject traceparent with invalid version."""
        header = "XX-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"

        result = parse_traceparent(header)

        assert result is None

    def test_parse_invalid_trace_id_length(self) -> None:
        """Reject traceparent with wrong trace ID length."""
        header = "00-0af7651916cd43dd8448eb211c8031-b7ad6b7169203331-01"

        result = parse_traceparent(header)

        assert result is None

    def test_parse_invalid_parent_id_length(self) -> None:
        """Reject traceparent with wrong parent ID length."""
        header = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b71692033-01"

        result = parse_traceparent(header)

        assert result is None

    def test_parse_empty_string(self) -> None:
        """Reject empty traceparent."""
        result = parse_traceparent("")

        assert result is None

    def test_parse_none_string(self) -> None:
        """Reject None traceparent."""
        result = parse_traceparent(None)  # type: ignore[arg-type]

        assert result is None


class TestTraceContextFormat:
    """Tests for formatting traceparent headers."""

    def test_format_traceparent(self) -> None:
        """Format valid traceparent from components."""
        result = format_traceparent(
            trace_id="0af7651916cd43dd8448eb211c80319c",
            parent_id="b7ad6b7169203331",
            version="00",
            trace_flags=1,
        )

        assert result == "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"

    def test_format_traceparent_not_sampled(self) -> None:
        """Format traceparent with sampled=0."""
        result = format_traceparent(
            trace_id="0af7651916cd43dd8448eb211c80319c",
            parent_id="b7ad6b7169203331",
            trace_flags=0,
        )

        assert result == "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-00"

    def test_format_traceparent_custom_flags(self) -> None:
        """Format traceparent with custom flags."""
        result = format_traceparent(
            trace_id="0af7651916cd43dd8448eb211c80319c",
            parent_id="b7ad6b7169203331",
            trace_flags=0x03,  # sampled + other flags
        )

        assert result == "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-03"


class TestTraceContextDataclass:
    """Tests for TraceContext dataclass behavior."""

    def test_default_values(self) -> None:
        """Create TraceContext with defaults."""
        ctx = TraceContext()

        assert ctx.version == "00"
        assert ctx.trace_id == ""
        assert ctx.parent_id == ""
        assert ctx.trace_flags == 0
        assert ctx.tracestate is None
        assert ctx.sampled is False

    def test_is_valid_true(self) -> None:
        """Check valid TraceContext."""
        ctx = TraceContext(
            trace_id="0af7651916cd43dd8448eb211c80319c",
            parent_id="b7ad6b7169203331",
        )

        assert ctx.is_valid is True

    def test_is_valid_empty(self) -> None:
        """Check invalid (empty) TraceContext."""
        ctx = TraceContext()

        assert ctx.is_valid is False

    def test_is_valid_wrong_length(self) -> None:
        """Check invalid (wrong length) TraceContext."""
        ctx = TraceContext(
            trace_id="tooshort",
            parent_id="b7ad6b7169203331",
        )

        assert ctx.is_valid is False

    def test_sampled_true(self) -> None:
        """Check sampled flag when set."""
        ctx = TraceContext(trace_flags=1)

        assert ctx.sampled is True

    def test_sampled_false(self) -> None:
        """Check sampled flag when not set."""
        ctx = TraceContext(trace_flags=0)

        assert ctx.sampled is False

    def test_format_traceparent_method(self) -> None:
        """Use format_traceparent method."""
        ctx = TraceContext(
            version="00",
            trace_id="0af7651916cd43dd8448eb211c80319c",
            parent_id="b7ad6b7169203331",
            trace_flags=1,
        )

        result = ctx.format_traceparent()

        assert result == "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"

    def test_to_headers(self) -> None:
        """Convert to HTTP headers dict."""
        ctx = TraceContext(
            trace_id="0af7651916cd43dd8448eb211c80319c",
            parent_id="b7ad6b7169203331",
            trace_flags=1,
        )

        headers = ctx.to_headers()

        assert headers["traceparent"] == "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"
        assert "tracestate" not in headers

    def test_to_headers_with_tracestate(self) -> None:
        """Convert to headers with tracestate."""
        ctx = TraceContext(
            trace_id="0af7651916cd43dd8448eb211c80319c",
            parent_id="b7ad6b7169203331",
            trace_flags=1,
            tracestate="vendor=value",
        )

        headers = ctx.to_headers()

        assert headers["tracestate"] == "vendor=value"

    def test_from_headers(self) -> None:
        """Parse TraceContext from headers."""
        headers = {
            "traceparent": "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"
        }

        ctx = TraceContext.from_headers(headers)

        assert ctx is not None
        assert ctx.trace_id == "0af7651916cd43dd8448eb211c80319c"

    def test_from_headers_case_insensitive(self) -> None:
        """Parse from headers with different case."""
        headers = {
            "Traceparent": "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"
        }

        ctx = TraceContext.from_headers(headers)

        assert ctx is not None

    def test_from_headers_missing(self) -> None:
        """Return None when traceparent missing."""
        headers = {}

        ctx = TraceContext.from_headers(headers)

        assert ctx is None

    def test_with_new_parent_id(self) -> None:
        """Create child context with new parent ID."""
        parent = TraceContext(
            trace_id="0af7651916cd43dd8448eb211c80319c",
            parent_id="b7ad6b7169203331",
            trace_flags=1,
            tracestate="vendor=value",
        )

        child = parent.with_new_parent_id("1111111111111111")

        assert child.trace_id == parent.trace_id  # Same trace
        assert child.parent_id == "1111111111111111"  # New span ID
        assert child.trace_flags == parent.trace_flags
        assert child.tracestate == parent.tracestate


class TestTraceContextFormatEnum:
    """Tests for TraceContextFormat class."""

    def test_simple_format(self) -> None:
        """SIMPLE format constant."""
        assert TraceContextFormat.SIMPLE == "simple"

    def test_w3c_format(self) -> None:
        """W3C format constant."""
        assert TraceContextFormat.W3C == "w3c"

    def test_is_valid_true(self) -> None:
        """Check valid formats."""
        assert TraceContextFormat.is_valid("simple") is True
        assert TraceContextFormat.is_valid("w3c") is True

    def test_is_valid_false(self) -> None:
        """Check invalid format."""
        assert TraceContextFormat.is_valid("invalid") is False
        assert TraceContextFormat.is_valid("") is False
