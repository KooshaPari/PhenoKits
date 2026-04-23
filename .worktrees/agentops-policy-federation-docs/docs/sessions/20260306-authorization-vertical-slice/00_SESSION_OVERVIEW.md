# Session Overview

## Goal

Implement the first executable authorization vertical slice for policy federation:

- typed `policy.authorization` rules
- runtime evaluation for command execution
- target compilation for Codex, Factory Droid, Cursor Agent, and Claude Code
- launcher-level local entrypoint wrapping for Codex, Cursor, Droid, and Claude
- reversible uninstall path for wrappers, hook patches, and launcher restoration

## Success criteria

- layered policies can express `allow`, `deny`, and `ask` for `exec`
- rule priority and path-aware matching are deterministic
- conditional rules are not silently flattened during compilation
- tests cover precedence, worktree scoping, compile output, and Claude hook enforcement
