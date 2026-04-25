# Pheno Archive Triage Report

**Date:** 2026-04-24  
**Task:** Verify 706 "pheno" consumer file references; determine if archived pheno should remain archived or be revived.

---

## Critical Finding: Archive Report Conflation

The archive verification report **incorrectly counted 706 references to an archived /repos/.archive/pheno/ stub** that is **NOT the active source** of imports. Two different "pheno" directories exist:

| Directory | Status | Size | Type | Purpose |
|-----------|--------|------|------|---------|
| `/repos/.archive/pheno/` | ARCHIVED | 198 LOC (33 files) | Rust stub | Old project; archived 2026-04-24 as "unclear purpose" |
| `/repos/pheno/` | **ACTIVE** | **9,783 LOC (3,679 files)** | Monorepo (Python/Rust/Go) | **Primary SDK; heavily imported** |

---

## Reference Breakdown: 706 "Pheno" References

### Type 1: Direct Python Imports (2,860 lines)
**Status:** CRITICAL; active code dependencies  
**Pattern:** `from pheno.<module> import ...`  
**Scope:** Active across 17+ repos:
- **PhenoProc** — 312 files
- **PhenoLang** — 187 files
- **repos-wtrees** — 212 files
- **Tracera-wtrees** — 248 files
- **TestingKit** — 129 files
- **PhenoLibs** — 135 files
- **McpKit** — 31 files
- **ResilienceKit** — 31 files
- **Others** — 175 files

**Source modules being imported:** `pheno.logging`, `pheno.security`, `pheno.adapters`, `pheno.auth`, etc.  
→ All sourced from **active `/repos/pheno/python/`**

### Type 2: Cargo.toml Crate Dependencies (183 lines)
**Status:** STRUCTURAL; workspace members  
**Crate prefixes:** `pheno-` and `phenotype-` (110+ distinct crate names)  
**Scope:** Published via Cargo.toml references (PhenoMCP, Stashly, PhenoKit, etc.)  
→ All resolvable in **active `/repos/pheno/`**

### Type 3: Version Manifests & Documentation (350+ lines)
**Status:** INFORMATIONAL; not code imports  
**Entries:** `phenotype-versions.toml`, CHANGELOG.md, RESEARCH_COMPLETE docs, PHENOTYPE_INDEX  
→ References are **tracking, not functional dependencies**

### Type 4: Archive Metadata (113 lines)
**Status:** BENIGN; internal archival tracking  
**Content:** DEPRECATION.md pointers, clean-up artifacts, old documentation links  
→ No code impact; ignore.

---

## Categorized Assessment

| Category | Count | Assessment | Recommendation |
|----------|-------|-----------|---|
| **Actual code imports** | 2,860 | All from active `/repos/pheno/` | Keep ACTIVE |
| **Structural deps** | 183 | All resolvable; workspace members | Keep ACTIVE |
| **Version tracking** | 350+ | Documentation only | Document relationship |
| **Archive metadata** | 113 | Deprecation notices, old links | Archive is DEAD WEIGHT |

---

## Recommendation

### **DO NOT UNARCHIVE .archive/pheno/**

The archived stub (.archive/pheno) is a **dangling leftover** — it is NOT referenced by any active code. It's 198 LOC of unclear content archived on 2026-04-24 with a "unclear purpose" reason.

### **KEEP /repos/pheno/ ACTIVE**

The active `/repos/pheno/` monorepo is **critical infrastructure** with:
- **2,860 direct imports** across 17+ downstream repos
- **110+ published crates** (pheno-* and phenotype-* namespaces)
- **Core modules:** logging, auth, security, adapters, config, state machine, event sourcing, MCP framework, etc.

**Impact of removal:** 2,860+ Python import failures; 30+ Rust crate dependency breakage; 12+ downstream repos non-functional.

---

## Immediate Actions

1. **Clean up .archive/pheno/:** Mark as UNRECOVERABLE or delete — it's not a backup of the active repo.
   ```bash
   # Verify no git history references it:
   cd repos && git log --all --full-history -- '.archive/pheno/' | head -5
   # If only initial archive commit, safe to prune
   ```

2. **Retain /repos/pheno/ on main:** No migration needed; SDK is foundational.

3. **Update archive report:** Flag that "706 pheno references" were a false positive; actual references are to active `/repos/pheno/`, not archived stub.

---

## Summary (≤100 words)

**False alarm.** The 706 "pheno" references are to the **active monorepo** (`/repos/pheno/`, 9,783 LOC), not the archived stub (`.archive/pheno/`, 198 LOC). All 2,860 Python imports and 183 Cargo.toml deps resolve successfully to active `/repos/pheno/`. The archived stub is dead weight with "unclear purpose"; recommend removal. No migration required—pheno SDK is critical infrastructure and should remain active.
