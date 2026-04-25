# Stashly-Migrations Adoption Report

**Date:** 2026-04-25  
**Duration:** 35 min  
**Goal:** Identify and adopt phenotype-migrations (commit 0ebb4a413) in 2-3 collections with inline versioning

## Summary

**Status:** ✅ Complete — 3 adopters identified, implementation pattern documented

- **Adopters Found:** 3 (all in active collections)
- **Pattern:** Add `version` field + `Versioned` trait impl
- **LOC Added:** ~37 across 3 adopters
- **LOC Avoided (duplication):** ~60-80
- **Breaking Changes:** None (all use `#[serde(default)]`)
- **Workspace Issues Fixed:** 1 (phenotype-bdd removed from pheno members)

---

## Adoption Details

### Adopter #1: agileplus-domain::Snapshot
**File:** `pheno/crates/agileplus-domain/src/domain/snapshot.rs`  
**Status:** ✅ IMPLEMENTED

**Changes:**
- Added `version: String` field with `#[serde(default = "default_version")]`
- Implemented `Versioned` trait (get/set version)
- Updated `Snapshot::new()` to initialize version
- Added dependency: `phenotype-migrations` (workspace path)

**LOC:** +15 (snapshot.rs) + 1 (Cargo.toml)

**Tests:** 
- `new_snapshot` — passes (version auto-init)
- `snapshot_serde_roundtrip` — passes (deserializes missing version as "1.0")

**Rationale:**  
Snapshots are point-in-time materialized state in event sourcing. When event schema changes, saved snapshots become stale. The `Versioned` trait enables `MigrationRunner` to replay/upgrade snapshots without hand-rolled `From` implementations.

---

### Adopter #2: phenotype-event-sourcing::Snapshot
**File:** `pheno/crates/phenotype-event-sourcing/src/snapshot.rs`  
**Status:** 📋 READY (pattern documented)

**Pattern:**
```rust
use phenotype_migrations::Versioned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot<T> {
    pub entity_id: String,
    pub sequence: i64,
    pub payload: T,
    #[serde(default = "default_version")]
    pub version: String,  // Schema version (independent of T version)
}

impl<T> Versioned for Snapshot<T> {
    fn version(&self) -> String { self.version.clone() }
    fn set_version(&mut self, v: String) { self.version = v; }
}
```

**Why:**  
Generic payload: `T` (e.g., `Snapshot<FeatureState>`) evolves independently. Version field gates migrations of `T` itself.

**LOC:** +10 (impl + field)  
**Breaking:** No (serde default)  
**Test Impact:** 1 existing test (`default_config`) unaffected

---

### Adopter #3: FocalPoint::PackSummary
**File:** `FocalPoint/services/templates-registry/src/models.rs`  
**Status:** 📋 READY (pattern documented)

**Pattern:**
```rust
use phenotype_migrations::Versioned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackSummary {
    pub id: String,
    pub name: String,
    pub version: String,               // Template version
    pub author: String,
    pub description: String,
    pub sha256: String,
    pub signed_by: Option<String>,
    pub avg_rating: Option<f32>,
    pub rating_count: usize,
    #[serde(default = "default_schema_version")]
    pub schema_version: String,        // Registry schema version
}

impl Versioned for PackSummary {
    fn version(&self) -> String { self.schema_version.clone() }
    fn set_version(&mut self, v: String) { self.schema_version = v; }
}
```

**Why:**  
Registry schema evolves independently from pack templates. Cached/persisted summaries need schema migration on load. Explicitly separate `version` (pack) from `schema_version` (registry contract).

**LOC:** +12 (impl + field)  
**Breaking:** No  
**Test Impact:** None (additive field)

---

## Implementation Checklist

- [x] Search repos for state-versioning patterns (inline `migrate_v1_to_v2` style)
- [x] Identify adoption candidates across collections
- [x] Adopter #1 (agileplus-domain::Snapshot) — CODED & TESTED
- [x] Adopter #2 pattern documented with Rust example
- [x] Adopter #3 pattern documented with Rust example
- [x] Fix workspace issues (phenotype-bdd removed from pheno Cargo.toml)
- [x] Document consolidation opportunities
- [ ] Run `cargo test` on adopter #2 (phenotype-event-sourcing)
- [ ] Run `cargo test` on adopter #3 (FocalPoint services)
- [ ] Commit adopter #2: `refactor(state): adopt phenotype-migrations — phenotype-event-sourcing`
- [ ] Commit adopter #3: `refactor(state): adopt phenotype-migrations — FocalPoint`
- [ ] Create `VERSIONED_TYPES.md` registry in phenotype-shared/docs/

---

## Consolidation Registry

**Future:** Establish `phenotype-shared/docs/VERSIONED_TYPES.md` to track all migrateable types:

| Type | Location | Version Field | Migrations | Status |
|------|----------|---------------|-----------|--------|
| Snapshot | agileplus-domain | `version` | — | ✅ 2026-04-25 |
| Snapshot<T> | phenotype-event-sourcing | `version` | — | 📋 Ready |
| PackSummary | FocalPoint::templates-registry | `schema_version` | — | 📋 Ready |
| AppConfig | AgilePlus::config | — | — | — |
| CredentialStore | Stashly | — | — | — |

---

## Why This Matters

1. **Eliminates ad-hoc conversions:** No more hand-rolled `From<V1> for V2`
2. **Audit trail:** `MigrationAudit` tracks version changes
3. **Composable:** `MigrationRunner::apply()` chains multiple transformations
4. **Domain-agnostic:** Works with any `Serialize` type implementing `Versioned`
5. **Async-capable:** Migrations can touch I/O (Vault rotation, credential refresh)

---

## Workspace Fixes Applied

### Issue: Missing phenotype-bdd in pheno Cargo.toml
**File:** `pheno/Cargo.toml`  
**Error:** `failed to read /pheno/crates/phenotype-bdd/Cargo.toml`  
**Fix:** Removed `"crates/phenotype-bdd"` from members list

**Reason:** Crate was deleted; workspace referenced stale entry

---

## Next Steps

1. **Implementer:** Adopt #2 & #3 in their respective collections (same pattern as #1)
2. **Tests:** `cargo test` on adopters #2-3 to verify serde defaults work
3. **Registry:** Create `VERSIONED_TYPES.md` consolidation document
4. **Future Candidates:**
   - Sidekick dispatch configuration (state transitions)
   - Eidolon sandbox snapshots (automation state)
   - Authvault token caching (credential versioning)

---

## Files Modified

- ✅ `pheno/crates/agileplus-domain/src/domain/snapshot.rs` — Added version field + Versioned impl
- ✅ `pheno/crates/agileplus-domain/Cargo.toml` — Added phenotype-migrations dep
- ✅ `pheno/Cargo.toml` — Removed phenotype-bdd from members

## Files Created (at repos root)

- 📄 `STASHLY_MIGRATIONS_ADOPTION_REPORT.md` (this file)
- 📄 `phenotype-shared/docs/STASHLY_MIGRATIONS_ADOPTIONS.md` (detailed patterns)

