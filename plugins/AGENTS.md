# AGENTS.md — plugins

## Project Overview

- **Name**: plugins
- **Description**: Plugin system for the Phenotype ecosystem
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/plugins`
- **Language Stack**: Mixed
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/plugins

# Check structure
ls -la docs/
ls -la embeddings/
```

## Architecture

```
plugins/
├── docs/                     # Plugin documentation
└── embeddings/               # Embedding plugins
```

## Quality Standards

### Plugin Standards
- Well-defined interfaces
- Version compatibility
- Documentation required
- Security sandboxing

## Git Workflow

### Branch Naming
Format: `plugins/<type>/<description>`

## File Structure

```
plugins/
├── docs/
└── embeddings/
```

## CLI Commands

```bash
# Check components
ls -la docs/
ls -la embeddings/
```

## Dependencies

- **PhenoPlugins/**: Related project

## Agent Notes

When working in plugins:
1. Plugin infrastructure
2. Coordinate with PhenoPlugins
