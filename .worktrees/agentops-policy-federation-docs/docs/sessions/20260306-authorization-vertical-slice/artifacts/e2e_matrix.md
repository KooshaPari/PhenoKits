# e2e Policy Matrix

| harness | case | expected | actual | exit_code | status |
|---|---|---|---|---:|---|
| codex | exec_allow_git_commit_in_worktree | allow | allow | 0 | PASS |
| codex | exec_deny_git_commit_outside_worktree | deny | deny | 2 | PASS |
| codex | exec_ask_network_install | allow | allow | 0 | PASS |
| codex | write_deny_outside_worktree | deny | deny | 2 | PASS |
| codex | network_check_ask_outside_allow_mode | ask | ask | 3 | PASS |
| cursor-agent | exec_allow_git_commit_in_worktree | allow | allow | 0 | PASS |
| cursor-agent | exec_deny_git_commit_outside_worktree | deny | deny | 2 | PASS |
| cursor-agent | exec_ask_network_install | allow | allow | 0 | PASS |
| cursor-agent | write_deny_outside_worktree | deny | deny | 2 | PASS |
| cursor-agent | network_check_ask_outside_allow_mode | ask | ask | 3 | PASS |
| factory-droid | exec_allow_git_commit_in_worktree | allow | allow | 0 | PASS |
| factory-droid | exec_deny_git_commit_outside_worktree | deny | deny | 2 | PASS |
| factory-droid | exec_ask_network_install | allow | allow | 0 | PASS |
| factory-droid | write_deny_outside_worktree | deny | deny | 2 | PASS |
| factory-droid | network_check_ask_outside_allow_mode | ask | ask | 3 | PASS |
| claude-code | exec_allow_git_commit_in_worktree | allow | allow | 0 | PASS |
| claude-code | exec_deny_git_commit_outside_worktree | deny | deny | 2 | PASS |
| claude-code | exec_ask_network_install | allow | allow | 0 | PASS |
| claude-code | write_deny_outside_worktree | deny | deny | 2 | PASS |
| claude-code | network_check_ask_outside_allow_mode | ask | ask | 3 | PASS |
