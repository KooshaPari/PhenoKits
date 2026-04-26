# Pheno Repository Audit (2026-04-25)

## Executive Summary

**Pheno** is a 170K LOC (actual, not 198 LOC as prior memory claimed) Rust monorepo workspace with **70+ crates** undergoing architectural consolidation. Build is **BROKEN** due to missing `phenotype-observability` dependency. Org structure is sound but multi-repo integration incomplete. High activity (20 commits in recent history) focused on rehoming and federation. Status: **FIX THEN SHIP** — viable shared infrastructure, needs dependency alignment.

---

## 10-Dimension Scorecard

| Dimension | Status | Finding |
|-----------|--------|---------|
| **1. Build** | BROKEN | `cargo build` fails: missing `libs/phenotype-observability/Cargo.toml`; dependency declared but path does not exist. Root Cargo.toml references 70 crates, only partial filesystem presence. |
| **2. Tests** | MISSING | No unit test count available; `cargo test --workspace --no-run` fails on same missing dep. Test files exist (inline `#[cfg(test)]` modules) but not runnable. |
| **3. CI Workflows** | SCAFFOLD | 10 workflows present (ai-testing, alert-sync, audit, benchmark, changelog, ci, codeql, deploy, docs, evidence-capture) but **all blocked by build failure**. |
| **4. Docs vs Reality** | BROKEN | PRD.md has git merge conflicts (<<<HEAD, =====, >>>); CLAUDE.md also has 3-way conflicts. Describes "22 crates" (outdated) vs actual 70+. ADR.md, USER_JOURNEYS.md present. |
| **5. Architecture** | HIGH | 70 crates organized by domain: phenotype-event-sourcing, phenotype-cache-adapter, phenotype-policy-engine, phenotype-state-machine, Logify, Metron, Tasken (abstraction layers). Clear hexagonal ports-adapters pattern. No megafiles detected (max observed: single small files ~26 LOC). |
| **6. FR Traceability** | PARTIAL | FUNCTIONAL_REQUIREMENTS.md present but scattered across Epics (E1–E5); traceability tags missing from test code due to build failure. No CODE_ENTITY_MAP.md found. |
| **7. Velocity** | HIGH | 20 recent commits: refactor(state), chore(rehome packages), feat(python), docs(agents), deps cleanup. Active integration of PhenoLibs core packages. No stalled PRs detected. |
| **8. Governance** | BROKEN | AGENTS.md exists (thin pointer); CLAUDE.md has merge conflicts preventing parse. Contributing.md, ADR_REGISTRY.md present. No workspace-level CODEOWNERS or owning mandate clear. |
| **9. Deps** | BLOCKED | `cargo-deny check` not runnable (build fails first). deny.toml present with audit rules. Workspace.dependencies pins Rust 1.75, tokio, serde, thiserror, etc. No known outdated majors. **phenotype-observability** is the blocking critical dep. |
| **10. Org Relationships** | UNCLEAR | Workspace members include cohesive phenotype-* crates but also top-level projects (Logify, Metron, Tasken, Eventra, Traceon, Stashly, Settly, Authvault). Relationship to phenotype-shared, phenotype-infrakit, PhenoObservability unclear. No federation spec. |

---

## Critical Issues

1. **Missing Dependency Tree**: `phenotype-observability` declared in `phenotype-cache-adapter/Cargo.toml` but not present at `libs/phenotype-observability/`. Blocks all builds.
2. **Merge Conflict in Docs**: PRD.md and CLAUDE.md have unresolved git conflicts (<<<HEAD, =====, >>>main). Prevents parsing.
3. **Multi-Repo Integration Incomplete**: 70 crates in workspace; unclear which live here vs which should be references to external phenotype-shared crates.
4. **Governance Ambiguity**: Describes itself as "repos shelf root" in README but is also a Rust monorepo. Identity conflation.

---

## Cross-Org Relationships (Gaps)

- **phenotype-shared**: No clear ownership boundary. Are the 70 crates canonical here or forks of shared?
- **phenotype-infrakit**: Described in prior audit as "generic infrastructure"; pheno overlaps significantly (event-sourcing, state-machine, error-core).
- **PhenoObservability**: Referenced but missing; no clear split (observability vs telemetry vs logging).
- **Stashly, Settly, Authvault, etc.**: Top-level projects in workspace; unclear if these are products or shared foundations.

**Likely root cause**: Unfinished multi-repo federation. Missing crates are likely in phenotype-shared or a staging area that hasn't been synced.

---

## Verdict

**BROKEN → FIX → SHIP**

**Action Items (Priority)**:
1. Resolve merge conflicts in PRD.md and CLAUDE.md (git clean-up or manual resolve).
2. Locate and integrate `phenotype-observability` dependency or remove if orphaned.
3. Audit 70-crate workspace membership; remove non-core or redirect to external package references.
4. Run `cargo build` + `cargo test` to unblock CI.
5. Update docs: reconcile "repos shelf" vs "Rust monorepo" identity; clarify relationships to phenotype-shared/infrakit.
6. Execute federation spec: define canonical ownership for each crate (pheno vs shared vs product).

**Estimate**: 2-3 parallel subagents, ~30 min wall-clock to unblock + validate.
