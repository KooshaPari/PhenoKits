"""Unified HTTP client using httpx."""
from __future__ import annotations
from typing import Any
import httpx

class HTTPClient:
    def __init__(self, base_url: str | None = None, timeout: float = 30.0, headers: dict | None = None):
        self._client = httpx.AsyncClient(base_url=base_url or httpx.URL("http://localhost"), timeout=timeout, headers=headers or {})
    
    async def get(self, url: str, **kwargs): return await self._client.get(url, **kwargs)
    async def post(self, url: str, **kwargs): return await self._client.post(url, **kwargs)
    async def request(self, method: str, url: str, **kwargs): return await self._client.request(method, url, **kwargs)
    async def __aenter__(self): return self
    async def __aexit__(self, *args): await self._client.aclose()

def get_http_client(base_url: str | None = None, timeout: float = 30.0) -> HTTPClient:
    return HTTPClient(base_url=base_url, timeout=timeout)
