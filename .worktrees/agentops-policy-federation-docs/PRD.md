# Product Requirements Document — AgentOps Policy Federation

**Version:** 1.1.0
**Stack:** Python 3.10+ (uv), Typer/Click CLI, YAML policy files, JSON schemas
**Repo:** `agentops-policy-federation`
**CLI entrypoint:** `policyctl` (`python -m policy_federation.cli`)

---

## Overview

AgentOps Policy Federation is a policy enforcement layer for AI agent harnesses (Codex, Cursor,
Factory, Claude Code). It resolves, compiles, and enforces YAML-based policies governing what
actions agents may execute — covering shell commands, file writes, and network access.

The system operates in three phases: **resolve** (merge YAML policy documents from seven ordered
scopes), **compile** (emit harness-native config for unconditional rules), and **intercept**
(gate execution at runtime via guard shims and Claude Code hooks).

```
policies/**/*.yaml      extensions/registry.yaml
       |                         |
       v                         v
   Resolver (7-scope chain + _extends) -----> compiled native config (codex/cursor/claude)
       |
       v
   Interceptor (exec / write / network guard shims)
       |
       +---> Audit log (JSONL)
       |           |
       v           v
   Risk scorer  Gap detector --> Learner (rule suggestions)
                                     |
                                     v
                               policies/suggestions/
```

---

## E1: Deterministic Policy Resolution

### E1.1: Seven-Scope Resolution Chain

**As** an operator,
**I want** policy resolved through a fixed seven-scope chain
(system > user > harness > repo > task\_domain > task\_instance > task\_overlay)
**so that** the effective policy is always deterministic and auditable.

**Acceptance Criteria:**

- Resolution order is fixed; later scopes override earlier ones via configured merge strategy.
- Each scope loads a YAML document from `policies/<scope>/<label>.yaml`.
- `_extends` field allows inheriting from another YAML file before merge.
- Merge strategies supported: `merge_map` (default), `replace`, `append`, `append_unique`.
- Resolved payload includes `policy_hash` (SHA-256 of sorted merged policy JSON).
- Payload records `scope_chain`, `source_files`, `contract_ids`, `contract_versions`,
  and `conflicts` list.
- `policyctl resolve --harness <h> --repo <r> --domain <d>` prints resolved JSON.
- Unknown merge strategies are reported as conflicts and fall back to `merge_map`.

### E1.2: Authorization DSL

**As** a security engineer,
**I want** an authorization rule DSL with effects (allow/deny/ask), priority ordering, and
pattern matching
**so that** agent actions are governed by explicit, reviewable policy.

**Acceptance Criteria:**

- Rules have `effect` (allow/deny/ask), `priority` (int, higher wins), and `actions` list.
- `match` sub-object supports: `command_patterns`, `cwd_patterns`, `actor_patterns`,
  `target_path_patterns` (all fnmatch/glob).
- Tie-breaking at equal priority: deny > ask > allow.
- Default effect (allow or deny) configurable per-policy; applied when no rule matches.
- `authorization_summary` in resolve output includes `default_effects` and `rule_count`.

### E1.3: Extension Registry

**As** an operator,
**I want** optional policy extensions declared in `extensions/registry.yaml`
**so that** capabilities are toggled without editing core policy files.

**Acceptance Criteria:**

- Extensions have `id`, `enabled`, `scope_selector` (glob), and `description`.
- Enabled extensions are merged into the resolve output under `extensions`.
- `policyctl resolve` includes extension activation status in output.

---

## E2: Runtime Enforcement

### E2.1: Multi-Harness Intercept

**As** an operator,
**I want** `policyctl intercept` to evaluate a single action against resolved policy
**so that** guard scripts gate exec, write, and network actions before execution.

**Acceptance Criteria:**

- Exit code 0 = allow, 2 = deny, 3 = ask.
- Accepts: `--action exec|write|network`, `--command <cmd>`, `--cwd <dir>`,
  `--actor <id>`, `--target-paths <paths>`.
- Calls `evaluate_authorization` and returns `final_decision` plus evaluation detail.
- Appends structured event to audit log (JSONL at configurable path).
- Writes per-run sidecar JSON (`session-sidecar.schema.json` contract).

### E2.2: Risk Scoring for Ask Decisions

**As** an operator,
**I want** risk scores computed for `ask` decisions
**so that** low-risk asks auto-delegate without blocking the agent.

**Acceptance Criteria:**

- Score 0.0–1.0 computed from four weighted factors:
  - `action_type`: exec=0.2, write=0.6, network=0.8 (weight 0.3)
  - `target_scope`: worktree=low, canonical=high, system path=highest (weight 0.3)
  - `command_familiarity`: frequency from audit log history (weight 0.2)
  - `bypass_indicators`: presence of `--force`, `sudo`, `-f` flags (weight 0.2)
- `delegation_eligible` = score < 0.7; `auto_delegate` = score < 0.3.

### E2.3: Headless Delegation

**As** an operator,
**I want** `ask` decisions above confidence threshold routed to a headless agent reviewer
**so that** non-interactive environments resolve policy without blocking.

**Acceptance Criteria:**

- Harness configured via `POLICY_DELEGATE_HARNESS` environment variable.
- Structured prompt sent includes: command, action, cwd, paths, risk score, triggered rule.
- Response expected as JSON: `{decision, reasoning, confidence}`.
- Falls back to `ask` if no harness configured or non-JSON returned.
- Supports Forge minimax and Cursor gemini delegate harnesses.

### E2.4: Runtime Installation

**As** an operator,
**I want** `policyctl install-runtime` to install guard shims and launcher wrappers
**so that** policy enforcement activates with a single command.

**Acceptance Criteria:**

- Installs `codex_exec_guard.sh`, `codex_write_guard.sh`, `codex_network_guard.sh` into
  harness bin directories.
- Patches harness config files (Codex settings, Cursor `.cursorrules`, Claude hooks) with
  policy metadata.
- Creates timestamped backups: `<file>.policy-federation.backup.<timestamp>`.
- `policyctl uninstall-runtime` removes all shims and restores originals.
- Launcher wrappers installed in `~/.local/bin/` for codex, cursor, droid, claude.
- Wrappers export `POLICY_HARNESS`, `POLICY_REPO`, `POLICY_TASK_DOMAIN`, `POLICY_ASK_MODE`
  before delegating to original binary (preserved as `*.policy-federation.real`).

### E2.5: Claude Code Hooks Integration

**As** a Claude Code operator,
**I want** policy enforcement integrated as Claude Code PreToolUse/PostToolUse hooks
**so that** tool calls are intercepted before execution without separate guard scripts.

**Acceptance Criteria:**

- Hooks parse Claude Code JSON hook payloads (tool\_name, tool\_input).
- PreToolUse hook calls `intercept_command` and emits block/allow JSON.
- `policyctl claude-hook install` registers hooks in Claude Code configuration.
- Hook output is valid JSON matching Claude Code hook contract.

---

## E3: Target Compilation

### E3.1: Native Config Compilation

**As** an operator,
**I want** `policyctl compile` to emit harness-native configuration
**so that** unconditional allow/deny rules are enforced natively without shim overhead.

**Acceptance Criteria:**

- Compiles to Codex, Cursor, Factory, and Claude native config formats.
- Rules with runtime-only conditions flagged with `requires_runtime_check: true`.
- `policyctl compile --target codex|cursor|factory|claude` selects output format.
- Compilation is deterministic from the same resolved policy.

---

## E4: Policy Observability and Learning

### E4.1: Audit Log

**As** an operator,
**I want** a structured JSONL audit log of all intercept decisions
**so that** I can review and investigate agent actions.

**Acceptance Criteria:**

- One event per line: timestamp, action, command, cwd, actor, final\_decision, rule\_id,
  risk\_score, source.
- `policyctl audit tail --n <N>` shows the N most recent events.
- Sidecar JSON written per run with full resolution context.

### E4.2: Gap Detection

**As** a security engineer,
**I want** `policyctl gap-detect` to analyze audit logs and detect policy coverage gaps
**so that** policy completeness improves over time.

**Acceptance Criteria:**

- Detects high-frequency `ask` decisions (configurable `min_ask_frequency` threshold).
- Reports `no_rule_matches`: commands with no matching rules.
- Identifies `dead_rules`: rules that never matched in the audit window.
- Flags `priority_conflicts` between rules.

### E4.3: Policy Learning

**As** a security engineer,
**I want** `policyctl learn` to suggest rules from audit decision patterns
**so that** policy evolves based on actual agent behavior.

**Acceptance Criteria:**

- Clusters allowed/denied commands by generalizable prefix (`git commit`, `cargo build`, etc.).
- Suggests `allow` rules for clusters with high allow-rate and confidence >= 0.8.
- Suggests `deny` rules for consistently-denied clusters.
- Outputs YAML-formatted rule suggestions with evidence count and sample commands.
- Writes to `policies/suggestions/` directory.

### E4.4: Policy Diff

**As** a security engineer,
**I want** `policyctl diff` to compare two policy resolutions
**so that** impact of policy changes is visible before deployment.

**Acceptance Criteria:**

- Accepts two scope contexts or two resolution JSON payloads.
- Reports added, removed, and changed rules by rule ID.
- Reports changes in default effects.

---

## E5: Validation and Manifest

### E5.1: Schema Validation

**As** a policy author,
**I want** `policyctl check` to validate policy files against JSON schemas
**so that** malformed policies are caught before deployment.

**Acceptance Criteria:**

- Validates `policies/**/*.yaml` against `schemas/policy.schema.json`.
- Reports errors with file path and field location.
- Exit code 0 = valid, non-zero = invalid.

### E5.2: Policy Manifest

**As** an operator,
**I want** `policyctl manifest` to generate a manifest for a given context
**so that** the effective policy is snapshotted and pinnable for audits.

**Acceptance Criteria:**

- Manifest includes source files, their hashes, contract IDs, versions, and scope chain.
- Manifest is JSON, suitable for CI artifact storage.
- `policyctl verify` compares current resolution against a stored manifest.

---

## E6: CLI UX

### E6.1: Consistent CLI Structure

**As** a CLI user,
**I want** consistent subcommand structure across all operations
**so that** the tool is discoverable and predictable.

**Acceptance Criteria:**

- All commands follow `policyctl <verb> [options]` pattern.
- `--help` works on all commands and subcommands.
- JSON output via `--json` flag on key commands.
- Typer-based CLI with rich help text and typed arguments.
