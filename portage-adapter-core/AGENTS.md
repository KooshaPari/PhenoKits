# AGENTS.md — portage-adapter-core

## Project Overview

- **Name**: portage-adapter-core
- **Description**: Core adapter framework for portage integration
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/portage-adapter-core`
- **Language Stack**: Mixed (src, tests structure)
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/portage-adapter-core

# Check structure
ls -la src/
ls -la tests/
```

## Architecture

```
portage-adapter-core/
├── src/                      # Source code
└── tests/                    # Test suite
```

## Quality Standards

### Code Standards
- **Line length**: 100 characters
- Language-specific formatters
- Tests required for adapters

## Git Workflow

### Branch Naming
Format: `portage-core/<type>/<description>`

Examples:
- `portage-core/feat/adapter-interface`
- `portage-core/fix/adapter-lifecycle`

## File Structure

```
portage-adapter-core/
├── src/
└── tests/
```

## CLI Commands

```bash
# Check structure
ls -la src/
ls -la tests/

# Language-specific commands
# (depends on actual implementation language)
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Adapter errors | Check adapter interface compliance |
| Test failures | Verify test fixtures |

## Dependencies

- **portage/**: Related portage project
- **PhenoLibs/pheno_adapters**: Adapter implementations

## Agent Notes

When working in portage-adapter-core:
1. Core adapter framework
2. Coordinate with portage/ project
3. Adapter pattern implementation
