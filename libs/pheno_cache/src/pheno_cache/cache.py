"""Unified cache with LRU/TTL/Disk tiers."""
from __future__ import annotations
import hashlib, json
from functools import wraps
from pathlib import Path
from typing import Any, TypeVar, Callable
from cachetools import LRUCache, TTLCache
from diskcache import Cache as DiskCache

T = TypeVar("T")

class Cache:
    def __init__(self, disk_path: Path | str = ".cache", lru_size: int = 1000, ttl: int = 300):
        self._disk_path = Path(disk_path)
        self._disk_path.mkdir(parents=True, exist_ok=True)
        self._lru = LRUCache(maxsize=lru_size)
        self._ttl = TTLCache(maxsize=1000, ttl=ttl)
        self._disk = DiskCache(str(self._disk_path))

    def get(self, key: str) -> Any | None:
        if key in self._lru: return self._lru[key]
        if key in self._ttl: return self._ttl[key]
        v = self._disk.get(key)
        if v is not None: self._ttl[key] = v
        return v

    def set(self, key: str, value: Any, tier: str = "ttl") -> None:
        if tier == "lru": self._lru[key] = value
        elif tier == "ttl": self._ttl[key] = value
        elif tier == "disk": self._disk.set(key, value); self._ttl[key] = value

    def delete(self, key: str) -> bool:
        deleted = False
        for c in [self._lru, self._ttl, self._disk]:
            try:
                if key in c: del c[key]; deleted = True
            except: pass
        return deleted

    def clear(self, tier: str | None = None) -> None:
        if tier is None or tier == "lru": self._lru.clear()
        if tier is None or tier == "ttl": self._ttl.clear()
        if tier is None or tier == "disk": self._disk.clear()

_default_cache: Cache | None = None

def get_cache(disk_path: Path | str = ".cache", lru_size: int = 1000) -> Cache:
    global _default_cache
    if _default_cache is None:
        _default_cache = Cache(disk_path=disk_path, lru_size=lru_size)
    return _default_cache

def cached(cache: Cache | None = None, tier: str = "ttl"):
    _cache = cache or get_cache()
    def decorator(func: Callable[..., T]) -> Callable[..., T]:
        @wraps(func)
        def wrapper(*args, **kwargs):
            key = f"{func.__name__}:{hashlib.sha256(json.dumps({'args': args, 'kwargs': kwargs}).encode()).hexdigest()}"
            result = _cache.get(key)
            if result is not None: return result
            result = func(*args, **kwargs)
            _cache.set(key, result, tier=tier)
            return result
        return wrapper
    return decorator
