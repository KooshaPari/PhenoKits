"""In-memory cache implementation.

This adapter implements the Cache port using an in-memory dictionary.
For production use, implement RedisCache or FileCache.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from portkey.application.ports import Cache

if TYPE_CHECKING:
    from portkey.domain.models import Response


class InMemoryCache(Cache):
    """Simple in-memory cache for responses.

    Note: This cache is not persistent and will be cleared on restart.
    For production, use RedisCache or another persistent implementation.
    """

    def __init__(self) -> None:
        self._cache: dict[str, Response] = {}

    def get(self, key: str) -> Response | None:
        return self._cache.get(key)

    def set(self, key: str, response: Response, ttl: int | None = None) -> None:
        self._cache[key] = response

    def delete(self, key: str) -> None:
        self._cache.pop(key, None)

    def clear(self) -> None:
        self._cache.clear()

    def __len__(self) -> int:
        return len(self._cache)

    def keys(self) -> list[str]:
        return list(self._cache.keys())
