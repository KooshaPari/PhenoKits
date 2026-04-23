"""Pheno HTTP - Unified HTTP client (httpx)."""
import httpx

class HTTPClient:
    def __init__(self, base_url=None, timeout=30.0):
        self._client = httpx.AsyncClient(base_url=base_url or "http://localhost", timeout=timeout)
    
    async def get(self, url, **kwargs):
        return await self._client.get(url, **kwargs)
    
    async def post(self, url, **kwargs):
        return await self._client.post(url, **kwargs)
    
    async def request(self, method, url, **kwargs):
        return await self._client.request(method, url, **kwargs)
    
    async def __aenter__(self):
        return self
    
    async def __aexit__(self, *args):
        await self._client.aclose()

def get_http_client(base_url=None, timeout=30.0):
    return HTTPClient(base_url=base_url, timeout=timeout)

__all__ = ["HTTPClient", "get_http_client"]
