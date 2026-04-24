# phenotype-docs-engine

Phenotype documentation generation engine. Generates structured documentation from code and specs using Python + TypeScript tooling.

## Stack
- Language: Python (primary), TypeScript (secondary)
- Key deps: pytest, pyproject.toml, tsconfig.json, setup.py
- Status: Archived (see ARCHIVED.md)

## Structure
- `src/`: Python source for doc generation logic
- `tests/`: pytest test suite
- `tsconfig.json`: TypeScript compilation config for any TS components

## Key Patterns
- Python-first implementation with TypeScript for any frontend/tooling
- pytest for all Python tests
- Output: structured Markdown docs from source analysis

## Adding New Functionality
- New generators: add modules in `src/`
- Tests required for all new generators
- Run `pytest` to verify, `ruff check .` to lint
