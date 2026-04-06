# ADR-003: Webhook Delivery Strategy - At-Least-Once with Exponential Backoff

## Status

Accepted

## Context

Webhooks are HTTP callbacks that enable event-driven integration with external systems. pheno-events needs a reliable webhook delivery system for:

- Third-party integrations (Slack, Stripe, etc.)
- Microservice communication
- External API notifications
- User-defined integrations

The challenge is ensuring reliable delivery despite network failures, receiver downtime, and transient errors.

## Decision

We will implement webhook delivery with:

1. **At-least-once delivery**: Events may be delivered multiple times
2. **Exponential backoff**: Retry delays increase exponentially
3. **HMAC-SHA256 signatures**: Industry-standard payload verification
4. **Delivery state machine**: Pending → Retrying → Delivered/Failed
5. **Configurable retry policy**: Adapt to different endpoint requirements
6. **Async HTTP client**: httpx for non-blocking I/O

### Delivery State Machine

```
┌──────────┐    Attempt     ┌───────────┐
│  PENDING │───────────────▶│  RETRYING │
└────┬─────┘                └─────┬─────┘
     │                            │
     │ Success                    │ Failure + More Attempts
     ▼                            │
┌──────────┐                      │
│ DELIVERED│◀─────────────────────┘
└──────────┘
     │
     │ Failure + No More Attempts
     ▼
┌──────────┐
│  FAILED  │
└──────────┘
```

### Retry Policy

**Exponential backoff with configurable parameters**:

```python
@dataclass
class RetryPolicy:
    max_attempts: int = 3          # Total delivery attempts
    initial_delay: int = 60        # Seconds before first retry
    multiplier: float = 2.0        # Delay multiplier
    max_delay: int = 3600          # Cap at 1 hour
```

**Retry schedule example** (default settings):

| Attempt | Delay | Total Elapsed |
|---------|-------|---------------|
| 1 | 0s | Immediate |
| 2 | 60s | 1 minute |
| 3 | 120s | 3 minutes |

**With jitter** (future enhancement):
```python
delay = min(initial * (multiplier ** attempt), max_delay)
jittered = delay * (0.5 + random())  # 50-150% of delay
```

### Signature Scheme

**HMAC-SHA256 with timestamp** (GitHub/Stripe compatible):

```
Header: X-Webhook-Signature: sha256=<hex_digest>
Digest: HMAC_SHA256(secret, payload)
```

**Webhook payload structure**:

```json
{
    "event_type": "order.created",
    "event_id": "uuid",
    "timestamp": "2024-01-01T00:00:00Z",
    "data": {...}
}
```

**Verification example**:

```python
from pheno_events.webhooks import WebhookSigner, WebhookReceiver

# Sending
signer = WebhookSigner("secret-key")
signature = signer.sign(payload_json)
headers = {
    "X-Webhook-Signature": signature,
    "X-Webhook-Event": "order.created",
    "X-Webhook-ID": event_id,
}

# Receiving
receiver = WebhookReceiver("secret-key")
if receiver.verify(payload, signature):
    process_webhook(payload)
else:
    reject_request(401)
```

### Webhook Manager API

```python
from pheno_events.webhooks import WebhookManager

# Initialize
manager = WebhookManager(
    secret="webhook-signing-secret",
    retry_policy=RetryPolicy(max_attempts=5),
    timeout=30
)

# Deliver webhook
delivery_id = await manager.deliver(
    url="https://partner.com/webhook",
    event_type="order.created",
    payload={"order_id": "123", "amount": 100.00}
)

# Process pending (call periodically or via cron)
await manager.process_pending()

# Get delivery status
status = manager.get_delivery(delivery_id)
print(status.status)  # "delivered", "pending", "retrying", "failed"

# Statistics
stats = manager.get_stats()
# {"total": 100, "delivered": 95, "pending": 2, "failed": 3}

# Cleanup
await manager.close()
```

### Delivery Tracking

```python
@dataclass
class WebhookDelivery:
    id: str                      # UUID
    url: str                     # Endpoint
    payload: dict                # Event data
    event_type: str              # Event category
    status: WebhookStatus        # Current state
    attempts: int                # Delivery attempts made
    max_attempts: int            # Maximum attempts
    next_retry: datetime | None  # When to retry
    last_error: str | None       # Last failure reason
    created_at: datetime         # When created
    delivered_at: datetime | None # When delivered (if success)
```

### HTTP Headers

**Standard headers sent with each webhook**:

| Header | Value | Purpose |
|--------|-------|---------|
| Content-Type | application/json | Content type |
| X-Webhook-Event | {event_type} | Event category |
| X-Webhook-ID | {delivery_id} | Unique delivery ID |
| X-Webhook-Attempt | {attempt_number} | Current attempt |
| X-Webhook-Signature | sha256={hmac} | Payload verification |
| User-Agent | pheno-events/0.1.0 | Identification |

## Consequences

### Positive

1. **Industry standard**: HMAC-SHA256 is widely supported
2. **Resilient**: Automatic retries handle transient failures
3. **Observable**: Full delivery tracking and statistics
4. **Flexible**: Configurable retry policies per use case
5. **Secure**: Signature verification prevents tampering
6. **Async**: Non-blocking delivery for high throughput

### Negative

1. **Memory storage**: Delivery records kept in memory (lost on restart)
2. **No persistence**: Failed deliveries lost if process exits
3. **No circuit breaker**: Will keep retrying failing endpoints
4. **No deduplication**: At-least-once means duplicates possible
5. **Single-process**: No coordination between instances

### Mitigations

| Issue | Mitigation | Priority |
|-------|-----------|----------|
| Memory storage | Document limitation; add DB backend later | P1 |
| No persistence | Export/Import functions for backup | P2 |
| No circuit breaker | Add CircuitBreaker integration | P2 |
| No deduplication | Document idempotency key usage | P1 |
| Single-process | Add Redis-backed coordination | P3 |

## Alternatives Considered

### Alternative 1: Exactly-Once Delivery

Implement deduplication with persistent storage.

**Rejected**: Adds significant complexity for rare benefit. Consumers should be idempotent anyway.

### Alternative 2: Linear Retry

Fixed delay between retries (e.g., 60s every time).

**Rejected**: Exponential backoff is industry standard and more effective for recovering systems.

### Alternative 3: Third-Party Service (Svix/Hookdeck)

Use dedicated webhook SaaS.

**Rejected**: Adds external dependency and cost. Integration option for enterprise tier.

### Alternative 4: Synchronous Delivery

Block until delivery completes.

**Rejected**: Unacceptable for high-throughput systems. Delivery must be async.

### Alternative 5: No Retries

Single attempt only.

**Rejected**: Too unreliable for production use.

## Security Considerations

### Signature Verification

```python
class WebhookSigner:
    def sign(self, payload: str) -> str:
        """Generate HMAC-SHA256 signature."""
        signature = hmac.new(
            self.secret.encode(),
            payload.encode(),
            hashlib.sha256
        ).hexdigest()
        return f"sha256={signature}"

    @staticmethod
    def verify(payload: str, signature: str, secret: str) -> bool:
        """Verify signature using constant-time comparison."""
        expected = WebhookSigner(secret).sign(payload)
        return hmac.compare_digest(expected, signature)
```

**Constant-time comparison prevents timing attacks**.

### HTTPS Only

```python
if not url.startswith("https://"):
    raise ValueError("Webhooks must use HTTPS")
```

### Secret Rotation

```python
# Support multiple secrets for rotation period
manager = WebhookManager(
    secrets=["new-secret", "old-secret"],  # Try both
    active_secret="new-secret"  # Use for signing
)
```

## Implementation Details

### Delivery Attempt

```python
async def _attempt_delivery(self, delivery: WebhookDelivery) -> bool:
    delivery.attempts += 1
    delivery.status = WebhookStatus.RETRYING

    try:
        payload_str = json.dumps(delivery.payload)

        headers = {
            "Content-Type": "application/json",
            "X-Webhook-Event": delivery.event_type,
            "X-Webhook-ID": delivery.id,
            "X-Webhook-Attempt": str(delivery.attempts),
        }

        if self.signer:
            headers["X-Webhook-Signature"] = self.signer.sign(payload_str)

        client = await self._get_client()
        response = await client.post(
            delivery.url,
            content=payload_str,
            headers=headers,
        )

        if 200 <= response.status_code < 300:
            delivery.status = WebhookStatus.DELIVERED
            delivery.delivered_at = datetime.utcnow()
            self._notify_success(delivery)
            return True

        raise Exception(f"HTTP {response.status_code}")

    except Exception as e:
        delivery.last_error = str(e)
        if delivery.attempts >= delivery.max_attempts:
            delivery.status = WebhookStatus.FAILED
            self._notify_failure(delivery)
        return False
```

### Callbacks

```python
def on_success(self, callback: Callable):
    """Register callback for successful deliveries."""
    self._success_callbacks.append(callback)
    return callback  # For decorator usage

def on_failure(self, callback: Callable):
    """Register callback for failed deliveries."""
    self._failure_callbacks.append(callback)
    return callback
```

## Integration Examples

### FastAPI Integration

```python
from fastapi import FastAPI, Request, Header, HTTPException
from pheno_events.webhooks import WebhookReceiver

app = FastAPI()
receiver = WebhookReceiver("my-secret")

@app.post("/webhook")
async def receive_webhook(
    request: Request,
    signature: str = Header(..., alias="X-Webhook-Signature")
):
    payload = await request.body()

    if not receiver.verify(payload.decode(), signature):
        raise HTTPException(status_code=401, detail="Invalid signature")

    data = await request.json()
    process_event(data)
    return {"status": "ok"}
```

### Periodic Processing (Celery)

```python
from celery import Celery
from pheno_events.webhooks import WebhookManager

app = Celery()
manager = WebhookManager(secret="secret")

@app.task
def process_webhooks():
    asyncio.run(manager.process_pending())

# Schedule every minute
app.conf.beat_schedule = {
    "process-webhooks": {
        "task": "process_webhooks",
        "schedule": 60.0,
    },
}
```

### Event Bus Integration

```python
from pheno_events import EventBus
from pheno_events.webhooks import WebhookManager

bus = EventBus()
manager = WebhookManager(secret="secret")

@bus.on("user.created")
async def notify_partner(event):
    """Forward user.created to partner webhook."""
    await manager.deliver(
        url="https://partner.com/webhook",
        event_type="user.created",
        payload=event.data
    )
```

## Future Enhancements

1. **Circuit breaker**: Stop hammering failing endpoints
2. **Persistent storage**: SQLite/PostgreSQL for delivery records
3. **Batch delivery**: Multiple events per HTTP request
4. **Delivery logs**: Export to S3/BigQuery for analysis
5. **Retry jitter**: Randomize delays to prevent thundering herd
6. **Endpoint health checks**: Pre-delivery connectivity test
7. **IP allowlisting**: Restrict delivery to trusted IPs
8. **Webhook subscriptions**: Self-service endpoint registration

## References

- [Event Bus SOTA Research](./../research/EVENT_BUS_SOTA.md) - Webhook section
- [Webhook Best Practices](https://webhooks.fyi/)
- [Stripe Webhook Signatures](https://stripe.com/docs/webhooks/signatures)
- [GitHub Webhook Events](https://docs.github.com/en/webhooks)
- [Svix Webhook Service](https://www.svix.com/)

---

*ADR-003 | pheno-events Webhooks | Accepted*
