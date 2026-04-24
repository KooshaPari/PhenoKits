# CLAUDE.md - Development Guidelines for phenotype-cli-extensions

## Project Overview

CLI extensions for the Phenotype ecosystem

## Key Files

-  - Project overview
- See project-specific directories

## Development Commands

```bash
npm install && npm test && npm run build
```

## Architecture Principles

- **SOLID** - Single Responsibility, Dependency Inversion
- **DRY** - Shared abstractions
- **PoLA** - Descriptive error types

## Phenotype Org Rules

- UTF-8 encoding only in all text files
- Worktree discipline: canonical repo stays on `main`
- CI completeness: fix all CI failures before merging
- Never commit agent directories (`.claude/`, `.codex/`, `.cursor/`)
