# AGENTS.md — phenotype-middleware-py

## Project Overview

- **Name**: phenotype-middleware-py
- **Description**: Python middleware components for the Phenotype ecosystem
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-middleware-py`
- **Language Stack**: Python
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-middleware-py

# Check structure
ls -la docs/
```

## Architecture

```
phenotype-middleware-py/
└── docs/                     # Documentation
```

## Quality Standards

### Python Standards
- **Line length**: 100 characters
- **Formatter**: `ruff format` or `black`
- **Linter**: `ruff check`
- **Type checker**: `mypy`
- **Tests**: `pytest`

## Git Workflow

### Branch Naming
Format: `middleware-py/<type>/<description>`

Examples:
- `middleware-py/feat/auth-middleware`
- `middleware-py/fix/logging-context`

## File Structure

```
phenotype-middleware-py/
└── docs/
```

## CLI Commands

```bash
# When implemented:
pip install -e ".[dev]"
pytest
ruff check .
ruff format .
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Minimal project | Currently docs-only |

## Dependencies

- **phenotype-middleware-ts**: TS equivalent
- **crates/phenotype-http-adapter**: Rust equivalent
- **libs/**: Python library ecosystem

## Agent Notes

When working in phenotype-middleware-py:
1. Minimal current state
2. Python equivalent of middleware implementations
3. Coordinate with TS and Rust middleware
