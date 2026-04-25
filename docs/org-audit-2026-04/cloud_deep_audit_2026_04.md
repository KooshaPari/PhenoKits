# cloud (kilocode-backend) — Deep Audit Report 2026-04

**Audit Date:** 2026-04-24  
**Scope:** /repos/cloud (kilocode-backend)  
**Status:** CRITICAL TYPECHECK BLOCKER (in progress)

---

## 1. Overview & Stack

**Project Name:** kilocode-backend (cloud)  
**Description:** Next.js-based multi-tenant backend platform for Kilocode ecosystem  
**Primary Purpose:** Backend infrastructure for workspace management, AI model gateway, payment processing, real-time collaboration

### Technology Stack

| Category | Technology |
|----------|-----------|
| **Framework** | Next.js 16 (React 19, TypeScript) |
| **Language** | TypeScript (1,946 files, 305K LOC) |
| **Package Manager** | pnpm 10.27.0 |
| **Database** | Drizzle ORM + PostgreSQL (prod), SQLite (dev) |
| **Monorepo** | pnpm workspace (4 internal packages) |
| **AI SDKs** | Vercel AI SDK, @anthropic-ai, @openai, @mistralai |
| **Auth** | NextAuth.js, Stytch, WorkOS |
| **Payments** | Stripe |
| **Build/Lint** | oxlint, oxfmt, TypeScript, Jest, Playwright |

---

## 2. Repository Metrics

### Code Size & Composition

```
Source LOC (src/):     313,943 lines
  TypeScript:         305,831 (97.4%)
  JSON:                 3,860
  HTML:                 3,003
  CSS:                    309

Files in src/:         2,025 total
  TypeScript files:    1,946 (96.1%)
  Test files (.test.ts): 370 (19% of TS files)

Languages by presence:
  TypeScript:   1,946 files
  JSON:            36 files
  HTML:            22 files
  Markdown:         9 files
  CSS:              3 files
```

### Workspace Structure

```
cloud/
├── src/                          # Next.js app (305K LOC)
├── packages/                     # Internal pnpm workspace
│   ├── trpc/                    # tRPC API router
│   ├── db/                      # Drizzle schema + migrations
│   ├── encryption/              # Encryption utilities
│   └── worker-utils/            # Cloudflare Worker helpers
├── cloudflare-*/                # 14 Cloudflare Workers
│   ├── cloud-agent/
│   ├── cloud-agent-next/
│   ├── cloudflare-app-builder/
│   ├── cloudflare-db-proxy/
│   ├── cloudflare-deploy-infra/
│   ├── cloudflare-gastown/
│   ├── cloudflare-security-*/
│   └── ... (8 more workers)
├── kiloclaw/                    # Rule engine (separate workspace)
├── storybook/                   # UI component library
├── docs/                        # ADRs, FRs, journeys, traceability
├── scripts/                     # Dev/deploy automation (bash)
└── Makefile                     # Build automation
```

### Dependencies

- **Direct Dependencies:** 112 (@ai-sdk/*, @anthropic-ai, @openai, @mistralai, @radix-ui/*, next, react, stripe, etc.)
- **Dev Dependencies:** 33 (jest, playwright, @types/node, typescript, oxlint, etc.)
- **Internal Packages:** 4 (@kilocode/db, @kilocode/encryption, @kilocode/worker-utils, @kilocode/kiloclaw-secret-catalog)

---

## 3. Top 5 Largest Files by LOC

| File | LOC | Type | Purpose |
|------|-----|------|---------|
| **kiloclaw-billing-router.test.ts** | 2,779 | Test | Comprehensive billing subscription test suite |
| **KiloclawInstanceDetail.tsx** | 2,749 | Component | Complex UI component for instance configuration |
| **kiloclaw-router.ts** | 2,732 | Route Handler | tRPC router for KiloClaw rule engine management |
| **router.d.ts** (gastown types) | 2,533 | Type Def | Generated TypeScript type definitions |
| **gitlab/adapter.ts** | 2,448 | Integration | GitLab OAuth/API adapter implementation |

**Observation:** Large test files and component files indicate complex features (billing, integrations) and UI richness. Type definition file size suggests heavy use of generated or complex API contracts.

---

## 4. Build, Typecheck, Lint, Test Status

### Current State: BLOCKED

**Typecheck Status:** ❌ **FAILING** (critical)

```
Error: pnpm install → pnpm run typecheck fails with multiple TS2769 + TS2345 errors

Key Issues:
  1. Buffer / ArrayBuffer type incompatibility (Node.js / Web API mismatch)
  2. Location: src/lib/byok/encryption.ts, oauth-state.ts, github/adapter.ts, gitlab/adapter.ts
  3. Root cause: TypeScript 5.x stricter ArrayBuffer/Buffer assignment checks
  4. Impact: @kilocode/trpc package build fails → cascading build failure
```

**Lint Status:** ⏸️ **UNKNOWN** (blocked by prepare failure)  
**Test Status:** ⏸️ **UNKNOWN** (blocked by prepare failure)  
**Format Status:** ⏸️ **UNKNOWN** (blocked by prepare failure)

### Root Cause Analysis

The project has a **prepare hook** (`pnpm run prepare`) that runs `@kilocode/trpc build` before typecheck. This build invokes `tsgo` (TypeScript build) which fails on:

- `src/lib/byok/encryption.ts(65,35)` — Buffer passed where ArrayBuffer expected
- `src/lib/integrations/oauth-state.ts(85,29)` — Buffer incompatible with ArrayBufferView
- Similar errors in GitHub and GitLab adapters

**Workaround:** The recent worklog (2026-04-24) documents a partial fix for `@phenotype/ui-utils` missing dep, but the Buffer/ArrayBuffer issue remains unresolved.

---

## 5. Governance & Documentation State

### Present (Good)

- ✅ **CLAUDE.md** — Basic project governance (stub, 8 lines)
- ✅ **AGENTS.md** — Thin pointer to CLAUDE.md
- ✅ **README.md** — Rewritten 2026-04-24 with project summary, stack, quick start
- ✅ **FUNCTIONAL_REQUIREMENTS.md** — Created 2026-04-24 with FR-CLOUD-001 through FR-CLOUD-010 (10 stubs, all Draft status)
- ✅ **WORKLOG.md** — 2026-04-24 entry documenting @phenotype/ui-utils fix + FR scaffolding
- ✅ **docs/adr/, docs/journeys/, docs/stories/, docs/traceability/** — Skeleton directories present
- ✅ **CHANGELOG.md** — Present (minimal; "Unreleased" section only)

### Missing / Incomplete

- ❌ **PLAN.md** — Referenced in FR docs, not created
- ❌ **Detailed FR specifications** — FRs are stubs; no acceptance criteria, test counts, or implementation details
- ❌ **Test Coverage Tracker** — No test count targets or traceability from FRs to tests
- ❌ **Architecture Decision Record (ADR)** — adr/ directory exists but empty
- ❌ **Deployment Guides** — scripts/ exist but undocumented

### Quality Checklist

| Item | Status | Notes |
|------|--------|-------|
| Typecheck passing | ❌ FAIL | Buffer/ArrayBuffer issue blocks prepare |
| Lint configured | ✅ PASS | .oxlintrc.json present |
| Tests present | ✅ PASS | 370 test files found |
| Documentation | ⚠️ PARTIAL | FR stubs only, no detailed specs |
| AgilePlus tracking | ⚠️ TODO | Specs need formal AgilePlus entries |
| Test traceability | ⚠️ TODO | No FR-to-test mapping visible |

---

## 6. Status Classification

**Active + Blocked**

- **Active:** Heavy recent commits (33 in 25 days), active development on billing/KiloClaw features
- **Deployed:** Vercel (primary) + Cloudflare Workers (14 edge functions)
- **Maturity:** ~70% complete; core infrastructure stable, features in active development
- **Blockers:** Typecheck failure prevents validation and CI; must be unblocked urgently

---

## 7. Top 3 Next Actions

### Action 1: Resolve Buffer/ArrayBuffer Typecheck Blocker (URGENT)

**Scope:** Fix src/lib/byok/encryption.ts, oauth-state.ts, adapters  
**Approach:**
- Audit Node.js Buffer usage in crypto/BYOK code
- Convert Buffer → Uint8Array where Web API compatibility needed
- Update type annotations to match Node.js v22 stdlib
- Verify tsconfig.json lib settings (dom, dom.iterable compatibility)

**Effort:** ~15 min (1 agent, 3-4 tool calls)  
**Blocks:** Everything downstream (lint, test, CI, deployment)

### Action 2: Promote FUNCTIONAL_REQUIREMENTS to AgilePlus + Assign Ownership

**Scope:** FR-CLOUD-001 through FR-CLOUD-010  
**Approach:**
- Create AgilePlus spec for each FR (acceptance criteria, test counts, priority)
- Assign ownership + milestone (e.g., "Cloud v0.2.0")
- Add test coverage targets (e.g., "FR-CLOUD-007 requires ≥85% coverage")
- Link FRs to existing test files via comments

**Effort:** ~20 min (user or agent, AgilePlus CLI)  
**Unblocks:** Work tracking + test traceability

### Action 3: Decompose Largest Files + Extract Shared Utilities

**Scope:** Top 3 files (2.7K–2.7K LOC each)  
**Approach:**
- **kiloclaw-router.ts (2.7K):** Split tRPC routes by domain (config, instances, rules, events)
- **KiloclawInstanceDetail.tsx (2.7K):** Extract form sections into sub-components
- **gitlab/adapter.ts (2.4K):** Extract OAuth flow, API methods into separate modules

**Effort:** ~45 min (1-2 agents, refactor + test)  
**Impact:** Improves maintainability + code review cycles; reduces cognitive load

---

## 8. Quality Gate Summary

| Gate | Status | Details |
|------|--------|---------|
| **Typecheck** | ❌ FAIL | Buffer/ArrayBuffer type errors |
| **Lint** | ⏸️ BLOCKED | Awaiting typecheck fix |
| **Format** | ⏸️ BLOCKED | Awaiting typecheck fix |
| **Test** | ⏸️ BLOCKED | Awaiting typecheck fix |
| **Build** | ❌ FAIL | pnpm install → prepare hook fails |
| **Documentation** | ✅ PARTIAL | FRs scaffolded; need AgilePlus promotion |
| **AgilePlus Tracking** | ⚠️ TODO | No specs created yet |

---

## 9. Metrics Summary

```
Total LOC (src/):        313,943
  Code:                  305,831 (97.4% of total)
  Comments:               23,178 (7.4% comment ratio)
  Blank lines:            39,007 (12.4% whitespace)

Files:
  TS/TSX:                 1,946 (96.1%)
  Test files:               370 (19% of TS)
  Packages:                  4 (internal workspace)
  Cloudflare Workers:       14 (separate deployments)

Dependencies:            145 (112 prod + 33 dev)

Deployment Targets:      2 (Vercel primary, Cloudflare Workers edge)
```

---

## 10. Risk Assessment

**High Risk:**

1. **Type System Instability** — Buffer/ArrayBuffer inconsistency across crypto, OAuth, adapter code suggests inconsistent Node.js vs. Web API usage. Fix required before production changes.
2. **Large Test Files** — 2.7K LOC test file suggests brittle, tightly-coupled test suite. Decompose into smaller, focused tests.
3. **Undocumented Deployment** — 14 Cloudflare Workers with no deployment guide; risk of inconsistent deployments or missed updates.

**Medium Risk:**

4. **Incomplete FR Specifications** — FRs are stubs; no acceptance criteria or test targets. Risk of scope creep and incomplete implementations.
5. **Missing Architecture Documentation** — adr/ directory empty; no decisions documented for multi-cloud deployment, AI gateway, or event streaming patterns.

---

## 11. Opportunities for Improvement

1. **Extract Shared Crypto Utilities** — Consolidate Buffer/ArrayBuffer handling into a single, well-tested utility module
2. **Cloudflare Worker Registry** — Document and centralize deployment targets (14 workers across 4 domains)
3. **Test Traceability Dashboard** — Add FR-to-test mapping in docs/reference/TEST_COVERAGE_TRACKER.md
4. **ADR Backfill** — Document decisions for OAuth flow, multi-cloud deployment, KiloClaw rule engine integration
5. **Component Library** — storybook/ present but undocumented; promote as reusable UI foundation across Kilocode ecosystem

---

## Conclusion

**cloud** is an **active, large (313K LOC), deployed system** with **critical technical blockers** that must be resolved immediately. The project has good governance scaffolding (CLAUDE.md, AGENTS.md, FR docs) but needs:

1. **Urgent:** Resolve typecheck blocker (Buffer/ArrayBuffer issue) — blocks all validation and CI
2. **Near-term:** Promote FRs to AgilePlus + add test traceability
3. **Medium-term:** Decompose large files, document architecture, centralize Cloudflare deployments

**Recommended Dispatch:** Send 1 agent to fix typecheck blocker (15 min), then 1 agent to promote FRs to AgilePlus (20 min).

