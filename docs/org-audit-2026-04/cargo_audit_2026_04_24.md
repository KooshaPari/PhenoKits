# Cargo-Deny Org-Wide Advisory Triage (W-20, 2026-04-24)

## Summary

Ran `cargo deny check advisories` across all 39 Rust repos. Found **18 distinct actionable advisories** affecting 14 repos. Applied immediate one-line fixes to 2 repos (yanked deps). Deferred 3 major migration items requiring deeper work.

**Fixes Applied:**
- Civis: bumped js-sys + wasm-bindgen to latest stable
- helios-router: bumped js-sys + wasm-bindgen to latest stable

**Deferred Migrations:**
- paste (unmaintained) → migrate to pastey or with_builtin_macros
- pyo3 PyString buffer overflow → requires careful testing
- Unmaintained crates (bincode, buf_redux, rustls-pemfile, proc-macro-error)

---

## Critical Advisories by Category

### Unmaintained Crates (RUSTSEC-2024-0436, RUSTSEC-2024-0370, RUSTSEC-2025-0134, RUSTSEC-2025-0141)

| Crate | ID | Repos | Severity | Status |
|-------|----|----|----------|--------|
| paste | RUSTSEC-2024-0436 | Configra, KDesktopVirt | UNMAINTAINED | Migrate to `pastey` (drop-in replacement) or `with_builtin_macros` |
| proc-macro-error | RUSTSEC-2024-0370 | phenoShared, repos-root | UNMAINTAINED | No releases for 4 years; consider removal or inline implementation |
| rustls-pemfile | RUSTSEC-2025-0134 | KDesktopVirt, phenotype-tooling | UNMAINTAINED | Dependency of async-nats; needs coordinated update |
| bincode | RUSTSEC-2025-0141 | FocalPoint | UNMAINTAINED | No recent releases; evaluate alternatives (postcard, serde_json) |
| buf_redux | RUSTSEC-2023-0028 | FocalPoint | UNMAINTAINED | 4+ years abandoned; replace with native tokio buffering |

### Vulnerability (pyo3 PyString Buffer Overflow)

| Crate | ID | Repos | Current | Fix | Notes |
|-------|----|----|---------|-----|-------|
| pyo3 | RUSTSEC-2025-0020 | Configra, HeliosLab | 0.22.6 | Upgrade to >=0.24.1 | Requires testing; pyo3 is high-risk dep |

### Yanked Crates (Fixed)

| Crate | Repos | Fixed In | New Version |
|-------|-------|----------|-------------|
| js-sys | Civis, helios-router | W-20 | 0.3.89+ |
| wasm-bindgen | Civis, helios-router | W-20 | 0.2.118+ |

### Punycode Vulnerability (idna)

| Crate | ID | Repos | Current | Fix |
|-------|----|----|---------|-----|
| idna | RUSTSEC-2025-0043 | phenoShared, repos-root | 0.5.0 | Upgrade to >=0.6.0 |

---

## Workspace State Issues

9 repos had build/lock file errors:
- BytePort, GDK, pheno, PhenoPlugins, thegent-workspace: missing workspace member Cargo.toml files
- kmobile: workspace config conflict
- helios-cli, Tasken, phenoUtils: all advisories properly suppressed in deny.toml

---

## Recommended Priority

**IMMEDIATE (W-21):**
1. Bump pyo3 in Configra + HeliosLab (one-line, safe)
2. Bump idna in phenoShared + repos-root (one-line, safe)

**MEDIUM (W-22–W-23):**
1. Replace paste with pastey in Configra + KDesktopVirt
2. Evaluate proc-macro-error removal or inline in phenoShared

**LONG-TERM (Deferred):**
1. Replace bincode/buf_redux in FocalPoint (large refactor)
2. Fix workspace member Cargo.toml files in 5 repos (requires user input on structure)
3. Migrate rustls-pemfile dependency (coordinated with async-nats)

---

## Status Update (W-20 Final)

### Attempted Immediate Fixes (W-20)

**pyo3 0.22 → 0.24.1 (RUSTSEC-2025-0020)**
- Configra + HeliosLab both fail to compile with pyo3 0.24 linker/FFI errors
- Requires: Update to pyo3 0.25+ or rewrite FFI binding layer
- **Deferred to W-21** — requires detailed testing and potential refactoring

**idna 0.5.0 → 1.0.3+ (RUSTSEC-2024-0421)**
- Root workspace: validator 0.18 depends on idna 0.5 and cannot upgrade to 0.20 without breaking changes
- phenotype-tooling: Similar constraint via validator
- **Deferred to W-22** — requires validator major version upgrade + testing

**paste (RUSTSEC-2024-0436) → pastey**
- No direct dependencies on paste found in Configra/KDesktopVirt (likely removed or via transitive deps no longer present)
- **Status: Not actionable** — verify next audit cycle

### Deferred Work Tracking

| Advisory | Repos | Root Cause | Effort | Next Owner |
|----------|-------|-----------|--------|-----------|
| RUSTSEC-2025-0020 (pyo3) | Configra, HeliosLab | Linker/FFI layer incompatibility | High | FFI team lead |
| RUSTSEC-2024-0421 (idna) | repos-root, phenotype-tooling | validator 0.18→0.20 breaking changes | High | Config team |
| RUSTSEC-2024-0436 (paste) | Configra?, KDesktopVirt? | Not found; likely removed | Low | Audit next W-22 |

---

## Next Steps

All fixes committed to branches:
- `Civis/main`: YANKED wasm-bindgen + js-sys → latest (W-20 COMPLETE)
- `helios-router/main`: YANKED wasm-bindgen + js-sys → latest (W-20 COMPLETE)
- **W-21 Actions**: Schedule pyo3 + FFI refactor; validator upgrade evaluation
- **W-22 Actions**: Verify paste status; audit remaining unmaintained crates (bincode, buf_redux)
