# AGENTS.md — bed

## Project Overview

- **Name**: bed
- **Description**: SchemaForge schema management submodule - bedrock for data schemas
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/bed`
- **Language Stack**: Mixed (submodule for SchemaForge)
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to the submodule
cd /Users/kooshapari/CodeProjects/Phenotype/repos/bed

# Parent project integration
# This is a submodule of schemaforge - see parent for commands
```

## Architecture

```
bed/
└── schemaforge/          # SchemaForge submodule root
```

This directory serves as a submodule mount point for SchemaForge integration.

## Quality Standards

- **Submodule hygiene**: Keep submodule references clean
- **No direct commits**: Changes should be made in the canonical schemaforge repo
- **Sync protocol**: Use `git submodule update --init --recursive`

## Git Workflow

### Branch Naming
- This is a submodule - branch in parent `schemaforge` repo
- Use: `schemaforge/<type>/<description>`

### Commit Format
- Follow schemaforge parent project conventions
- Reference: `platforms/thegent/governance/AGENTS.base.md`

## File Structure

```
bed/
└── schemaforge/          # SchemaForge canonical repository
```

## CLI Commands

```bash
# Initialize submodule
git submodule update --init --recursive

# Update submodule reference
git submodule update --remote

# Check submodule status
git submodule status
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Submodule not initialized | Run `git submodule update --init` |
| Detached HEAD in submodule | Check parent repo for correct reference |
| Changes in submodule | Changes belong in schemaforge canonical repo |

## Dependencies

- **Parent**: schemaforge (external canonical repo)
- **Consumers**: Phenotype schema validation systems

## Notes

This is a git submodule mount point. Do not commit directly here.
All changes should flow through the canonical schemaforge repository.
