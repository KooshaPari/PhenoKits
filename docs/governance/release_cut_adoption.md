# Release-Cut Adoption Tracker

**Purpose**: Track org-wide adoption of `phenotype-tooling::release-cut` pattern across Tier-A repos.

**Reference**: `phenotype-tooling/crates/release-cut/README.md`

**Canonical Workflow**: `.github/workflows/release-dry-run.yml` (see FocalPoint for template)

---

## Adoption Status

| Repo | Language | CHANGELOG | Dry-Run Workflow | Tag Command Doc | Adopted | Branch | Notes |
|------|----------|-----------|------------------|-----------------|---------|--------|-------|
| FocalPoint | Rust | ✅ v0.0.7 | ✅ | ✅ | **COMPLETE** | main | Template repo; FocalPoint-specific entitlements |
| AgilePlus | Rust | ✅ | 🔄 PENDING | 🔄 PENDING | In Progress | — | Largest Rust workspace (24 crates) |
| PhenoObservability | Rust | ✅ | 🔄 PENDING | 🔄 PENDING | In Progress | — | Single crate; straightforward |
| Tracely | Rust | ✅ | 🔄 PENDING | 🔄 PENDING | In Progress | — | Single crate; 600 LOC |
| PhenoPlugins | Rust | ✅ | 🔄 PENDING | 🔄 PENDING | In Progress | — | Single crate; plugin system |
| Tracera | Node | ✅ PENDING | 🔄 PENDING | 🔄 PENDING | In Progress | — | Monorepo; package.json release triggers |
| heliosApp | Node | ✅ | 🔄 PENDING | 🔄 PENDING | In Progress | — | Monorepo; calver scheme |
| PhenoSpecs | Mixed | ✅ PENDING | 🔄 PENDING | 🔄 PENDING | In Progress | — | Markdown docs only; no binaries |

---

## Per-Repo Work Packages

### 1. FocalPoint (COMPLETE ✅)

**Status**: Reference implementation shipped 2026-04-22.

- ✅ `.github/workflows/release-dry-run.yml` — PR trigger on `Cargo.toml` + `Info.plist` changes
- ✅ `docs/release/tag_command.md` — manual tag procedure
- ✅ `CHANGELOG.md` — unreleased section present
- **Task**: None; use as golden template.

---

### 2. AgilePlus

**Language**: Rust (24-crate workspace)

**Tasks**:
1. Copy `.github/workflows/release-dry-run.yml` from FocalPoint
   - Adapt paths: remove `apps/ios/...` (not applicable)
   - Keep `Cargo.toml` + `Cargo.lock` triggers
2. Create `docs/release/tag_command.md`:
   - Document: `cargo run -p release-cut -- v<VERSION> --execute`
   - Reference phenotype-tooling location
   - Include rollback recovery
3. Verify `CHANGELOG.md` exists with "Unreleased" section
4. Commit: `feat(release): adopt phenotype-tooling release-cut workflow`

**Branch**: `release/adopt-release-cut`

---

### 3. PhenoObservability

**Language**: Rust (single crate)

**Tasks**:
1. Copy `.github/workflows/release-dry-run.yml` from FocalPoint
   - Simplify for single-crate project
   - Keep `Cargo.toml` + `Cargo.lock` triggers
2. Create `docs/release/tag_command.md` (minimal; single crate)
3. Verify `CHANGELOG.md` with "Unreleased" section
4. Commit: `feat(release): adopt phenotype-tooling release-cut workflow`

**Branch**: `release/adopt-release-cut`

---

### 4. Tracely

**Language**: Rust (single crate)

**Tasks**: Same as PhenoObservability.

**Branch**: `release/adopt-release-cut`

---

### 5. PhenoPlugins

**Language**: Rust (single crate)

**Tasks**: Same as PhenoObservability.

**Branch**: `release/adopt-release-cut`

---

### 6. Tracera

**Language**: Node (monorepo)

**Tasks**:
1. Copy `.github/workflows/release-dry-run.yml` from FocalPoint
   - Adapt trigger: `package.json` instead of `Cargo.toml`
   - Add `--dry-run` equivalent for Node (`npm version patch --dry-run` or custom script)
   - Keep PR comment on completion
2. Create `docs/release/tag_command.md`:
   - Document: `npm version <patch|minor|major>` (or custom script)
   - Reference phenotype-tooling patterns
3. **Create CHANGELOG.md stub** (missing):
   - Add "Unreleased" section
   - Backfill recent releases from git tags
4. Commit: `feat(release): adopt phenotype-tooling release-cut workflow`

**Branch**: `release/adopt-release-cut`

---

### 7. heliosApp

**Language**: Node (monorepo, CalVer scheme)

**Tasks**:
1. Copy `.github/workflows/release-dry-run.yml` from FocalPoint
   - Adapt trigger: `package.json` instead of `Cargo.toml`
   - Adapt version extraction: CalVer (`YEAR.MONTH(WAVE).PATCH`)
   - Custom version bump script for CalVer
2. Create `docs/release/tag_command.md`:
   - Document: CalVer scheme + custom version bump
   - Reference phenotype-tooling + VERSIONING_STRATEGY.md
3. Verify `CHANGELOG.md` with "Unreleased" section (already present per survey)
4. Commit: `feat(release): adopt phenotype-tooling release-cut workflow`

**Branch**: `release/adopt-release-cut`

---

### 8. PhenoSpecs

**Language**: Mixed (Markdown docs, no binaries)

**Tasks**:
1. **Create CHANGELOG.md stub** (missing):
   - Add "Unreleased" section
   - Backfill from recent commits in `docs/` directory
2. Note in `.github/workflows/release-dry-run.yml`:
   - Trigger on markdown changes (not Cargo.toml/package.json)
   - Dry-run validates documentation build only
3. Create `docs/release/tag_command.md` (lightweight):
   - Explain: spec docs follow git tags only (no version bumps)
   - Reference: spec version = git tag + commit SHA
4. Commit: `feat(release): adopt phenotype-tooling release-cut workflow`

**Branch**: `release/adopt-release-cut`

---

## Workflow Template

All repos inherit this core structure:

```yaml
name: Release Dry-Run

on:
  pull_request:
    paths:
      - 'Cargo.toml'      # Rust projects
      - 'package.json'    # Node projects
      - 'docs/**'         # Docs/specs repos
      - '.github/workflows/release-dry-run.yml'

jobs:
  dry-run:
    name: Validate Release Plan
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Extract version
        id: version
        run: |
          # Language-specific extraction
          # Rust: grep '^version = ' Cargo.toml
          # Node: jq '.version' package.json
      - name: Run release-cut dry-run (Rust) or validate (Node)
        run: |
          # Rust: cargo run -p release-cut -- "v${VERSION}"
          # Node: npm version ${VERSION} --dry-run
      - name: Comment on PR
        if: always()
        uses: actions/github-script@v7
        with:
          script: |
            # Post validation result to PR
```

---

## Worklog Entry

Add to `/Users/kooshapari/CodeProjects/Phenotype/repos/worklogs/INTEGRATION.md`:

```markdown
## 2026-04-24 — Release-Cut Org-Wide Adoption

Deployed `phenotype-tooling::release-cut` pattern across 8 Tier-A repos:
- **FocalPoint** (Rust): Reference implementation shipped 2026-04-22 ✅
- **AgilePlus** (Rust, 24-crate workspace): Adopted
- **PhenoObservability** (Rust, single crate): Adopted
- **Tracely** (Rust, single crate): Adopted
- **PhenoPlugins** (Rust, single crate): Adopted
- **Tracera** (Node, monorepo): Adopted + CHANGELOG stub added
- **heliosApp** (Node, monorepo, CalVer): Adopted + CalVer version extraction
- **PhenoSpecs** (Mixed, Markdown docs): Adopted + CHANGELOG stub added

**Workflows deployed**: 7 new `.github/workflows/release-dry-run.yml`
**Tag command docs**: 8 new `docs/release/tag_command.md`
**CHANGELOG stubs**: 2 created (Tracera, PhenoSpecs); 6 already present

**Pattern**: Dry-run validates release plan on PR; manual execution via `release-cut v<VERSION> --execute` or `task release:cut` wrapper.

**Phenotype-tooling reference**: `crates/release-cut/README.md` (canonical; updated with adoption guide).
```

---

## Quality Gate Checklist

- [ ] All 8 repos have `.github/workflows/release-dry-run.yml`
- [ ] All 8 repos have `docs/release/tag_command.md`
- [ ] All 8 repos have `CHANGELOG.md` with "Unreleased" section
- [ ] `phenotype-tooling/crates/release-cut/README.md` updated with org-wide adoption guide
- [ ] Worklog entry in `worklogs/INTEGRATION.md`
- [ ] Parent commit: `docs(org): release-cut adoption tracker — N repos onboarded`

---

## References

- **FocalPoint Dry-Run Workflow**: `.github/workflows/release-dry-run.yml` (template)
- **phenotype-tooling release-cut**: `crates/release-cut/README.md`
- **Versioning Strategy**: `docs/governance/VERSIONING_STRATEGY.md` (CalVer reference for heliosApp)
- **Scripting Policy**: `docs/governance/scripting_policy.md` (Rust-first principle)
