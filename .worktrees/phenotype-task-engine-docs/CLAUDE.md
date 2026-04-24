# phenotype-task-engine

Phenotype task orchestration engine. Manages task lifecycle, scheduling, and dependency resolution for agent-driven workflows.

## Stack
- Language: Python (primary), TypeScript (secondary)
- Key deps: pytest, pyproject.toml, tsconfig.json
- Status: Archived (see ARCHIVED.md)

## Structure
- `src/`: Python source for task orchestration
- `tests/`: pytest test suite

## Key Patterns
- DAG-based task dependency resolution
- Agent-friendly: tasks are composable and parallelizable
- Clear failure semantics: tasks fail loudly, not silently

## Adding New Functionality
- New task types: add handlers in `src/`
- Tests required before implementation (TDD)
- Run `pytest` to verify
