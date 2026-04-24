# Product Requirements Document — httpkit

## Overview

`httpkit` is a Rust HTTP utility library for the Phenotype ecosystem. It provides ergonomic middleware, request/response helpers, routing utilities, and common HTTP patterns (rate limiting, retries, circuit breaking) as composable building blocks for Phenotype HTTP services.

## Problem Statement

HTTP concerns — retries, rate limiting, header manipulation, body parsing, CORS, circuit breaking — are repeatedly implemented across Phenotype Rust services. `httpkit` consolidates these patterns into a single, well-tested library.

## Goals

- Composable async HTTP middleware stack (tower-compatible).
- Request/response builder utilities.
- Rate limiting (token bucket, fixed window).
- Retry logic with exponential backoff and jitter.
- Circuit breaker pattern.
- CORS helper.
- Publish to crates.io as `httpkit`.

## Non-Goals

- Not an HTTP framework (not replacing Axum or Hyper).
- Does not provide routing or application server functionality.
- Does not handle authentication — see `cryptokit`.

## Epics & User Stories

### E1 — Middleware Layer
- E1.1: Middleware components implement `tower::Layer` / `tower::Service` for composability.
- E1.2: Middleware chain is constructed via builder pattern.

### E2 — Rate Limiting
- E2.1: `RateLimiter::token_bucket(capacity, refill_rate)` creates a token bucket rate limiter.
- E2.2: `RateLimiter::fixed_window(limit, window_duration)` creates a fixed-window limiter.
- E2.3: When a request exceeds the limit, the middleware returns HTTP 429 with `Retry-After`.

### E3 — Retry
- E3.1: `RetryLayer::new(max_retries, backoff)` retries failed requests with exponential backoff.
- E3.2: Retry is only applied to idempotent methods (GET, PUT, DELETE) by default.
- E3.3: Maximum retry delay is configurable.

### E4 — Circuit Breaker
- E4.1: `CircuitBreaker::new(threshold, timeout)` tracks failure rate and opens the circuit on threshold breach.
- E4.2: Half-open state allows one probe request before fully closing.

### E5 — Helpers
- E5.1: `ResponseBuilder` creates typed JSON responses with standard status codes.
- E5.2: `RequestExtractor` parses path params, query params, and JSON bodies from `hyper::Request`.

### E6 — Testing
- E6.1: `cargo test` passes with zero failures.
- E6.2: Rate limiter and circuit breaker tests use deterministic time injection.

## Acceptance Criteria

- `cargo build` and `cargo test` succeed.
- `cargo clippy -- -D warnings` exits 0.
- All public types have doc comments.
