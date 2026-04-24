"""Shared API authorization helpers."""

from __future__ import annotations

import inspect
from typing import Any

from tracertm.api.middleware.auth import (
    check_permissions,
    check_project_access,
    ensure_credential_access,
    ensure_project_access,
    ensure_read_permission,
    ensure_write_permission,
    is_system_admin,
    verify_token,
)


async def _maybe_await(value: object) -> object:
    """Await values only when needed."""
    if inspect.isawaitable(value):
        return await value
    return value


__all__ = [
    "_maybe_await",
    "Any",
    "check_permissions",
    "check_project_access",
    "ensure_credential_access",
    "ensure_project_access",
    "ensure_read_permission",
    "ensure_write_permission",
    "is_system_admin",
    "verify_token",
]
