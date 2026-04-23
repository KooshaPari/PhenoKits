"""Contract tests for structured logging following BDD principles.

These tests verify the contract of the structured logging API
using a behavior-driven approach with Given-When-Then scenarios.

Example:
    $ pytest tests/contract/test_logging_contract.py -v
"""
from __future__ import annotations

import json
import logging
import sys
from io import StringIO
from typing import Any

import pytest

from phenotype_kit.logging import configure_logging, get_logger, StructuredFormatter


class TestLoggingConfigurationContract:
    """Contract tests for configure_logging function behavior.

    Given: a fresh logging configuration
    When: configure_logging is called with specific parameters
    Then: logging should behave according to the contract
    """

    def test_configure_logging_sets_json_output_by_default(self) -> None:
        """JSON output should be enabled by default."""
        # Capture log output
        output = StringIO()
        handler = logging.StreamHandler(output)

        # Configure with JSON output
        configure_logging(level="DEBUG", json_output=True)

        # Get logger and emit a test message
        logger = get_logger("test")
        logger.info("test message", key="value")

        # Verify JSON was written
        log_content = output.getvalue()
        parsed = json.loads(log_content)

        assert parsed["message"] == "test message"
        assert "level" in parsed
        assert "timestamp" in parsed

    def test_configure_logging_allows_plaintext_output(self) -> None:
        """Plaintext output should be used when json_output=False."""
        output = StringIO()
        handler = logging.StreamHandler(output)

        # Configure with plaintext output
        configure_logging(level="DEBUG", json_output=False)

        # Get logger and emit a test message
        logger = get_logger("test")
        logger.info("plaintext message")

        # Verify plaintext was written
        log_content = output.getvalue()
        assert "plaintext message" in log_content

    def test_configure_logging_accepts_service_name(self) -> None:
        """Service name should be included in all log entries."""
        output = StringIO()
        handler = logging.StreamHandler(output)

        # Configure with service name
        configure_logging(level="DEBUG", service_name="test-service")

        # Get logger and emit a test message
        logger = get_logger("test")
        logger.info("service test")

        # Verify service name in output
        log_content = output.getvalue()
        parsed = json.loads(log_content)

        assert parsed["service"] == "test-service"

    def test_configure_logging_respects_log_level(self) -> None:
        """Messages below the configured level should be suppressed."""
        output = StringIO()
        handler = logging.StreamHandler(output)

        # Configure with WARNING level
        configure_logging(level="WARNING", json_output=True)

        # Get logger and emit DEBUG (should be suppressed)
        logger = get_logger("test")
        logger.debug("should not appear")

        # No output should be produced
        assert output.getvalue() == ""


class TestStructuredFormatterContract:
    """Contract tests for StructuredFormatter behavior.

    Given: a StructuredFormatter instance
    When: format is called with a log record
    Then: the output should conform to the JSON log schema
    """

    def test_formatter_includes_required_fields(self) -> None:
        """Formatter output must include timestamp, level, logger, message."""
        formatter = StructuredFormatter()
        record = logging.LogRecord(
            name="test.logger",
            level=logging.INFO,
            pathname="test.py",
            lineno=10,
            msg="test message",
            args=(),
            exc_info=None,
        )

        formatted = formatter.format(record)
        parsed = json.loads(formatted)

        assert "timestamp" in parsed
        assert "level" in parsed
        assert "logger" in parsed
        assert "message" in parsed

    def test_formatter_includes_service_name_when_provided(self) -> None:
        """Formatter should include service field when service_name is set."""
        formatter = StructuredFormatter(service_name="my-service")
        record = logging.LogRecord(
            name="test.logger",
            level=logging.INFO,
            pathname="test.py",
            lineno=10,
            msg="service test",
            args=(),
            exc_info=None,
        )

        formatted = formatter.format(record)
        parsed = json.loads(formatted)

        assert parsed["service"] == "my-service"

    def test_formatter_includes_extra_fields(self) -> None:
        """Formatter should include extra fields from record.__dict__."""
        formatter = StructuredFormatter()
        record = logging.LogRecord(
            name="test.logger",
            level=logging.INFO,
            pathname="test.py",
            lineno=10,
            msg="extra fields test",
            args=(),
            exc_info=None,
        )
        record.custom_field = "custom_value"

        formatted = formatter.format(record)
        parsed = json.loads(formatted)

        assert parsed["custom_field"] == "custom_value"


class TestGetLoggerContract:
    """Contract tests for get_logger function behavior.

    Given: configure_logging has been called
    When: get_logger is called with a name
    Then: a logger with the correct name should be returned
    """

    def test_get_logger_returns_structlog_logger(self) -> None:
        """get_logger should return a structlog BoundLogger."""
        configure_logging(level="DEBUG", json_output=False)

        logger = get_logger("test_module")

        # Should have info method (structlog bound logger)
        assert hasattr(logger, "info")
        assert hasattr(logger, "debug")
        assert hasattr(logger, "warning")
        assert hasattr(logger, "error")

    def test_get_logger_includes_context_in_messages(self) -> None:
        """Logger should include bound context in messages."""
        output = StringIO()
        handler = logging.StreamHandler(output)

        configure_logging(level="DEBUG", json_output=True)

        logger = get_logger("context_test")
        logger.bind(user_id="123", action="login").info("user action")

        log_content = output.getvalue()
        parsed = json.loads(log_content)

        assert parsed["user_id"] == "123"
        assert parsed["action"] == "login"
        assert parsed["message"] == "user action"


class TestLoggingEdgeCases:
    """Edge case tests for logging behavior."""

    def test_handles_unicode_in_message(self) -> None:
        """Logger should correctly handle Unicode characters."""
        configure_logging(level="DEBUG", json_output=True)

        logger = get_logger("unicode_test")
        logger.info("Unicode: 你好 🔬 émoji")

        # If no exception, test passes

    def test_handles_empty_message(self) -> None:
        """Logger should handle empty messages."""
        configure_logging(level="DEBUG", json_output=True)

        logger = get_logger("empty_test")
        logger.info("")

        # If no exception, test passes

    def test_handles_special_characters_in_message(self) -> None:
        """Logger should handle special characters in messages."""
        configure_logging(level="DEBUG", json_output=True)

        logger = get_logger("special_test")
        logger.info('Special chars: "quotes" \\backslash\\ /slash/')

        # If no exception, test passes
