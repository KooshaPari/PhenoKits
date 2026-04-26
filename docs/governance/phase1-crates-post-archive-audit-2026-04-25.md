# Phase 1 Shared Crates — Post-Archive Audit (2026-04-25)

**Scope:** Determine where Phase 1 shared crates (`phenotype-error-core`, `phenotype-config-core`, `phenotype-health`) live now that `phenotype-infrakit` is archived.
**Mode:** Read-only, GitHub API only. No clones, commits, or PRs.

## Headline Findings

1. **PR #87 is CLOSED, NOT MERGED** in `phenotype-infrakit`
   (`feat/phase1-loc-reduction-launch` -> `main`, `merged: false`, `merge_commit_sha: null`).
   The MEMORY.md note "PR #87: open for review" is stale; the work was never merged via that PR. The crates DO exist on `main` of `phenotype-infrakit` (so they were merged via a different commit/PR).
2. **All three crates are NOT marooned.** Real source exists on the `main` branch of multiple non-archived repos (`pheno`, `HexaKit`, `PhenoLang`, `PhenoProc`). The archived copy in `phenotype-infrakit` is still readable.
3. **`phenoShared` (the canonical-home ADR target) does NOT contain any of the three crates.** Its `crates/` only holds `phenotype-{cache-adapter, event-sourcing, policy-engine, state-machine, port-interfaces, *-adapter, application, domain, nanovms-client}`.
4. The most up-to-date copy is in **`KooshaPari/pheno`** (most recent push 2026-04-25; health crate has an extra `project.rs` not present in infrakit).

## Per-Crate Status Table

| Crate | Status | Primary Source (canonical-quality) | Secondary Copies | Archived Copy | LOC (src) |
|---|---|---|---|---|---|
| `phenotype-error-core` | **Live (multi-homed)** | `KooshaPari/pheno:crates/phenotype-error-core/` (lib.rs 13,632 B) | HexaKit, PhenoLang, PhenoProc | phenotype-infrakit (lib.rs 13,632 B, identical) | ~13.6 KB |
| `phenotype-config-core` | **Live (multi-homed, drift)** | `KooshaPari/pheno:crates/phenotype-config-core/` (lib.rs 7,499 B) | HexaKit, PhenoLang, PhenoProc | phenotype-infrakit (lib.rs 7,438 B — older by 61 B) | ~7.5 KB |
| `phenotype-health` | **Live (multi-homed, drift)** | `KooshaPari/pheno:crates/phenotype-health/` (lib.rs 5,336 + checkers 4,075 + project 7,735 + tests 4,881) | HexaKit, PhenoLang, PhenoProc, PhenoObservability (`rust/phenotype-health/`) | phenotype-infrakit (lib.rs 5,183 + checkers + tests; **no project.rs**) | ~22 KB in pheno; ~14 KB in infrakit |

Verdict: **Migrated (de facto), not Marooned.** The crates were sprawl-replicated into multiple repos before the archive — there is no single "canonical home" yet. Recovery from infrakit is unnecessary; the live copies are equal or newer.

## Consumer Inventory (Cargo.toml dependency declarations)

`phenotype-error-core`:
- `pheno` (workspace + agileplus-error-core, agileplus-p2p, agileplus-cache, agileplus-sync, agileplus-nats, phenotype-contracts, phenotype-policy-engine)
- `PhenoProc` (workspace + phenotype-errors)
- `PhenoLang` (workspace + agileplus-domain, phenotype-policy-engine)
- `HexaKit` (workspace)
- `AuthKit` (`rust/phenotype-authz-engine`)
- `ResilienceKit` (`rust/phenotype-state-machine`)

`phenotype-health`:
- `pheno` (workspace + agileplus-nats)
- `HexaKit` (workspace)
- `PhenoLang` (workspace + agileplus-nats)
- `PhenoProc` (workspace + phenotype-project-registry, phenotype-compliance-scanner, phenotype-health-cli, phenotype-health-axum, phenotype-security-aggregator)
- `PhenoObservability` (`rust/phenotype-health-cli`, `rust/phenotype-health-axum`)
- `ObservabilityKit` (`rust/phenotype-health-cli`, `rust/phenotype-health-axum`)
- `TestingKit` (`rust/phenotype-compliance-scanner`)
- `AuthKit` (`rust/phenotype-security-aggregator`)

`phenotype-config-core`:
- `pheno` (workspace), `HexaKit` (workspace), `PhenoLang` (audit only), `PhenoProc` (workspace).
  Far fewer concrete consumers than the other two — adoption is light.

## Recommendations

The work to do is NOT "rescue from archive." It is **collapse the sprawl into one canonical home (`phenoShared`)** and re-target the forced-adoption spec there.

### Recommended next-step PR (read-only audit only — not executed here)

PR target: `KooshaPari/phenoShared` — branch `feat/phase1-canonical-import-from-pheno`.

Steps for the implementer:
1. Copy `crates/phenotype-error-core/`, `crates/phenotype-config-core/`, `crates/phenotype-health/` from **`pheno@main`** (newest, has `project.rs` for health) into `phenoShared/crates/`.
2. Add to `phenoShared/Cargo.toml` workspace members.
3. Run `cargo build -p phenotype-error-core -p phenotype-config-core -p phenotype-health` in phenoShared.
4. Open PR; in body, link this audit and call out drift between pheno (newer) and infrakit (frozen).

### Follow-on: re-target adoption spec
- Update **forced-adoption spec #32** and **spec 013** to point at `phenoShared` once #1 lands.
- Open de-dup PRs against `pheno`, `HexaKit`, `PhenoLang`, `PhenoProc` to switch their workspace deps from path/local copies to `phenoShared` (Git or path dep) and delete the in-repo duplicates. This is forward-only migration per the Phenotype Org Cross-Project Reuse Protocol.

### Do NOT
- Do NOT migrate from the archived `phenotype-infrakit` copy — it is older than `pheno`'s copy (config-core differs by 61 B; health is missing `project.rs`).
- Do NOT delete the archived copy as a recovery step — archived repos are still readable; the risk is zero and the audit trail is useful.

## Evidence trail

- `gh api repos/KooshaPari/phenotype-infrakit --jq '.archived'` => `true`, last push `2026-04-03T11:45:43Z`.
- `gh api repos/KooshaPari/phenotype-infrakit/pulls/87 --jq '.merged'` => `false`, state `closed`.
- `gh api .../phenotype-infrakit/contents/crates/phenotype-{error-core,config-core,health}/src` => content present.
- `gh api .../pheno/contents/crates/phenotype-{error-core,config-core,health}/src` => content present and >= infrakit sizes.
- `gh api .../phenoShared/contents/crates --jq '.[].name'` => no Phase 1 crates listed.
- Consumer searches via `gh search code 'phenotype-{error-core,health,config-core} =' --owner KooshaPari` => 8+ repos depend on each.
