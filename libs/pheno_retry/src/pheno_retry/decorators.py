"""Retry decorators."""
from __future__ import annotations
from functools import wraps
from typing import Any, Callable, TypeVar
import asyncio
T = TypeVar("T")

def retry_with_backoff(max_attempts: int = 3, min_wait: int = 2, max_wait: int = 10):
    from tenacity import retry, stop_after_attempt, wait_exponential
    return retry(stop=stop_after_attempt(max_attempts), wait=wait_exponential(multiplier=1, min=min_wait, max=max_wait), reraise=True)

def async_retry(max_attempts: int = 3):
    def decorator(func: Callable[..., T]) -> Callable[..., T]:
        @wraps(func)
        async def wrapper(*args, **kwargs):
            last_error = None
            for attempt in range(max_attempts):
                try: return await func(*args, **kwargs)
                except Exception as e:
                    last_error = e
                    if attempt < max_attempts - 1: await asyncio.sleep(2 ** attempt)
            raise last_error
        return wrapper
    return decorator
