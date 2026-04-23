# Getting Started

## Installation

Install from PyPI using pip or uv:

```bash
pip install phenotype-middleware
# or
uv add phenotype-middleware
```

For development:

```bash
pip install -e ".[dev]"
```

## Project Layout

The library follows hexagonal (ports-and-adapters) architecture:

| Layer | Path |
|-------|------|
| Domain | `src/phenotype_middleware/domain/` |
| Application | `src/phenotype_middleware/application/` |
| Ports | `src/phenotype_middleware/ports/` |
| Infrastructure | `src/phenotype_middleware/infrastructure/` |

## Development

Run tests and linting:

```bash
pytest
ruff check src tests
mypy src
```

## Architecture

See `adr/ADR-001-architecture.md` in the repository for full architecture decision records.

## Requirements

- Python >= 3.10
