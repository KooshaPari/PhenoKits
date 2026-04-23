# AGENTS.md - phenotype-dep-guard

## Project Overview

- **Name**: phenotype-dep-guard
- **Description**: Malicious dependency analysis and supply chain security guard
- **Language**: Python (3.10+)
- **Location**: Phenotype repos shelf

## Layer Contract

- **layer_type**: security_ops
- **layer_name**: phenotype-dep-guard
- **versioning**: semver

## Mission

Analyze direct and transitive dependencies for malicious code, vulnerabilities, and anomalous behavior.

## Features

- High-velocity multi-source dependency resolution
- Heuristic and static triage (AST parsing, .pth/setup.py scanning)
- Agentic LLM deep analysis
- Reporting and alerting

## Agent Rules

### Project-Specific Rules

1. **Security First**
   - Never skip security checks
   - Log all analysis results
   - Fail on critical findings

2. **Layer Contract Compliance**
   - Run `task check` before release
   - Keep manifest/reconcile files aligned for contract-affecting changes
   - Run `task release:prep` as final pre-release gate

3. **Spec Kitty Workflow**
   ```bash
   spec-kitty research --feature layered-template-platform --force
   ```

### Phenotype Org Standard Rules

1. **UTF-8 encoding** in all text files
2. **Worktree discipline**: canonical repo stays on `main`
3. **CI completeness**: fix all CI failures before merging
4. **Never commit** agent directories (`.claude/`, `.codex/`, `.cursor/`)

## Quality Standards

```bash
# Run checks
task check

# Release preparation
task release:prep
```

## Git Workflow

1. Create feature branch: `git checkout -b feat/my-feature`
2. Run security analysis
3. Ensure all checks pass
4. Create PR with analysis results
