# Security Model

This document describes the threat model, bypass mitigations, audit integrity, and policy scoping for the policy federation system.

## Overview

The policy federation system enforces authorization rules over agent operations (exec, write, network). It is designed to:

1. **Enforce policy rules** — Approve, deny, or ask for approval on operations
2. **Detect bypass attempts** — Identify write-via-exec, env-var override, and TOCTOU attacks
3. **Maintain audit integrity** — Record all decisions with tamper detection
4. **Support policy composition** — Layer policies from system to task-specific scopes

## Threat Model

### Attackers

- **Untrusted Agent Code**: May attempt to circumvent authorization rules
- **Policy Tampering**: May try to modify policies to allow unauthorized operations
- **Audit Log Tampering**: May try to hide evidence of violations

### Attacks

| Attack | Mitigation | Reference |
|--------|-----------|-----------|
| **Write-via-Exec** | Bash command write patterns detected and reclassified | Bypass Detection |
| **Env-Var Override** | POLICY_* env vars stripped from commands | Env Sanitization |
| **TOCTOU** | Policy sources hash-checked before and after allow decision | TOCTOU Protection |
| **Policy Replacement** | SHA-256 hash baseline prevents file modification | Policy Integrity |
| **Audit Forgery** | Audit events validated for required fields | Audit Log Integrity |
| **Direct Tool Usage** | Tool action classification (write vs exec vs network) | Authorization Check |

---

## Bypass Detection

The system detects and blocks common bypass attempts by identifying patterns in commands that are performing file writes without using the explicit Write tool action.

### Write-via-Exec Patterns

Detected patterns indicate a Bash command is writing files (bypassing the write action classification):

| Pattern | Example | Detection Regex |
|---------|---------|-----------------|
| **Python file write** | `python -c 'open(...).write(...)'` | `python[23]? -c.*(?:open\|write)` |
| **Shell redirect write** | `echo content > /path/file` | `[^2]?> /` |
| **Tee write** | `cat file \| tee output.txt` | `\btee\s+(?:-a\s+)?/` |
| **DD write** | `dd if=in of=/path/out` | `\bdd\b.*\bof=` |
| **Heredoc write** | `cat <<EOF > file` | `<<\s*['\"]?EOF` |
| **Perl write** | `perl -e 'open(...)' file` | `perl\s+-[ep].*(?:open\|>)` |
| **Ruby write** | `ruby -e 'File.write(...)'` | `ruby\s+-e.*File\.write` |
| **Node write** | `node -e 'fs.writeFileSync(...)'` | `node\s+-e.*writeFile` |
| **Sed in-place** | `sed -i 's/a/b/' file` | `\bsed\s+-i` |
| **Copy** | `cp src dest` | `\bcp\b` |
| **Move** | `mv src dest` | `\bmv\b` |
| **Install** | `install -m 644 file` | `\binstall\b\s+(?:-[a-zA-Z]*\s+)*\S` |
| **Xargs write** | `find . \| xargs cp` | `\|\s*xargs\s+(?:rm\|cp\|mv\|tee\|dd)` |
| **Subshell write** | `$(sed -i 's/a/b/' file)` | Detected recursively in $(...) |

### Detection Behavior

When a write-via-exec bypass is detected:

1. **Reclassify action**: Change from `exec` to `write`
2. **Extract targets**: Attempt to extract file paths from command:
   - Python open() calls: `open("path")`
   - Redirect targets: `> /path`
   - Tee targets: `tee /path`
3. **Add indicators**: Record bypass indicator names (e.g., "shell-redirect-write")
4. **Apply write rules**: Use write-action authorization rules for this operation

Example audit event:
```json
{
  "action": "write",
  "command": "echo content > /tmp/file",
  "bypass_indicators": ["shell-redirect-write"],
  "target_paths": ["/tmp/file"],
  "final_decision": "deny"
}
```

### Compound Commands

Bash compound commands are split on `&&`, `;`, and `||` operators and each segment is checked independently:

```bash
# Each segment checked separately
cd /tmp && echo test > file && cat file
```

This ensures writes in the middle of a command chain are detected.

### Subshell Detection

Write commands inside subshell expansions are detected:

```bash
# Both detected as write attempts
echo $(sed -i 's/a/b/' file)
echo `cp src dest`
```

---

## Environment Variable Sanitization

Protected environment variables (POLICY_* vars) are stripped from commands to prevent actors from overriding policy parameters.

### Protected Variables

```
POLICY_REPO
POLICY_TASK_DOMAIN
POLICY_TASK_INSTANCE
POLICY_TASK_OVERLAY
POLICY_ACTOR
POLICY_ASK_MODE
POLICY_SIDECAR_PATH
POLICY_AUDIT_LOG_PATH
```

### Patterns Blocked

| Pattern | Example | Blocked? |
|---------|---------|----------|
| Direct assignment | `POLICY_REPO=evil cmd` | Yes |
| Env prefix | `env POLICY_REPO=evil cmd` | Yes |
| Export | `export POLICY_REPO=evil; cmd` | Yes |
| Export with chaining | `export POLICY_REPO=evil && cmd` | Yes |
| Legitimate env var | `MY_VAR=value cmd` | No |

### Detection Example

Command: `POLICY_REPO=malicious policyctl resolve --harness claude-code --domain dev`

Result:
```json
{
  "env_overrides": ["POLICY_REPO"],
  "bypass_indicators": ["env-override:POLICY_REPO"],
  "action": "write",  // reclassified
  "final_decision": "deny"
}
```

---

## TOCTOU Protection

Time-of-Check-Time-of-Use (TOCTOU) attacks are prevented by re-verifying the policy hash before allowing an operation.

### How It Works

1. **Check phase**: Compute hash of policy source files
2. **Resolve**: Load and merge policies
3. **Evaluate**: Compute authorization decision
4. **Allow decision only if**: Re-compute policy hash and compare
5. **If mismatch**: Deny operation (policy was modified between check and use)

### Implementation

File: `cli/src/policy_federation/interceptor.py`

```python
# Before allow decision
sources_hash = hash_policy_sources(source_paths)

# ... evaluation ...

# Before allowing
if allowed and hash_policy_sources(source_paths) != sources_hash:
    exit_code = DENY_EXIT_CODE
    allowed = False
    final_decision = "deny"
    evaluation = {**evaluation, "decision": "deny", "reason": "policy-tampered"}
```

### Audit Event

```json
{
  "action": "exec",
  "command": "some-command",
  "final_decision": "deny",
  "reason": "policy-tampered",
  "timestamp": "2024-01-15T10:30:45Z"
}
```

---

## Audit Log Integrity

Audit logs are append-only JSONL files with event structure validation and optional chain verification.

### Event Structure

Each audit event is a JSON object on a single line with required fields:

```json
{
  "timestamp": "2024-01-15T10:30:45Z",
  "run_id": "abc123def456",
  "action": "write|exec|network",
  "command": "the command",
  "final_decision": "allow|deny|ask",
  "actor": "actor-name",
  "policy_hash": "hash of policy sources",
  "scope_chain": [
    "system/base",
    "harness/claude-code",
    "repo/myapp"
  ],
  "bypass_indicators": ["indicator-name"],
  "target_paths": ["/path/to/file"],
  "evaluation": {...}
}
```

### Required Fields

Verification checks that all events have:
- `run_id` — Unique run identifier
- `action` — exec, write, or network
- `final_decision` — allow, deny, or ask

### Append-Only

Audit log is written with append flag (`"a"` mode):

```python
def append_audit_event(*, audit_log_path: Path, event: dict) -> None:
    audit_log_path.parent.mkdir(parents=True, exist_ok=True)
    with audit_log_path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(event, sort_keys=True) + "\n")
```

### Chain Verification

The `verify-chain` flag checks all events have required fields:

```bash
policyctl audit --verify-chain
```

Output:
```json
{
  "valid": true,
  "message": "All 150 event(s) are valid",
  "events_checked": 150
}
```

If invalid:
```json
{
  "valid": false,
  "message": "Found 2 event(s) with missing required fields",
  "invalid_events": [
    {
      "index": 42,
      "event": {...},
      "missing_fields": ["final_decision"]
    }
  ]
}
```

### Tamper Detection

Policy integrity is verified by comparing SHA-256 hashes:

```bash
policyctl verify
```

Creates `.policyctl.verify` baseline on first run:
```
sha256:abc123def456...
```

Subsequent runs compare against baseline. Exit code 1 if tampered.

---

## Policy Scoping

Policies are organized in layers from system-wide to task-specific. Each layer is a scope, and policies are merged following a well-defined order.

### Scope Hierarchy

```
System (broadest)
  |
User
  |
Harness
  |
Repository
  |
Task Domain
  |
Task Instance
  |
Task Overlay (narrowest)
```

### Scope Directories

| Scope | Directory | Example |
|-------|-----------|---------|
| **system** | `policies/system/` | `base.yaml` |
| **user** | `policies/user/` | `org-default.yaml` |
| **harness** | `policies/harness/` | `claude-code.yaml` |
| **repo** | `policies/repo/` | `myapp.yaml`, `phenotype-wtrees.yaml` |
| **task-domain** | `policies/task-domain/` | `development.yaml`, `devops.yaml` |
| **task-instance** | `policies/task-instance/` | `task-42.yaml` |
| **task-overlay** | `policies/task-overlay/` | `experimental.yaml` |

### Merge Order

When resolving policy for `--harness claude-code --domain development --instance task-42`:

1. Load `policies/system/base.yaml`
2. Load `policies/user/org-default.yaml` and merge
3. Load `policies/harness/claude-code.yaml` and merge
4. Load `policies/repo/{repo}.yaml` (and any other repo/*.yaml files) and merge
5. Load `policies/task-domain/development.yaml` and merge
6. Load `policies/task-instance/task-42.yaml` and merge (if exists)
7. Load `policies/task-overlay/{overlay}.yaml` and merge (if provided)

Later layers override earlier ones.

### Resolved Output

```json
{
  "policy": { ... merged rules ... },
  "scope_chain": [
    "system/base",
    "user/org-default",
    "harness/claude-code",
    "repo/myapp",
    "task-domain/development",
    "task-instance/task-42"
  ],
  "source_files": [
    "/abs/path/to/policies/system/base.yaml",
    ...
  ],
  "policy_hash": "sha256:abc123...",
  "policy_hash_timestamp": "2024-01-15T10:30:45Z"
}
```

### Multi-Repo Composition

All files in `policies/repo/` are loaded (sorted alphabetically). This enables:

- Primary repo policy: `policies/repo/{repo}.yaml`
- Shared policies: `policies/repo/shared.yaml`
- Platform policies: `policies/repo/platform.yaml`

Example for `--repo thegent`:
```
policies/repo/platform.yaml       # Loaded
policies/repo/phenotype-wtrees.yaml  # Loaded
policies/repo/thegent.yaml        # Loaded (primary)
```

Merged in alphabetical order: platform -> phenotype -> thegent.

---

## Bootstrap Procedure: Safe Policy Modification

To safely modify policies, use the "ask" rule pattern:

1. **Create an "ask" rule** in system/base.yaml:

```yaml
rules:
  - id: ask-policy-edit
    effect: ask
    actions: [write]
    priority: 1000
    match:
      target_path_patterns: ["policies/**/*.yaml"]
    description: "Ask before modifying policies"
```

2. **Require explicit approval** before policy writes are allowed:

```bash
policyctl add-rule \
  --file policies/system/base.yaml \
  --id allow-verified-policy-edit \
  --effect allow \
  --actions write \
  --priority 1001 \
  --target-path-patterns "policies/**/*.yaml" \
  --audit-log-path ~/.policy-federation/audit.jsonl
```

3. **Execute with approval prompt**:

```bash
policyctl exec \
  --harness claude-code \
  --domain development \
  --ask-mode prompt \
  --prompt-text "Approve policy modification?" \
  -- policyctl add-rule --file policies/system/base.yaml ...
```

4. **Verify changes**:

```bash
policyctl check
policyctl audit --since 2024-01-15T10:00:00Z --decision allow
```

---

## Authorization Evaluation

When a command is intercepted:

1. **Extract action type** from tool or command pattern
2. **Resolve policy** for the scope chain
3. **Evaluate rules** in priority order (higher priority first)
4. **Match conditions**: command patterns, target paths, cwd patterns
5. **Apply effect**:
   - **allow**: Operation proceeds
   - **deny**: Operation blocked (exit code 2)
   - **ask**: Prompt user (exit code 3 if rejected)
6. **TOCTOU re-verify**: If allowing, re-hash policies
7. **Record audit event**: Timestamp, decision, policy hash, scope chain

### Rule Matching

Rules are evaluated in priority order. First matching rule wins.

Rule structure:
```yaml
rules:
  - id: rule-1
    effect: allow
    actions: [exec, write]
    priority: 100
    match:
      command_patterns: ["ls*", "find*"]
      target_path_patterns: ["*.py"]
      cwd_patterns: ["/home/**"]
    description: "Allow safe commands in home directory"
```

Match conditions (all must match if specified):
- `command_patterns`: Glob patterns against normalized command
- `target_path_patterns`: Glob patterns against target file paths
- `cwd_patterns`: Glob patterns against working directory

---

## Implementation Files

| File | Purpose |
|------|---------|
| `cli/src/policy_federation/cli.py` | CLI entry point, all commands |
| `cli/src/policy_federation/claude_hooks.py` | Claude hook integration, bypass detection |
| `cli/src/policy_federation/interceptor.py` | Command interception, TOCTOU protection |
| `cli/src/policy_federation/resolver.py` | Policy loading, merging, scope chains |
| `cli/src/policy_federation/authorization.py` | Rule evaluation, decision logic |
| `cli/src/policy_federation/runtime_artifacts.py` | Audit logging, sidecar management |
| `cli/src/policy_federation/policy_editor.py` | Rule add/remove operations |
| `cli/src/policy_federation/policy_diff.py` | Policy comparison |
| `cli/src/policy_federation/validate.py` | YAML validation, schema checking |

---

## See Also

- [Policy Management Guide](../guides/policy-management.md) — CLI command reference
- `tests/unit/test_claude_hooks.py` — Bypass detection tests
- `tests/unit/test_interceptor.py` — TOCTOU protection tests
- `tests/unit/test_audit.py` — Audit log integrity tests
