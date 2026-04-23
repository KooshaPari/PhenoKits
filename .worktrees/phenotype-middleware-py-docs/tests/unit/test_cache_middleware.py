"""Tests for CacheMiddleware.

Traces to: FR-BUILTIN-006
"""

import asyncio

import pytest

from phenotype_middleware.domain import MiddlewareResult, Request, Response
from phenotype_middleware.infrastructure.builtin import CacheMiddleware


class TestCacheMiddlewareBasics:
    """Basic caching functionality tests."""

    @pytest.mark.asyncio
    async def test_cache_miss_returns_ok(self) -> None:
        """Cache miss should return ok to continue chain."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET")

        result = await cache.process(request)

        assert result.success is True
        assert result.response is None
        assert "_cache_key" in request.context

    @pytest.mark.asyncio
    async def test_cache_hit_returns_cached_response(self) -> None:
        """Cache hit should return cached response."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET")

        # First request - cache miss
        await cache.process(request)

        # Simulate response being cached
        response = Response(status_code=200, body=b'{"data": "test"}')
        await cache.cache_response(request, response)

        # Second request - should hit cache
        request2 = Request(path="/api/data", method="GET")
        result = await cache.process(request2)

        assert result.success is True
        assert result.response is not None
        assert result.response.status_code == 200
        assert result.response.headers.get("X-Cache") == "HIT"

    @pytest.mark.asyncio
    async def test_non_cacheable_method_passes_through(self) -> None:
        """POST requests should not be cached."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="POST", body=b'{"update": true}')

        result = await cache.process(request)

        assert result.success is True
        assert result.response is None
        assert "_cache_key" not in request.context

    @pytest.mark.asyncio
    async def test_head_requests_cacheable(self) -> None:
        """HEAD requests should be cacheable."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="HEAD")

        await cache.process(request)

        assert "_cache_key" in request.context


class TestCacheTTL:
    """TTL and expiration tests."""

    @pytest.mark.asyncio
    async def test_cache_entry_expires(self) -> None:
        """Cached entries should expire after TTL."""
        cache = CacheMiddleware(ttl_seconds=0.1)  # 100ms TTL
        request = Request(path="/api/data", method="GET")

        # Cache a response
        await cache.process(request)
        response = Response(status_code=200, body=b'{"data": "test"}')
        await cache.cache_response(request, response)

        # Should hit cache immediately
        request2 = Request(path="/api/data", method="GET")
        result = await cache.process(request2)
        assert result.response is not None

        # Wait for expiration
        await asyncio.sleep(0.15)

        # Should miss cache after expiration
        request3 = Request(path="/api/data", method="GET")
        result = await cache.process(request3)
        assert result.response is None  # Cache miss

    @pytest.mark.asyncio
    async def test_different_ttl_values(self) -> None:
        """Different TTL values should work."""
        short_cache = CacheMiddleware(ttl_seconds=1)
        long_cache = CacheMiddleware(ttl_seconds=3600)

        assert short_cache.ttl_seconds == 1
        assert long_cache.ttl_seconds == 3600


class TestCacheKeyGeneration:
    """Cache key generation tests."""

    @pytest.mark.asyncio
    async def test_default_key_includes_method_and_path(self) -> None:
        """Default key should include method and path."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET")

        await cache.process(request)

        cache_key = request.context["_cache_key"]
        assert "GET" in cache_key
        assert "/api/data" in cache_key

    @pytest.mark.asyncio
    async def test_different_paths_different_keys(self) -> None:
        """Different paths should have different cache keys."""
        cache = CacheMiddleware(ttl_seconds=60)
        request1 = Request(path="/api/data", method="GET")
        request2 = Request(path="/api/other", method="GET")

        await cache.process(request1)
        await cache.process(request2)

        assert request1.context["_cache_key"] != request2.context["_cache_key"]

    @pytest.mark.asyncio
    async def test_custom_key_extractor(self) -> None:
        """Custom key extractor should be used."""
        custom_key = "custom-key-value"
        cache = CacheMiddleware(
            ttl_seconds=60,
            key_extractor=lambda r: custom_key
        )
        request = Request(path="/api/data", method="GET")

        await cache.process(request)

        assert request.context["_cache_key"] == custom_key

    @pytest.mark.asyncio
    async def test_key_includes_body_hash_for_post(self) -> None:
        """Cache key should include body hash for requests with body."""
        cache = CacheMiddleware(
            ttl_seconds=60,
            cacheable_methods={"POST"}  # Allow POST caching
        )
        request = Request(
            path="/api/data",
            method="POST",
            body=b'{"query": "test"}'
        )

        await cache.process(request)

        cache_key = request.context["_cache_key"]
        # Key should have 3 parts: method, path, body_hash
        parts = cache_key.split(":")
        assert len(parts) == 3


class TestCacheableResponses:
    """Response caching criteria tests."""

    @pytest.mark.asyncio
    async def test_only_successful_responses_cached(self) -> None:
        """Only 2xx responses should be cached."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET")

        # Cache 200 response
        await cache.process(request)
        success_response = Response(status_code=200, body=b'{"data": "ok"}')
        await cache.cache_response(request, success_response)

        # Should hit cache
        request2 = Request(path="/api/data", method="GET")
        result = await cache.process(request2)
        assert result.response is not None

    @pytest.mark.asyncio
    async def test_error_responses_not_cached(self) -> None:
        """Error responses should not be cached."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET")

        # Try to cache 500 response
        await cache.process(request)
        error_response = Response(status_code=500, body=b'{"error": "server error"}')
        await cache.cache_response(request, error_response)

        # Should NOT hit cache
        request2 = Request(path="/api/data", method="GET")
        result = await cache.process(request2)
        assert result.response is None  # Not cached

    @pytest.mark.asyncio
    async def test_custom_cacheable_statuses(self) -> None:
        """Custom cacheable statuses should be respected."""
        cache = CacheMiddleware(
            ttl_seconds=60,
            cacheable_statuses={200, 404}  # Cache 404s too
        )
        request = Request(path="/api/data", method="GET")

        # Cache 404 response
        await cache.process(request)
        not_found_response = Response(status_code=404, body=b'{"error": "not found"}')
        await cache.cache_response(request, not_found_response)

        # Should hit cache
        request2 = Request(path="/api/data", method="GET")
        result = await cache.process(request2)
        assert result.response is not None
        assert result.response.status_code == 404


class TestCacheStats:
    """Cache statistics tests."""

    @pytest.mark.asyncio
    async def test_cache_stats_initially_zero(self) -> None:
        """Initial stats should all be zero."""
        cache = CacheMiddleware(ttl_seconds=60)
        stats = cache.get_stats()

        assert stats["hits"] == 0
        assert stats["misses"] == 0
        assert stats["size"] == 0

    @pytest.mark.asyncio
    async def test_stats_increment_on_hit_and_miss(self) -> None:
        """Stats should track hits and misses."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET")

        # First request - process (sets up for caching)
        await cache.process(request)
        # Misses only counted when response is cached
        stats = cache.get_stats()
        assert stats["misses"] == 0  # Not yet cached
        assert stats["hits"] == 0

        # Cache a response - this counts as a miss (store operation)
        response = Response(status_code=200, body=b'{"data": "test"}')
        await cache.cache_response(request, response)
        stats = cache.get_stats()
        assert stats["misses"] == 1  # Now counted
        assert stats["hits"] == 0

        # Second request - hit
        request2 = Request(path="/api/data", method="GET")
        await cache.process(request2)
        stats = cache.get_stats()
        assert stats["misses"] == 1
        assert stats["hits"] == 1

    @pytest.mark.asyncio
    async def test_stats_size_tracks_cache_entries(self) -> None:
        """Size stat should track number of cached entries."""
        cache = CacheMiddleware(ttl_seconds=60)

        # Cache multiple responses
        for i in range(3):
            request = Request(path=f"/api/data/{i}", method="GET")
            await cache.process(request)
            response = Response(status_code=200, body=b'{"data": "test"}')
            await cache.cache_response(request, response)

        stats = cache.get_stats()
        assert stats["size"] == 3


class TestCacheInvalidation:
    """Cache invalidation tests."""

    @pytest.mark.asyncio
    async def test_invalidate_all_clears_cache(self) -> None:
        """Invalidate all should clear all entries."""
        cache = CacheMiddleware(ttl_seconds=60)

        # Add some entries
        for i in range(3):
            request = Request(path=f"/api/data/{i}", method="GET")
            await cache.process(request)
            response = Response(status_code=200, body=b'{"data": "test"}')
            await cache.cache_response(request, response)

        assert cache.get_stats()["size"] == 3

        # Invalidate all
        count = cache.invalidate()
        assert count == 3
        assert cache.get_stats()["size"] == 0

    @pytest.mark.asyncio
    async def test_invalidate_by_pattern(self) -> None:
        """Invalidate by pattern should only match specific entries."""
        cache = CacheMiddleware(ttl_seconds=60)

        # Add entries with different paths
        request1 = Request(path="/api/users/1", method="GET")
        request2 = Request(path="/api/users/2", method="GET")
        request3 = Request(path="/api/posts/1", method="GET")

        for req in [request1, request2, request3]:
            await cache.process(req)
            response = Response(status_code=200, body=b'{"data": "test"}')
            await cache.cache_response(req, response)

        assert cache.get_stats()["size"] == 3

        # Invalidate only user entries
        count = cache.invalidate(pattern="/api/users")
        assert count == 2
        assert cache.get_stats()["size"] == 1

    @pytest.mark.asyncio
    async def test_invalidate_no_match_returns_zero(self) -> None:
        """Invalidating non-matching pattern should return 0."""
        cache = CacheMiddleware(ttl_seconds=60)

        request = Request(path="/api/data", method="GET")
        await cache.process(request)
        response = Response(status_code=200, body=b'{"data": "test"}')
        await cache.cache_response(request, response)

        count = cache.invalidate(pattern="/nonexistent")
        assert count == 0
        assert cache.get_stats()["size"] == 1


class TestCacheConfiguration:
    """Configuration option tests."""

    def test_default_ttl_is_300_seconds(self) -> None:
        """Default TTL should be 5 minutes."""
        cache = CacheMiddleware()
        assert cache.ttl_seconds == 300

    def test_custom_cacheable_methods(self) -> None:
        """Custom cacheable methods should be respected."""
        cache = CacheMiddleware(
            ttl_seconds=60,
            cacheable_methods={"GET", "POST", "PUT"}
        )

        assert cache.cacheable_methods == {"GET", "POST", "PUT"}

    @pytest.mark.asyncio
    async def test_post_cacheable_when_configured(self) -> None:
        """POST should be cacheable when configured."""
        cache = CacheMiddleware(
            ttl_seconds=60,
            cacheable_methods={"POST"}
        )
        request = Request(path="/api/data", method="POST", body=b'{"query": "test"}')

        await cache.process(request)

        assert "_cache_key" in request.context

    def test_default_cacheable_methods(self) -> None:
        """Default cacheable methods should be GET and HEAD."""
        cache = CacheMiddleware()
        assert cache.cacheable_methods == {"GET", "HEAD"}

    def test_default_cacheable_statuses(self) -> None:
        """Default cacheable statuses should be 200-299."""
        cache = CacheMiddleware()
        assert cache.cacheable_statuses == set(range(200, 300))


class TestCacheEdgeCases:
    """Edge case tests."""

    @pytest.mark.asyncio
    async def test_concurrent_cache_access(self) -> None:
        """Cache should handle concurrent access safely."""
        cache = CacheMiddleware(ttl_seconds=60)

        async def access_cache(i: int) -> MiddlewareResult:
            request = Request(path="/api/data", method="GET")
            return await cache.process(request)

        # Run many concurrent accesses
        results = await asyncio.gather(*[access_cache(i) for i in range(100)])

        # All should succeed
        assert all(r.success for r in results)

    @pytest.mark.asyncio
    async def test_empty_body_does_not_affect_key(self) -> None:
        """Empty body should not add hash to key."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET", body=b"")

        await cache.process(request)

        # Key should only have 2 parts (method, path)
        cache_key = request.context["_cache_key"]
        parts = cache_key.split(":")
        assert len(parts) == 2

    @pytest.mark.asyncio
    async def test_none_body_does_not_affect_key(self) -> None:
        """None body should not add hash to key."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET")

        await cache.process(request)

        # Key should only have 2 parts (method, path)
        cache_key = request.context["_cache_key"]
        parts = cache_key.split(":")
        assert len(parts) == 2

    @pytest.mark.asyncio
    async def test_cache_response_without_prior_process(self) -> None:
        """cache_response should work even without process being called first."""
        cache = CacheMiddleware(ttl_seconds=60)
        request = Request(path="/api/data", method="GET")
        response = Response(status_code=200, body=b'{"data": "test"}')

        # Should not raise
        await cache.cache_response(request, response)

        # Response should be cached
        request2 = Request(path="/api/data", method="GET")
        result = await cache.process(request2)
        assert result.response is not None
