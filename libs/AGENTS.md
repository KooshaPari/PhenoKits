# AGENTS.md — libs

## Project Overview

- **Name**: libs
- **Description**: Shared libraries for the Phenotype ecosystem - adapters, agents, cache, config, and more
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/libs`
- **Language Stack**: Python (primary), with typed interfaces
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to libs
cd /Users/kooshapari/CodeProjects/Phenotype/repos/libs

# Python operations
python -m pytest
python -m mypy .
ruff check .
ruff format .
```

## Architecture

```
libs/
├── pheno_adapters/         # Adapter implementations
├── pheno_agent/            # Agent utilities
├── pheno_cache/            # Caching layer
├── pheno_config/           # Configuration management
├── pheno_governance/       # Governance utilities
├── pheno_http/             # HTTP utilities
├── pheno_llm/              # LLM integrations
├── pheno_retry/            # Retry mechanisms
└── src/                    # Source code root
```

## Quality Standards

### Python Standards
- **Line length**: 100 characters
- **Formatter**: `ruff format`
- **Linter**: `ruff check`
- **Type checker**: `mypy` (strict mode)
- **Tests**: `pytest`

### Code Style
```python
# Use type annotations
def process_data(data: dict[str, Any]) -> Result:
    ...

# Docstrings for public APIs
def public_function():
    """Description of function."""
    ...
```

## Git Workflow

### Branch Naming
Format: `libs/<type>/<description>` or `pheno-<name>/<type>/<description>`

Examples:
- `libs/feat/new-adapter`
- `pheno-cache/fix/ttl-bug`

### Commit Format
```
<type>(<lib>): <description>

Examples:
- feat(pheno-cache): add Redis adapter
- fix(pheno-config): resolve env var parsing
```

## File Structure

```
libs/
├── pheno_<name>/
│   ├── __init__.py
│   ├── pyproject.toml        # Per-library config
│   └── src/
├── src/                      # Shared source
└── [library-specific files]
```

## CLI Commands

```bash
# Quality checks
ruff check .
ruff format .
mypy .
pytest

# Specific library
cd pheno_<name> && python -m pytest
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Import errors | Check `__init__.py` files |
| Type errors | Run `mypy` with strict mode |
| Test failures | Check `pytest` configuration |

## Dependencies

- **Python**: 3.11+
- **pheno_* libraries**: Interdependent
- **crates/**: Rust equivalents ( FFI possible)

## Agent Notes

When working in libs:
1. Each pheno_* directory is a semi-independent library
2. Check for pyproject.toml in subdirectories
3. Keep interfaces compatible with Rust crates where applicable
4. This is the Python equivalent of `crates/` directory
