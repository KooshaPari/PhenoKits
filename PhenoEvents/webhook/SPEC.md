# Webhook Library Specification

> Secure Webhook Delivery with HMAC Verification - Event Notifications

**Version**: 1.0  
**Status**: Production  
**Last Updated**: 2026-04-05

---

## Table of Contents

1. [Overview](#1-overview)
2. [Architecture](#2-architecture)
3. [Configuration](#3-configuration)
4. [Usage Patterns](#4-usage-patterns)
5. [Security](#5-security)
6. [Appendices](#6-appendices)

---

## 1. Overview

### 1.1 Purpose

The webhook library provides secure webhook delivery with HMAC signature verification for Go applications. It enables:

- **Secure delivery**: HMAC-SHA256 signature verification
- **Reliable delivery**: Exponential backoff retries
- **Event tracking**: Unique delivery IDs for idempotency
- **Verification**: Request signature validation

### 1.2 Goals

| Goal | Priority | Status |
|------|----------|--------|
| HMAC-SHA256 signing | P0 | ✅ Implemented |
| Exponential retry | P0 | ✅ Implemented |
| Delivery tracking | P0 | ✅ Implemented |
| Signature verification | P0 | ✅ Implemented |
| Idempotency support | P1 | ✅ Implemented |

### 1.3 Definitions

| Term | Definition |
|------|------------|
| **Webhook** | HTTP callback for event notification |
| **Delivery** | Single webhook transmission attempt |
| **Signature** | HMAC-SHA256 of payload |
| **Idempotency** | Safe duplicate delivery handling |
| **Retry** | Automatic retransmission on failure |

---

## 2. Architecture

### 2.1 Delivery Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      Webhook Delivery Architecture                          │
│                                                                             │
│  Sender                                                        Receiver    │
│    │                                                              │        │
│    │  1. Create payload                                            │        │
│    │  2. Sign with HMAC-SHA256                                     │        │
│    │  3. POST to webhook URL                                       │        │
│    ├────────────────────────────────────────────────────────────►│        │
│    │                                                              │        │
│    │  POST /webhook HTTP/1.1                                      │        │
│    │  X-Webhook-Signature: sha256=abc123...                        │        │
│    │  X-Webhook-Event: order.created                             │        │
│    │  X-Webhook-Delivery: 20260405012345-xyz789                   │        │
│    │  Content-Type: application/json                               │        │
│    │                                                              │        │
│    │  {"order_id": "123", "amount": 100.00}                        │        │
│    │                                                              │        │
│    │                                                              │  4. Verify signature
│    │                                                              │  5. Check idempotency
│    │                                                              │  6. Process event
│    │◄────────────────────────────────────────────────────────────┤  7. Return 2xx
│    │                                                              │        │
│    │  HTTP 200 OK                                                │        │
│    │                                                              │        │
│    │  [Success - no retry]                                        │        │
│    │                                                              │        │
│    │  OR (on failure)                                            │        │
│    │                                                              │        │
│    │  HTTP 5xx or timeout                                        │        │
│    │                                                              │        │
│    │  [Exponential backoff retry]                                 │        │
│    │  Retry 1: 1s delay                                          │        │
│    │  Retry 2: 2s delay                                          │        │
│    │  Retry 3: 4s delay                                          │        │
│    │  ... up to max retries                                       │        │
│    │                                                              │        │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Configuration

### 3.1 DeliveryConfig

```go
type DeliveryConfig struct {
    Timeout       time.Duration // Request timeout (default: 30s)
    MaxRetries    int           // Max retry attempts (default: 5)
    RetryBackoff  time.Duration // Initial retry delay (default: 1s)
    SignAlgorithm string        // Signing algorithm (default: "hmac-sha256")
}

var DefaultDeliveryConfig = DeliveryConfig{
    Timeout:       30 * time.Second,
    MaxRetries:    5,
    RetryBackoff:  time.Second,
    SignAlgorithm: "hmac-sha256",
}
```

---

## 4. Usage Patterns

### 4.1 Sending Webhooks

```go
package main

import (
    "github.com/KooshaPari/phenotype-go-kit/webhook"
)

func main() {
    // Create delivery service
    config := webhook.DefaultDeliveryConfig
    service := webhook.NewDeliveryService("my-secret-key", config, logger)
    
    // Send webhook
    payload := OrderCreatedEvent{
        OrderID: "order-123",
        Amount:  99.99,
    }
    
    delivery, err := service.Deliver(ctx, "order.created", "https://example.com/webhook", payload)
    if err != nil {
        log.Printf("Delivery failed: %v", err)
    }
    
    log.Printf("Delivery ID: %s, Success: %v", delivery.ID, delivery.Success)
}
```

### 4.2 Verifying Webhooks

```go
// HTTP handler for receiving webhooks
func handleWebhook(w http.ResponseWriter, r *http.Request) {
    // Read body
    body, _ := io.ReadAll(r.Body)
    
    // Verify signature
    err := webhook.VerifyRequest(r, body, "my-secret-key")
    if err != nil {
        http.Error(w, "Invalid signature", http.StatusUnauthorized)
        return
    }
    
    // Check idempotency
    deliveryID := webhook.GetDeliveryID(r)
    if isDuplicate(deliveryID) {
        w.WriteHeader(http.StatusOK) // Already processed
        return
    }
    
    // Process event
    eventType := webhook.GetEventType(r)
    switch eventType {
    case "order.created":
        processOrderCreated(body)
    }
    
    w.WriteHeader(http.StatusOK)
}
```

### 4.3 Manual Signature Verification

```go
// Verify raw payload
payload := []byte(`{"order_id": "123"}`)
signature := "abc123..."
secret := "my-secret-key"

if !webhook.VerifySignature(payload, signature, secret) {
    log.Fatal("Invalid signature")
}
```

---

## 5. Security

### 5.1 Signature Verification

| Aspect | Implementation |
|--------|----------------|
| **Algorithm** | HMAC-SHA256 |
| **Header** | X-Webhook-Signature |
| **Format** | hex encoded |
| **Comparison** | Constant-time |

### 5.2 Best Practices

| DO | DON'T |
|----|-------|
| Use HTTPS endpoints | Send over HTTP |
| Verify every request | Trust unverified payloads |
| Check idempotency | Process duplicates |
| Rotate secrets regularly | Hardcode secrets |
| Log delivery attempts | Ignore failures |

---

## 6. Appendices

### 6.1 API Reference

See [delivery.go](../delivery.go) and [verify.go](../verify.go) for complete API documentation.

### 6.2 Headers

| Header | Description |
|--------|-------------|
| X-Webhook-Signature | HMAC-SHA256 signature |
| X-Webhook-Event | Event type |
| X-Webhook-Delivery | Unique delivery ID |
| Content-Type | application/json |

### 6.3 Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-04-05 | Initial release |

---

*This specification defines the webhook library v1.0 for Phenotype Go Kit.*
