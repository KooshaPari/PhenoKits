# Sentinel Core — Resilience Patterns

## Overview

Rate limiting, circuit breaking, and bulkhead isolation for Rust services.

## Features

### Core Patterns

1. **Token Bucket** — Token bucket rate limiting with configurable refill rates
2. **Leaky Bucket** — Leaky bucket algorithm for traffic shaping
3. **Circuit Breaker** — Failure threshold-based circuit breaker pattern
4. **Bulkhead** — Partition isolation for resource protection

## Requirements

- FR-001: Token bucket rate limiter with configurable capacity and refill rate
- FR-002: Circuit breaker with configurable failure thresholds
- FR-003: Bulkhead isolation by partition
- FR-004: Async-first implementation using tokio
- FR-005: No external runtime dependencies

## Architecture

```
src/
├── lib.rs              # Public API
├── rate_limiter.rs     # Token bucket implementation
├── circuit_breaker.rs  # Circuit breaker state machine
├── bulkhead.rs         # Partition isolation
└── metrics.rs          # Observable metrics
```
