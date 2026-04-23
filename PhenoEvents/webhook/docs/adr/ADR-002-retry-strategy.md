# ADR-002: Exponential Backoff Retry

**Date**: 2026-04-05  
**Status**: Accepted  
**Deciders**: Phenotype Engineering Team

## Context

Webhook deliveries may fail due to transient network issues or recipient downtime. Retries with backoff improve delivery reliability.

## Decision Drivers

- **Reliability**: Ensure eventual delivery
- **Respect**: Don't overwhelm recipients
- **Visibility**: Track delivery attempts
- **Configurability**: Adapt to recipient capabilities

## Options Considered

### Option A: Exponential Backoff (Selected)

```go
delay = retryBackoff * (1 << attempt) // 1s, 2s, 4s, 8s, 16s...
```

**Pros**:
- Industry standard (Stripe, GitHub)
- Automatic rate reduction
- Configurable limits

**Cons**:
- Long delays for late retries
- May exceed recipient's retention window

### Option B: Fixed Interval

```go
delay = 1 * time.Second // Always 1 second
```

**Pros**:
- Simple to understand
- Predictable

**Cons**:
- No backoff on failures
- Can overwhelm recipients

### Option C: No Retry

Single delivery attempt only.

**Pros**:
- Simplest
- No duplicate risk

**Cons**:
- Poor reliability
- Missed events on transient failures

## Decision

**Implement exponential backoff with configurable max retries and max delay.**

## Configuration

```go
type DeliveryConfig struct {
    Timeout       time.Duration // Default: 30s
    MaxRetries    int           // Default: 5
    RetryBackoff  time.Duration // Default: 1s
}
```

## Consequences

### Positive
- High delivery success rate
- Automatic congestion avoidance
- Industry-standard behavior

### Negative
- Duplicate event risk
- Delayed delivery on failures
- Storage needed for retries

## Implementation

See [delivery.go](../../delivery.go) for retry implementation.

---

*Retry behavior is configurable per webhook endpoint.*
