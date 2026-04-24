# Org Health Automation

Automated monitoring and reporting of org-wide metrics across 59+ repositories.

## Automation Schedule

| Job | Frequency | Trigger | Output |
|-----|-----------|---------|--------|
| **Worklog Index** | Weekly | Monday 9am Eastern | `docs/org-audit-YYYY-MM/worklog_index.md` |
| **Org Audit Index** | Weekly | Monday 9am Eastern | `docs/org-audit-YYYY-MM/INDEX.md` |
| **LOC Drift Report** | Monthly | 1st of month, 10am Eastern | `docs/org-audit-YYYY-MM/loc_drift_YYYY_MM.md` |

### GitHub Actions Workflows

- `.github/workflows/org-audit-weekly.yml` — Runs worklog and org-audit aggregators
- `.github/workflows/loc-drift-monthly.yml` — Generates cloc-based LOC statistics

**Billing Note:** Both workflows use `continue-on-error: true` per GitHub Actions billing constraints. Failures do not block merges to main.

## Manual Execution

### Worklog Aggregator

Scan all repos for worklog entries and generate consolidated index.

```bash
cd tooling/worklog-aggregator
cargo build --release
./target/release/worklog-aggregator \
  --output docs/org-audit-2026-04/worklog_index.md \
  --repos-root /Users/kooshapari/CodeProjects/Phenotype/repos
```

**Environment Variables:**
- `REPOS_ROOT` — Root directory of all repositories (default: current dir)
- `OUTPUT_FILE` — Destination for index markdown (required)

### Org Audit Aggregator

*(Optional; only if org-audit-aggregator tooling is available)*

```bash
cd tooling/org-audit-aggregator
cargo build --release
./target/release/org-audit-aggregator \
  --output docs/org-audit-2026-04/INDEX.md \
  --repos-root /Users/kooshapari/CodeProjects/Phenotype/repos
```

### LOC Drift Report

Generate lines-of-code statistics using cloc.

```bash
YEAR=$(date +%Y)
MONTH=$(date +%m)
OUTPUT_DIR="docs/org-audit-${YEAR}-$(printf '%02d' $((10#$MONTH)))"
mkdir -p "$OUTPUT_DIR"

cloc . \
  --exclude-dir=.git,node_modules,.worktrees,target,build,dist,.cargo,.archive \
  --quiet \
  --md > "$OUTPUT_DIR/loc_drift_${YEAR}_${MONTH}.md"
```

## Metrics Tracked

### Worklog Coverage

- **Per-repo worklog entries:** ARCHITECTURE, DUPLICATION, DEPENDENCIES, INTEGRATION, PERFORMANCE, RESEARCH, GOVERNANCE categories
- **Agent activity:** Decision logs, findings, completion summaries
- **Policy compliance:** UTF-8 encoding validation, spec governance, AgilePlus tracking

### Org Audit Index

- **Build status:** Cargo build, npm/yarn builds
- **Test coverage:** Unit tests, integration tests, spec traceability
- **CI health:** Workflow status, required checks, billing constraints
- **Documentation:** README, docs, governance compliance
- **Debt tracking:** Dead code, suppressions, known issues

### LOC Drift

- **Total LOC by language:** Rust, Go, TypeScript, Python, YAML, JSON, Markdown
- **Top files by size:** Complexity hotspots, decomposition candidates
- **Excluded paths:** `.git`, `node_modules`, `.worktrees`, `target`, `build`, `dist`

## Failure Handling

GitHub Actions workflows **continue on error** due to persistent billing constraints. Expected failure modes:

1. **Aggregator binary missing** — Workflow logs note missing binaries; manual build required
2. **Repository access denied** — Likely due to disk space; purge `~/.Trash` and retry
3. **Network timeouts** — Transient; retry via `workflow_dispatch`
4. **Commit conflict** — Multiple agents pushing simultaneously; pull and retry

**Recovery:**
- Review workflow logs: `.github/workflows/org-audit-weekly.yml` → Actions tab
- Manual re-run: **Actions → Org Audit Weekly → Run workflow → Run workflow** (dispatches immediately)
- Commit status: `git log --oneline docs/org-audit-*/` to verify latest audit

## Output Structure

```
docs/org-audit-2026-04/
  ├── INDEX.md                          # Status matrix (57+ repos)
  ├── worklog_index.md                  # Aggregated worklog entries
  ├── loc_drift_2026_04.md              # LOC statistics (monthly)
  ├── SYSTEMIC_ISSUES.md                # Cross-org patterns
  └── [individual repo reports]
```

## Policy Reference

- **Worklogs:** `/Users/kooshapari/CodeProjects/Phenotype/repos/worklogs/README.md`
- **Scripting:** `docs/governance/scripting_policy.md`
- **CI Completeness:** `~/.claude/references/QA_GOVERNANCE.md`
- **Git Delivery:** `~/.claude/CLAUDE.md` → "Phenotype Git and Delivery Workflow Protocol"

## See Also

- **AgilePlus:** `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus` — spec tracking and execution
- **Worklog Aggregator:** `tooling/worklog-aggregator/` — source code and CLI reference
