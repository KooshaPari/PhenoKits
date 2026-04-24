# AGENTS.md — phenotype-infrakit

## Project Overview

- **Name**: phenotype-infrakit
- **Description**: Infrastructure toolkit - path resolution and resource management
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit`
- **Language Stack**: Mixed (docs, Go, etc.)
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit

# Check components
ls -la phenotype-path-resolve/
ls -la phenotype-resources/
```

## Architecture

```
phenotype-infrakit/
├── docs/                     # Documentation
├── phenotype-path-resolve/   # Path resolution utilities
└── phenotype-resources/      # Resource management
```

## Quality Standards

### Path Resolution
- Cross-platform path handling
- Security considerations (path traversal prevention)

### Resource Management
- Proper cleanup semantics
- Resource limits enforcement

## Git Workflow

### Branch Naming
Format: `infrakit/<type>/<description>` or `<component>/<type>/<description>`

Examples:
- `infrakit/feat/path-helpers`
- `phenotype-resources/fix/leak`

## File Structure

```
phenotype-infrakit/
├── docs/
├── phenotype-path-resolve/
└── phenotype-resources/
```

## CLI Commands

```bash
# Check components
cd phenotype-path-resolve && ls -la
cd phenotype-resources && ls -la
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Path issues | Check resolution logic |
| Resource leaks | Verify cleanup code |

## Dependencies

- **pheno/**: Core path conventions
- **crates/phenotype-infrastructure**: Rust equivalents

## Agent Notes

When working in phenotype-infrakit:
1. Infrastructure utilities
2. May contain Go implementations
3. Coordinate with Rust infrastructure crates
