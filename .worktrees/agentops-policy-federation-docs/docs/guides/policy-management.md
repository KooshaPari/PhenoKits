# Policy Management with policyctl

The `policyctl` CLI tool provides complete policy lifecycle management: validation, resolution, modification, querying, and audit logging. This guide covers each command with practical examples.

## Command Overview

| Command | Purpose |
|---------|---------|
| `check` | Validate policy file syntax |
| `resolve` | Resolve merged policy for a scope chain |
| `add-rule` | Add an authorization rule to a policy file |
| `remove-rule` | Remove a rule from a policy file |
| `diff` | Compare two policies and show changes |
| `audit` | Query and filter audit log events |
| `verify` | Verify policy file integrity against baseline |

## check: Validate Policy Files

Validates YAML syntax and schema of policy files without loading them.

### Syntax

```bash
policyctl check [path]
```

### Arguments

- `path` (optional): Path to specific policy file. If omitted, checks all policies in `policies/**/*.yaml`

### Examples

Check a single policy file:
```bash
policyctl check policies/system/base.yaml
```

Check all policies:
```bash
policyctl check
```

Output:
```json
{
  "result": "policy-ok",
  "count": 12
}
```

## resolve: Resolve Merged Policy

Resolves the complete effective policy by merging the scope chain. Returns the final policy document with all parent policies merged, plus metadata.

### Syntax

```bash
policyctl resolve --harness HARNESS --domain DOMAIN [--repo REPO] [--instance INSTANCE] [--overlay OVERLAY]
```

### Arguments

- `--harness` (required): Harness name (e.g., "claude-code")
- `--domain` (required): Task domain (e.g., "devops", "development")
- `--repo` (optional): Repository name (default: current directory name)
- `--instance` (optional): Task instance ID for task-specific policies
- `--overlay` (optional): Task overlay name for conditional policy layers

### Scope Chain

Policies are loaded in order and merged (later overrides earlier):

1. `system/base` — Platform-wide defaults
2. `user/org-default` — Organization defaults
3. `harness/{harness}` — Harness-specific rules
4. `repo/{repo}` — Repository-specific rules (plus any multi-repo files)
5. `task-domain/{domain}` — Domain-specific overrides
6. `task-instance/{instance}` — Task-specific overrides (if provided)
7. `task-overlay/{overlay}` — Conditional overrides (if provided)

### Examples

Resolve policy for a development task:
```bash
policyctl resolve --harness claude-code --domain development --repo myapp --instance task-42
```

Output:
```json
{
  "policy": {
    "rules": [...],
    "defaults": {...}
  },
  "policy_hash": "abc123def456",
  "scope_chain": [
    "system/base",
    "user/org-default",
    "harness/claude-code",
    "repo/myapp",
    "task-domain/development",
    "task-instance/task-42"
  ],
  "source_files": [
    "/path/to/policies/system/base.yaml",
    ...
  ]
}
```

## add-rule: Add Authorization Rule

Adds a new rule to a policy file. The rule is validated and appended to the policy's rule list.

### Syntax

```bash
policyctl add-rule --file PATH --id ID --effect EFFECT --actions ACTIONS --priority PRIORITY [--command-patterns PATTERNS] [--target-path-patterns PATTERNS] [--cwd-patterns PATTERNS] [--audit-log-path LOG_PATH]
```

### Arguments

- `--file` (required): Path to policy YAML file
- `--id` (required): Unique rule identifier (e.g., "allow-read-tools")
- `--effect` (required): `allow`, `deny`, or `ask`
- `--actions` (required): Comma-separated action types (exec, write, network)
- `--priority` (required): Integer priority (higher = checked first)
- `--command-patterns` (optional): Comma-separated command glob patterns to match
- `--target-path-patterns` (optional): Comma-separated target path patterns
- `--cwd-patterns` (optional): Comma-separated working directory patterns
- `--audit-log-path` (optional): Audit log to record the rule addition

### Examples

Add a rule allowing all read-only tool calls:
```bash
policyctl add-rule \
  --file policies/system/base.yaml \
  --id allow-read-tools \
  --effect allow \
  --actions exec \
  --priority 100 \
  --command-patterns "ls*,find*,cat*,grep*" \
  --audit-log-path ~/.policy-federation/audit.jsonl
```

Add a rule denying writes to sensitive directories:
```bash
policyctl add-rule \
  --file policies/repo/myapp.yaml \
  --id deny-system-writes \
  --effect deny \
  --actions write \
  --priority 10 \
  --target-path-patterns "/etc/*,/sys/*,/root/*"
```

Add a rule that asks for approval on network requests:
```bash
policyctl add-rule \
  --file policies/task-domain/devops.yaml \
  --id ask-external-network \
  --effect ask \
  --actions network \
  --priority 50 \
  --command-patterns "curl*,wget*"
```

Output:
```json
{
  "result": "rule-added",
  "policy": "policies/system/base.yaml",
  "rule_id": "allow-read-tools"
}
```

## remove-rule: Remove Authorization Rule

Removes a rule from a policy file by its ID. The rule is located and deleted.

### Syntax

```bash
policyctl remove-rule --file PATH --id ID [--audit-log-path LOG_PATH]
```

### Arguments

- `--file` (required): Path to policy YAML file
- `--id` (required): Rule ID to remove
- `--audit-log-path` (optional): Audit log to record the rule removal

### Examples

Remove a rule:
```bash
policyctl remove-rule \
  --file policies/system/base.yaml \
  --id allow-read-tools \
  --audit-log-path ~/.policy-federation/audit.jsonl
```

Output:
```json
{
  "result": "rule-removed",
  "policy": "policies/system/base.yaml",
  "rule_id": "allow-read-tools"
}
```

## diff: Compare Policies

Shows the differences between two policy files. Reports added, removed, modified, and effect-changed rules.

### Syntax

```bash
policyctl diff BEFORE_PATH AFTER_PATH
```

### Arguments

- `BEFORE_PATH`: Path to original policy file
- `AFTER_PATH`: Path to new policy file

### Examples

Compare policy versions:
```bash
policyctl diff policies/system/base.yaml policies/system/base.yaml.new
```

Output (formatted with colors):
```
Policy Diff Report
============================================================

Added Rules (1)
  + allow-new-rule: allow
    Allow new network access

Removed Rules (0)

Modified Rules (1)
  ~ deny-old-rule
    Before: deny
    After:  ask

Effect Changes (1)
  ! update-rule: allow -> ask
    Changed from allow to ask for sensitive operations

Summary: +1 -0 ~1 !1
============================================================
```

Also outputs as JSON:
```json
{
  "added_rules": [
    {
      "id": "allow-new-rule",
      "effect": "allow",
      "description": "Allow new network access"
    }
  ],
  "removed_rules": [],
  "modified_rules": [...],
  "effect_changes": [...]
}
```

## audit: Query Audit Log

Reads and filters audit log events. Supports time-range filtering, decision/action filtering, and chain verification.

### Syntax

```bash
policyctl audit [--log-path PATH] [--since DATETIME] [--until DATETIME] [--action ACTION] [--decision DECISION] [--actor PATTERN] [--verify-chain] [--summary]
```

### Arguments

- `--log-path` (optional): Path to audit log (default: `$POLICY_AUDIT_LOG_PATH` or `~/.policy-federation/audit.jsonl`)
- `--since` (optional): ISO-8601 datetime (e.g., "2024-01-01T00:00:00Z")
- `--until` (optional): ISO-8601 datetime
- `--action` (optional): Filter by action (`exec`, `write`, `network`)
- `--decision` (optional): Filter by decision (`allow`, `deny`, `ask`)
- `--actor` (optional): Filter by actor name (regex pattern or substring)
- `--verify-chain` (optional): Verify audit chain integrity
- `--summary` (optional): Show summary statistics instead of individual events

### Examples

Show all audit events:
```bash
policyctl audit
```

Filter by decision:
```bash
policyctl audit --decision deny
```

Filter by action and actor:
```bash
policyctl audit --action write --actor claude-code
```

Filter by time range:
```bash
policyctl audit \
  --since 2024-01-01T00:00:00Z \
  --until 2024-01-31T23:59:59Z
```

Show summary statistics:
```bash
policyctl audit --summary
```

Output:
```json
{
  "total": 150,
  "by_decision": {
    "allow": 120,
    "deny": 25,
    "ask": 5
  },
  "by_action": {
    "exec": 80,
    "write": 50,
    "network": 20
  }
}
```

Verify audit chain integrity:
```bash
policyctl audit --verify-chain
```

Output:
```
Chain verification: All 150 event(s) are valid
```

Individual audit event (JSONL):
```json
{
  "timestamp": "2024-01-15T10:30:45Z",
  "run_id": "abc123def456",
  "action": "write",
  "command": "edit myfile.py",
  "final_decision": "allow",
  "actor": "claude-code",
  "policy_hash": "policy123",
  "scope_chain": ["system/base", "harness/claude-code"]
}
```

## verify: Verify Policy Integrity

Verifies that policy files have not been tampered with. Uses SHA-256 hashes of all policy source files to detect modifications.

### Syntax

```bash
policyctl verify [--repo-root PATH]
```

### Arguments

- `--repo-root` (optional): Repository root (default: inferred from command location)

### How It Works

1. **First run**: Computes and records SHA-256 hash of all `policies/**/*.yaml` files in `.policyctl.verify` baseline
2. **Subsequent runs**: Computes current hash and compares against baseline
3. **Result**:
   - `ok`: Hashes match, no tampering detected
   - `tampered`: Hashes differ, policy files were modified

### Examples

Record baseline:
```bash
policyctl verify
```

Output:
```json
{
  "status": "baseline-recorded",
  "hash": "sha256:abc123def456...",
  "file_count": 12
}
```

Verify against baseline:
```bash
policyctl verify
```

Output (if unchanged):
```json
{
  "status": "ok",
  "hash": "sha256:abc123def456...",
  "file_count": 12
}
```

Output (if tampered):
```json
{
  "status": "tampered",
  "current_hash": "sha256:xyz789...",
  "baseline_hash": "sha256:abc123def456...",
  "file_count": 12
}
```

Exit code is 1 if tampered.

## Workflow: Modify and Deploy Policies

Typical workflow to safely modify policies:

1. **Create a new policy file** (e.g., for a new rule)
2. **Validate syntax**:
   ```bash
   policyctl check policies/system/new-rule.yaml
   ```
3. **Preview changes**:
   ```bash
   policyctl diff policies/system/base.yaml policies/system/new-rule.yaml
   ```
4. **Add rule** (if modifying existing file):
   ```bash
   policyctl add-rule --file policies/system/base.yaml \
     --id new-rule-id --effect allow --actions exec --priority 100
   ```
5. **Verify resolution** (test scope chain):
   ```bash
   policyctl resolve --harness claude-code --domain development
   ```
6. **Record in audit log**:
   ```bash
   policyctl audit --summary
   ```
7. **Verify no tampering**:
   ```bash
   policyctl verify
   ```

## Multi-Repo Policy Composition

When resolving policies for a repository, all YAML files in `policies/repo/` are loaded and merged in order. This enables:

- Primary repo policy: `policies/repo/{repo}.yaml`
- Additional repo policies: `policies/repo/*.yaml` (sorted alphabetically)
- Multi-repo setups: Load policies for multiple repos in one resolution chain

Example:
```bash
# For POLICY_REPO=thegent, loads both:
# - policies/repo/thegent.yaml (primary)
# - policies/repo/phenotype-worktrees.yaml (additional, alphabetically)
policyctl resolve --harness claude-code --domain development --repo thegent
```

## Environment Variables

- `POLICY_AUDIT_LOG_PATH`: Default audit log path (overrides `~/.policy-federation/audit.jsonl`)
- `POLICY_REPO`: Default repository name (overrides current directory name)
- `POLICY_TASK_DOMAIN`: Default task domain
- `POLICY_TASK_INSTANCE`: Default task instance
- `POLICY_TASK_OVERLAY`: Default task overlay

## See Also

- [Security Model](../reference/security-model.md) — Threat model, bypass detection, audit integrity
- `cli/src/policy_federation/cli.py` — CLI implementation
- `policies/` — Policy file directory structure
