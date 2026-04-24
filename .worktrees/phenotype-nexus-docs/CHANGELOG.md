# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-03-28

### Added
- VitePress docsite with Mermaid support and dark theme
- CLAUDE.md project guidelines and development documentation
- .pre-commit-config.yaml for automated code quality checks
- Integration tests for service registry functionality
- Complete nexus docs site with validation harness

### Changed
- Renamed package from `phenotype-nexus` to `nexus` for registry publishing
- Upgraded VitePress docsite with enhanced markdown support

### Fixed
- Service registry error handling and edge cases

## [0.1.0] - 2026-02

### Added
- Initial nexus crate - service registry implementation
- Core service discovery and registration abstractions
- Package manifest and publishing configuration
- CI/CD pipeline with automated testing
- Taskfile.yml with standard development tasks
- PRD, FUNCTIONAL_REQUIREMENTS, and ADR specification documents

[Unreleased]: https://github.com/KooshaPari/phenotype-nexus/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/KooshaPari/phenotype-nexus/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/KooshaPari/phenotype-nexus/releases/tag/v0.1.0
