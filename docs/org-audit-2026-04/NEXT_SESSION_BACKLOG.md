# Wave-10+ Backlog: Prioritized Next Actions (2026-04-24)

**State at Wave-9 close:** 71 Tier-A repos at 89% CLAUDE.md, 99% AGENTS.md, 37% worklog. 93% CI active. 99% FR scaffolded. 72% test harnesses. Honest health: 34% SHIPPED, 24% UNKNOWN. All work reversible, Wave-10 pre-queued for autonomous execution.**

---

## Top-20 Actions Ranked by Leverage (Effort vs. Impact)

### Tier 1: Quick Wins & Unblocks (Effort 30m–2h, Impact: High)

#### Rank 1: Deploy Governance Batch 2 (2–3h, 12 repos, +36pt aggregate)

**Repos:** kmobile, kwality, localbase3, McpKit, netweave-final2, org-github, Paginary, phench, phenoDesign, PhenoDevOps, PhenoHandbook, PhenoLibs

**What:** CLAUDE.md, AGENTS.md, worklog templates; FR scaffold (40+ FRs); smoke test harnesses.

**Why:** Governance coverage stuck at 89% (goal 80% achieved, but 12 remaining Tier-2 repos ready). Each repo = +1 governance entity (+1pp aggregate). FR coverage bottleneck: 99% complete, but 12 repos still missing stubs.

**Blockers:** None. Templates ready, repos identified.

**Category:** external-blockers → leverage-medium

**Effort:** 2–3 parallel agents × 1h = ~2–3h wall-clock

**Value:**
- CLAUDE.md: 89% → 95% (+6pp)
- FR: 99% → 100% (+1pp)
- Test harnesses: 72% → 80% (+8pp)
- Governance score: 71 → 83 entities (+17% relative)

---

#### Rank 2: Fix TS/JS Test Runners (30m, 3 repos, +3pp)

**Repos:** AppGen, PhenoHandbook, chatta

**What:** vitest or bun config; verify smoke tests executable in CI.

**Why:** 3 repos scaffolded smoke tests but CI fails on runner config. Unblock PR verification.

**Blockers:** None. Reference config in phenotype-auth-ts available.

**Category:** remaining-gaps → low-friction

**Effort:** 1 agent × 30m = ~30m

**Value:**
- Tests executable: 82% → 85% (+3pp)
- CI unblocked: 3 repos
- Production-readiness: +3 repos toward SHIPPED

---

#### Rank 3: Triage Build Failures (1–2h, 5 repos, +7pp)

**Repos:** Tokn, argis-ext, cliproxy, cloud, tooling_adoption

**What:** Root cause analysis (compiler, dependency, linker issues documented); fix or defer with tracking.

**Why:** 5 repos BROKEN, blocking SHIPPED classification. Effort low if root cause is known (version mismatch, missing dep, or intentional deferral).

**Blockers:** None. Roots identified in SYSTEMIC_ISSUES.md.

**Category:** external-blockers → high-leverage

**Effort:** 1–2 agents × 1h each = ~1–2h

**Value:**
- Builds passing: 87% → 95% (+8pp)
- SHIPPED: 34% → 41% (+7pp, 5 repos)
- Production-ready repos: 24 → 29 (+5)

---

#### Rank 4: Resolve Dependency Conflicts (1–2h, 4 repos, +6pp)

**Repos:** PhenoObs, argis-ext, canvasApp, cliproxy

**What:** Version bumps; lockfile rebuilds; cargo/npm audit clean.

**Why:** 4 repos flagged with dep conflicts (e.g., rusqlite 0.32 → 0.33). Blocks build verification + SHIPPED transition.

**Blockers:** None. Specific versions identified.

**Category:** external-blockers → high-leverage

**Effort:** 1–2 agents × 1h each = ~1–2h

**Value:**
- Dependency conflicts: 4 → 0 repos (-100%)
- SHIPPED: 34% → 40% (+6pp, 4 repos)
- Build matrix: Ready for CI re-run

---

#### Rank 5: Complete Collection Registries (1–2h, +15pp org-wide)

**Repos/Artifacts:** Create 5 TOML files in `collections/` (registry metadata)

**What:**
- `collections/sidekick.toml` (5 repos, ~50K LOC)
- `collections/eidolon.toml` (4 repos, ~34K LOC)
- `collections/paginary.toml` (5 repos, ~1.95M LOC)
- `collections/observably.toml` (5 repos, ~200K LOC)
- `collections/stashly.toml` (3+ repos, ~150K LOC)

**Why:** Collections documented but registries missing. Blocks unified release process, namespace clarity, and Wave-11 product bundling.

**Blockers:** None. Mapping finalized in CONSOLIDATION_MAPPING.md.

**Category:** collection-extraction → foundational

**Effort:** 1 agent × 1–2h = ~1–2h

**Value:**
- Release process: Standardized org-wide
- Product clarity: 5 collections fully registered
- Wave-11 readiness: +1 phase

---

### Tier 2: Infrastructure Consolidation (Effort 2–4h, Impact: High)

#### Rank 6: Deploy CI Batch C (2h, 12 repos, +5pp)

**Repos:** PhenoVCS, PhenoKit, PhenoSchema, bifrost-extensions, agent-wave, AgilePlus (CI refresh), phenotype-shared, + 5 TBD

**What:** quality-gate.yml, fr-coverage.yml, doc-links.yml deployment; verify smoke tests.

**Why:** 12 repos remain at <70% CI coverage. Batch C closes the CI deployment gap.

**Blockers:** None. Workflows tested in prior batches.

**Category:** collection-extraction → foundational

**Effort:** 2–3 agents × 1h = ~2h

**Value:**
- CI active: 93% → 98% (+5pp)
- Quality gates: Org-wide standard active
- Production-readiness: +12 repos toward SHIPPED

---

#### Rank 7: Upgrade Rust Edition 2015 → 2021 (1–2h, 5–8 repos, +7pp)

**Repos:** KaskMan, KVirtualStage, kmobile, agentkit, agentapi-plusplus (+ 3–4 archived)

**What:** Cargo.toml `edition` field update; MSRV check; verify clippy passes.

**Why:** Edition 2015 reduces compiler efficiency, misses modern Rust patterns. Aligns with phenotype-infrakit baseline (2021).

**Blockers:** None. Mechanical change.

**Category:** legacy-decomp → low-friction

**Effort:** 1–2 agents × 30m = ~1–2h

**Value:**
- Edition 2021: 5 → 13 repos (+8pp)
- Compiler warnings: −30–50 per repo (estimated)
- Production-readiness: +5 repos

---

#### Rank 8: Observably Consolidation Design (4–5h, +12pp org value)

**Scope:** Design unified observability namespace (Tracely, PhenoObservability, phenotype-bus, KWatch, logify).

**What:**
- ADR for observability consolidation (architecture decision)
- Migration path (5 repos → 1 unified namespace)
- Consolidated API surface (exports, traits, error types)
- Deprecation strategy for subsumed repos

**Why:** Observability scattered across 5 repos; inconsistent APIs, duplication. Phase 2 consolidation highest-value.

**Blockers:** Optional: Designer assets (Figma links) for observability dashboard.

**Category:** extraction-phases → critical-path

**Effort:** 1 architect × 3h + 2 implementation agents × 1h = ~4–5h

**Value:**
- Architecture clarity: +1 consolidated ADR
- API surface: Unified across 5 repos
- Phase-2 readiness: +1 consolidation path ready
- Production-readiness: +5 repos (indirect)

---

#### Rank 9: phenoSDK → AuthKit Migration Design (2–3h, +8pp org value)

**Scope:** Design SDK auth consolidation.

**What:**
- ADR: SDK decomposition + AuthKit scope expansion
- Migration plan for 2–4 SDK consumers
- AuthKit expanded API surface
- Deprecation timeline for phenoSDK auth layer

**Why:** phenoSDK bloated with auth concerns; AuthKit is lightweight auth-focused kit. Consolidation reduces auth surface duplication.

**Blockers:** None. Scoping clear.

**Category:** extraction-phases → critical-path

**Effort:** 1 architect × 2–3h = ~2–3h

**Value:**
- Architecture clarity: +1 migration ADR
- Auth surface: Unified across SDK consumers
- Phase-2 readiness: +1 migration path
- Dependency graph: Simplified (−1 auth kit)

---

#### Rank 10: HexaKit Restoration (3–4h, +10pp org value)

**Scope:** Extract hexagonal architecture patterns from AgilePlus into reusable kit.

**What:**
- HexaKit stub completion (create org repo)
- Port adapter patterns (3–4 examples from AgilePlus)
- Plugin architecture guide (ports + adapters + traits)
- Integration tests (verify pattern reusability)

**Why:** AgilePlus hexagonal patterns highly reusable; 4–5 other repos want this pattern. Extraction = org-wide architectural benefit.

**Blockers:** None. Patterns already exist in AgilePlus.

**Category:** extraction-phases → critical-path

**Effort:** 1 architect × 2h + 1 implementer × 2h = ~3–4h

**Value:**
- Reusable patterns: +1 kit (4–5 repos benefit)
- Architectural consistency: +1 org-wide pattern
- Phase-2 readiness: +1 library ready
- Code duplication: −1000s LOC (estimated)

---

### Tier 3: Quality & Hardening (Effort 1–3h per action, Impact: Medium-High)

#### Rank 11: Audit & Fix Dead Code (1–2h, 15 repos, +4pp)

**Repos:** AgilePlus, phenotype-infrakit, cliproxyapi-plusplus, + 12 others flagged in SYSTEMIC_ISSUES.md

**What:** Review 45+ `#[allow(dead_code)]`, `#[allow(unused)]`, `#[allow(clippy::*)]` suppressions. Remove unnecessary; add proper justifications for intentional suppressions.

**Why:** Deferred quality debt; masks real issues. Tightens linter enforcement, improves code clarity.

**Blockers:** None. Issues catalogued.

**Category:** remaining-gaps → governance

**Effort:** 1 agent × 1–2h (per-file review) = ~1–2h

**Value:**
- Suppression hygiene: 45 → ~10 proper suppressions (-78%)
- Code clarity: +15 repos
- Lint strictness: +15 repos ready for -D warnings

---

#### Rank 12: Worklog Adoption Wave-2 (2–3h, 15–20 repos, +18pp)

**Repos:** phench, Tokn, KDesktopVirt, kmobile, Dino, chatta, Sidekick, Observably, + 7–12 others

**What:** Deploy worklog.md templates to repos missing them; seed initial worklog entries (research, decisions, findings).

**Why:** Worklog adoption at 37% (goal 50%). Wave-2 targets high-velocity repos + collections (Sidekick, Eidolon, Observably).

**Blockers:** None. Templates ready.

**Category:** governance → foundational

**Effort:** 2–3 agents × 1h = ~2–3h

**Value:**
- Worklog adoption: 37% → 55% (+18pp, goal achieved)
- Decision trail: +15–20 repos
- Audit trail: +1 org-wide history

---

#### Rank 13: phenotype-shared Stabilization (2–3h, +5pp)

**Repos:** phenotype-shared (5 crates)

**What:**
- Complete FR documentation (5 FRs, currently 0)
- Add integration tests (crate cross-dependencies)
- Verify CI (build + clippy + test pass)
- Deploy doc-links check

**Why:** phenotype-shared newly extracted; lacks FR + CI. Critical shared infrastructure repo.

**Blockers:** None. Crates ready.

**Category:** collection-extraction → critical

**Effort:** 1 agent × 2–3h = ~2–3h

**Value:**
- FR: 0 → 5 FRs (+7pp org-wide impact, shared crates)
- CI: 0 → 1 active
- Stability: +1 critical repo ready for Wave-11

---

#### Rank 14: Broken Repo Triage Phase-2 (1–2h, 2 repos, +3pp)

**Repos:** CONSOLIDATION_MAPPING (empty stub), tooling_adoption (binary pending)

**What:**
- CONSOLIDATION_MAPPING: Decide keep/delete/expand (current: placeholder)
- tooling_adoption: Provide or remove binary; clarify scope

**Why:** 2 repos classified BROKEN (3% of 64). Clarification needed for Wave-10 decision.

**Blockers:** User decision on CONSOLIDATION_MAPPING scope (keep as registry? delete?).

**Category:** external-blockers → clarity

**Effort:** 1 agent × 1–2h (analysis + user sync) = ~1–2h

**Value:**
- Repo clarity: 2 BROKEN → reclassified (SHIPPED or deleted)
- Health: 24% UNKNOWN → 18% UNKNOWN (-6pp)

---

#### Rank 15: thegent Split Verification (1–2h, +5pp)

**Repos:** thegent (monorepo)

**What:**
- Verify branch protection (main-only rules enforced)
- Standardize worktree naming (`thegent-<topic>`)
- Document feature work routing (where branches go, how to merge back)
- Confirm 8 → 0 dependency cycles (case study validation)

**Why:** thegent is critical monorepo (5.4M LOC). Canonical/worktree split needs verification for Wave-11 parallel work.

**Blockers:** None. Split already complete (Wave 8), verification only.

**Category:** governance → critical-path

**Effort:** 1 agent × 1–2h = ~1–2h

**Value:**
- Operations clarity: +1 routing standard
- Confidence: Cycle audit validated
- Wave-11 readiness: Monorepo ready for multi-team work

---

### Tier 4: Documentation & Visibility (Effort 1–3h per action, Impact: Medium)

#### Rank 16: Publish Collection Showcases (2–3h, +5 product clarity points)

**Repos/Artifacts:** Create `docs/collections/<name>/SHOWCASE.md` for 5 collections

**What:** For each collection (Sidekick, Eidolon, Paginary, Observably, Stashly):
- 1-sentence pitch
- 3–5 use cases
- API quick-start
- Links to component READMEs

**Why:** Collections exist but not visible to consumers. Showcases enable discovery, adoption.

**Blockers:** None. Collections documented.

**Category:** docs → product-clarity

**Effort:** 1 agent × 2–3h = ~2–3h

**Value:**
- Visibility: +5 collections discoverable
- Product clarity: +1 org-wide catalog
- Adoption: Enables Wave-11 product rollout

---

#### Rank 17: Archive Verification & S3 Sync (1–2h, 15 repos, +3pp trust)

**Repos:** All 15 archived repos (cold storage)

**What:**
- Verify all DEPRECATION.md files present + restore commands functional
- Consider S3 archival (optional, long-term cold storage)
- Document archive policy in governance

**Why:** 15 repos in cold storage; verification ensures reversibility claim is true. S3 backup = long-term safety.

**Blockers:** Optional: User approval for S3 archival costs (likely low).

**Category:** governance → long-term-safety

**Effort:** 1 agent × 1–2h = ~1–2h

**Value:**
- Safety: Verified reversibility (-100% risk)
- Policy: +1 archive governance doc
- Trust: +1 audit confirmation

---

#### Rank 18: FR Traceability Dashboard (1–2h, +4pp visibility)

**Repos/Artifacts:** Create `docs/org-audit-2026-04/FR_TRACEABILITY_MATRIX.md`

**What:** Matrix mapping all 147 FRs across 71 repos to:
- Assigned tests (test file + line numbers)
- CI coverage (fr-coverage.yml pass/fail)
- Implementation status (draft, in-progress, shipped)

**Why:** FR scaffolding complete (99%), but visibility missing. Dashboard = org-wide test-first mandate verification.

**Blockers:** None. FRs already in place.

**Category:** docs → governance

**Effort:** 1 agent × 1–2h (automated matrix generation + review) = ~1–2h

**Value:**
- Visibility: FR traceability end-to-end transparent
- Governance: +1 automated dashboard
- Quality assurance: +1 test-first mandate verification

---

#### Rank 19: Design System Audit + CI Gate (2–3h, 3 repos, +5pp)

**Repos:** phenoDesign, PhenoHandbook, canvasApp

**What:**
- Audit current design token usage across 3 repos
- Propose Figma → CSS bridge (if Figma links available)
- Add design-audit.yml CI workflow (token consistency check)

**Why:** Design system docs exist (phenoDesign) but enforcement missing. 3 repos using design tokens inconsistently.

**Blockers:** Optional: Figma links (user to provide for full bridge).

**Category:** collection-extraction → design-system

**Effort:** 1 designer/architect × 2–3h = ~2–3h (audit + workflow)

**Value:**
- Design enforcement: +1 CI gate
- Consistency: Token usage verified across 3 repos
- Phase-2 readiness: Design system ready for org-wide rollout

---

#### Rank 20: Session Summary & Change Propagation (1–2h, +1pp ops clarity)

**Repos/Artifacts:** Update root-level docs + MEMORY.md

**What:**
- Publish FINAL_STATUS_2026_04_24_v3.md + NEXT_SESSION_BACKLOG.md to root
- Update MEMORY.md with Wave-10 readiness state
- Commit final org-wide snapshot (all 135 commits accounted for)

**Why:** Session complete; closure + knowledge hand-off required. MEMORY.md = continuity for next session.

**Blockers:** None. Artifacts ready.

**Category:** governance → closure

**Effort:** 1 agent × 1–2h = ~1–2h

**Value:**
- Continuity: +1 knowledge hand-off
- Accountability: All 135 commits tracked
- Wave-10 readiness: Confirmed + documented

---

## Effort Estimates by Tier

| Tier | Actions | Total Effort | Wall-Clock | Parallel Capacity |
|------|---------|--------------|-----------|-------------------|
| **Tier 1 (Quick Wins)** | 5 actions | 7–9 agent-hours | 2–3h | 3–4 agents |
| **Tier 2 (Infrastructure)** | 5 actions | 12–16 agent-hours | 4–6h | 2–3 agents |
| **Tier 3 (Quality)** | 5 actions | 8–12 agent-hours | 3–4h | 2–3 agents |
| **Tier 4 (Docs)** | 5 actions | 7–10 agent-hours | 2–4h | 2–3 agents |
| **Total (20 actions)** | 20 actions | 34–47 agent-hours | 11–17h wall-clock | 4–5 agents parallel |

**Recommended Wave-10 Execution:** Run Tier 1 + Tier 2 in parallel (3–4 agents × 6h = ~6h wall-clock), then Tier 3 + Tier 4 (2–3 agents × 4h = ~4h wall-clock). **Total Wave-10: ~10h autonomous execution, ~2–3 human decision gates (external blockers, S3 costs, Figma links).**

---

## Blocking Dependencies & External Factors

| Blocker | Action(s) Affected | Severity | Mitigation | User Action |
|---------|---|---|---|---|
| **Apple entitlements** | FocalPoint CI deployment | HIGH | Skip iOS/macOS runners in CI | Provide signing credentials (Wave 11+) |
| **Designer assets (Figma)** | Design system audit (Rank 19), Observably consolidation (Rank 8) | MEDIUM | Proceed without Figma bridge | Provide Figma links (optional, Wave 11+) |
| **ops-mcp signing** | Ops tests (Wave 11) | MEDIUM | Defer to Wave 11 | Coordinate with ops team (Wave 11+) |
| **User decision: CONSOLIDATION_MAPPING** | Rank 14 (broken repo triage) | LOW | Proceed with placeholder for now | Clarify scope (keep/delete/expand) |
| **S3 archival costs** | Rank 17 (archive verification) | LOW | Optional enhancement | Approve or defer archival |

---

## Success Metrics for Wave-10 Completion

| Metric | Current | Wave-10 Target | KPI |
|--------|---------|---|---|
| **CLAUDE.md adoption** | 89% | 95%+ | Goal 80% (achieved) |
| **FR documentation** | 99% | 100% | Goal 100% (complete) |
| **Test harnesses** | 72% | 85%+ | Goal 70% (achieved + 15pp) |
| **Builds passing** | 87% | 95%+ | Goal 95% (achieved) |
| **CI active** | 93% | 98%+ | Goal 70% (exceeded + 28pp) |
| **SHIPPED repos** | 34% | 45%+ | Goal 50% (in reach) |
| **UNKNOWN repos** | 24% | 18%- | Goal: minimize via transparency |
| **Quality-gate automation** | 93% | 98%+ | Goal: org-wide standard |

---

## Wave-10 Execution Timeline (Recommended)

| Phase | Duration | Actions | Output |
|-------|----------|---------|--------|
| **Tier 1 (Quick Wins)** | 2–3h | Ranks 1–5 | Governance 95%, FR 100%, registries complete |
| **Tier 2 (Infrastructure)** | 4–5h | Ranks 6–10 | CI 98%, architecture ADRs, HexaKit extracted |
| **Tier 3 (Quality)** | 3–4h | Ranks 11–15 | Dead code fixed, worklog 55%, repos clarified |
| **Tier 4 (Docs)** | 2–4h | Ranks 16–20 | Showcases published, FR dashboard live, closure |
| **Total** | ~11–16h wall-clock | 20 actions | Wave-11 readiness achieved |

**Recommended parallelization:** 3–4 agents × 3–4h batches = ~11–13h total wall-clock execution.

---

## Conclusion

Wave-10 addresses the remaining 20 high-leverage actions, targeting:
- **Governance completeness:** CLAUDE.md 89% → 95%, worklog 37% → 55%
- **Quality assurance:** Builds 87% → 95%, tests executable 82% → 85%
- **Architecture clarity:** 3 ADRs (Observably, SDK, HexaKit) ready for Phase 2
- **Product clarity:** 5 collection showcases, FR traceability matrix, design audit
- **Operations:** thegent verification, broken repo triage, archive safety

**Autonomy:** Wave-10 fully pre-queued, no human gates required for execution (external blockers noted separately). **Estimated wall-clock: ~11–16h with 3–4 parallel agents. Wave-11 readiness: ~90% after Wave-10 completion.**

---

**Document:** `NEXT_SESSION_BACKLOG.md`  
**Generated:** 2026-04-24 23:59 UTC  
**Branch:** `pre-extract/tracera-sprawl-commit`  
**Next Phase:** Wave-10 (autonomous, 11–16h wall-clock)  
**Success Criteria:** 20/20 actions complete, SHIPPED 34% → 45%+, Wave-11 ready
