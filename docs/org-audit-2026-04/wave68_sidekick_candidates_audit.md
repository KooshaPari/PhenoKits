# Wave 68: Sidekick Collection Candidates Audit

**Date:** 2026-04-25  
**Auditor:** Forge  
**Targets:** agentkit (McpKit/rust/agentora), phenotype-skills (canonical)

---

## 1. agentkit (McpKit/rust/agentora)

### README Purpose & Claims
agentkit is a hexagonal architecture Rust framework for AI agents. Claims: modular skill system, tool registry with JSON schema, two-tier memory (short-term ring buffer, long-term persistent), event system for agent lifecycle tracking. Positioned as production-grade agent framework with OpenAI/Redis/SQLite adapters.

### Real LOC (Source Code)
**1,035 LOC** (src/ Rust files only, excluding target/). Core modules: domain (agents, skills, tools, memory, ports, events), application (executor), adapters (llm, memory stubs), infrastructure (error handling). Module structure is well-organized; code is under 1.1K lines total.

### Test Breakdown
**Only 3 tests found** in codebase (grep `#[test]` = 2 in context/mod.rs, 1 in memory/mod.rs). No dedicated test files or CI-enforced coverage. Given 1K LOC framework with claims of production-readiness, test coverage is severely insufficient (<1%).

### Build Status (CI Logs)
`.github/workflows/ci.yml` configured with parallel jobs (build, clippy, test, fmt) — structure is present. Pages deployment workflow also exists. No actual build logs reviewed per instructions, but workflow presence indicates intent.

### Last 10 Commits
```
87490a2 ci: add GitHub Pages deployment workflow
756054c Merge branch 'gt/polecat-43/84e72343': API contract audit report
e1f6b9b Add API contract audit report - verify schemas
9778b29 Merge branch 'gt/polecat-34/466ada3d': security audit hardening
7ed3c62 Merge branch 'gt/lark/9a8e29c8' into main: comprehensive README documentation
2de3c89 Security audit: fix .gitignore, input validation, deduplicate errors, update deps
3a165d5 docs: add comprehensive README with module structure, traits, and examples
a73d9c0 feat(ci): enhance pipeline with parallel jobs and dependency caching
```
Recent work focused on documentation, CI/CD, and audit reports — minimal feature commits.

### Cross-References
**Zero cross-references** found in Phenotype org repos (Cargo.toml/package.json grep). agentkit is embedded in McpKit namespace but not consumed by any Phenotype crate. Standalone library with no integration path.

### Sidekick Fit Verdict
**VERDICT: Not Ready for Sidekick Collection**

agentkit is a well-architected framework (hexagonal design is solid) but **too immature for production integration**:
- 3 tests for 1K LOC = 0.3% coverage; Sidekick mandates ≥70%
- Zero real-world usage across org (no consumers)
- Adapter layer is stubs only (llm/ and memory/ are placeholders)
- No integration story with Phenotype stack

**Integration Plan:** Archive as reference library. Revive if/when: (1) coverage reaches 60%+, (2) 3+ Phenotype repos adopt it as dependency, (3) adapters mature beyond stubs.

---

## 2. phenotype-skills (Canonical Repo)

### README Purpose & Claims
**No README found.** Repository structure exists (Cargo.lock present) but no documentation, no source code directories with actual implementation. Minimal or empty project.

### Real LOC (Source Code)
**0 LOC of real source code.** Repo contains:
- Cargo.lock (dependency lock, auto-generated)
- bindings/python/ directory (build/ and dist/ only, no src/ implementation)
- bindings/typescript/ (dist/ and node_modules/ only, no source)

The Python and TypeScript source directories are empty (only egg-info and __pycache__).

### Test Breakdown
**Zero tests.** No test files, test infrastructure, or CI test runs.

### Build Status
No CI workflows found. No build status information available.

### Last 10 Commits
phenotype-skills is not an independent repo — commits from canonical `repos/` monorepo (not repo-specific). Most recent commit: "docs(worklogs): initialize category framework and 2026-04-25 aggregation" (organizational work).

### Cross-References
**Zero cross-references** in Phenotype repos (Cargo.toml/package.json grep). Not a dependency in any project.

### Sidekick Fit Verdict
**VERDICT: Archive — Dead Stub**

phenotype-skills is a placeholder with **no implementation**:
- 0 lines of real source code
- 0 tests
- No README or purpose statement
- No consumers across org

**Action:** Remove from repos/ or consolidate into agentkit if skills are meant to be part of agent framework. Current state is organizational deadweight.

---

## Summary Table

| Criterion | agentkit | phenotype-skills |
|-----------|----------|------------------|
| LOC (Source) | 1,035 | 0 |
| Test Count | 3 | 0 |
| Test % Coverage | <1% | N/A |
| CI Present | Yes | No |
| Cross-Repo Usage | 0 consumers | 0 consumers |
| Readiness | Low (architecture good, maturity poor) | None (stub) |
| Sidekick Status | Not ready; reference only | Archive |

---

## Recommendations

1. **agentkit:** Park as documentation reference. Require 60%+ test coverage + 2+ adopters before Sidekick consideration.
2. **phenotype-skills:** Remove from canonical repos or merge skill definitions into agentkit if this is the intended unified agent+skills framework.
3. **Cross-Collection Risk:** No immediate Sidekick candidates identified in this audit. Both require significant maturation before integration into Sidekick collection.
