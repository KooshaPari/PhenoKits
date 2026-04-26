# DEPENDENCIES

Entries tracking external dependencies, version management, upgrades, forks, and cross-project reuse patterns.

**Last updated:** 2026-04-25  
**Entries:** 2

---

## 2026-04-25 utoipa 5 API Removal (agileplus-api doctest)

**Summary:** agileplus-api doctest references `OpenApi::to_yaml()`, removed in utoipa 5. Workspace pinned to utoipa = "5"; doc example authored against utoipa 4.

**Classification:** (c) test fixture out of date; borderline (a) public-facing doc defect.

**Fix:** Replace with `serde_yaml::to_string(&ApiDoc::openapi())` or wire `utoipa-yaml` adapter. Est. 5 min.

**Pattern:** Library major-version bumps without doctest validation. Recommend `cargo test --doc` in CI.

**Source:** [AGILEPLUS_TEST_FAILURE_TRIAGE_2026_04_25.md#doctest-utoipa-5](./AGILEPLUS_TEST_FAILURE_TRIAGE_2026_04_25.md) (lines 75–101)

---

## 2026-04-25 agentapi-plusplus Vendor Directory Drift (4,681 tracked changes)

**Summary:** agentapi-plusplus branch `chore/infrastructure-push` accumulates 3,336 vendor/github.com + 771 vendor/golang.org deletions mixed with 7 local commits for governance/docs/test hygiene. Unclear whether vendor deletions are intentional refactoring or collateral clutter from a cargo-vendoring command gone wrong.

**Decision Required:** Review vendor intent before replaying docs/governance commits separately.

**Next:** Create separate salvage branches for vendor cleanup vs. governance/docs/import/test hygiene lane.

**Source:** [PHENOTYPE_LOCAL_DRIFT_MANIFEST_2026_04_25.md#agentapi-plusplus](./PHENOTYPE_LOCAL_DRIFT_MANIFEST_2026_04_25.md) (lines 25–54)
