"""Pheno Cache - Unified caching library."""
from functools import wraps
import hashlib
import json

class Cache:
    def __init__(self):
        self._store = {}
    
    def get(self, key):
        return self._store.get(key)
    
    def set(self, key, value, **kwargs):
        self._store[key] = value
    
    def delete(self, key):
        self._store.pop(key, None)
    
    def clear(self):
        self._store.clear()

_cache: Cache | None = None

def get_cache() -> Cache:
    global _cache
    if _cache is None:
        _cache = Cache()
    return _cache

def cached(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        key_data = json.dumps({"args": args, "kwargs": kwargs}, sort_keys=True)
        key = f"{func.__name__}:{hashlib.md5(key_data.encode()).hexdigest()}"
        cache = get_cache()
        result = cache.get(key)
        if result is not None:
            return result
        result = func(*args, **kwargs)
        cache.set(key, result)
        return result
    return wrapper

__all__ = ["Cache", "get_cache", "cached"]
