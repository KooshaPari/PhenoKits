"""Tests for CompressionMiddleware.

Traces to: WP-015-COMPRESSION, FR-BUILTIN-007
"""

import gzip
import zlib

import pytest

from phenotype_middleware.domain import Request, Response
from phenotype_middleware.infrastructure import CompressionMiddleware


class TestCompressionBasic:
    """Test basic compression functionality."""

    @pytest.mark.asyncio
    async def test_given_gzip_accepted_when_processed_then_compresses_with_gzip(self):
        """Given gzip accepted, when processing response, then compresses with gzip."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        result = await middleware.process(request)
        assert result.success is True

        # Create a response with compressible content
        body = b'{"message": "This is a test message that should be compressed"}' * 10
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        compressed = middleware.compress_response(request, response)

        assert compressed.headers.get("Content-Encoding") == "gzip"
        assert "Content-Length" in compressed.headers
        assert int(compressed.headers["Content-Length"]) < len(body)
        assert "Vary" in compressed.headers
        assert compressed.headers["Vary"] == "Accept-Encoding"

    @pytest.mark.asyncio
    async def test_given_deflate_accepted_when_processed_then_compresses_with_deflate(self):
        """Given deflate accepted, when processing response, then compresses with deflate."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "deflate"},
        )

        await middleware.process(request)

        body = b'{"message": "This is a test message that should be compressed"}' * 10
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        compressed = middleware.compress_response(request, response)

        assert compressed.headers.get("Content-Encoding") == "deflate"
        assert int(compressed.headers["Content-Length"]) < len(body)

    @pytest.mark.asyncio
    async def test_given_both_accepted_when_processed_then_uses_preferred_encoding(self):
        """Given both encodings accepted, when processing, then uses preferred (first in list)."""
        # gzip is first in default encodings
        middleware = CompressionMiddleware(min_size=1, encodings=("gzip", "deflate"))
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "deflate, gzip"},
        )

        await middleware.process(request)

        body = b'{"message": "Test data for compression"}' * 10
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        compressed = middleware.compress_response(request, response)

        # Should prefer gzip (first in our encodings list)
        assert compressed.headers.get("Content-Encoding") == "gzip"


class TestCompressionConditions:
    """Test when compression should/shouldn't occur."""

    @pytest.mark.asyncio
    async def test_given_no_accept_encoding_when_processed_then_no_compression(self):
        """Given no Accept-Encoding header, when processing, then no compression occurs."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={},  # No Accept-Encoding
        )

        await middleware.process(request)

        body = b'{"message": "Test data"}' * 100
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        result = middleware.compress_response(request, response)

        assert "Content-Encoding" not in result.headers
        assert result.body == body  # Original, uncompressed

    @pytest.mark.asyncio
    async def test_given_content_too_small_when_processed_then_no_compression(self):
        """Given content smaller than min_size, when processing, then no compression."""
        middleware = CompressionMiddleware(min_size=1000)  # 1KB min
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        # Small body, should not be compressed
        body = b'{"small": "data"}'  # Less than 1000 bytes
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        result = middleware.compress_response(request, response)

        assert "Content-Encoding" not in result.headers
        assert result.body == body

    @pytest.mark.asyncio
    async def test_given_already_compressed_when_processed_then_no_double_compression(self):
        """Given response already has Content-Encoding, when processing, then no double compression."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        body = b'{"message": "Already compressed"}' * 100
        response = Response(
            status_code=200,
            headers={
                "Content-Type": "application/json",
                "Content-Encoding": "gzip",  # Already compressed
            },
            body=body,
        )

        result = middleware.compress_response(request, response)

        # Should not compress again
        assert result.headers.get("Content-Encoding") == "gzip"  # Original
        assert result.body == body

    @pytest.mark.asyncio
    async def test_given_compressed_would_be_larger_when_processed_then_uses_original(self):
        """Given compressed size would be larger, when processing, then uses original."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        # Very small body that won't compress well
        body = b'x'  # Single byte - compression will make it larger
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        result = middleware.compress_response(request, response)

        # Should not compress since it would be larger
        assert "Content-Encoding" not in result.headers
        assert result.body == body

    @pytest.mark.asyncio
    async def test_given_no_body_when_processed_then_no_compression(self):
        """Given response has no body, when processing, then no compression."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        response = Response(
            status_code=204,  # No content
            headers={"Content-Type": "application/json"},
            body=b"",
        )

        result = middleware.compress_response(request, response)

        assert "Content-Encoding" not in result.headers


class TestContentTypes:
    """Test content type handling."""

    @pytest.mark.asyncio
    async def test_given_text_plain_when_processed_then_compresses(self):
        """Given text/plain content, when processing, then compresses."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/text",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        body = b"This is plain text content that should be compressed. " * 50
        response = Response(
            status_code=200,
            headers={"Content-Type": "text/plain"},
            body=body,
        )

        compressed = middleware.compress_response(request, response)

        assert compressed.headers.get("Content-Encoding") == "gzip"

    @pytest.mark.asyncio
    async def test_given_text_html_when_processed_then_compresses(self):
        """Given text/html content, when processing, then compresses."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/page",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        body = b"<html><body>HTML content</body></html>" * 100
        response = Response(
            status_code=200,
            headers={"Content-Type": "text/html"},
            body=body,
        )

        compressed = middleware.compress_response(request, response)

        assert compressed.headers.get("Content-Encoding") == "gzip"

    @pytest.mark.asyncio
    async def test_given_binary_content_when_processed_then_no_compression(self):
        """Given binary content type, when processing, then no compression."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/image.png",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        body = b"\x89PNG\r\n\x1a\n" + b"binary image data" * 1000
        response = Response(
            status_code=200,
            headers={"Content-Type": "image/png"},
            body=body,
        )

        result = middleware.compress_response(request, response)

        assert "Content-Encoding" not in result.headers
        assert result.body == body

    @pytest.mark.asyncio
    async def test_given_content_type_with_charset_when_processed_then_strips_charset(self):
        """Given content type with charset suffix, when processing, then strips and compresses."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        body = b'{"message": "Test"}' * 100
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json; charset=utf-8"},
            body=body,
        )

        compressed = middleware.compress_response(request, response)

        assert compressed.headers.get("Content-Encoding") == "gzip"


class TestConfiguration:
    """Test middleware configuration options."""

    @pytest.mark.asyncio
    async def test_given_custom_min_size_when_processed_then_respects_threshold(self):
        """Given custom min_size, when processing, then respects threshold."""
        middleware = CompressionMiddleware(min_size=500)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        # Body just under threshold (~350 bytes)
        body = b'{"data": "x"}' * 25
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        result = middleware.compress_response(request, response)
        assert "Content-Encoding" not in result.headers  # Too small

        # Body over threshold (~560 bytes)
        body = b'{"data": "x"}' * 40
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        result = middleware.compress_response(request, response)
        assert result.headers.get("Content-Encoding") == "gzip"  # Large enough

    @pytest.mark.asyncio
    async def test_given_custom_compression_level_when_processed_then_uses_level(self):
        """Given custom compression level, when compressing, then uses that level."""
        # Note: We can't easily verify the compression level was used,
        # but we can verify the middleware accepts and stores the value
        middleware_low = CompressionMiddleware(min_size=1, level=1)
        middleware_high = CompressionMiddleware(min_size=1, level=9)

        assert middleware_low.level == 1
        assert middleware_high.level == 9

    @pytest.mark.asyncio
    async def test_given_level_out_of_range_when_initialized_then_clamps(self):
        """Given level outside 1-9, when initializing, then clamps to valid range."""
        middleware_low = CompressionMiddleware(min_size=1, level=0)  # Too low
        middleware_high = CompressionMiddleware(min_size=1, level=10)  # Too high

        assert middleware_low.level == 1  # Clamped to minimum
        assert middleware_high.level == 9  # Clamped to maximum

    @pytest.mark.asyncio
    async def test_given_custom_content_types_when_processed_then_respects_list(self):
        """Given custom content_types, when processing, then respects list."""
        middleware = CompressionMiddleware(
            min_size=1,
            content_types={"application/custom"},
        )
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        # Standard JSON should NOT compress with custom types
        body = b'{"data": "test"}' * 100
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        result = middleware.compress_response(request, response)
        assert "Content-Encoding" not in result.headers

        # Custom type SHOULD compress
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/custom"},
            body=body,
        )

        result = middleware.compress_response(request, response)
        assert result.headers.get("Content-Encoding") == "gzip"

    @pytest.mark.asyncio
    async def test_given_wildcard_encoding_when_processed_then_compresses(self):
        """Given wildcard (*) in Accept-Encoding, when processing, then compresses."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "*"},  # Any encoding acceptable
        )

        await middleware.process(request)

        body = b'{"data": "test"}' * 100
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=body,
        )

        result = middleware.compress_response(request, response)
        assert result.headers.get("Content-Encoding") == "gzip"


class TestCompressionAlgorithm:
    """Test compression algorithm correctness."""

    @pytest.mark.asyncio
    async def test_given_gzip_compression_when_decompressed_then_matches_original(self):
        """Given gzip compression, when decompressing, then matches original."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        await middleware.process(request)

        original = b'{"message": "Test compression round-trip"}' * 50
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=original,
        )

        compressed_response = middleware.compress_response(request, response)

        # Verify we can decompress and get original
        compressed_body = compressed_response.body
        decompressed = gzip.decompress(compressed_body)

        assert decompressed == original

    @pytest.mark.asyncio
    async def test_given_deflate_compression_when_decompressed_then_matches_original(self):
        """Given deflate compression, when decompressing, then matches original."""
        middleware = CompressionMiddleware(min_size=1)
        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "deflate"},
        )

        await middleware.process(request)

        original = b'{"message": "Test deflate compression"}' * 50
        response = Response(
            status_code=200,
            headers={"Content-Type": "application/json"},
            body=original,
        )

        compressed_response = middleware.compress_response(request, response)

        # Verify we can decompress
        compressed_body = compressed_response.body
        # Use zlib with negative wbits for deflate format
        decompressed = zlib.decompress(compressed_body, wbits=-zlib.MAX_WBITS)

        assert decompressed == original


class TestStats:
    """Test statistics functionality."""

    def test_given_new_middleware_when_get_stats_then_returns_zero_values(self):
        """Given new middleware, when getting stats, then returns zeros."""
        middleware = CompressionMiddleware()
        stats = middleware.get_stats()

        assert stats["compressed"] == 0
        assert stats["skipped"] == 0


class TestIntegration:
    """Integration tests with middleware chain."""

    @pytest.mark.asyncio
    async def test_compression_in_chain_context(self):
        """Test compression works within middleware chain context."""
        from phenotype_middleware.application import MiddlewareChain

        compression = CompressionMiddleware(min_size=1)
        chain = MiddlewareChain()
        chain.add(compression)

        request = Request(
            path="/api/data",
            method="GET",
            headers={"Accept-Encoding": "gzip"},
        )

        # Process through chain
        result = await chain.handle(request)

        # Verify context has compression info
        assert "_compression_middleware" in result.request.context
        assert "_accept_encoding" in result.request.context
