~There are ~3 other repos that are identical in scope i bleieve, eval. one is policy-contract

# AgentOps Policy Federation

Single source of truth for agent/devops governance, scope federation, and runtime extensions.

## Scope contract

- system
- user
- harness
- repo
- task_domain
- task_instance
- task_overlay (optional)

Resolved policy is deterministic and includes a content hash for auditability.

## Repository layout

- `policies/`: YAML policy payloads by scope.
- `extensions/`: registry and extension manifests.
- `schemas/`: JSON schema contracts.
- `cli/`: policy resolution tools.
- `contracts/`: run/session artifacts contracts.
- `docs/`: governance + extension authoring.
- `scripts/`: common maintenance checks.

## Quick bootstrap

- `cd agentops-policy-federation`
- create/modify policy files in `policies/`
- `./scripts/resolve_check.sh`

## Supported scope model

- `system`: root platform defaults (`policies/system/*`)
- `user`: user/org defaults (`policies/user/*`)
- `harness`: agent runtime profile (`policies/harness/*`)
- `repo`: project profile (`policies/repo/*`)
- `task_domain`: workflow class profile (`policies/task-domain/*`)
- `task_instance`: per-task optional profile (`policies/task-instance/*`)
- `task_overlay`: optional manual overlay (`policies/task-overlay/*`)

Resolution order is fixed and deterministic:
`system -> user -> harness -> repo -> task_domain -> task_instance -> task_overlay`.

## CLI usage

From repo root:

- `python -m policy_federation.cli resolve --harness codex --domain devops --repo thegent`
- `python -m policy_federation.cli resolve --harness codex --domain query --instance deploy-window --overlay emergency`
- `python -m policy_federation.cli evaluate --harness codex --domain devops --repo thegent --action exec --command "git commit -m test" --cwd /Users/kooshapari/CodeProjects/Phenotype/repos/thegent-wtrees/demo`
- `python -m policy_federation.cli compile --target codex --harness codex --domain devops --repo thegent`
- `python -m policy_federation.cli intercept --harness codex --domain devops --repo thegent --action exec --command "git commit --no-verify -m test" --cwd /Users/kooshapari/CodeProjects/Phenotype/repos/thegent-wtrees/demo`
- `python -m policy_federation.cli write-check --harness codex --domain devops --repo thegent --target-path /tmp/file.txt --cwd /tmp`
- `python -m policy_federation.cli network-check --harness codex --domain devops --repo thegent --command "curl https://example.com" --cwd /tmp`
- `python -m policy_federation.cli install-runtime`
- `python -m policy_federation.cli uninstall-runtime`
- `./scripts/runtime/codex_exec_guard.sh echo ok`
- `./scripts/runtime/codex_write_guard.sh /tmp/file.txt`
- `./scripts/runtime/codex_network_guard.sh curl https://example.com`
- `python -m policy_federation.cli check`
- `python -m policy_federation.cli manifest --harness codex --domain devops --repo thegent`

Installed script entrypoint:
- `policyctl resolve --harness codex --domain devops --repo thegent`
- `policyctl review --harness codex --domain devops --repo thegent --action network --command "curl https://example.com" --cwd /tmp`

`policyctl check` validates a single file if `path` is provided, or all files in `policies/**` when omitted.

## Authorization DSL

- `policy.authorization.defaults` defines fallback effects per action, for example `exec: ask` (canonical resolution is non-interactive via `--ask-mode review`).
- `policy.authorization.rules` defines ordered rules with:
  - `effect`: `allow`, `deny`, or `ask` (an `ask` outcome is resolved by `--ask-mode`; canonical is `review`)
  - `actions`: `exec` compiles to native target allow/deny/ask where feasible; non-`exec` actions compile as runtime shims
  - `priority`: higher priority wins; ties resolve `deny > ask > allow`
  - `match.command_patterns`
  - optional `match.cwd_patterns`, `match.actor_patterns`, `match.target_path_patterns`
- `policyctl evaluate` evaluates one concrete execution context against the resolved policy.
- `policyctl compile` emits target-native config for unconditional command rules and explicitly reports runtime-only shim rules.
- Shim rows include `requires_runtime_check` and normalized `actions` metadata for downstream runtime consumers.
- `policyctl intercept` evaluates one concrete command context and exits `0` for allow, `2` for deny, and `3` for ask when `--ask-mode review` cannot resolve the decision.
- `--ask-mode review` routes ambiguous decisions through the headless Codex reviewer as the canonical non-interactive resolution path.
- `policyctl` defaults `--ask-mode` to `review` for `intercept`, `exec`, `write-check`, and `network-check`.
- `policyctl exec -- ...` is a thin guarded executor for shell wrappers.
- `policyctl write-check` evaluates write intent against target paths.
- `policyctl network-check` evaluates network intent against the requested command.
- `policyctl install-runtime` installs wrapper shims into `~/.cursor/bin`, `~/.factory/bin`, `~/.codex/bin`, and `~/.claude/bin`, then patches the corresponding local config files with runtime metadata and backups.
- `policyctl uninstall-runtime` removes managed wrapper shims, removes managed config metadata, strips the managed Claude hooks, and restores any launcher wrappers that were replaced under `~/.local/bin`.
- `policyctl install-runtime` also installs launcher wrappers into `~/.local/bin` for `codex`, `cursor`, `droid`, and `claude`, preserving prior local launchers as `*.policy-federation.real` when present.

## Codex runtime wrapper

- `scripts/runtime/codex_exec_guard.sh` is the first runnable harness shim.
- `scripts/runtime/codex_write_guard.sh` and `scripts/runtime/codex_network_guard.sh` extend the same live policy path to write and network actions.
- `scripts/runtime/codex_runtime_manifest.json` is the concrete launcher/hook artifact for Codex-side integration.
- `scripts/runtime/cursor_runtime_manifest.json` and `scripts/runtime/factory_runtime_manifest.json` provide the same integration contract for Cursor and Factory.
- `scripts/runtime/claude_runtime_manifest.json` and `scripts/runtime/claude_pretool_hook.py` provide the live Claude Code hook bridge for `Bash`, `Write`, `Edit`, `MultiEdit`, `NotebookEdit`, `WebFetch`, and `WebSearch`.
- The exec guard resolves policy from this repo, evaluates the requested command, and only executes when the decision is allowed.
- Environment knobs:
- `POLICY_HARNESS`
- `POLICY_TASK_DOMAIN`
- `POLICY_REPO`
- `POLICY_ASK_MODE`
- `POLICY_REVIEW_MODEL`
- `POLICY_REVIEW_BIN`
- `POLICY_ACTOR`
- `POLICY_SIDECAR_PATH`
- `POLICY_AUDIT_LOG_PATH`

## Installed local config surfaces

- Cursor: `~/.cursor/cli-config.json`
- Factory: `~/.factory/settings.json`
- Codex: `~/.codex/config.json` and `~/.codex/config.toml`
- Claude Code: `~/.claude/settings.json`

Each install run creates timestamped backups beside the original files.

## Installed launcher wrappers

- `~/.local/bin/codex`
- `~/.local/bin/cursor`
- `~/.local/bin/droid`
- `~/.local/bin/claude`

Each wrapper exports default policy context before delegating to the preserved original launcher or upstream binary:

- `POLICY_HARNESS`
- `POLICY_REPO`
- `POLICY_TASK_DOMAIN`
- `POLICY_ASK_MODE=review`

## Extension behavior

- `extensions/registry.yaml` declares optional extensions.
- `scope_selector.includes` uses shell-style glob matching against scope labels in the resolution chain.
- `extensions` block in the resolve payload includes `enabled` and `disabled` entries and reasons for disablement.

## Next implementation steps

1. Add real extension entrypoint implementations (`policy_federation.extensions.*`).
2. Add machine-generated `policy_version` manifests for immutable audit.
3. Hook `policyctl resolve` into runtime start (`thegent`, `Factory Droid`, `Cursor-agent`) and attach `policy_hash` + `scope_chain` to run metadata.
