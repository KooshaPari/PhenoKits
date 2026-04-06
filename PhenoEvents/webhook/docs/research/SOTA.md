# Webhook Library - State of the Art

> Secure Webhook Delivery with HMAC Verification - Event Notifications

**Version**: 1.0  
**Status**: Active  
**Last Updated**: 2026-04-05

---

## Part I: Webhook Landscape (2024-2026)

### 1.1 Webhook Evolution

Webhooks have evolved from simple HTTP callbacks to sophisticated event delivery systems with retries, verification, and idempotency guarantees.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      Webhook Evolution                                      │
│                                                                             │
│  HTTP Callback → Signed Payloads → Retries → Queues → Idempotency → Schema  │
│                                                                             │
│  2008           2012            2015    2018    2020       2024+            │
│    │              │               │       │       │          │              │
│    ▼              ▼               ▼       ▼       ▼          ▼              │
│  ┌────┐       ┌────┐         ┌────┐  ┌────┐  ┌────┐     ┌────┐            │
│  │POST│       │HMAC│         │Exp │  │SQS/ │  │Idem│     │Clou│            │
│  │url │       │sha2│         │Back│  │Kafka│  │Key │     │deve│            │
│  │    │       │56  │         │off │  │     │  │    │     │nts│            │
│  └────┘       └────┘         └────┘  └────┘  └────┘     └────┘            │
│                                                                             │
│  Basic         Security       Reliability   Async       Safety       Types  │
│  delivery      verification  & retries     processing   guarantees   safety │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1.2 Webhook Providers

| Provider | Signature | Retry Strategy | Idempotency | Schema |
|----------|-----------|----------------|-------------|--------|
| **Stripe** | HMAC-SHA256 | Exponential | Event ID | Versioned |
| **GitHub** | HMAC-SHA256 | Exponential | Delivery ID | Versioned |
| **Slack** | HMAC-SHA256 | Exponential | None | Versioned |
| **AWS SNS** | SigningCert | Fixed | Message ID | XML/JSON |
| **Twilio** | HMAC-SHA1 | Exponential | None | Stable |
| **Shopify** | HMAC-SHA256 | Exponential | None | Versioned |

### 1.3 Security Standards

| Standard | Mechanism | Strength | Verification |
|----------|-----------|----------|------------|
| **HMAC-SHA256** | Shared secret | High | Constant-time compare |
| **HMAC-SHA1** | Shared secret | Medium | Deprecated |
| **Ed25519** | Asymmetric | Very High | Public key |
| **RS256** | Asymmetric | High | Certificate chain |
| **mTLS** | Certificate | Very High | Mutual TLS |

---

## Part II: Delivery Patterns

### 2.1 Retry Strategies

| Provider | Initial | Max | Backoff | Jitter |
|----------|---------|-----|---------|--------|
| **Stripe** | Immediate | 3 days | Exponential | Yes |
| **GitHub** | Immediate | 24 hours | Linear | No |
| **AWS** | Immediate | 1 hour | Exponential | Yes |

### 2.2 Delivery Verification

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      Webhook Delivery Flow                                  │
│                                                                             │
│  Sender                          Receiver                                   │
│    │                                │                                       │
│    │  1. Create payload              │                                       │
│    │  2. Sign with HMAC              │                                       │
│    │  3. POST to URL                 │                                       │
│    ├───────────────────────────────►│                                       │
│    │  X-Signature: sha256=abc123     │                                       │
│    │  X-Event-ID: evt_123           │                                       │
│    │                                │  4. Verify signature                   │
│    │                                │  5. Check idempotency                  │
│    │                                │  6. Process event                       │
│    │◄───────────────────────────────┤  7. Return 2xx                          │
│    │  HTTP 200 OK                   │                                       │
│    │                                │                                       │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Part III: Go Implementation

### 3.1 Delivery Structure

```go
type Delivery struct {
    ID         string          // Unique delivery ID
    EventType  string          // Type of event
    URL        string          // Destination URL
    Payload    json.RawMessage // Event payload
    Headers    http.Header     // Request headers
    StatusCode int             // Response status
    Success    bool            // Delivery success
    Retries    int             // Number of retries
    CreatedAt  time.Time       // Creation timestamp
    SentAt     *time.Time      // Send timestamp
}
```

### 3.2 Signature Verification

```go
const SignatureHeaderName = "X-Webhook-Signature"

func VerifySignature(payload []byte, signature string, secret string) bool {
    expected := computeSignature(payload, secret)
    return hmac.Equal([]byte(expected), []byte(signature))
}

func computeSignature(payload []byte, secret string) string {
    mac := hmac.New(sha256.New, []byte(secret))
    mac.Write(payload)
    return hex.EncodeToString(mac.Sum(nil))
}
```

---

## Part IV: References

| Resource | URL | Description |
|----------|-----|-------------|
| Stripe Webhooks | https://stripe.com/docs/webhooks | Best practices |
| GitHub Webhooks | https://docs.github.com/webhooks | Implementation guide |
| OWASP | https://owasp.org/ | Security guidelines |

---

*This document reflects SOTA in webhook implementation as of April 2026.*
