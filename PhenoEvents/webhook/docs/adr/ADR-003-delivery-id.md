# ADR-003: Delivery ID for Idempotency

**Date**: 2026-04-05  
**Status**: Accepted  
**Deciders**: Phenotype Engineering Team

## Context

Recipients need to detect duplicate webhook deliveries to avoid processing the same event multiple times.

## Decision Drivers

- **Idempotency**: Safe to retry without side effects
- **Debugging**: Trace specific deliveries
- **Deduplication**: Recipient-side duplicate detection
- **Audit**: Delivery tracking

## Options Considered

### Option A: Unique Delivery ID (Selected)

```go
func generateDeliveryID() string {
    return time.Now().Format("20060102150405") + "-" + randomString(12)
}

// Header: X-Webhook-Delivery: 20260405012345-abc123def456
```

**Pros**:
- Unique per delivery attempt
- Timestamp embedded (debugging)
- Recipient can deduplicate

**Cons**:
- Not the same as event ID
- Storage required for dedup

### Option B: Event ID Only

Use event ID as deduplication key.

**Pros**:
- Natural deduplication
- Business-level identifier

**Cons**:
- Can't distinguish retries
- Same ID on redelivery

### Option C: No ID

Rely on payload content for dedup.

**Pros**:
- Simple

**Cons**:
- Payload hashing overhead
- False positives possible

## Decision

**Include unique delivery ID in webhook headers separate from event ID.**

## Headers

| Header | Purpose |
|--------|---------|
| X-Webhook-Delivery | Unique delivery attempt ID |
| X-Webhook-Event | Event type |
| X-Webhook-Signature | HMAC signature |

## Consequences

### Positive
- Recipient can track all delivery attempts
- Debug individual deliveries
- Safe retry behavior

### Negative
- Additional header processing
- Storage for deduplication

---

*Delivery ID is required for all webhook deliveries.*
