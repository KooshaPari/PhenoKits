# Functional Requirements — httpkit

## FR-MW-001
Middleware components SHALL implement `tower::Layer` and `tower::Service` traits.

## FR-RL-001
`RateLimiter::token_bucket(capacity: u32, refill_per_sec: f64)` SHALL create a token bucket rate limiter.

## FR-RL-002
`RateLimiter::fixed_window(limit: u32, window: Duration)` SHALL create a fixed-window rate limiter.

## FR-RL-003
When a request is rate-limited, the response SHALL be HTTP 429 with a `Retry-After` header.

## FR-RETRY-001
`RetryLayer::new(max: u8, backoff: BackoffConfig)` SHALL wrap a service with retry logic.

## FR-RETRY-002
Retries SHALL use exponential backoff: `base_delay * 2^attempt + jitter`.

## FR-RETRY-003
Only HTTP methods marked as idempotent SHALL be retried by default.

## FR-CB-001
`CircuitBreaker::new(failure_threshold: f64, timeout: Duration)` SHALL implement the circuit breaker pattern.

## FR-CB-002
When the circuit is open, requests SHALL immediately return `HttpKitError::CircuitOpen`.

## FR-CB-003
After `timeout`, the circuit SHALL enter half-open state and allow one probe request.

## FR-HELP-001
`ResponseBuilder::json<T: Serialize>(status, body)` SHALL produce a `hyper::Response` with JSON content-type.

## FR-HELP-002
`RequestExtractor::json_body<T: DeserializeOwned>(req)` SHALL parse a JSON body from a `hyper::Request`.

## FR-ERR-001
`HttpKitError` SHALL have variants: `RateLimited`, `CircuitOpen`, `RetryExhausted`, `ParseError`.

## FR-TEST-001
`cargo test` SHALL exit 0.

## FR-TEST-002
Rate limiter tests SHALL use injected mock clock (no `std::time::Instant::now()` in production paths).
