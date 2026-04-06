# phenotypeActions SPEC

**Type**: Shared GitHub Actions repository
**Language**: YAML (composite actions + reusable workflows)
**Consumers**: All Phenotype repositories

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                   phenotypeActions                        │
│                                                          │
│  ┌───────────┐ ┌───────────┐ ┌────────────┐ ┌─────────┐ │
│  │  review-  │ │  lint-    │ │  policy-   │ │template-│ │
│  │orchestrator│ │  test     │ │   gate     │ │  sync   │ │
│  │           │ │           │ │            │ │         │ │
│  │ Wave-based│ │ Stack     │ │ Merge/NS/  │ │ Drift   │ │
│  │ bot review│ │ auto-     │ │ branch     │ │ detect  │ │
│  │ triggering│ │ detect    │ │ policy     │ │ + PR    │ │
│  └─────┬─────┘ └─────┬─────┘ └──────┬─────┘ └────┬────┘ │
│        │              │              │             │      │
│  ┌─────▼──────────────▼──────────────▼─────────────▼────┐ │
│  │            .github/workflows/ (15 workflows)         │ │
│  │  review-wave-orchestrator, ci, policy-gate,          │ │
│  │  quality-gate, security, release, template-sync, ... │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                          │
│  ┌───────────┐ ┌───────────┐ ┌─────────────────────────┐ │
│  │ schemas/  │ │ contracts/│ │ docs/                   │ │
│  │ JSON      │ │ Interface │ │ Migration audits,       │ │
│  │ schemas   │ │ specs     │ │ stage-gates artifacts   │ │
│  └───────────┘ └───────────┘ └─────────────────────────┘ │
└──────────────────────────────────────────────────────────┘

Consumer repo:
  jobs:
    review-waves:
      uses: KooshaPari/phenotypeActions/.github/workflows/review-wave-orchestrator.yml@v0
```

## Components

### Composite Actions (`actions/`)

| Action | Inputs | Purpose |
|--------|--------|---------|
| `review-orchestrator` | `pr_number`, `bots`, `wave`, `cooldown_minutes`, `retry_budget` | Trigger review bots with cooldown markers and bounded retries |
| `lint-test` | `working-directory`, `skip-tests`, `bun-version` | Auto-detect stack (Bun/Biome/Go/Rust) and run lint + tests |
| `policy-gate` | `base_branch`, `block_merge_commits`, `require_layered_fix`, `allowed_namespace`, `matrix_type` | Enforce merge-commit, namespace, layered-fix policies; generate CI matrices |
| `template-sync` | `template_repo`, `target_repos`, `dry_run`, `github_token` | Detect governance drift across repos, open sync PRs |

### Reusable Workflows (`.github/workflows/`)

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `review-wave-orchestrator.yml` | `pull_request`, `workflow_call` | Orchestrate wave1/wave2 bot review |
| `ci.yml` | Push/PR | Standard CI pipeline |
| `policy-gate.yml` | PR | Policy enforcement |
| `quality-gate.yml` | PR | Quality checks |
| `security.yml` | Schedule/PR | Security scanning |
| `template-sync.yml` | Schedule | Cross-repo governance sync |
| `release.yml` / `release-drafter.yml` | Tags/push | Release automation |

## Data Models

### Review Orchestrator State

```
BotTrigger {
  bot:               String       # e.g. "coderabbitai"
  wave:              String       # "wave1" | "wave2"
  cooldown_minutes:  Number       # Default: 15
  retry_budget:      Number       # Default: 2
  marker:            String       # "bot-review-trigger: {bot} {timestamp} {wave}"
}
```

State is stored as PR comments with marker prefixes. No external database.

### Policy Gate Outputs

```
PolicyResult {
  passed:            bool
  bazel_matrix:      JSON  # CI matrix for Bazel builds
  lint_build_matrix: JSON  # Rust lint/build matrix
  tests_matrix:      JSON  # Rust test matrix
  build_matrix:      JSON  # Rust release build matrix
}
```

### Template Sync Drift Report

```
DriftReport {
  repo:                    String
  run_outcome:             String   # in_sync | governance_drift | workflow_drift | ...
  remediation_category:    String
  governance_drift:        bool
  workflow_drift:          bool
  stage_gates_drift:       bool
  strictness_drift:        bool
  canary_ready:            bool
}
```

Output formats: CSV matrix, Markdown summary, JSON decision.

## Consumption Pattern

```yaml
# In a consumer repo
jobs:
  lint:
    uses: KooshaPari/phenotypeActions/.github/workflows/review-wave-orchestrator.yml@v0
    with:
      wave1_bots: "coderabbitai,gemini-code-assist"
      wave2_bots: "augment,codex"

  policy:
    uses: KooshaPari/phenotypeActions/actions/policy-gate@main
    with:
      block_merge_commits: "true"
      matrix_type: "rust-ci"
```

Pin to release tag (`@v0`) or commit SHA in production.

## Stack Detection (lint-test)

```
bun.lock/bunfig.toml  →  Bun + Biome
go.mod                →  Go (go vet + go test)
Cargo.toml            →  Rust (cargo clippy + cargo test)
package.json          →  Node/npm
```

## Quality Gates

- All composite actions must pass shellcheck
- Reusable workflows must validate with `act` or GitHub's workflow linter
- Template sync must handle unreachable repos gracefully
- Policy gate must fail-closed (deny on error)
