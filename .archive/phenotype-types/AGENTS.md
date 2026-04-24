# AGENTS.md — phenotype-types

## Project Overview

- **Name**: phenotype-types
- **Description**: Shared type definitions for the Phenotype ecosystem
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-types`
- **Language Stack**: TBD (documentation current)
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-types

# Check docs
ls -la docs/
```

## Architecture

```
phenotype-types/
└── docs/                     # Type documentation
```

## Quality Standards

### Type Definitions
- Clear type hierarchies
- Documentation for each type
- Cross-language type alignment

## Git Workflow

### Branch Naming
Format: `types/<type>/<description>`

Examples:
- `types/feat/domain-models`
- `types/fix/type-hierarchy`

## File Structure

```
phenotype-types/
└── docs/
```

## CLI Commands

```bash
# Check docs
cat docs/*.md
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Minimal project | Currently documentation |

## Dependencies

- **packages/types**: TypeScript types package
- **crates/phenotype-domain**: Rust domain types

## Agent Notes

When working in phenotype-types:
1. Central type definitions
2. Coordinate with packages/types
3. Align with crates/phenotype-domain
