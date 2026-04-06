# CLAUDE.md - Development Guidelines for portkey

## Project Overview

portkey is a library for building LLM-powered applications with consistent interfaces across different providers (OpenAI, Anthropic, Ollama, etc.).

## Key Files

- `README.md` - Project overview and provider list
- `src/portkey/domain/` - Pure business logic (models, errors)
- `src/portkey/application/` - Use cases and ports
- `src/portkey/infrastructure/` - Provider implementations

## Development Commands

```bash
pip install -e ".[all]"  # Install with dev deps
pytest tests/             # Run tests
mypy src/                # Type check
ruff check src/          # Lint
```

## Architecture Principles

- **Hexagonal Architecture** - Ports & Adapters isolation
- **SOLID** - Single Responsibility, Dependency Inversion via ports
- **DRY** - Shared port interfaces
- Domain layer should have minimal dependencies

## Phenotype Org Rules

- UTF-8 encoding only in all text files
- Worktree discipline: canonical repo stays on `main`
- CI completeness: fix all CI failures before merging
- Never commit agent directories (`.claude/`, `.codex/`, `.cursor/`)
