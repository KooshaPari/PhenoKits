# AGENTS.md — PhenoSchema

## Project Overview

- **Name**: PhenoSchema
- **Description**: Schema management and validation - XDD (eXternal Data Definition) library
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoSchema`
- **Language Stack**: Minimal current state, references to XDD
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to PhenoSchema
cd /Users/kooshapari/CodeProjects/Phenotype/repos/PhenoSchema

# Check structure
ls -la
cat README.md 2>/dev/null || echo "No README"

# Check pheno-xdd-lib
cd pheno-xdd-lib && ls -la
```

## Architecture

```
PhenoSchema/
├── pheno-xdd-lib/             # XDD library
└── Schemaforge/               # SchemaForge integration
    └── AGENTS.md              # Has own agent rules
```

## Quality Standards

- Follow SchemaForge patterns
- XDD (eXternal Data Definition) compliance

## Git Workflow

### Branch Naming
Format: `phenoschema/<type>/<description>`

Examples:
- `phenoschema/feat/xdd-parser`
- `phenoschema/fix/schema-validation`

### Commit Format
```
<type>(phenoschema): <description>

Examples:
- feat(xdd): add JSON schema export
- fix(schemaforge): resolve type resolution
```

## File Structure

```
PhenoSchema/
├── pheno-xdd-lib/
└── Schemaforge/
    └── AGENTS.md
```

## CLI Commands

```bash
# Check components
cd pheno-xdd-lib && ls -la
cd Schemaforge && cat AGENTS.md
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Schema validation fails | Check SchemaForge documentation |
| XDD issues | Verify XDD specification compliance |

## Dependencies

- **SchemaForge**: Primary schema framework (has own AGENTS.md)
- **bed/schemaforge**: Submodule reference
- **XDD spec**: eXternal Data Definition specification

## Agent Notes

When working in PhenoSchema:
1. Check `Schemaforge/AGENTS.md` for detailed rules
2. May contain XDD library implementation
3. Coordinate with `bed/` (schemaforge submodule)
4. Related to schema validation across the ecosystem
