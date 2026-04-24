# Contributing to phenotype-dep-guard

Thank you for your interest in contributing to phenotype-dep-guard.

## Layer Contract

- **layer_type**: security_ops
- **layer_name**: phenotype-dep-guard
- **versioning**: semver

## Development Setup

```bash
# Clone the repository
git clone https://github.com/Phenotype-Enterprise/phenotype-dep-guard
cd phenotype-dep-guard

# Run checks
task check

# Run tests
pytest
```

## Mission

Analyze direct and transitive dependencies for malicious code, vulnerabilities, and anomalous behavior.

## Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes
4. Run `task check` to verify compliance
5. Ensure all tests pass
6. Update version following semver
7. Update manifest/reconcile files if needed
8. Create PR with description of changes

## Release Process

1. Run `task release:prep` as final pre-release gate
2. Update version following semver
3. Push and create PR
