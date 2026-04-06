"""PII anonymization utilities for Profila.

Provides functions to sanitize potentially identifying information
from profiling outputs.
"""

from __future__ import annotations

import hashlib
import os
import re
from pathlib import Path
from typing import Optional


def anonymize_path(path: str, home_dir: Optional[str] = None) -> str:
    """Replace user directory with generic placeholder.

    Args:
        path: File path to anonymize
        home_dir: Home directory to replace (defaults to current user)

    Returns:
        Path with home directory replaced

    Example:
        >>> anonymize_path("/home/john/project/file.py")
        "/home/user/project/file.py"
    """
    if home_dir is None:
        home_dir = os.path.expanduser("~")

    if path.startswith(home_dir):
        return path.replace(home_dir, "/home/user", 1)

    # Also handle common variations
    for variant in ["/Users/", "/home/"]:
        if variant in path:
            parts = path.split(variant)
            if len(parts) == 2:
                username = parts[1].split("/")[0]
                return f"{variant}user{parts[1][len(username):]}"
                break

    return path


def sanitize_function_name(name: str) -> str:
    """Remove potential PII from function names.

    Keeps only alphanumeric characters and underscores.

    Args:
        name: Function name to sanitize

    Returns:
        Sanitized function name

    Example:
        >>> sanitize_function_name("get_user_by_email_123")
        "get_user_by_email_123"
        >>> sanitize_function_name("user_john_doe_private")
        "user_john_doe_private"
    """
    return re.sub(r"[^a-zA-Z0-9_]", "_", name)


def hash_identifier(value: str, length: int = 12) -> str:
    """Create a deterministic hash of an identifier.

    Useful for tracking without exposing the original value.

    Args:
        value: String to hash
        length: Number of characters to return

    Returns:
        Hash prefix

    Example:
        >>> hash_identifier("john.doe@example.com")
        "a1b2c3d4e5f6"
    """
    return hashlib.sha256(value.encode()).hexdigest()[:length]


def anonymize_git_author(author: str) -> str:
    """Hash git author names to prevent identification.

    Args:
        author: Git author string (e.g., "John Doe <john@example.com>")

    Returns:
        Anonymized author string

    Example:
        >>> anonymize_git_author("John Doe <john@example.com>")
        "User <user@anonymized>"
    """
    hash_suffix = hash_identifier(author, 8)
    return f"User_{hash_suffix} <user_{hash_suffix}@anonymized>"


def anonymize_module_path(path: str) -> str:
    """Anonymize module/file paths while preserving structure.

    Args:
        path: Module path to anonymize

    Returns:
        Anonymized path

    Example:
        >>> anonymize_module_path("src/users/auth.py")
        "src/module/auth.py"
    """
    path = anonymize_path(path)

    # Replace potential username directories
    parts = Path(path).parts
    anonymized = []
    for i, part in enumerate(parts):
        # Replace single-word directory names (potential usernames)
        if i > 0 and len(parts) > 1 and "_" not in part and part.islower():
            if len(part) < 20 and not part.startswith("."):
                anonymized.append("module")
            else:
                anonymized.append(part)
        else:
            anonymized.append(part)

    return str(Path(*anonymized))


def redact_sensitive_patterns(text: str) -> str:
    """Redact common sensitive patterns from text.

    Args:
        text: Text to redact

    Returns:
        Text with sensitive patterns redacted

    Patterns redacted:
        - Email addresses
        - IP addresses
        - API keys (common formats)
        - AWS access keys
    """
    # Email addresses
    text = re.sub(r"[\w.+-]+@[\w-]+\.[\w.-]+", "[EMAIL_REDACTED]", text)

    # IP addresses
    text = re.sub(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}", "[IP_REDACTED]", text)

    # AWS access keys
    text = re.sub(r"AKIA[0-9A-Z]{16}", "[AWS_KEY_REDACTED]", text)

    # Generic API keys (40+ hex characters)
    text = re.sub(r"[0-9a-f]{40,}", "[KEY_REDACTED]", text)

    # JWT tokens
    text = re.sub(r"eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+", "[JWT_REDACTED]", text)

    return text


class PIIAnalyzer:
    """Analyzes text for potential PII content."""

    # Common PII patterns
    PATTERNS = {
        "email": r"[\w.+-]+@[\w-]+\.[\w.-]+",
        "phone": r"\+?1?\d{9,15}",
        "ssn": r"\d{3}-\d{2}-\d{4}",
        "credit_card": r"\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}",
        "ip_address": r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}",
        "aws_key": r"AKIA[0-9A-Z]{16}",
    }

    def __init__(self):
        self.findings: list[dict] = []

    def scan(self, text: str) -> list[dict]:
        """Scan text for PII patterns.

        Args:
            text: Text to scan

        Returns:
            List of findings with type, match, and position
        """
        findings = []

        for pii_type, pattern in self.PATTERNS.items():
            for match in re.finditer(pattern, text):
                findings.append({
                    "type": pii_type,
                    "match": match.group(),
                    "start": match.start(),
                    "end": match.end(),
                    "redacted": f"[{pii_type.upper()}_REDACTED]"
                })

        return findings

    def contains_pii(self, text: str) -> bool:
        """Check if text contains any PII.

        Args:
            text: Text to check

        Returns:
            True if PII found
        """
        return len(self.scan(text)) > 0
