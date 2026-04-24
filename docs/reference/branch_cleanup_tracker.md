# Branch Cleanup Tracker

## Status: Audit Complete (2026-04-24)

### Cleanup Checklist

- [ ] **AgilePlus:** Delete 4 merged-stale branches (22+ days)
  - [ ] `chore/remove-codeowners`
  - [ ] `fix/policy-gate-clean`
  - [ ] `chore/workspace-phantom-fix`
  - [ ] `chore/workspace-phantom-member-fix`
  - [ ] Run `git worktree prune` (2 prunable)

- [ ] **heliosApp:** Delete 5 merged-stale + 7 Dependabot branches
  - [ ] `chore/agent-readiness-governance`
  - [ ] `feat/fix-ts-and-vite`
  - [ ] `feat/fix-vite-federation-rebased`
  - [ ] `feat/governance-and-ci-updates`
  - [ ] `feat/vite-federation-fix`
  - [ ] 7× `dependabot/*` branches

- [ ] **Team Review (Do NOT delete without approval):**
  - [ ] AgilePlus 33 unmerged branches (triage with team)
  - [ ] PhenoObservability 6 unmerged branches (team decision)

- [ ] **Worktree cleanup:** Evaluate 60+ abandoned entries in `.worktrees/`
  - [ ] Verify against parent repo merge status
  - [ ] Remove after confirmation

## Reference
- Full audit: `docs/reports/branch_hygiene_audit_2026_04_24.md`
- Audit method: git branch -a, merged/unmerged, worktree list
- Last scan: 2026-04-24
