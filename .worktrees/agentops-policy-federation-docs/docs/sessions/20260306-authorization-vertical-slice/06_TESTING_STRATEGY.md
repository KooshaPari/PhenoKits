# Testing Strategy

## Coverage in this slice

- policy file validation with authorization blocks
- rule precedence for worktree-scoped git writes
- deny override for bypass flags
- compile behavior for unconditional native rules vs conditional shim rules
- intercept exit-code behavior for deny and ask paths
- Codex wrapper execution for an allowed command
- write-path denial outside approved worktrees
- network-path ask handling: `review` as the canonical non-interactive route
- sidecar and audit artifact emission
- Claude compile output and `PreToolUse` hook decisions

## Commands

- `python -m unittest tests.unit.test_authorization`
- `python -m policy_federation.cli resolve --harness codex --domain devops --repo thegent`
- `python -m policy_federation.cli evaluate --harness codex --domain devops --repo thegent --action exec --command "git commit -m test" --cwd /Users/kooshapari/CodeProjects/Phenotype/repos/thegent-wtrees/demo`
- `python -m policy_federation.cli intercept --harness codex --domain devops --repo thegent --action exec --command "git commit --no-verify -m test" --cwd /Users/kooshapari/CodeProjects/Phenotype/repos/thegent-wtrees/demo`
- `python -m policy_federation.cli write-check --harness codex --domain devops --repo thegent --target-path /tmp/file.txt --cwd /tmp`
- `python -m policy_federation.cli network-check --harness codex --domain devops --repo thegent --command "curl https://example.com" --cwd /tmp`
- `python scripts/validate_e2e_matrix.py`

Recent matrix evidence
- `docs/sessions/20260306-authorization-vertical-slice/artifacts/e2e_matrix.json`
- `docs/sessions/20260306-authorization-vertical-slice/artifacts/e2e_matrix.md`
- `./scripts/runtime/codex_exec_guard.sh echo ok`
- `python -m policy_federation.cli compile --target claude-code --harness claude-code --domain devops --repo thegent`
- `printf '%s\n' '{"tool_name":"Bash","tool_input":{"command":"git commit --no-verify -m test"},"cwd":"/Users/kooshapari/CodeProjects/Phenotype/repos/thegent-wtrees/demo"}' | ./scripts/runtime/claude_pretool_hook.py`
- `printf '%s\n' '{"tool_name":"NotebookEdit","tool_input":{"notebook_path":"/tmp/test.ipynb"},"cwd":"/tmp"}' | POLICY_REPO=thegent POLICY_TASK_DOMAIN=devops ./scripts/runtime/claude_pretool_hook.py`
- `~/.local/bin/codex --version`
- `~/.local/bin/claude --version`
- `python -m policy_federation.cli uninstall-runtime`
- `python -m policy_federation.cli install-runtime`
