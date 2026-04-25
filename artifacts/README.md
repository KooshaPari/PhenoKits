# Artifacts

Shared artifacts and outputs registry for the Phenotype ecosystem.

## Purpose

Artifacts is a centralized repository for storing, versioning, and distributing build outputs, generated documentation, test fixtures, and other shared deliverables across Phenotype projects. This ensures consistency and enables cross-project reuse of compiled binaries, schemas, and reference materials.

## Features

- **Artifact Registry** — Version-controlled storage of build outputs and generated assets
- **Cross-Project Sharing** — Share compiled binaries, schemas, and fixtures across repos
- **Documentation Generation** — Store and distribute auto-generated documentation
- **Test Fixtures** — Centralized test data and fixtures for consistent testing
- **Build Outputs** — Archive releases and deployment artifacts

## Usage

1. **Storing Artifacts** — Commit versioned outputs to the appropriate subdirectory
2. **Retrieving Artifacts** — Reference artifacts from dependent projects via git submodule or direct import
3. **Versioning** — Follow semver tagging for artifact releases

## Project Status

- **Status**: Active
- **Part of**: Phenotype Ecosystem
- **Collection**: Documentation & Build Outputs

## Governance

- **Quality**: Vale + Markdownlint for documentation artifacts
- **Specs**: Track artifact management in AgilePlus
- **CI/CD**: Automated artifact generation via GitHub Actions
- **Versioning**: Semantic versioning for artifact releases

## References

- **Phenotype Org Docs**: See `repos/docs/` for architecture and governance
- **Related Projects**: Cross-project sharing protocol in `CLAUDE.md`
- **Worklogs**: Audit trail in `docs/worklogs/` (if present)
