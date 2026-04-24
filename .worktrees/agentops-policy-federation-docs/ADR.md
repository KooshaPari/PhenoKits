# Architecture Decision Records — AgentOps Policy Federation

**Version:** 1.1.0

---

## ADR-001: Python CLI with Typer/Click

**Status:** Accepted
**Context:** Policy tooling requires a CLI for resolution, evaluation, compilation, and runtime
management. The team uses Python (uv) as the primary scripting language.
**Decision:** Python CLI (`policyctl`) built on Typer (wraps Click), packaged under
`policy_federation`, invokable as `python -m policy_federation.cli` or via `policyctl` entrypoint.
**Consequences:**
- Rich subcommand CLI with typed arguments and auto-generated `--help`.
- Installable via `pip install -e .` or `uv pip install -e .`.
- Ruff enforces style; pytest for testing; uv.lock pins dependencies.
**Alternatives considered:** Click directly (less ergonomic typing); Go CLI (different stack from
policy engine Python code); shell-only (not testable).
**Code location:** `cli/src/policy_federation/cli.py`, `cli/src/policy_federation/cli_parsers.py`

---

## ADR-002: Seven-Scope Fixed Resolution Order

**Status:** Accepted
**Context:** Policies exist at multiple granularities: org-wide system policy, per-user config,
per-harness config, per-repo policy, per-task-domain, per-task-instance, per-run overlay.
**Decision:** Fixed scope chain (system > user > harness > repo > task\_domain > task\_instance >
task\_overlay). Later scopes override earlier ones. Chain is immutable in code.
**Consequences:**
- Zero ambiguity about effective policy; no runtime scope discovery.
- `task_overlay` is always highest priority, supporting per-run ad-hoc overrides.
- Adding a new scope requires a version bump and migration.
**Alternatives considered:** Dynamic scope graph (too complex to audit); scope weights (numeric
priority, ambiguous when scopes are unrelated).
**Code location:** `cli/src/policy_federation/resolver_layers.py`

---

## ADR-003: Authorization DSL with Priority-Based Conflict Resolution

**Status:** Accepted
**Context:** Agent actions span a spectrum from safe (read-only queries) to dangerous (destructive
writes to canonical repos, network calls). Policy must be granular and auditable.
**Decision:** Authorization rules have `effect` (allow/deny/ask), `priority` (int),
`actions` list, and a `match` object with fnmatch patterns on command, cwd, actor, target\_paths.
Ties at equal priority resolve: deny > ask > allow. A default effect applies when no rule matches.
**Consequences:**
- Expressive policy; operators can layer global deny rules atop permissive defaults.
- `ask` mode enables non-binary decisions; routes to headless delegate or blocks for human review.
- Priority conflicts are surfaced in gap detection reports.
**Code location:** `cli/src/policy_federation/authorization.py`

---

## ADR-004: Shell-Based Runtime Guards

**Status:** Accepted
**Context:** Agent harnesses (Codex, Cursor, Factory) intercept execution via configurable
guard scripts. A Python subprocess call from every guarded action would be acceptable latency.
**Decision:** Bash guard scripts (`codex_exec_guard.sh`, `codex_write_guard.sh`,
`codex_network_guard.sh`) that invoke `policyctl intercept` and return exit codes 0/2/3.
**Consequences:**
- Portable: Bash runs anywhere Python is installed.
- Latency: each guarded action pays one `policyctl` subprocess + policy resolution cost.
- Easily audited: guards are human-readable Bash.
**Alternatives considered:** Native harness plugins (harness-specific, fragile to updates);
eBPF-based interception (too invasive, requires root).
**Code location:** `cli/src/policy_federation/runtime_launchers.py`,
`cli/src/policy_federation/interceptor.py`

---

## ADR-005: YAML Policy Files with JSON Schema Validation

**Status:** Accepted
**Context:** Policies are authored by operators, must be version-controllable, and diffable in
code review. Machine validation is required to prevent malformed policies from silently breaking
enforcement.
**Decision:** YAML files in `policies/<scope>/<label>.yaml`. JSON schema at
`schemas/policy.schema.json` validates all policy files. `policyctl check` runs validation.
**Consequences:**
- Human-readable and diff-friendly in PRs.
- Schema validation catches structural errors early.
- Policy diffs are legible in code review.
**Alternatives considered:** TOML (less expressive for nested structures); JSON (harder to
author; no comments); HCL (non-standard).
**Code location:** `policies/`, `schemas/policy.schema.json`,
`cli/src/policy_federation/validate.py`

---

## ADR-006: Launcher Wrapper Pattern

**Status:** Accepted
**Context:** Agent harnesses need default policy context (harness name, repo, task domain)
injected before startup. Environment variables are the simplest harness-agnostic mechanism.
**Decision:** Install per-harness wrapper scripts in `~/.local/bin/`. Wrappers export
`POLICY_HARNESS`, `POLICY_REPO`, `POLICY_TASK_DOMAIN`, `POLICY_ASK_MODE` then exec the
real binary (preserved as `<binary>.policy-federation.real`).
**Consequences:**
- Transparent to users; original harness behavior is preserved.
- Uninstall fully reversible via `policyctl uninstall-runtime`.
- Config file backups prevent data loss during installation.
**Code location:** `cli/src/policy_federation/runtime_launchers.py`,
`cli/src/policy_federation/runtime_config_patches.py`

---

## ADR-007: Four-Factor Risk Scoring for Ask Decisions

**Status:** Accepted
**Context:** In `ask` mode, blocking the agent for every uncertain action degrades developer
experience. Risk scoring allows low-risk asks to auto-delegate without interruption.
**Decision:** Weighted risk score 0.0-1.0 from action type (0.3), target scope (0.3),
command familiarity from audit history (0.2), and bypass indicators (0.2). Score < 0.3 =
auto-delegate; score < 0.7 = delegation-eligible; score >= 0.7 = require explicit review.
**Consequences:**
- Worktree-scoped actions (low scope score) rarely require human review.
- Canonical repo writes and network calls have high base scores.
- Audit log history reduces risk for frequently-approved commands.
**Code location:** `cli/src/policy_federation/risk.py`

---

## ADR-008: JSONL Audit Log with Per-Run Sidecar

**Status:** Accepted
**Context:** Policy enforcement decisions must be auditable and searchable. Audit data drives
gap detection and policy learning.
**Decision:** Append-only JSONL audit log (one event per line) at a configurable path. Each
intercept event also writes a per-run sidecar JSON file conforming to
`contracts/session-sidecar.schema.json`.
**Consequences:**
- JSONL is streamable and grep-able.
- Sidecar captures full resolution context for post-mortem analysis.
- Audit log growth must be managed externally (rotation not built in).
**Code location:** `cli/src/policy_federation/runtime_artifacts.py`,
`contracts/session-sidecar.schema.json`

---

## ADR-009: Audit-Driven Policy Learning

**Status:** Accepted
**Context:** Policy gaps emerge over time as agents perform actions not anticipated during
initial policy authoring. Manual gap identification does not scale.
**Decision:** `policyctl gap-detect` analyzes the audit log for high-frequency `ask` decisions,
dead rules, and no-match commands. `policyctl learn` clusters allowed/denied commands by
generalizable prefix and emits YAML rule suggestions to `policies/suggestions/`.
**Consequences:**
- Policy evolves based on real agent behavior rather than guesswork.
- Suggestions are non-authoritative; operators review before promoting to active policy.
- High-confidence clusters (>= 0.8) are candidates for immediate promotion.
**Code location:** `cli/src/policy_federation/gap_detector.py`,
`cli/src/policy_federation/learner.py`
