# phenotype-middleware-py — AGENTS.md

## Project Overview

Python middleware patterns following hexagonal architecture.

## Agent Rules

1. **Read CLAUDE.md first** before making changes
2. **Test first** - write tests before implementation
3. **Lint clean** - all lint checks must pass before PR
4. **UTF-8 only** - all text files must use UTF-8 encoding

## Quality Gates

```bash
pip install -e ".[dev]"
pytest
ruff check .
mypy src/
```

## Branching

- Feature branches: `feat/<feature-name>`
- Bug fixes: `fix/<issue-description>`
- Main branch: `main`

## Architecture

Follow hexagonal architecture:
- **Domain** - Pure business logic, no external dependencies
- **Ports** - Interface traits defining boundaries
- **Adapters** - Implementations of ports

## See Also

- **CLAUDE.md**: `./CLAUDE.md`
- **PRD.md**: `./PRD.md`
