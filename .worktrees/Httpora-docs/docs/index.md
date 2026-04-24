---
layout: home
title: Httpora - HTTP Client/Server Toolkit
titleTemplate: false
---

# Httpora

HTTP client/server toolkit with middleware support for the Phenotype ecosystem.

## Overview

`Httpora` (also published as `httpkit`) is a Rust HTTP utility library providing ergonomic middleware, request/response helpers, and common HTTP patterns (rate limiting, retries, circuit breaking) as composable building blocks.

## Features

- **Composable Middleware Stack**: Tower-compatible async HTTP middleware
- **Rate Limiting**: Token bucket and fixed-window rate limiters
- **Retry Logic**: Exponential backoff with jitter for failed requests
- **Circuit Breaker**: Automatic failure detection and recovery
- **CORS Helpers**: Cross-origin resource sharing utilities
- **Request/Response Builders**: Ergonomic HTTP message construction

## Quick Start

```rust
use httpkit::{RateLimiter, RetryLayer, CircuitBreaker};
use std::time::Duration;

// Create a token bucket rate limiter
let limiter = RateLimiter::token_bucket(100, 10.0); // 100 capacity, 10/sec refill

// Configure retry with exponential backoff
let retry = RetryLayer::new(
    3, // max retries
    Duration::from_millis(100), // base delay
);

// Create a circuit breaker
let cb = CircuitBreaker::new(0.5, Duration::from_secs(30)); // 50% threshold, 30s timeout
```

## Links

- [Repository](https://github.com/KooshaPari/Httpora)
