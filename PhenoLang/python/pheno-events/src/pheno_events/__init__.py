"""
pheno.events - Event bus and webhook management

This module re-exports from pheno_events package.

Migrated to pheno-events package.
"""

from __future__ import annotations

from pheno_events import (
    Event,
    EventBus,
    EventStore,
    NATSConnectionFactory,
    NatsEventBus,
    SimpleEventBus,
    StoredEvent,
    WebhookDelivery,
    WebhookManager,
    WebhookReceiver,
    WebhookSigner,
    ensure_consumer,
    ensure_stream,
    ensure_workqueue,
)

__version__ = "0.1.0"

__all__ = [
    "Event",
    "EventBus",
    "EventStore",
    "NATSConnectionFactory",
    "NatsEventBus",
    "SimpleEventBus",
    "StoredEvent",
    "WebhookDelivery",
    "WebhookManager",
    "WebhookReceiver",
    "WebhookSigner",
    "ensure_consumer",
    "ensure_stream",
    "ensure_workqueue",
]
