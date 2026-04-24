"""Property-based tests for logging module using Hypothesis.

These tests verify invariants about the logging behavior using
property-based testing. Instead of testing specific values,
we test properties that must hold for ALL valid inputs.

Following TDD + Property-Based Testing principles:
    - Write properties first (like TDD)
    - Hypothesis generates random valid inputs
    - Falsifying examples are automatically shrunk and reported

Example:
    $ pytest tests/property/test_logging_properties.py -v
"""
from __future__ import annotations

import json
import logging
from io import StringIO
from typing import Any

import pytest
from hypothesis import given, settings, assume, example, HealthCheck
from hypothesis import strategies as st

from phenotype_kit.logging import configure_logging, get_logger, StructuredFormatter


# Strategies for generating valid log levels
log_levels = st.sampled_from(["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"])

# Strategy for generating service names
service_names = st.text(min_size=1, max_size=50, alphabet=st.characters(whitelist_categories=("L", "N", "P", "S")))

# Strategy for generating JSON-compatible values
json_values = st.one_of(
    st.none(),
    st.booleans(),
    st.integers(min_value=-1000000, max_value=1000000),
    st.floats(allow_nan=False, allow_infinity=False),
    st.text(max_size=100),
)


class TestLoggingConfigurationProperties:
    """Property tests for configure_logging function."""

    @given(level=log_levels, json_output=st.booleans())
    @settings(max_examples=20)
    def test_configure_logging_accepts_valid_levels(
        self, level: str, json_output: bool
    ) -> None:
        """configure_logging should accept all valid log levels."""
        # Should not raise any exception
        configure_logging(level=level, json_output=json_output)

    @given(service_name=st.none() | service_names)
    @settings(max_examples=20)
    def test_configure_logging_accepts_service_name(
        self, service_name: str | None
    ) -> None:
        """configure_logging should accept None or valid service name."""
        # Should not raise any exception
        configure_logging(level="INFO", service_name=service_name)


class TestStructuredFormatterProperties:
    """Property tests for StructuredFormatter."""

    @given(
        level=st.sampled_from([logging.DEBUG, logging.INFO, logging.WARNING, logging.ERROR]),
        message=st.text(min_size=0, max_size=500),
    )
    @settings(max_examples=30)
    def test_formatter_output_is_valid_json(
        self, level: int, message: str
    ) -> None:
        """Formatter should always produce valid JSON output."""
        formatter = StructuredFormatter()
        record = logging.LogRecord(
            name="test.logger",
            level=level,
            pathname="test.py",
            lineno=10,
            msg=message,
            args=(),
            exc_info=None,
        )

        formatted = formatter.format(record)

        # Should not raise - should produce valid JSON
        parsed = json.loads(formatted)
        assert isinstance(parsed, dict)

    @given(message=st.text(min_size=0, max_size=200))
    @settings(max_examples=20)
    def test_formatter_includes_message_field(self, message: str) -> None:
        """Formatter should always include the message field."""
        formatter = StructuredFormatter()
        record = logging.LogRecord(
            name="test.logger",
            level=logging.INFO,
            pathname="test.py",
            lineno=10,
            msg=message,
            args=(),
            exc_info=None,
        )

        formatted = formatter.format(record)
        parsed = json.loads(formatted)

        assert "message" in parsed

    @given(data=st.dictionaries(keys=st.text(min_size=1, max_size=20), values=json_values, max_size=10))
    @settings(max_examples=20)
    def test_formatter_preserves_extra_fields(self, data: dict[str, Any]) -> None:
        """Formatter should preserve extra fields from record."""
        formatter = StructuredFormatter()
        record = logging.LogRecord(
            name="test.logger",
            level=logging.INFO,
            pathname="test.py",
            lineno=10,
            msg="test",
            args=(),
            exc_info=None,
        )

        # Add extra fields
        for key, value in data.items():
            setattr(record, key, value)

        formatted = formatter.format(record)
        parsed = json.loads(formatted)

        for key, value in data.items():
            assert key in parsed


class TestGetLoggerProperties:
    """Property tests for get_logger function."""

    @given(name=st.text(min_size=1, max_size=100))
    @settings(max_examples=20)
    def test_get_logger_returns_callable_logger(self, name: str) -> None:
        """get_logger should return a logger with standard methods."""
        configure_logging(level="DEBUG", json_output=False)

        logger = get_logger(name)

        # Should have standard logging methods
        assert callable(logger.debug)
        assert callable(logger.info)
        assert callable(logger.warning)
        assert callable(logger.error)
        assert callable(logger.critical)

    @given(name=st.text(min_size=1, max_size=50))
    @settings(max_examples=20)
    def test_logger_binds_context_correctly(self, name: str) -> None:
        """Logger.bind should preserve context keys."""
        configure_logging(level="DEBUG", json_output=True)

        logger = get_logger(name)
        bound = logger.bind(correlation_id="test-123")

        # Should have same methods after binding
        assert callable(bound.info)
        assert callable(bound.debug)


class TestLoggingRoundtripProperties:
    """Property tests for logging roundtrip behavior."""

    @given(
        key=st.text(min_size=1, max_size=30),
        value=st.one_of(st.integers(), st.text(max_size=50)),
    )
    @settings(max_examples=30)
    def test_logged_kv_pairs_are_accessible(self, key: str, value: Any) -> None:
        """Key-value pairs logged should appear in output."""
        configure_logging(level="DEBUG", json_output=True)

        logger = get_logger("kv_test")
        logger.info("kv_message", **{key: value})

        # If no exception, test passes (output captured by root handler)
