# Observably — Phenotype Observability Collection

Observably is a curated collection of independent, production-grade observability crates for Rust. Tracing, logging, metrics, and resilience patterns as standalone cargo-installable packages.

## Crates

| Crate | Purpose | Status |
|-------|---------|--------|
| `observably-tracing` | OTEL-based distributed tracing + Prometheus metrics + PII filter | Extracted from FocalPoint |

## Quick Start

```toml
[dependencies]
observably-tracing = { path = "crates/observably-tracing" }
```

Each crate is independently importable and has no inter-crate dependencies.

## Workspace

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Provenance

- **observably-tracing**: Extracted from `FocalPoint/crates/focus-observability`
- Source repos retained; these are copies for productized distribution.

## License

Apache-2.0
