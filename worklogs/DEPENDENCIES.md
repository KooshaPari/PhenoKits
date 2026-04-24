# DEPENDENCIES Worklog
**Category: DEPENDENCIES**

External deps, forks, modernization, Dependabot coverage.

## 2026-04-24 — Dependabot config coverage audit

**Scope:** All 64 active (non-archived, non-fork) repos in `KooshaPari/`.
**Method:** For each repo, checked (a) presence of `.github/dependabot.yml`, (b) declared `package-ecosystem` entries, (c) actual manifest files in HEAD tree, (d) Dependabot alerts endpoint status. See `/tmp/audit_one.sh` and `/tmp/classify.py` for reproduction.

### Summary counts

| Classification | Count | Meaning |
|---|---|---|
| COMPLETE | 12 | `dependabot.yml` declares all actual ecosystems |
| PARTIAL | 21 | `dependabot.yml` exists but misses 1+ actual ecosystems |
| MISSING_CONFIG | 29 | No `dependabot.yml`; alerts endpoint reachable (bootstrap needed) |
| NO_COVERAGE | 0 | No `dependabot.yml` AND alerts disabled (403) |
| EMPTY | 2 | No tracked manifest files (skip) |
| **Total** | **64** | |

Alerts infrastructure is live on every non-empty repo (no 403s observed). The gap is purely config files.

### Full coverage table

| Repo | Classification | Declared ecosystems | Actual ecosystems | Missing |
|---|---|---|---|---|
| AgilePlus | PARTIAL | cargo,github-actions,pip | cargo,github-actions,gomod,npm,pip | gomod,npm |
| Apisync | PARTIAL | cargo,gomod,npm,pip | cargo,github-actions | github-actions |
| AuthKit | COMPLETE | cargo,github-actions,gomod,npm,pip | cargo,github-actions,npm,pip | - |
| Benchora | MISSING_CONFIG | - | cargo,github-actions | cargo,github-actions |
| BytePort | MISSING_CONFIG | - | cargo,github-actions,gomod,npm | cargo,github-actions,gomod,npm |
| Civis | PARTIAL | cargo,github-actions,npm | cargo,github-actions,npm,pip | pip |
| Configra | PARTIAL | github-actions,npm | cargo,github-actions,pip | cargo,pip |
| DataKit | COMPLETE | cargo,npm,pip | cargo,pip | - |
| DevHex | MISSING_CONFIG | - | gomod | gomod |
| Dino | PARTIAL | github-actions,nuget | cargo,github-actions,gomod,npm,pip | cargo,gomod,npm,pip |
| GDK | COMPLETE | cargo,github-actions | cargo,github-actions | - |
| HexaKit | PARTIAL | cargo,github-actions,pip | cargo,github-actions,gomod,npm,pip | gomod,npm |
| Httpora | MISSING_CONFIG | - | github-actions,pip | github-actions,pip |
| KDesktopVirt | MISSING_CONFIG | - | cargo,github-actions,gomod | cargo,github-actions,gomod |
| McpKit | COMPLETE | cargo,github-actions,gomod,npm,pip | cargo,github-actions,gomod,npm,pip | - |
| Metron | MISSING_CONFIG | - | cargo,github-actions | cargo,github-actions |
| ObservabilityKit | MISSING_CONFIG | - | cargo,pip | cargo,pip |
| Parpoura | PARTIAL | github-actions,npm | github-actions,npm,pip | pip |
| PhenoCompose | MISSING_CONFIG | - | cargo,gomod,npm | cargo,gomod,npm |
| PhenoDevOps | COMPLETE | cargo,gomod,npm | cargo,npm | - |
| PhenoHandbook | COMPLETE | github-actions,npm | github-actions,npm | - |
| PhenoKits | MISSING_CONFIG | - | cargo,github-actions,gomod,npm,pip | cargo,github-actions,gomod,npm,pip |
| PhenoLang | PARTIAL | cargo,github-actions,pip | cargo,github-actions,gomod,npm,pip | gomod,npm |
| PhenoMCP | COMPLETE | cargo,github-actions,gomod,npm,pip | cargo,github-actions,gomod,npm,pip | - |
| PhenoObservability | MISSING_CONFIG | - | cargo,github-actions,pip | cargo,github-actions,pip |
| PhenoPlugins | MISSING_CONFIG | - | cargo,github-actions | cargo,github-actions |
| PhenoProc | PARTIAL | cargo | cargo,github-actions,gomod,npm,pip | github-actions,gomod,npm,pip |
| PhenoProject | MISSING_CONFIG | - | npm,pip | npm,pip |
| PhenoRuntime | MISSING_CONFIG | - | cargo,github-actions,gomod,npm,pip | cargo,github-actions,gomod,npm,pip |
| PhenoSpecs | PARTIAL | github-actions | github-actions,npm | npm |
| PhenoVCS | MISSING_CONFIG | - | cargo,github-actions | cargo,github-actions |
| PlayCua | PARTIAL | cargo,github-actions | cargo,github-actions,pip | pip |
| QuadSGM | MISSING_CONFIG | - | github-actions,npm,pip | github-actions,npm,pip |
| ResilienceKit | COMPLETE | cargo,pip | cargo,pip | - |
| Stashly | PARTIAL | cargo,gomod,npm,pip | cargo,github-actions | github-actions |
| Tasken | PARTIAL | cargo,gomod,npm,pip | cargo,github-actions,npm,pip | github-actions |
| TestingKit | MISSING_CONFIG | - | cargo,pip | cargo,pip |
| Tokn | PARTIAL | cargo,gomod,npm,pip | cargo,github-actions,npm | github-actions |
| Tracera | COMPLETE | docker,github-actions,gomod,npm,pip | github-actions,npm,pip | - |
| agent-devops-setups | MISSING_CONFIG | - | github-actions,npm | github-actions,npm |
| agent-user-status | COMPLETE | github-actions,pip | github-actions,pip | - |
| argis-extensions | PARTIAL | cargo,gomod,npm,pip | github-actions,gomod,pip | github-actions |
| dinoforge-packs | EMPTY | - | - | - |
| foqos-private | MISSING_CONFIG | - | github-actions | github-actions |
| helios-router | MISSING_CONFIG | - | cargo,github-actions,npm | cargo,github-actions,npm |
| heliosApp | COMPLETE | github-actions,npm | github-actions,npm | - |
| heliosBench | MISSING_CONFIG | - | pip | pip |
| hwLedger | MISSING_CONFIG | - | cargo,github-actions,npm,pip | cargo,github-actions,npm,pip |
| nanovms | PARTIAL | cargo,gomod,npm,pip | github-actions,gomod,npm | github-actions |
| pheno | PARTIAL | cargo,github-actions,pip | cargo,github-actions,gomod,npm,pip | gomod,npm |
| phenoAI | MISSING_CONFIG | - | cargo | cargo |
| phenoData | MISSING_CONFIG | - | cargo | cargo |
| phenoDesign | COMPLETE | github-actions,npm | github-actions,npm | - |
| phenoResearchEngine | PARTIAL | cargo,gomod,npm,pip | github-actions,npm,pip | github-actions |
| phenoShared | PARTIAL | cargo,gomod,npm,pip | cargo,github-actions,npm | github-actions |
| phenoUtils | MISSING_CONFIG | - | cargo | cargo |
| phenoXdd | PARTIAL | cargo,npm | github-actions | github-actions |
| phenodocs | PARTIAL | github-actions,npm | github-actions,gomod,npm,pip | gomod,pip |
| phenotype-auth-ts | MISSING_CONFIG | - | npm | npm |
| phenotype-hub | EMPTY | - | - | - |
| phenotype-journeys | MISSING_CONFIG | - | cargo,github-actions,npm | cargo,github-actions,npm |
| phenotype-registry | MISSING_CONFIG | - | github-actions | github-actions |
| thegent | MISSING_CONFIG | - | cargo,github-actions,gomod,npm,pip | cargo,github-actions,gomod,npm,pip |
| vibeproxy-monitoring-unified | MISSING_CONFIG | - | github-actions | github-actions |

### Top-10 bootstrap targets (MISSING_CONFIG, multi-ecosystem, active repos)

Ranked by ecosystem breadth × recency (all pushed within last 24h):

| Rank | Repo | Missing ecosystems | Notes |
|---|---|---|---|
| 1 | thegent | cargo, github-actions, gomod, npm, pip | Core dotfiles platform; 5-ecosystem polyglot; highest ROI |
| 2 | PhenoKits | cargo, github-actions, gomod, npm, pip | Go-primary polyglot kit; full stack |
| 3 | PhenoRuntime | cargo, github-actions, gomod, npm, pip | Runtime; full polyglot stack |
| 4 | BytePort | cargo, github-actions, gomod, npm | Go/Rust/TS; no pip |
| 5 | hwLedger | cargo, github-actions, npm, pip | 4-ecosystem; Rust+TS+Python |
| 6 | KDesktopVirt | cargo, github-actions, gomod | Active rebuild (eco-011); Rust+Go |
| 7 | PhenoObservability | cargo, github-actions, pip | Rust+Python observability |
| 8 | PhenoCompose | cargo, gomod, npm | No GHA; Rust+Go+TS |
| 9 | QuadSGM | github-actions, npm, pip | HTML+JS+Python |
| 10 | helios-router | cargo, github-actions, npm | Router; Rust+TS |

**Also notable (3-way MISSING_CONFIG):** phenotype-journeys (cargo, GHA, npm), PhenoProject (npm, pip).

### Surprising finds

1. **thegent has NO `dependabot.yml`** despite being the canonical dotfiles platform that *provides* Dependabot templates to downstream repos. This is the single highest-priority fix.
2. **Many PARTIAL configs declare ecosystems that don't exist** (e.g., Stashly, Tasken, Tokn declare cargo/gomod/npm/pip but actually only use cargo+GHA; nanovms, phenoShared, argis-extensions, phenoResearchEngine declare broadly but miss github-actions, the one ecosystem every repo has via workflows). Suggests these repos were scaffolded from a generic template before manifests were added — inverse drift from the usual pattern.
3. **Dino declares `nuget`** but has no `.csproj` in tree-scan; primary language is C# per GitHub metadata — manual check warranted.
4. **github-actions is the single most-missed ecosystem** across PARTIAL repos (8 repos miss it: Apisync, Stashly, Tasken, Tokn, argis-extensions, nanovms, phenoResearchEngine, phenoShared, phenoXdd). Low-effort fix.
5. **Tracera declares `docker`** (positive outlier — only repo covering container image updates).
6. **Alerts are enabled org-wide** — no `403` (disabled) responses observed on any non-empty repo. The gap is 100% config-surface, not account-level disablement.

### Proposed default `dependabot.yml` template (per tier)

**Tier-1 (active polyglot services: thegent, PhenoKits, PhenoRuntime, PhenoMCP-class):**

```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule: { interval: "weekly", day: "monday" }
    open-pull-requests-limit: 10
    groups:
      rust-deps: { patterns: ["*"] }
  - package-ecosystem: "gomod"
    directory: "/"
    schedule: { interval: "weekly", day: "monday" }
    open-pull-requests-limit: 10
    groups:
      go-deps: { patterns: ["*"] }
  - package-ecosystem: "npm"
    directory: "/"
    schedule: { interval: "weekly", day: "monday" }
    open-pull-requests-limit: 10
    groups:
      js-deps: { patterns: ["*"] }
  - package-ecosystem: "pip"
    directory: "/"
    schedule: { interval: "weekly", day: "monday" }
    open-pull-requests-limit: 10
    groups:
      py-deps: { patterns: ["*"] }
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule: { interval: "weekly", day: "monday" }
    open-pull-requests-limit: 5
    groups:
      actions: { patterns: ["*"] }
```

**Tier-2 (focused 1-2 ecosystem repos):** strip to only the ecosystems present in-tree. Always include `github-actions` if `.github/workflows/` exists.

**Tier-3 (archive / empty repos: dinoforge-packs, phenotype-hub):** skip; no manifests.

### Recommended rollout

1. Land Tier-1 template on top-10 bootstrap targets (29 `MISSING_CONFIG` total; start with the 10 above).
2. Patch 21 `PARTIAL` configs — the github-actions-missing cluster (8 repos) is a trivial mechanical add.
3. Reconcile over-declared PARTIAL repos (Stashly, Tasken, Tokn, argis-extensions, nanovms, phenoResearchEngine, phenoShared) — either remove declarations for absent ecosystems or add the missing manifests if they're expected.
4. Re-audit monthly until COMPLETE count == 62 (all non-empty repos).

Tags: `[cross-repo]` `[security]` `[dependabot]`
