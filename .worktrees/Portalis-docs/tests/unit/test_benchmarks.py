"""Benchmark tests for cache operations.

Traces to: FR-PERF-001
"""
from __future__ import annotations

import pytest

from portkey.application.ports import Cache
from portkey.domain.models import Provider, Response
from portkey.infrastructure.cache import InMemoryCache


# Traces to: FR-PERF-001
class TestCacheBenchmark:
    """Benchmark tests for cache operations."""

    @pytest.fixture
    def large_cache(self) -> Cache:
        """Create a cache with 1000 entries."""
        cache: Cache = InMemoryCache()
        for i in range(1000):
            response = Response(
                content=f"content_{i}" * 100,  # ~1400 chars per response
                model="benchmark-model",
                provider=Provider.OPENAI,
            )
            cache.set(f"key_{i}", response)
        return cache

    def test_cache_set_benchmark(self, benchmark: pytest.BenchmarkBuiltin) -> None:
        """Benchmark cache set operation."""
        cache: Cache = InMemoryCache()

        def set_operation() -> None:
            for i in range(100):
                response = Response(
                    content=f"benchmark_content_{i}",
                    model="test",
                    provider=Provider.OPENAI,
                )
                cache.set(f"benchmark_key_{i}", response)

        result = benchmark(set_operation)
        assert result is None or isinstance(result, int)
        assert len(cache) == 100

    def test_cache_get_benchmark(self, large_cache: Cache, benchmark: pytest.BenchmarkBuiltin) -> None:
        """Benchmark cache get operation."""
        def get_operation() -> Response | None:
            return large_cache.get("key_500")

        result = benchmark(get_operation)
        assert result is not None

    def test_cache_set_get_cycle_benchmark(self, benchmark: pytest.BenchmarkBuiltin) -> None:
        """Benchmark set-get cycle."""
        cache: Cache = InMemoryCache()

        def cycle_operation() -> Response | None:
            key = "cycle_key"
            response = Response(
                content="cycle_content",
                model="test",
                provider=Provider.ANTHROPIC,
            )
            cache.set(key, response)
            return cache.get(key)

        result = benchmark(cycle_operation)
        assert result is not None
        assert result.content == "cycle_content"

    def test_cache_clear_benchmark(self, large_cache: Cache, benchmark: pytest.BenchmarkBuiltin) -> None:
        """Benchmark cache clear operation."""
        def clear_operation() -> None:
            large_cache.clear()

        benchmark(clear_operation)
        assert len(large_cache) == 0

    def test_cache_iteration_benchmark(self, large_cache: Cache, benchmark: pytest.BenchmarkBuiltin) -> None:
        """Benchmark iterating over cache entries."""
        def iterate_operation() -> list[str]:
            return list(large_cache.keys())

        result = benchmark(iterate_operation)
        assert len(result) == 1000

    def test_cache_delete_benchmark(self, large_cache: Cache, benchmark: pytest.BenchmarkBuiltin) -> None:
        """Benchmark cache delete operation."""
        def delete_operation() -> None:
            for i in range(100):
                large_cache.delete(f"key_{i}")

        benchmark(delete_operation)
        assert len(large_cache) == 900

    def test_cache_len_benchmark(self, large_cache: Cache, benchmark: pytest.BenchmarkBuiltin) -> None:
        """Benchmark cache len operation."""
        def len_operation() -> int:
            return len(large_cache)

        result = benchmark(len_operation)
        assert result == 1000


# Traces to: FR-PERF-001
class TestCacheThroughput:
    """Throughput tests for cache operations."""

    def test_high_throughput_set_operations(self) -> None:
        """Test high-throughput set operations."""
        cache: Cache = InMemoryCache()
        iterations = 1000

        for i in range(iterations):
            response = Response(
                content=f"throughput_test_{i}",
                model="test",
                provider=Provider.OPENAI,
            )
            cache.set(f"throughput_key_{i}", response)

        assert len(cache) == iterations

    def test_high_throughput_mixed_operations(self) -> None:
        """Test high-throughput mixed operations."""
        cache: Cache = InMemoryCache()
        iterations = 500

        # Pre-populate
        for i in range(iterations):
            response = Response(
                content=f"mixed_test_{i}",
                model="test",
                provider=Provider.ANTHROPIC,
            )
            cache.set(f"mixed_key_{i}", response)

        # Mix of gets and sets
        for i in range(iterations):
            _ = cache.get(f"mixed_key_{i}")  # Get
            if i % 2 == 0:
                response = Response(
                    content=f"new_content_{i}",
                    model="test",
                    provider=Provider.OPENAI,
                )
                cache.set(f"new_key_{i}", response)  # Set

        assert len(cache) >= iterations

    def test_large_response_handling(self) -> None:
        """Test handling of large responses."""
        cache: Cache = InMemoryCache()
        large_content = "x" * 100000  # 100KB response

        response = Response(
            content=large_content,
            model="large-model",
            provider=Provider.OPENAI,
        )
        cache.set("large_key", response)

        retrieved = cache.get("large_key")
        assert retrieved is not None
        assert len(retrieved.content) == 100000
