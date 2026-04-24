# Observably — Extraction Plan

## Bootstrap (2026-04-24)

### Crates Extracted

| Crate | Source | Status | Notes |
|-------|--------|--------|-------|
| observably-tracing | `FocalPoint/crates/focus-observability` | Copied | OTEL + Prometheus + PII filter |

### Provenance

All crates in Observably are **copies**, not moves. Source repositories are retained for reference and continued development.

### Future Candidates

- **observably-sentinel**: Patterns from PhenoObservability's sentinel module (rate-limit, circuit-breaker, bulkhead)
- **observably-logging**: helix-logging patterns (structured logging, context propagation)
- **observably-metrics**: Lightweight Prometheus text format emitter

### Testing

```bash
cargo check --workspace  # 0 errors
cargo test --workspace   # All tests pass
cargo clippy --workspace # No warnings
```

All crates build and test cleanly as of 2026-04-24.
