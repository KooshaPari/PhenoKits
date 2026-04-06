# Httpora Specification

> Rust HTTP utility library with composable middleware for the Phenotype ecosystem.

**Version**: 1.0 | **Status**: Draft | **Last Updated**: 2026-04-02

## Overview

Tower-compatible async HTTP middleware providing rate limiting, retry, circuit breaking, and CORS as composable layers.

**Language**: Rust (edition 2021) | **Published**: crates.io as `httpkit`

## Architecture

```
┌──────────────────────────────────────────────────┐
│                 Application                       │
└──────────────────────┬───────────────────────────┘
                       │
          ┌────────────▼────────────┐
          │      Tower Stack        │
          │  ┌──────────────────┐   │
          │  │   CorsLayer      │   │
          │  ├──────────────────┤   │
          │  │ RateLimitLayer   │   │
          │  ├──────────────────┤   │
          │  │  RetryLayer      │   │
          │  ├──────────────────┤   │
          │  │ CircuitBreaker   │   │
          │  ├──────────────────┤   │
          │  │  RequestHandler  │   │
          │  └──────────────────┘   │
          └─────────────────────────┘
          ┌─────────────────────────┐
          │   Builder Module        │
          │  ResponseBuilder        │
          │  RequestExtractor       │
          └─────────────────────────┘
          ┌─────────────────────────┐
          │   Error Types           │
          │  HttpKitError enum      │
          └─────────────────────────┘
```

## Components

| Component | Responsibility |
|-----------|---------------|
| `RateLimiter` | Token bucket and fixed-window rate limiting |
| `RetryLayer` | Exponential backoff with jitter |
| `CircuitBreaker` | Three-state breaker (closed/open/half-open) |
| `CorsLayer` | Cross-origin request handling |
| `ResponseBuilder` | Ergonomic response construction |
| `RequestExtractor` | Type-safe request body/header extraction |

## Data Models

```rust
pub enum HttpKitError {
    RateLimited { retry_after: Duration },
    CircuitOpen { reset_at: Instant },
    RetryExhausted { attempts: u32, last_error: Box<HttpKitError> },
    ParseError { detail: String },
}

pub struct RateLimitConfig {
    pub capacity: u32,
    pub refill_rate: f64,  // tokens per second
    pub strategy: RateLimitStrategy, // TokenBucket | FixedWindow
}

pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub jitter: bool,
}

pub struct CircuitBreakerConfig {
    pub failure_threshold: f64,  // 0.0-1.0
    pub reset_timeout: Duration,
    pub half_open_max_requests: u32,
}
```

## API Design

```rust
let limiter = RateLimiter::token_bucket(100, 10.0);
let retry = RetryLayer::new(3, Duration::from_millis(100));
let cb = CircuitBreaker::new(0.5, Duration::from_secs(30));

// Compose middleware stack
let service = ServiceBuilder::new()
    .layer(CorsLayer::permissive())
    .layer(limiter)
    .layer(retry)
    .layer(cb)
    .service(my_handler);

// Builders
let response = ResponseBuilder::json(200, &data)?;
let body: MyType = RequestExtractor::json_body(&request)?;
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Middleware overhead (per layer) | < 1μs |
| Rate limit check | < 100ns |
| Circuit breaker check | < 200ns |
| Full stack (5 layers) | < 5μs |
| Memory per circuit breaker | < 1KB |
| Memory per rate limiter | < 512B |

## Quality Gates

- `cargo test` — all tests pass
- `cargo clippy -- -D warnings` — zero warnings
- `cargo fmt --check` — formatted
- `cargo doc` — docs build clean
- No `unsafe` code

## License

MIT OR Apache-2.0
