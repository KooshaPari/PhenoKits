# Phenotype Local Drift Manifest - 2026-04-25

## Summary

This manifest classifies the first P0 dirty-repo tranche after the org PR cleanup.
No repo state was changed while collecting this evidence.

Primary finding: none of these repos should be pushed or reset as-is. The large
states are mixed wave artifacts and need split/recovery branches before cleanup.

## Priority Decisions

| Repo | Decision | Reason | Next action |
|---|---|---|---|
| `agentapi-plusplus` | Split | 4,681 tracked changes, nearly all vendor deletions, mixed with 7 local commits of docs/import/test hygiene | Create separate salvage branches for vendor cleanup and governance/docs/test changes. |
| `phenoSDK` | Split | 2,189 tracked changes, 11 untracked docs, generated-doc deletion plus decomposition/governance commits | Separate deprecation/governance docs from generated artifact cleanup and module removal. |
| `PhenoProc` | Split | ahead 8 / behind 12, 38 tracked changes, 68 untracked paths, 24 dirty gitlinks | Freeze current tree; make a nested-gitlink manifest before rebasing or pushing. |
| `AuthKit` | Keep, then rebase carefully | coherent docs/governance bootstrap lane, but behind 37 and one dirty `go` gitlink | Preserve local commits; inspect `go` gitlink; rebase in a clean worktree. |
| `Dino` | Split | ahead 8 / behind 62, docs/spec scaffold plus SDK/test seed | Split governance/docs from SDK/test work, then rebase from current main. |
| `PhenoSpecs` | Split | archive consolidation and registry/spec hygiene in one dirty tree | Split archive deletions from registry/SPEC/worklog updates. |
| shelf root `repos` | Split/quarantine | root checkout sees child repos/worktrees as untracked spillover; 15 tracked doc/spec deletions | Do not use root status as product truth; isolate governance doc cleanup only. |

## Repo Manifests

### `agentapi-plusplus`

- Path: `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-plusplus`
- Branch: `chore/infrastructure-push`
- Tracking: `origin/chore/infrastructure-push`
- Divergence: ahead 7 / behind 0
- Dirty counts: 4,681 tracked changes, 0 untracked
- Change shape:
  - `vendor/github.com`: 3,336
  - `vendor/golang.org`: 771
  - `vendor/honnef.co`: 246
  - `vendor/google.golang.org`: 146
  - `vendor/go.uber.org`: 80
  - `vendor/mvdan.cc`: 28
  - `.worktrees`: 2
- Recent local commits:
  - `0c5f958 docs(fr): update coverage matrix - 19 FRs traced to tests`
  - `0905219 test(fr): annotate top 15 FR test traces`
  - `05551ad fix(imports): resolve double-prefixed github.com imports in Go modules`
  - `1289560 docs(readme): hygiene round-5 - agentapi-plusplus`
  - `466104a docs(agents): harmonize AGENTS.md to thin pointer`
  - `3d310b1 chore(governance): adopt standard CLAUDE.md + AGENTS.md + worklog`
  - `75853df docs(fr): scaffold FUNCTIONAL_REQUIREMENTS.md with 3 FR stubs`
- Category: vendor cleanup plus docs/governance/import/test hygiene
- Disposition: split
- Next action:
  1. Record vendor deletion intent before touching the tree.
  2. If vendor deletion is desired, make it its own branch from current main.
  3. Replay docs/governance/test/import commits separately.
  4. Treat `.worktrees/*` dirt as operational state, not product code.

### `phenoSDK`

- Path: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenoSDK`
- Branch: `main`
- Tracking: `origin/main`
- Divergence: ahead 11 / behind 0
- Dirty counts: 2,189 tracked changes, 11 untracked
- Change shape:
  - `docs/api`: 1,060
  - `examples`: 68
  - `packages`: 61
  - `scripts/categories`: 50
  - `tools/quality`: 34
  - `.cicd-backups`: 30
  - `docs/kits`: 30
  - `tests/unit`: 30
  - `scripts/maintenance`: 28
  - `scripts/archive`: 26
  - `quality`: 20
- Untracked:
  - `CHARTER.md`
  - `README-ARCHIVED.md`
  - `docs/FUNCTIONAL_REQUIREMENTS.md`
  - `docs/adr/*`
  - `docs/research/*`
  - `worklog.md`
- Recent local commits:
  - `1536c55 docs(worklog): bootstrap worklog scaffolding (org-wide gap closure)`
  - `e189ade docs(agents): harmonize AGENTS.md to thin pointer`
  - `dc84453 docs(deprecation): add DEPRECATION.md directing to AuthKit consolidation`
  - `6c17ab5 docs(readme): expand (wave-2 hygiene)`
  - `88c24b3 docs(changelog): seed retroactive CHANGELOG from git history via git-cliff`
  - `41a97a0 chore(governance): adopt CLAUDE.md + governance framework`
  - `2575591 chore(ci): adopt phenotype-tooling workflows (wave-3)`
  - `c523501 feat: complete phenoSDK decomposition - move final core files`
  - `295b6b5 feat: remove all 48 extracted modules from phenoSDK`
- Category: deprecation/governance plus decomposition and generated artifact cleanup
- Disposition: split
- Next action:
  1. Preserve deprecation/governance docs as one branch.
  2. Review generated docs and backups for deletion separately.
  3. Replay module-removal commits only after AuthKit consolidation target is verified.

### `PhenoProc`

- Path: `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoProc`
- Branch: `main`
- Tracking: `origin/main`
- Divergence: ahead 8 / behind 12
- Dirty counts: 38 tracked changes, 68 untracked paths
- Gitlink dirt: 24 dirty gitlinks, 23 modified and 1 deleted (`crates/tokn`)
- Largest clusters:
  - `crates/`: 34 dirty entries
  - `phenotype-governance/`: 12
  - root package/doc trees under `application/`, `auth/`, `bus/`, `cache/`,
    `config/`, `docs/`, `domain/`, `infrastructure/`, `pheno-*`,
    `phenotype-*`, `registry/`, and `validation/`
- Recent local commits:
  - `d31a6c9 docs(readme): expand [wave-3 hygiene]`
  - `dc66adc docs(readme): expand [wave-3 hygiene]`
  - `36a0d06 docs(readme): expand [wave-3 hygiene]`
  - `82e9955 docs(changelog): seed retroactive CHANGELOG from git history via git-cliff`
  - `4426f60 ci(wave-6): add quality-gate and fr-coverage workflows`
  - `cbf130b chore(deps): align tokio + serde to org baseline (phenotype-versions.toml)`
  - `f85d98e docs(readme): expand README.md with purpose, stack, quick-start, related projects`
  - `f94e62a chore(ci): adopt phenotype-tooling workflows (wave-2)`
- Category: broad cross-repo bootstrap and nested crate refresh
- Disposition: split
- Next action:
  1. Create a nested gitlink manifest before any rebase.
  2. Decide whether deleted `crates/tokn` is intentional.
  3. Split docs/CI wave from crate dependency alignment.

### `AuthKit`

- Path: `/Users/kooshapari/CodeProjects/Phenotype/repos/AuthKit`
- Branch: `main`
- Tracking: `origin/main`
- Divergence: ahead 9 / behind 37
- Dirty counts: 4 tracked changes, 12 untracked paths
- Gitlink dirt: one dirty gitlink, `go`
- Largest clusters:
  - `docs/`: 8 untracked paths plus `docs/worklogs/README.md`
  - root docs: `ADR.md`, `CHARTER.md`, `PLAN.md`, `PRD.md`, `worklog.md`
  - manifests: `pyproject.toml`, `rust/Cargo.toml`
- Recent local commits:
  - `cb21b34 docs(agents): harmonize AGENTS.md to thin pointer`
  - `5e6b6e0 docs(related-projects): clarify phenoSDK deprecation and consolidation`
  - `c14c2be docs(changelog): seed retroactive CHANGELOG from git history via git-cliff`
  - `25f166d docs(readme): expand README.md with purpose, stack, quick-start, related projects`
  - `fc48afa docs(wave-4): scaffold FUNCTIONAL_REQUIREMENTS.md with 6 stubs`
  - `33686fd chore(ci): adopt phenotype-tooling workflows (wave-2)`
  - `3c20d13 test(smoke): seed minimal smoke test - proves harness works`
  - `e22b243 chore(governance): adopt standard CLAUDE.md + AGENTS.md + worklog`
  - `dca4478 docs(fr): scaffold FUNCTIONAL_REQUIREMENTS.md with 1 FR stubs`
- Category: coherent docs/governance bootstrap and scaffold alignment
- Disposition: keep, then rebase carefully
- Next action:
  1. Inspect `go` gitlink before rebase.
  2. Rebase in a clean worktree because the branch is 37 behind.
  3. Keep the docs/governance lane together unless conflicts force splitting.

### `Dino`

- Path: `/Users/kooshapari/CodeProjects/Phenotype/repos/Dino`
- Branch: `main`
- Tracking: `origin/main`
- Divergence: ahead 8 / behind 62
- Dirty counts: 2 tracked changes, 15 untracked
- Largest clusters:
  - root docs: 5
  - `docs/adr`: 4
  - `src/SDK`: 3
  - `docs/research`: 2
- Recent local commits:
  - `05cd0168 docs(agents): harmonize AGENTS.md to thin pointer`
  - `db105a7a docs(fr): scaffold FUNCTIONAL_REQUIREMENTS.md`
  - `b877e80c test(wave-4): seed minimal smoke test`
  - `7ffaad76 chore(ci): adopt phenotype-tooling workflows (wave-2)`
  - `8361efb4 chore(governance): adopt standard CLAUDE.md + AGENTS.md + worklog`
  - `c6b5c35c docs(fr): scaffold FUNCTIONAL_REQUIREMENTS.md with 7 FR stubs`
  - `6dab9554 ci(legacy-enforcement): add legacy tooling anti-pattern gate (WARN mode)`
  - `8146ebf4 docs: add README/SPEC/PLAN`
- Category: docs/spec scaffold plus SDK/test seed
- Disposition: split
- Next action:
  1. Split docs/governance from SDK/test work.
  2. Rebase from current main in a fresh branch due to 62-behind drift.
  3. Keep SDK integration files out of docs-only governance PRs.

### `PhenoSpecs`

- Path: `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoSpecs`
- Branch: `main`
- Tracking: `origin/main`
- Divergence: ahead 4 / behind 0
- Dirty counts: 60 tracked changes, 10 untracked
- Largest clusters:
  - `specs/archive`: 56
  - root docs/manifests: 7
  - `specs/platform`: 2
- Recent local commits:
  - `b364b6c docs(agents): harmonize AGENTS.md to thin pointer`
  - `b4d6e92 docs(readme): expand [wave-3 hygiene]`
  - `9eacba1 docs(changelog): seed retroactive CHANGELOG from git history via git-cliff`
  - `503ec96 chore(governance): adopt standard CLAUDE.md + AGENTS.md + worklog (wave-2)`
- Category: archive consolidation and spec/registry hygiene
- Disposition: split
- Next action:
  1. Separate `specs/archive` deletions from `SPEC.md` and `registry.yaml` updates.
  2. Verify archive deletions are represented in the canonical spec index before accepting them.
  3. Publish docs/worklog bootstrap separately if needed.

### Shelf root `repos`

- Path: `/Users/kooshapari/CodeProjects/Phenotype/repos`
- Branch: `adr/canonical-home-verification-2026-04-25`
- Tracking: `origin/adr/canonical-home-verification-2026-04-25`
- Divergence: ahead 0 / behind 0 at latest read
- Dirty counts: 15 tracked changes, large untracked spillover
- Tracked clusters:
  - `docs/governance`: 6
  - `docs/org-audit-2026-04`: 4
  - `kitty-specs/forced-adoption-phase1-shared-crates`: 3
  - `worklogs`: 3
  - `Cargo.toml`: 1
- Untracked clusters are mostly child repos and worktrees, not one product delta.
- Category: shelf/workspace drift with governance doc cleanup and sibling repo/worktree spillover
- Disposition: split/quarantine
- Next action:
  1. Treat shelf root as coordination state, not project truth.
  2. Isolate tracked governance/doc deletions from untracked child repo noise.
  3. Add or confirm ignore policy for child repo/worktree directories only after verifying this checkout is intended to track them as untracked.

## Next Execution Order

1. `AuthKit`: lowest-risk keep lane; preserve and rebase carefully after `go` gitlink check.
2. `PhenoSpecs`: split archive deletion from registry/spec updates.
3. `Dino`: split docs/governance from SDK/test seed.
4. `PhenoProc`: produce nested gitlink manifest before any rebase.
5. `agentapi-plusplus`: split vendor deletion from docs/test/import fixes.
6. `phenoSDK`: split deprecation/governance from generated artifact cleanup and module removals.
7. Shelf root: quarantine tracked governance cleanup from child-repo untracked spillover.

## Non-Negotiables

- Do not push or reset any listed repo as-is.
- Do not delete branch refs until stale-branch ledger captures branch tip and containment.
- Do not treat shelf-root untracked child repos as product files.
- Preserve worklogs and unique governance guidance before accepting deletions.
