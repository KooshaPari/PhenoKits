# Observably — Phenotype Observability Collection

![Observably Logo](assets/logo-placeholder.svg)

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

## Release Registry

See `release-registry.toml` for version metadata, stability information, and sub-crate status. The master index of all Phenotype collections is at `../phenotype-collections.toml`.

Schema documentation: `../docs/governance/release_registry_schema.md`

## Workspace

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Cross-Collection Integration

Observably is part of the **Phenotype named collections**:

- **Sidekick** — Agent dispatch & presence
- **Eidolon** — Device automation
- **Observably** (this) — Distributed tracing & observability
- **Stashly** — State, events, caching, migrations
- **Paginary** — Knowledge collection (specs, tutorials, handbooks)

### Event Bus

Observably uses **phenotype-bus** to subscribe to domain events from other collections and emit observability events:

```rust
use phenotype_bus::{Bus, Event};

// Subscribe to Eidolon automation events
let mut rx = automation_bus.subscribe();

while let Ok(event) = rx.recv().await {
    // Emit trace for the automation
    tracing::info!(
        target: "observably",
        event = event.event_name(),
        "Automation event received"
    );
    
    // Propagate to Stashly for event sourcing
    let trace_event = TraceEvent { event_id: event.id };
    trace_bus.publish(trace_event).await?;
}
```

See `../../phenotype-bus/README.md` and `../../docs/org-audit-2026-04/collection_build_matrix.md` for integration details.

## Governance & Contributing

**Development & AgilePlus**: All work tracked in `/repos/AgilePlus`. Check `CLAUDE.md` for governance policies and development standards.

**Workspace Commands**:
```bash
cargo build --workspace              # Build all crates
cargo test --workspace               # Run all tests
cargo clippy --workspace -- -D warnings  # Lint enforcement
cargo fmt --check                   # Format verification
```

**Quality Gates**: All code must pass clippy (zero warnings), format, and tests before committing.

## Provenance & Integration

- **observably-tracing**: Extracted from `FocalPoint/crates/focus-observability`
- Source repos retained; these are copies for productized distribution.
- Integration via **phenotype-bus**: See `../../phenotype-bus/README.md` for cross-collection event patterns.

## Related Phenotype Collections

- **[Sidekick](../Sidekick)** — Agent dispatch & presence tracking
- **[Stashly](../Stashly)** — State, events, caching
- **[Eidolon](../Eidolon)** — Device automation
- **[Paginary](../Paginary)** — Specifications & handbooks
- **[phenotype-shared](../phenotype-shared)** — Shared Rust toolkit

## License

Apache-2.0

**Status**: Active collection (Phase 2 expansion planned)  
**Maintained by**: Phenotype Org  
**Last Updated**: 2026-04-24
