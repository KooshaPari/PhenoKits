# AGENTS.md — phenotype-config-ts

## Project Overview

- **Name**: phenotype-config-ts
- **Description**: TypeScript implementation of Phenotype configuration management
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-config-ts`
- **Language Stack**: TypeScript
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-config-ts

# Check structure
cat CHARTER.md
ls -la
```

## Architecture

```
phenotype-config-ts/
└── CHARTER.md                # Project charter
```

## Quality Standards

### TypeScript Standards
- **Line length**: 100 characters
- **Formatter**: `prettier`
- **Linter**: `eslint`
- **Type checker**: `tsc --noEmit`

## Git Workflow

### Branch Naming
Format: `config-ts/<type>/<description>`

Examples:
- `config-ts/feat/providers`
- `config-ts/fix/env-parsing`

### Commit Format
```
<type>(config-ts): <description>

Examples:
- feat(config-ts): add YAML provider
- fix(config-ts): resolve nested key access
```

## File Structure

```
phenotype-config-ts/
└── CHARTER.md
```

## CLI Commands

```bash
# Check charter
cat CHARTER.md

# When implemented:
npm install
npm run build
npm test
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Minimal project | Currently charter-only |

## Dependencies

- **Conft/typescript**: Related config project
- **crates/phenotype-config**: Rust implementation reference
- **packages/**: May share types

## Agent Notes

When working in phenotype-config-ts:
1. This is currently minimal (CHARTER only)
2. TypeScript equivalent of `crates/phenotype-config`
3. Check CHARTER.md for design goals
4. Coordinate with Conft/typescript for alignment
