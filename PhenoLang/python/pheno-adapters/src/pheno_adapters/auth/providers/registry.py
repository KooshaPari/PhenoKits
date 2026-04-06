"""Registry utilities for authentication providers in the ``pheno`` namespace.

This module replaces ``pheno.auth.providers.registry`` while keeping identical
behaviour for provider registration and lookup.
"""

from __future__ import annotations

from typing import Any

from pheno.domain.auth.types import ConfigurationError
from pheno.ports.auth.providers import AuthProvider


class ProviderRegistry:
    """
    Central in-memory registry for authentication providers.
    """

    def __init__(self) -> None:
        self._providers: dict[str, type[AuthProvider]] = {}

    def register_provider(self, name: str, provider_class: type[AuthProvider]) -> None:
        if not issubclass(provider_class, AuthProvider):
            raise ValueError("Provider must inherit from AuthProvider")
        self._providers[name] = provider_class

    def get_provider_class(self, name: str) -> type[AuthProvider] | None:
        return self._providers.get(name)

    def create_provider(self, name: str, config: dict[str, Any]) -> AuthProvider:
        provider_class = self.get_provider_class(name)
        if not provider_class:
            raise ConfigurationError(f"Provider '{name}' not found")
        try:
            return provider_class(name, config)
        except Exception as exc:  # pragma: no cover - adapter failures bubble up
            raise ConfigurationError(f"Failed to create provider '{name}': {exc}") from exc

    def list_providers(self) -> list[str]:
        return list(self._providers.keys())

    def unregister_provider(self, name: str) -> None:
        self._providers.pop(name, None)

    def clear(self) -> None:
        self._providers.clear()


_global_registry = ProviderRegistry()


def get_registry() -> ProviderRegistry:
    return _global_registry


def register_provider(name: str, provider_class: type[AuthProvider]) -> None:
    _global_registry.register_provider(name, provider_class)


def create_provider(name: str, config: dict[str, Any]) -> AuthProvider:
    return _global_registry.create_provider(name, config)


__all__ = [
    "ProviderRegistry",
    "create_provider",
    "get_registry",
    "register_provider",
]
