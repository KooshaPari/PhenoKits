"""
Tests for pheno-events package.
"""

import pytest


class TestEventBus:
    """Tests for EventBus."""

    @pytest.mark.asyncio
    async def test_publish_and_subscribe(self):
        """Test basic publish/subscribe."""
        from pheno_events import EventBus

        bus = EventBus()
        received = []

        @bus.on("test.event")
        async def handler(event):
            received.append(event.data)

        await bus.publish("test.event", {"key": "value"})
        assert len(received) == 1
        assert received[0] == {"key": "value"}

    @pytest.mark.asyncio
    async def test_wildcard_subscription(self):
        """Test wildcard event subscription."""
        from pheno_events import EventBus

        bus = EventBus()
        received = []

        @bus.on("user.*")
        async def handler(event):
            received.append(event.name)

        await bus.publish("user.created", {"id": 1})
        await bus.publish("user.updated", {"id": 1})
        assert len(received) == 2


class TestSimpleEventBus:
    """Tests for SimpleEventBus."""

    def test_publish_and_subscribe_sync(self):
        """Test sync publish/subscribe."""
        from pheno_events import SimpleEventBus

        bus = SimpleEventBus()
        received = []

        @bus.on("test.event")
        def handler(event):
            received.append(event.data)

        bus.publish("test.event", {"key": "value"})
        assert len(received) == 1
        assert received[0] == {"key": "value"}


class TestEventStore:
    """Tests for EventStore."""

    @pytest.mark.asyncio
    async def test_append_and_get_stream(self):
        """Test event store append and retrieval."""
        from pheno_events import EventStore

        store = EventStore()

        await store.append(
            event_type="TestEvent",
            aggregate_id="test-123",
            aggregate_type="Test",
            data={"key": "value"},
        )

        events = await store.get_stream("test-123")
        assert len(events) == 1
        assert events[0].event_type == "TestEvent"


class TestWebhookSigner:
    """Tests for WebhookSigner."""

    def test_sign_and_verify(self):
        """Test signature generation and verification."""
        from pheno_events.webhooks import WebhookSigner

        signer = WebhookSigner("secret-key")
        payload = '{"event": "test"}'

        signature = signer.sign(payload)
        assert signature.startswith("sha256=")

        is_valid = WebhookSigner.verify(payload, signature, "secret-key")
        assert is_valid is True

    def test_verify_invalid_signature(self):
        """Test verification with invalid signature."""
        from pheno_events.webhooks import WebhookSigner

        payload = '{"event": "test"}'
        is_valid = WebhookSigner.verify(payload, "sha256=invalid", "secret-key")
        assert is_valid is False


class TestRetryPolicy:
    """Tests for RetryPolicy."""

    def test_next_retry_time(self):
        """Test retry time calculation."""
        from pheno_events.webhooks import RetryPolicy

        policy = RetryPolicy(max_attempts=3, initial_delay=60, multiplier=2.0)
        retry_time = policy.next_retry_time(0)
        assert retry_time is not None
