# ADR-001: HMAC-SHA256 Signature Verification

**Date**: 2026-04-05  
**Status**: Accepted  
**Deciders**: Phenotype Engineering Team

## Context

Webhook payloads must be verified to ensure they originate from trusted sources and haven't been tampered with.

## Decision Drivers

- **Security**: Cryptographically secure verification
- **Performance**: Fast signature computation
- **Compatibility**: Industry standard
- **Simplicity**: Easy to implement and debug

## Options Considered

### Option A: HMAC-SHA256 (Selected)

```go
func sign(payload []byte, secret string) string {
    mac := hmac.New(sha256.New, []byte(secret))
    mac.Write(payload)
    return hex.EncodeToString(mac.Sum(nil))
}
```

**Pros**:
- Industry standard (Stripe, GitHub, etc.)
- Fast computation
- No key distribution needed
- Well-understood security

**Cons**:
- Shared secret management
- Symmetric key limitations

### Option B: RSA Signature

Asymmetric cryptographic signature.

**Pros**:
- Public key verification
- Non-repudiation

**Cons**:
- Slower computation
- Key management complexity
- Overkill for most webhooks

### Option C: mTLS

Mutual TLS authentication.

**Pros**:
- Transport-level security
- No application code needed

**Cons**:
- Certificate management
- Infrastructure complexity
- No payload integrity

## Decision

**Use HMAC-SHA256 with constant-time comparison for webhook verification.**

## Consequences

### Positive
- Fast and secure
- Industry compatibility
- Simple implementation

### Negative
- Secret rotation required
- Shared secret distribution

## Implementation

See [verify.go](../../verify.go) and [delivery.go](../../delivery.go).

---

*HMAC-SHA256 is the standard for all Phenotype webhook integrations.*
