"""Central FastAPI router registration."""

from __future__ import annotations

import importlib
import logging
from collections.abc import Iterable
from typing import Any

from fastapi import FastAPI

logger = logging.getLogger(__name__)

ROUTER_MODULES: tuple[str, ...] = (
    "tracertm.api.routers.adrs",
    "tracertm.api.routers.agent",
    "tracertm.api.routers.auth",
    "tracertm.api.routers.blockchain",
    "tracertm.api.routers.contracts",
    "tracertm.api.routers.errors",
    "tracertm.api.routers.execution",
    "tracertm.api.routers.features",
    "tracertm.api.routers.health",
    "tracertm.api.routers.health_canary",
    "tracertm.api.routers.item_specs",
    "tracertm.api.routers.items",
    "tracertm.api.routers.mcp",
    "tracertm.api.routers.notifications",
    "tracertm.api.routers.oauth",
    "tracertm.api.routers.projects",
    "tracertm.api.routers.quality",
    "tracertm.api.routers.websocket",
)


def _iter_module_routers(module_name: str) -> Iterable[Any]:
    module = importlib.import_module(module_name)
    for attr_name in ("router", "webhook_router"):
        router = getattr(module, attr_name, None)
        if router is not None:
            yield router


def register_api_routers(app: FastAPI) -> None:
    """Register API routers exposed by the recovered Python backend."""
    for module_name in ROUTER_MODULES:
        for router in _iter_module_routers(module_name):
            app.include_router(router)
            logger.debug("Registered API router", extra={"module": module_name})
