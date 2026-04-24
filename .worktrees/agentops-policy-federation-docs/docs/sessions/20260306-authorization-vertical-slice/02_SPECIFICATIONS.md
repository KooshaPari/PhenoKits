# Specifications

## DSL

`policy.authorization` supports:

- `defaults`: per-action fallback effect
- `rules[]`:
  - `id`
  - `description`
  - `priority`
  - `effect`
  - `actions`
  - `match.command_patterns`
  - optional `match.cwd_patterns`
  - optional `match.actor_patterns`
  - optional `match.target_path_patterns`

## Evaluation semantics

- Higher `priority` wins.
- Ties resolve `deny > ask > allow`.
- If no rules match, action falls back to `defaults[action]`, then `defaults["*"]`, then `ask`.
- Runtime `ask_mode` supports `fail`, `allow`, and `review` as non-interactive resolution modes, with `review` as canonical.

## Compilation semantics

- Unconditional exec rules compile to native target config.
- Conditional exec rules compile to `shim_rules`.
- Non-`exec` actions compile to runtime-only shim requirements.
- All runtime shim entries set `requires_runtime_check: true` and expose `actions`.

## Runtime artifacts

- Guarded execution may emit a sidecar JSON payload with `run_id`, `policy_hash`, `scope_chain`, and audit fields.
- Guarded execution may append JSONL audit events for allow, deny, and ask decisions.
