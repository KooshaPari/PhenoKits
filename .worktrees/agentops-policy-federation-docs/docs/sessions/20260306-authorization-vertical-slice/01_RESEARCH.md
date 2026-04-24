# Research

## In-repo findings

- Existing resolver already merges deterministic scope layers and emits a `policy_hash`.
- Existing schema treated `policy` as an untyped object, so executable semantics had no contract.
- Existing harness policy files already modeled runtime metadata, making `policy.authorization` the correct insertion point.

## External findings used

- Cursor, Claude Code, Factory Droid, and Codex expose different command-permission surfaces.
- Native harness configs are not expressive enough for path-aware command authorization.
- Conditional rules therefore need an explicit runtime interceptor/shim path rather than silent lossy compilation.
- Claude Code `PreToolUse` hooks accept JSON on stdin and can return structured permission decisions, which makes them the correct live enforcement point for Bash, write/edit, and network-like tools without replacing existing hook chains.
