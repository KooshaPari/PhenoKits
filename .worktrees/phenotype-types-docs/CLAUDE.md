# phenotype-types

Shared TypedDict and dataclass definitions for Phenotype Python packages. Single source of truth for cross-package type contracts.

## Stack
- Language: Python 3.12+
- Key deps: no runtime dependencies (pure type definitions)
- Package: `phenotype_types/`

## Structure
- `phenotype_types/`: Python package with TypedDict and dataclass definitions
  - Organized by domain (e.g., tasks, research, docs, config)

## Key Patterns
- Pure type definitions only — no business logic
- Use `TypedDict` for dict-shaped data, `dataclass` for object-shaped data
- All types exported from `phenotype_types/__init__.py`
- Imported by other Phenotype Python packages as a shared dependency

## Adding New Functionality
- Add new type definitions in the appropriate submodule under `phenotype_types/`
- Export from `__init__.py`
- Keep backward compatible; use `Optional` fields for additive changes
