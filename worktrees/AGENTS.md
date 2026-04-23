# AGENTS.md — worktrees

## Project Overview

- **Name**: worktrees
- **Description**: Git worktree management directory for parallel development
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/worktrees`
- **Language Stack**: N/A (git worktrees)
- **Published**: N/A (development tool)

## Quick Start Commands

```bash
# Navigate to worktrees
cd /Users/kooshapari/CodeProjects/Phenotype/repos/worktrees

# List worktrees
ls -la

# Check specific worktree
cd <worktree-name> && git status
```

## Architecture

```
worktrees/
├── flowra/                   # Flowra worktree
├── phenoctl-unified/         # Phenoctl unified worktree
├── phenotype/                # Phenotype worktree
├── phenotype-agents/         # Agents worktree
├── phenotype-cache-unified/  # Cache unified worktree
├── phenotype-integration-plugins/  # Integration plugins
├── phenotype-router-shared/   # Router shared worktree
├── phenotype-toolkit/        # Toolkit worktree
├── phenotype-tui-widgets/    # TUI widgets worktree
├── seedloom/                 # Seedloom worktree
├── thegent-config-rust/      # thegent config Rust
└── thegent-phench-rust/      # thegent phench Rust
```

## What are Worktrees?

Git worktrees allow checking out multiple branches simultaneously without cloning the repository multiple times. Each worktree is a separate working directory linked to the same git repository.

## Quality Standards

- Worktrees should be kept clean
- Remove worktrees when done: `git worktree remove <name>`
- Don't commit worktree directories

## Git Workflow

### Creating Worktrees
```bash
# From repo root
git worktree add .worktrees/<name> -b <branch>
# OR
git worktree add worktrees/<name> <branch>
```

### Removing Worktrees
```bash
git worktree remove worktrees/<name>
```

## File Structure

```
worktrees/
├── <worktree-name>/
│   ├── .git                  # Git link file
│   └── [branch content]
└── ...
```

## CLI Commands

```bash
# List all worktrees
git worktree list

# Add new worktree
git worktree add worktrees/<name> -b <branch>

# Remove worktree
git worktree remove worktrees/<name>

# Prune stale worktrees
git worktree prune
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Worktree not updating | Check if branch exists |
| Cannot remove | Ensure no uncommitted changes |
| Stale worktrees | Run `git worktree prune` |

## Dependencies

- Git 2.5+ (for worktree support)
- Parent repository at root

## Agent Notes

When encountering worktrees/:
1. These are development worktrees, not the main repo
2. Each worktree is a separate branch checkout
3. Worktrees share the same git history
4. Can be removed when feature work is complete
5. Do not modify the `.git` file in worktrees
