# Wave2-F21 Operator Runbook

## Scope
This runbook defines the implementation procedure for Wave2-F21 adoption across target repositories in the `agentops-policy-federation` lane. It is designed for operators executing coordinated, branch-based rollout with explicit validation and recovery paths.

## Preconditions
- Access:
  - GitHub org/repo read-write access for all in-scope repositories
  - Ability to create branches and pull requests
  - Permission to view CI checks and workflow logs
- Local environment:
  - `git`, `gh`, and required repo toolchain installed
  - Authenticated GitHub CLI session (`gh auth status`)
  - Clean worktree in each target repository before edits
- Governance readiness:
  - Confirm required branch protections and merge rules for each repo
  - Confirm required checks list and expected policy gates
- Rollout inputs prepared:
  - Final Wave2-F21 policy/adoption spec
  - Repo inventory and rollout order
  - Owner/escalation contact list

## Execution Model
- Delivery mode: branch + PR per repository (no direct `main` writes)
- Rollout mode: forward-only adoption with explicit fix-forward path
- Concurrency: bounded lanes to reduce blast radius (recommend 3-6 active repos)
- Completion criteria per repo:
  - Implementation applied
  - Validation checks passed
  - PR approved/merged per policy

## Per-Repository Procedure
Repeat the following for each repository in the Wave2-F21 inventory.

### 1. Initialize lane branch
1. Ensure canonical checkout is up to date on `main`.
2. Create/switch to lane work branch:
   - `BRANCH_NAME="wave2-f21-adoption/<repo-or-scope>"`
   - `git checkout -b "$BRANCH_NAME"`

### 2. Apply Wave2-F21 adoption changes
1. Implement required policy/config/code/doc changes defined by Wave2-F21.
2. Ensure changes are complete (no partial migration or compatibility shim unless explicitly required by policy).
3. Keep scope limited to adoption work for this repo.

### 3. Stage and open PR
1. Stage and commit with lane-specific message.
2. Push branch and open PR:
   - `git push -u origin "$BRANCH_NAME"`
   - `gh pr create --fill --base main --head "$BRANCH_NAME"`
3. Add labels/reviewers/owners per repository protocol.

### 4. Validate checks
Run local and CI validation checkpoints (replace placeholders below per repo):

```bash
# [PLACEHOLDER] repo-local lint/format/type gates
<VALIDATION_CMD_1>
<VALIDATION_CMD_2>

# [PLACEHOLDER] repo-local policy/adoption verification
<VALIDATION_CMD_3>

# [PLACEHOLDER] PR checks status
gh pr checks --watch
```

Record results in PR comment or lane tracker:
- Validation timestamp
- Commands executed
- Pass/fail status
- Any exceptions with owner acknowledgement

### 5. Merge and close
1. Merge only when all required checks are green and review requirements are satisfied.
2. Use merge strategy aligned to repo policy.
3. Confirm `main` contains final adoption commit(s).

## Validation Command Placeholders (Template)
Use this block to define repo-specific commands before execution.

```bash
# Repo: <REPO_NAME>
# Branch: wave2-f21-adoption/<scope>

# Build / static checks
<REPO_BUILD_CMD>
<REPO_LINT_CMD>
<REPO_TYPECHECK_CMD>

# Test/profile checks required by repo policy
<REPO_REQUIRED_CHECK_CMD_1>
<REPO_REQUIRED_CHECK_CMD_2>

# Policy-specific validation
<REPO_WAVE2_F21_POLICY_VERIFY_CMD>

# PR/CI status
gh pr checks --watch
```

## Rollback and Forward-Fix Strategy

### Preferred: Forward fix
- If validation fails, patch on same lane branch when safe.
- If changeset is too broad, split into stacked PRs preserving dependency order.
- Re-run required validation and keep audit trail in PR updates.

### Rollback trigger conditions
Use rollback only when:
- Production/safety risk is confirmed
- Regulatory/policy violation cannot be fixed in time on current branch
- Critical workflow outage tied directly to Wave2-F21 change

### Rollback procedure (PR-based)
1. Create rollback branch from current `main`.
2. Revert only offending adoption commit set.
3. Open emergency rollback PR with incident context.
4. Merge via approved emergency path.
5. Immediately open forward-fix follow-up PR with corrected implementation.

### Post-rollback requirements
- Document root cause and scope of impact.
- Attach timeline and command evidence.
- Track forward-fix to closure before marking lane complete.

## Escalation Protocol
Escalate when any blocking condition persists beyond SLA or requires policy exception.

### Escalation triggers
- Repeated CI/policy gate failures without deterministic fix
- Merge blocked by org ruleset/protection mismatch
- Missing permissions/secrets preventing completion
- Cross-repo dependency deadlock
- Security/compliance concern introduced by adoption delta

### Escalation path
1. Repository owner / on-call maintainer
2. Wave2-F21 lane lead
3. Platform/governance owner
4. Security/compliance owner (if applicable)

### Escalation packet (required)
Provide:
- Repo name and PR link
- Current branch and latest commit SHA
- Exact failing check(s) and logs/URLs
- Actions attempted and outcomes
- Proposed decision request (exception, unblock, or rollback)

## Operator Checklist
- [ ] Preconditions verified
- [ ] Branch created per repo
- [ ] Wave2-F21 changes applied
- [ ] Validation commands executed and recorded
- [ ] PR opened with required reviewers/labels
- [ ] Required checks green
- [ ] PR merged per policy
- [ ] Lane tracker updated
- [ ] Escalations (if any) documented and resolved
