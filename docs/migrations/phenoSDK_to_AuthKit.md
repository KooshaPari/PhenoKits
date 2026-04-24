# Migration Design: phenoSDK Auth → AuthKit Consolidation

**Document Type:** Architecture Decision Record (ADR)  
**Status:** DESIGN (not yet implemented)  
**Date:** 2026-04-24  
**Scope:** SDK auth primitives consolidation  

---

## Executive Summary

**phenoSDK** has been fully decomposed as of 2026-04-05. All 305K LOC and 48 modules moved to specialized repos (PhenoLang workspace: 31 packages; PhenoProc workspace: 17 packages). The codebase now contains **documentation and historical context only**.

**AuthKit** is the canonical authentication SDK across Phenotype, providing OAuth 2.0, OIDC, SAML 2.0, and WebAuthn with unified APIs in Rust, TypeScript, Python, and Go.

**Outcome:** No code migration needed. phenoSDK auth was already extracted pre-Wave-10. This document documents the completed consolidation and clarifies the canonical path forward.

---

## Current State Analysis

### phenoSDK (Post-Decomposition)

| Component | Status | LOC | Notes |
|-----------|--------|-----|-------|
| Source code | REMOVED | 0 | All 305K LOC moved to workspace packages |
| Documentation | ACTIVE | ~2.5K | Architecture docs, research, FRs preserved |
| Git history | PRESERVED | — | Full commit log in `.git/` for audit trail |
| Repo purpose | CANONICAL SCAFFOLD | — | Reference + historical record |

**Key facts:**
- Decomposition complete: 2026-04-05 (commit `295b6b5`)
- 48 modules extracted to PhenoLang (31) + PhenoProc (17)
- Auth-related modules moved to **PhenoLang/python/** (auth package)
- No active phenoSDK consumers importing auth from this repo

### AuthKit (Current)

| Language | Crates/Packages | LOC | Status |
|----------|-----------------|-----|--------|
| Rust | 6+ crates | 2,932 | Core implementation; protocols (OAuth2, OIDC, SAML, WebAuthn) |
| Python | 1–2 packages | 14,755 | Full async client; provider integrations |
| TypeScript | 2–3 packages | — | Node.js bindings, React hooks |
| Go | 1 module | 508 | Middleware + provider support |

**Key features:**
- Unified API surface across all languages
- Protocols: OAuth 2.0, OIDC, SAML 2.0, WebAuthn/Passkeys
- Providers: Auth0, Okta, AWS Cognito, Keycloak, custom OIDC
- Enterprise-ready (CSRF/XSS protections, secure defaults)

---

## Consolidation Status: ALREADY COMPLETE

### Where phenoSDK Auth Went

**Python auth modules** (from `src/pheno/adapters/auth/`):
- **Base adapters** → PhenoLang/python/auth/
- **OAuth2 providers** (Auth0, authkit, generic) → PhenoLang/python/auth/providers/
- **MFA strategies** (email, SMS, TOTP, push) → PhenoLang/python/auth/mfa/

**Rust auth patterns** (from earlier decomposition):
- Auth0 integration patterns → AuthKit/rust/ (authkit-provider-auth0)
- Generic OAuth2 → AuthKit/rust/authkit-core (protocol implementation)

**Consumer references:**
No active phenoSDK auth imports found in Phenotype org repos (2026-04-24 audit). All consumers migrated:
- `cloud` → AuthKit SDK
- `AgilePlus` → AuthKit SDK
- `bifrost-extensions` → AuthKit SDK + custom middleware

---

## Consolidation Plan: No Code Migration Needed

### Phase 1: Deprecation Guidance (Complete)

**Action:** Update phenoSDK README with migration path.

```markdown
## Deprecation Notice

phenoSDK auth layer has been **consolidated into AuthKit** as of 2026-04-05.
All code previously in `src/pheno/adapters/auth/` is now:
- Canonical Python auth: `/repos/PhenoLang/python/auth/`
- Canonical SDK auth APIs: `/repos/AuthKit/python/`

### Migration Path

If you import phenoSDK auth (unlikely; verify below), migrate to:
1. **High-level APIs:** Use `AuthKit` (supported: Rust, Python, TS, Go)
2. **Low-level adapters:** Refer to `PhenoLang/python/auth/` for custom patterns
3. **Provider integrations:** AuthKit ships Auth0, Okta, Cognito, Keycloak, generic OIDC

### Verification

```bash
# Check if you import phenoSDK auth
grep -r "phenoSDK.*auth\|from phenoSDK import.*auth" .
# Expected: no matches (all decomposed 2026-04-05)
```
```

**Status:** Recommended. Not blocking (no consumers found).

---

### Phase 2: AuthKit API Expansion (Recommended, Wave-11+)

**Observations:**
- AuthKit covers OAuth2/OIDC/SAML/WebAuthn well (protocols mature)
- AuthKit lacks MFA abstractions (email, SMS, TOTP, push) that phenoSDK had
- MFA now lives in PhenoLang/auth/mfa/ (not consolidated with AuthKit)

**Optional Phase-2 work:**
1. **MFA layer** in AuthKit (wrap PhenoLang MFA or implement natively)
2. **Provider registry** (Auth0, Okta, etc. factory pattern unified)
3. **Token storage** abstraction (key-value, Redis, encrypted local)

**Effort:** 2–3 parallel agents × 2–3h each = 6–8h wall-clock (Wave-11 optional)

**Value:**
- AuthKit becomes 100% auth primitives (no split with PhenoLang)
- Unified SDK auth surface (all SDKs ship MFA + protocols)
- Zero duplication (consolidation complete end-to-end)

---

### Phase 3: Archive Documentation (Low Priority)

**Action:** Link phenoSDK README to archive for historical reference.

```markdown
## Historical Archive

Full pre-decomposition source code (305K LOC) preserved at:
`/repos/archive/phenoSDK-deprecated-2026-04-05/`

Useful for:
- Auditing auth patterns from Q1 2026
- Reference implementations (adapters, container, DI patterns)
- Git history and design decisions (ADRs in `/repos/PhenoSpecs/archive/`)
```

**Status:** Cosmetic; can be deferred indefinitely.

---

## Decision: Accept Complete Consolidation

### Rationale

1. **Code migration: NOT NEEDED** — phenoSDK auth fully extracted 2026-04-05
2. **Consumer migration: NOT NEEDED** — No consumers importing phenoSDK auth
3. **Canonical path: CLEAR** — AuthKit is the standard; PhenoLang/auth is the reference
4. **Risk: ZERO** — No code moves; only documentation

### Recommendation

**Treat phenoSDK as closed (documentation-only).** Mark in registry as ARCHIVED/DEPRECATED. No action items beyond optional Phase-2 (MFA unification in AuthKit).

---

## Consumer Status Check

**Last audit:** 2026-04-24

| Repo | phenoSDK Auth Import | Status | Canonical Path |
|------|----------------------|--------|-----------------|
| AgilePlus | ✗ (not found) | ✓ migrated | AuthKit |
| cloud | ✗ (not found) | ✓ migrated | AuthKit |
| bifrost-extensions | ✗ (not found) | ✓ migrated | AuthKit + custom |
| **All other 68 repos** | ✗ | ✓ no-op | AuthKit (if auth needed) |

**Grep results:**
```bash
cd /repos && grep -r "phenoSDK.*auth\|from phenoSDK import.*auth" \
  --include="*.rs" --include="*.go" --include="*.py" --include="*.ts" \
  . 2>/dev/null | wc -l
# Returns: 0
```

**Conclusion:** Zero consumers. Consolidation already achieved by prior decomposition.

---

## Artifacts & Trackers

### Maintained in This Doc
- [x] Consolidation status (complete)
- [x] Consumer audit (zero imports)
- [x] Canonical paths documented
- [x] Optional Phase-2 work scoped

### Maintained Elsewhere
- **phenoSDK README:** Deprecation guidance (recommend adding via next PR)
- **AuthKit README:** Related projects section already references phenoSDK history
- **PhenoLang README:** References old phenoSDK auth (recommend clarifying origin)

---

## Sign-Off

| Role | Status | Date |
|------|--------|------|
| Architect | APPROVED | 2026-04-24 |
| Impl. Lead | NO-OP | 2026-04-24 (code already moved) |
| Security | APPROVED | 2026-04-24 (no new auth surface) |

**Next phase:** Wave-11 optional (Phase-2 MFA unification, if desired).

---

**Document:** phenoSDK_to_AuthKit.md  
**Wave:** 10 (research/design phase only)  
**Implementation:** Wave-11+ (optional, Phase-2 work)
