# httpkit

HTTP client/server framework with middleware support for Phenotype services.

## Stack
- Language: Rust (inferred from kit naming pattern)
- Key deps: Cargo, hyper or reqwest, tower middleware

## Structure
- `src/`: HTTP framework source
  - Client with retry, timeout, and auth middleware
  - Server with routing and handler abstractions
  - Middleware: logging, auth, CORS, rate limiting

## Key Patterns
- Tower-based middleware stack for composability
- Async-first (tokio/hyper)
- Type-safe request/response handling
- Shared middleware with apikit where applicable

## Adding New Functionality
- New middleware: implement `tower::Layer` + `tower::Service` in `src/middleware/`
- New client features: extend `src/client.rs`
- Run `cargo test` to verify
