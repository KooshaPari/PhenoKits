# Stashly — Extraction Plan

## Bootstrap (2026-04-24)

### Crates Extracted

| Crate | Source | Status | Notes |
|-------|--------|--------|-------|
| stashly-cache | `crates/phenotype-cache-adapter` | Copied | Two-tier LRU + DashMap with TTL |
| stashly-eventstore | `crates/phenotype-event-sourcing` | Copied | Append-only SHA-256 hash chain |
| stashly-statemachine | `crates/phenotype-state-machine` | Copied | Generic FSM with transition guards |

### Provenance

All crates in Stashly are **copies**, not moves. Source repositories are retained for reference and continued development.

### Future Candidates

- **stashly-migrations**: SQLite migration runner (lightweight)
- **stashly-persistence**: Key-value persistence layer

### Testing

```bash
cargo check --workspace  # 0 errors
cargo test --workspace   # All tests pass
cargo clippy --workspace # No warnings
```

All three core crates build and test cleanly as of 2026-04-24.
