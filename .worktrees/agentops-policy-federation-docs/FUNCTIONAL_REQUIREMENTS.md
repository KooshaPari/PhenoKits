# Functional Requirements — AgentOps Policy Federation

**Version:** 1.1.0
**Traces to:** `PRD.md`

---

## FR-RES: Resolution

- **FR-RES-001:** The system SHALL resolve policy through a fixed seven-scope chain:
  system > user > harness > repo > task\_domain > task\_instance > task\_overlay.
  Traces to: E1.1
- **FR-RES-002:** The resolved payload SHALL include `policy_hash` (SHA-256 of sorted merged
  policy JSON) for integrity verification.
  Traces to: E1.1
- **FR-RES-003:** `policyctl resolve` SHALL accept `--harness`, `--repo`, `--domain`,
  `--instance`, and `--overlay` flags and print resolved JSON to stdout.
  Traces to: E1.1
- **FR-RES-004:** Each policy document SHALL support a `_extends` field containing a relative
  path to a base document; the base is merged before scope overrides are applied.
  Traces to: E1.1
- **FR-RES-005:** The system SHALL support merge strategies: `merge_map`, `replace`, `append`,
  `append_unique`. Unknown strategies SHALL fall back to `merge_map` and record a conflict.
  Traces to: E1.1
- **FR-RES-006:** The resolved payload SHALL include `scope_chain`, `source_files`,
  `contract_ids`, `contract_versions`, and `conflicts`.
  Traces to: E1.1

## FR-AUTH: Authorization

- **FR-AUTH-001:** Authorization rules SHALL support effects: `allow`, `deny`, `ask`.
  Traces to: E1.2
- **FR-AUTH-002:** At equal priority, conflict resolution SHALL apply: deny > ask > allow.
  Traces to: E1.2
- **FR-AUTH-003:** Rules SHALL support match fields: `command_patterns`, `cwd_patterns`,
  `actor_patterns`, `target_path_patterns` using fnmatch glob syntax.
  Traces to: E1.2
- **FR-AUTH-004:** `policyctl evaluate` SHALL evaluate a concrete execution context (action,
  command, cwd, actor, target\_paths) against resolved policy and print result JSON.
  Traces to: E1.2
- **FR-AUTH-005:** The `authorization_summary` in the resolved payload SHALL include
  `default_effects` per harness and `rule_count`.
  Traces to: E1.2

## FR-EXT: Extensions

- **FR-EXT-001:** Extensions SHALL be declared in `extensions/registry.yaml` with `id`,
  `enabled`, `scope_selector`, and `description` fields.
  Traces to: E1.3
- **FR-EXT-002:** Scope selectors SHALL use fnmatch glob matching against the scope label.
  Traces to: E1.3
- **FR-EXT-003:** The resolve output SHALL include an `extensions` key listing active extensions.
  Traces to: E1.3

## FR-RT: Runtime Intercept

- **FR-RT-001:** `policyctl intercept` SHALL exit 0 (allow), 2 (deny), 3 (ask).
  Traces to: E2.1
- **FR-RT-002:** Runtime guard Bash scripts SHALL resolve policy via `policyctl intercept` and
  gate exec, write, and network actions before delegating to the real binary.
  Traces to: E2.1
- **FR-RT-003:** Each intercept invocation SHALL append a JSONL event to the configured audit
  log file including timestamp, action, command, cwd, actor, final\_decision, rule\_id, source.
  Traces to: E2.1, E4.1
- **FR-RT-004:** `policyctl install-runtime` SHALL install shims into harness bin directories
  and patch harness config files.
  Traces to: E2.4
- **FR-RT-005:** `policyctl uninstall-runtime` SHALL reverse all managed changes and restore
  originals from backups.
  Traces to: E2.4
- **FR-RT-006:** `policyctl install-runtime` SHALL create timestamped backups
  (`<file>.policy-federation.backup.<timestamp>`) before modifying any config file.
  Traces to: E2.4
- **FR-RT-007:** Launcher wrappers SHALL export `POLICY_HARNESS`, `POLICY_REPO`,
  `POLICY_TASK_DOMAIN`, `POLICY_ASK_MODE` before delegating to the original binary.
  Traces to: E2.4
- **FR-RT-008:** Original launcher binaries SHALL be preserved as `*.policy-federation.real`.
  Traces to: E2.4

## FR-RISK: Risk Scoring

- **FR-RISK-001:** The system SHALL compute a risk score 0.0-1.0 for every `ask` decision
  from four weighted factors: action\_type (0.3), target\_scope (0.3),
  command\_familiarity (0.2), bypass\_indicators (0.2).
  Traces to: E2.2
- **FR-RISK-002:** Risk score < 0.7 SHALL set `delegation_eligible: true`; score < 0.3 SHALL
  set `auto_delegate: true`.
  Traces to: E2.2
- **FR-RISK-003:** Worktree paths (patterns: `*-wtrees/*`, `*/.worktrees/*`, `*/worktrees/*`)
  SHALL receive lower target\_scope scores than canonical repo paths.
  Traces to: E2.2

## FR-DEL: Headless Delegation

- **FR-DEL-001:** The system SHALL route `ask` decisions to a headless agent reviewer when
  `POLICY_DELEGATE_HARNESS` is set.
  Traces to: E2.3
- **FR-DEL-002:** The delegate prompt SHALL include command, action, cwd, target\_paths,
  risk\_score, risk\_factors, triggered rule ID, and scope\_chain.
  Traces to: E2.3
- **FR-DEL-003:** The delegate SHALL respond with JSON `{decision, reasoning, confidence}`.
  Non-JSON responses SHALL fall back to `ask`.
  Traces to: E2.3
- **FR-DEL-004:** The system SHALL support Forge minimax and Cursor gemini delegate harnesses.
  Traces to: E2.3

## FR-HOOKS: Claude Code Hooks

- **FR-HOOKS-001:** The system SHALL provide a PreToolUse hook that calls `intercept_command`
  and emits a JSON block/allow decision.
  Traces to: E2.5
- **FR-HOOKS-002:** `policyctl claude-hook install` SHALL register hooks in Claude Code
  configuration.
  Traces to: E2.5
- **FR-HOOKS-003:** Hook output SHALL conform to the Claude Code hook JSON contract.
  Traces to: E2.5

## FR-CMP: Compilation

- **FR-CMP-001:** `policyctl compile` SHALL emit target-native configuration for rules that
  can be enforced without a runtime shim.
  Traces to: E3.1
- **FR-CMP-002:** Rules requiring runtime context (CWD matching, actor patterns) SHALL be
  flagged with `requires_runtime_check: true` in compiled output.
  Traces to: E3.1
- **FR-CMP-003:** `policyctl compile` SHALL accept `--target codex|cursor|factory|claude`.
  Traces to: E3.1

## FR-AUDIT: Audit and Observability

- **FR-AUDIT-001:** `policyctl audit tail --n <N>` SHALL display the N most recent audit events.
  Traces to: E4.1
- **FR-AUDIT-002:** Per-run sidecar JSON SHALL be written alongside each audit event conforming
  to `contracts/session-sidecar.schema.json`.
  Traces to: E4.1

## FR-GAP: Gap Detection

- **FR-GAP-001:** `policyctl gap-detect` SHALL identify commands that triggered `ask` decisions
  above a configurable frequency threshold (`min_ask_frequency`).
  Traces to: E4.2
- **FR-GAP-002:** The gap report SHALL include `high_frequency_asks`, `no_rule_matches`,
  `dead_rules`, and `priority_conflicts`.
  Traces to: E4.2

## FR-LEARN: Policy Learning

- **FR-LEARN-001:** `policyctl learn` SHALL cluster audit events by generalizable command prefix
  and suggest allow/deny rules for stable clusters with confidence >= 0.8.
  Traces to: E4.3
- **FR-LEARN-002:** Suggested rules SHALL be emitted as YAML with `evidence_count` and
  `sample_commands` fields.
  Traces to: E4.3
- **FR-LEARN-003:** Suggestions SHALL be written to `policies/suggestions/` directory.
  Traces to: E4.3

## FR-DIFF: Policy Diff

- **FR-DIFF-001:** `policyctl diff` SHALL accept two scope contexts or two JSON payloads and
  report added, removed, and changed rules.
  Traces to: E4.4
- **FR-DIFF-002:** The diff output SHALL include changes in default effects.
  Traces to: E4.4

## FR-VAL: Validation and Manifest

- **FR-VAL-001:** `policyctl check` SHALL validate `policies/**/*.yaml` against
  `schemas/policy.schema.json` and report file path and field location for each error.
  Traces to: E5.1
- **FR-VAL-002:** `policyctl manifest` SHALL generate a JSON manifest containing source files,
  hashes, contract IDs, versions, and scope chain for a given context.
  Traces to: E5.2
- **FR-VAL-003:** `policyctl verify` SHALL compare current resolution against a stored manifest
  and exit non-zero if they differ.
  Traces to: E5.2

## FR-CLI: CLI UX

- **FR-CLI-001:** All subcommands SHALL follow the pattern `policyctl <verb> [options]`.
  Traces to: E6.1
- **FR-CLI-002:** `--help` SHALL be available on all commands and subcommands.
  Traces to: E6.1
- **FR-CLI-003:** Key commands SHALL support a `--json` flag for machine-readable output.
  Traces to: E6.1
