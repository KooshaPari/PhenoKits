# Deprecation Notice

**Observably has been consolidated into PhenoObservability** (April 24, 2026).

## What Changed

All sub-crates from Observably have been migrated to PhenoObservability under the `phenotype-observably-*` naming scheme:

| Old Crate | New Location |
|-----------|--------------|
| `observably-tracing` | `phenotype-observably-tracing` (PhenoObservability) |
| `observably-logging` | `phenotype-observably-logging` (PhenoObservability) |
| `observably-sentinel` | `phenotype-observably-sentinel` (PhenoObservability) |

## Migration Path

1. Update your `Cargo.toml` dependencies to use PhenoObservability workspace:
   ```toml
   # Before
   phenotype-observably-tracing = { path = "../../Observably/crates/observably-tracing" }

   # After
   phenotype-observably-tracing = { path = "../../PhenoObservability/crates/phenotype-observably-tracing" }
   ```

2. Type-alias shims are provided in the `observably` crate (this repo) for backward compatibility:
   ```rust
   // Import from observably shim crate
   use observably::tracing::*;  // re-exported from phenotype-observably-tracing
   ```

## Why Consolidate?

- **Single source of truth**: Both repositories implement observability and monitoring
- **Reduced duplication**: Consolidation under PhenoObservability simplifies maintenance
- **Unified dependencies**: Aligned dependency versions across observability ecosystem
- **Preserved compatibility**: Type aliases ensure existing imports continue to work

## Timeline

- **April 24, 2026**: Observably crates absorbed into PhenoObservability; Observably retained as shim
- **Future**: Observably may be archived after all consumers migrate

## Support

For issues or questions about the migration, reference the consolidation commit:
`chore(consolidation): absorb Observably sub-crates`
