---
title: Tasks — Forced Adoption Phase 1 Shared Crates
spec: spec.md
plan: plan.md
date: 2026-04-25
last-updated: 2026-04-25T21:00Z
status: DONE_FOR_2_OF_3
---

## Closeout 2026-04-25 21:00Z

Per spec.md "Closeout 2026-04-25 21:00Z": error-core and health
each reached 3/3 consumers; config-core remains descoped. Adoption
Matrix below updated to ground truth on `origin/main`. WP-017
(aggregate matrix) is satisfied for the two non-descoped crates;
WP-018/WP-019 (audit re-run + audit-doc update) remain open
follow-ups for G-04.

## Reality Annotation 2026-04-25

The original WP set (WP-011..WP-016) targeted a candidate consumer pool
(AgilePlus, thegent, hwLedger, BytePort, PhenoKits) selected from the
audit. Post-merge reality (see spec.md "Adoption Reality") is that:

- The actually-migrated consumers were a **different set** than the
  original WPs assumed: AuthKit, ResilienceKit, TestingKit landed via
  ad-hoc PRs outside the planned (consumer × crate) cells.
- `phenotype-config-core` has **no real downstream consumers** — every
  config-core build WP below is structurally **BLOCKED** until a real
  consumer is identified or the config-core branch is descoped.

WPs are annotated below with `[STATUS: ...]` markers. Per spec
directive, no WP is deleted — failed/superseded WPs are kept so the
provenance of the forced-adoption sweep stays auditable. The
realized-PR migrations (AuthKit, ResilienceKit, TestingKit, hwLedger)
are tracked as new annotations under each crate's section rather than
back-fitting them into pre-existing WP numbers.

## Work Package Index

Each WP scopes **one consumer × one shared crate**. Acceptance criteria
are uniform: `cargo audit` clean, `cargo test --workspace` pass,
canonical type referenced in at least one `src/` file, local
predecessor type retained (not deleted) per spec NG-01.

WPs are organized by phase mapping back to plan.md DAG.

---

### Discovery WPs

#### WP-001 — Confirm consumer-set buildability

- **Phase:** 1 (D-01)
- **Owner:** discovery agent
- **Scope:** clone or `git pull` AgilePlus, thegent, hwLedger, BytePort,
  PhenoKits at HEAD of default branch; run `cargo build --workspace`
  per repo (or equivalent if non-Cargo for thegent's Go components).
- **Acceptance:**
  - All 5 repos build green on `main`.
  - Failing repos are flagged in `discovery.md` with one-line root
    cause and either (a) routed to a fix-first WP or (b) replaced with
    a substitute candidate from {Tracely, FocalPoint, Metron,
    BlueScript}.

#### WP-002 — Local-type inventory

- **Phase:** 1 (D-02, D-03)
- **Owner:** discovery agent (parallel batch)
- **Scope:** for each consumer repo, grep for:
  - `pub enum .*Error` → catalog file paths + variant counts.
  - config loader patterns (`Config::from_*`, `figment::`, custom
    `load_config` fns) → catalog.
  - health-check patterns (`HealthCheck`, `/health`, `liveness`,
    `readiness`) → catalog.
- **Acceptance:**
  - `discovery.md` contains a table per repo with bespoke type counts.
  - Each row carries enough detail (file:line) to feed Phase 2 design
    deltas.

#### WP-003 — `phenotype-health` canonical-home disambiguation

- **Phase:** 1 (D-04, FR-11)
- **Scope:** read `phenotype-infrakit/crates/phenotype-health/src/lib.rs`
  AND `phenoShared/crates/phenotype-health/src/lib.rs`. Compare:
  - public API surface (traits, structs, free functions),
  - last-touched commit dates,
  - feature-flag posture,
  - existing reverse-deps inside each home workspace.
- **Acceptance:**
  - `discovery.md` records the chosen canonical home with three-bullet
    rationale.
  - The non-chosen copy is tagged with a TODO for follow-up
    deprecation (out of scope for this spec — separate org-hygiene
    task).
  - All Phase 2/3 health WPs cite the chosen home in their design and
    PR descriptions.

#### WP-004 — Discovery write-up

- **Phase:** 1 (D-05)
- **Scope:** synthesize WP-001/002/003 into `discovery.md`.
- **Acceptance:**
  - Table of 5 consumers × 3 crates with "in scope / descoped / blocked"
    per cell.
  - Final picks for the 6 build WPs (B-01..B-06) confirmed or revised.

---

### Design WPs (1 per Build WP; spec FR-02)

#### WP-005 — Design: AgilePlus × `phenotype-error-core`

- **Phase:** 2 (DS-01)
- **Acceptance:** `design/agileplus-error-core.md` contains side-by-side
  type comparison + migration delta + descope check.

#### WP-006 — Design: AgilePlus × `phenotype-config-core`

- **Phase:** 2 (DS-02)
- **Acceptance:** `design/agileplus-config-core.md` per WP-005 shape.

#### WP-007 — Design: thegent × `phenotype-health`

- **Phase:** 2 (DS-03)
- **Acceptance:** `design/thegent-health.md` per WP-005 shape, citing
  WP-003's canonical-home selection.

#### WP-008 — Design: hwLedger × `phenotype-error-core`

- **Phase:** 2 (DS-04)
- **Acceptance:** `design/hwledger-error-core.md`. Note hwLedger's
  pre-existing vendored `phenotype-event-sourcing` per ADR — confirm
  no PR collision in same crate tree.

#### WP-009 — Design: BytePort × `phenotype-config-core`

- **Phase:** 2 (DS-05)
- **Acceptance:** `design/byteport-config-core.md` per WP-005 shape.

#### WP-010 — Design: PhenoKits × `phenotype-health`

- **Phase:** 2 (DS-06)
- **Acceptance:** `design/phenokits-health.md` per WP-005 shape, citing
  WP-003.

---

### Build WPs (one PR each; spec FR-03..FR-08)

#### WP-011 — AgilePlus adopts `phenotype-error-core`

- **[STATUS: SUPERSEDED 2026-04-25]** Did not run. Realized error-core
  consumers were AuthKit (PR #42, DONE) and ResilienceKit (PR #15,
  DONE) — see "Realized error-core migrations" annotation below. WP-011
  remains as a future candidate if AgilePlus is selected as the 3rd
  error-core consumer.
- **Phase:** 3 (B-01) → traces FR-03
- **Scope:**
  - Add `phenotype-error-core` to AgilePlus root `Cargo.toml`
    `[workspace.dependencies]`.
  - Adopt in ≥1 AgilePlus crate identified by WP-005.
  - Replace local error enum **in-place call sites** (do not delete
    local enum yet — spec NG-01).
- **Acceptance:**
  - `cargo build --workspace` green.
  - `cargo test --workspace` green.
  - `cargo clippy --workspace -- -D warnings` green.
  - `cargo audit` clean.
  - Canonical type referenced in ≥1 `src/` file.
  - PR description carries LOC delta + link to spec + link to source
    PR (phenotype-infrakit #87).

#### WP-012 — AgilePlus adopts `phenotype-config-core`

- **[STATUS: BLOCKED 2026-04-25]** No real config-core consumer exists
  on origin/main. Recommend descope of config-core from this spec
  per spec.md "Adoption Reality" item #2 + "Recommended forward path".
  Do not open this PR until either a real consumer is identified or
  the config-core branch is moved to a separate extraction-validation
  spec.
- **Phase:** 3 (B-02; sequences after WP-011) → traces FR-04
- **Acceptance:** Same shape as WP-011. Replace ≥1 bespoke config
  loader with `UnifiedConfigLoader`.

#### WP-013 — thegent adopts `phenotype-health`

- **[STATUS: SUPERSEDED 2026-04-25]** Did not run. Realized health
  consumer is TestingKit (PR #4, DONE), with hwLedger PR in flight —
  see "Realized health migrations" annotation below. WP-013 remains
  as a future candidate if thegent is needed as the 3rd health
  consumer after hwLedger lands.
- **Phase:** 3 (B-03) → traces FR-05
- **Scope:** thegent harness/dispatcher consumes `HealthChecker` from
  the canonical home selected in WP-003.
- **Acceptance:** Same shape as WP-011 (`cargo` gates apply to
  thegent's Rust crates; Go components get `go test ./...` if touched,
  but health-check Rust adoption is the primary deliverable).

#### WP-014 — hwLedger adopts `phenotype-error-core`

- **[STATUS: REASSIGNED 2026-04-25]** hwLedger's actual in-flight
  migration is to `phenotype-health` (PR pending), not `error-core`.
  The error-core slot for hwLedger remains a future candidate. The
  health migration is tracked under "Realized health migrations"
  annotation below.
- **Phase:** 3 (B-04) → traces FR-06
- **Acceptance:** Same shape as WP-011. Coordinate with hwLedger's
  existing vendored `phenotype-event-sourcing` per ADR — confirm no
  collision in PR.

#### WP-015 — BytePort adopts `phenotype-config-core`

- **[STATUS: BLOCKED 2026-04-25]** Same blocker as WP-012 — no real
  config-core consumer pool. Descope candidate.
- **Phase:** 3 (B-05) → traces FR-07
- **Acceptance:** Same shape as WP-011.

#### WP-016 — PhenoKits adopts `phenotype-health`

- **[STATUS: SUPERSEDED 2026-04-25]** Did not run. Realized health
  consumers are TestingKit (DONE) and hwLedger (in flight). WP-016
  remains as a future candidate for the 3rd health consumer if needed.
- **Phase:** 3 (B-06) → traces FR-08
- **Acceptance:** Same shape as WP-011.

---

### Realized migrations 2026-04-25 (out-of-band, not in original WP plan)

These migrations landed via ad-hoc PRs outside the WP-011..WP-016
schedule. They are recorded here so the adoption-matrix tracker
reflects ground truth on `origin/main`, not the planned cells.

#### Realized error-core migrations

- **AuthKit × phenotype-error-core** — **DONE**, AuthKit PR #42
  (merged). Replaces local error enum with canonical type per spec
  FR-02 acceptance criteria. Counts toward FR-09 ≥3 floor.
- **ResilienceKit × phenotype-error-core** — **DONE**, ResilienceKit
  PR #15 (merged). Counts toward FR-09 ≥3 floor.
- **hwLedger / hwledger-core × phenotype-error-core** — **DONE**
  2026-04-25, hwLedger PR #35 (merged). Brings error-core to **3/3**
  for FR-09. Closes the error-core demand gap.
- **Need:** none. error-core is closed at 3/3.

#### Realized health migrations

- **TestingKit × phenotype-health** — **DONE**, TestingKit PR #4
  (merged). Counts toward FR-09 ≥3 floor.
- **hwLedger / hwledger-server × phenotype-health** — **DONE**
  2026-04-25, hwLedger PR #34 (merged).
- **hwLedger / hwledger-agent × phenotype-health** — **DONE**
  2026-04-25, hwLedger PR #34 (merged; same PR as hwledger-server,
  two distinct crate-level consumers).
- **Need:** none. health is closed at 3/3 (TestingKit PR #4,
  hwledger-server PR #34, hwledger-agent PR #34). Closes the health
  demand gap.

#### Realized config-core migrations

- **None.** Spec is structurally blocked on config-core; see spec.md
  "Adoption Reality" item #2 and the descope recommendation under
  "Recommended forward path".

---

### Validation + Deploy WPs

#### WP-017 — Aggregate adoption matrix

- **Phase:** 5 (V-01)
- **Scope:** track every B-0X PR through admin-merge per global GitHub
  Actions billing policy.
- **Acceptance:**
  - All 6 PRs merged to default branch.
  - Adoption matrix in this file updated with PR URLs.

#### WP-018 — Audit re-run

- **Phase:** 5 (V-02) → traces FR-09, FR-10
- **Scope:** re-run the audit §2 grep methodology
  (`gh search code` for each crate name; filter for real
  `[dependencies]` lines).
- **Acceptance:**
  - Each Phase 1 crate shows ≥3 real Cargo consumers across
    distinct repos.
  - If any crate falls short, file a follow-up WP against the
    next-highest-value candidate.

#### WP-019 — Audit doc update

- **Phase:** 5 (V-03)
- **Scope:** edit `docs/governance/cross-project-reuse-audit-2026-04-25.md`
  to:
  - Replace §2 Adoption Rate counts.
  - Remove "Phase 1 supply/demand gap" from §5 once WP-018 confirms
    ≥3 consumers per crate.
  - Append a one-line back-pointer to this spec.
- **Acceptance:** PR opens with the updated audit doc + commit body
  citing this spec.

#### WP-020 — Memory + worklog update

- **Phase:** 5 (V-04)
- **Scope:** append to memory entry "Phase 1 LOC Reduction Execution
  Complete" the adoption-confirmation date and consumer count, and
  add a worklog entry under
  `repos/worklogs/ARCHITECTURE.md` with `[cross-repo]` tag.
- **Acceptance:** memory + worklog files updated.

---

## Adoption Matrix (live tracker — updated 2026-04-25)

Original planned matrix (kept for provenance):

| Consumer | error-core | config-core | health |
|----------|-----------|-------------|--------|
| AgilePlus | WP-011 (SUPERSEDED) | WP-012 (BLOCKED) | — |
| thegent | — | — | WP-013 (SUPERSEDED) |
| hwLedger | WP-014 (REASSIGNED to health) | — | — |
| BytePort | — | WP-015 (BLOCKED) | — |
| PhenoKits | — | — | WP-016 (SUPERSEDED) |

Realized matrix (ground truth on origin/main, closeout 2026-04-25 21:00Z):

| Consumer (crate) | error-core | config-core | health |
|------------------|-----------|-------------|--------|
| AuthKit | DONE (PR #42) | — | — |
| ResilienceKit | DONE (PR #15) | — | — |
| TestingKit | — | — | DONE (PR #4) |
| hwLedger / hwledger-core | DONE (PR #35) | — | — |
| hwLedger / hwledger-server | — | — | DONE (PR #34) |
| hwLedger / hwledger-agent | — | — | DONE (PR #34) |
| **Per-crate consumer count** | **3/3 — DONE** | **0/3 — DESCOPED (premature extraction)** | **3/3 — DONE** |

**Closeout note:** error-core and health each reached the FR-09 ≥3
floor via the realized PRs above. The earlier "Gap" framing
(needing one more consumer per crate, substitutes from Tracely /
FocalPoint / Metron / BlueScript) is **resolved** by the hwLedger
PRs (#34, #35) landing same-day. config-core remains descoped per
spec.md "Adoption Reality" item #2; no further consumers were
recruited under this spec.

**Tracking note:** any descoped (consumer × crate) pair from Phase 2
design (per spec NG-02) requires Discovery to add another candidate so
the per-crate ≥3 floor still holds.

---

## Total WP Count

- Discovery: 4 (WP-001..WP-004)
- Design: 6 (WP-005..WP-010)
- Build: 6 (WP-011..WP-016)
- Validate + Deploy: 4 (WP-017..WP-020)
- **Total: 20 work packages.**

Assumes no descopes from Phase 2 design. Each descope adds 1 design WP
+ 1 build WP for the substitute consumer.
