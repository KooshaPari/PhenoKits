"""Pheno Retry - Unified retry logic (tenacity)."""
from tenacity import retry, stop_after_attempt, wait_exponential, RetryError

def retry_with_backoff(max_attempts=3, min_wait=2, max_wait=10):
    return retry(
        stop=stop_after_attempt(max_attempts),
        wait=wait_exponential(multiplier=1, min=min_wait, max=max_wait),
        reraise=True
    )

async def async_retry(max_attempts=3):
    def decorator(func):
        async def wrapper(*args, **kwargs):
            last_error = None
            for attempt in range(max_attempts):
                try:
                    return await func(*args, **kwargs)
                except Exception as e:
                    last_error = e
                    if attempt < max_attempts - 1:
                        import asyncio
                        await asyncio.sleep(2 ** attempt)
            raise last_error
        return wrapper
    return decorator

__all__ = ["retry", "stop_after_attempt", "wait_exponential", "RetryError", "retry_with_backoff", "async_retry"]
