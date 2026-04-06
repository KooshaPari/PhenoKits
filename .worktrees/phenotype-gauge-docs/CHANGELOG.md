# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-03-28

### Added
- Comprehensive specification documents (PRD, FUNCTIONAL_REQUIREMENTS, ADR)
- User journeys and implementation planning documents
- Traceability tracking with FR tracker and code entity map
- VitePress docsite with Mermaid support and dark theme
- Integration tests for xDD testing utilities

### Changed
- Renamed package from `phenotype-gauge` to `gauge` for registry publishing
- Refined documentation structure with specification verification

### Fixed
- Mutation score calculation and tracker assertions
- Code duplication issues across crate modules

## [0.2.0] - 2026-02

### Added
- VitePress docsite scaffold and verification harness
- .pre-commit-config.yaml for code quality automation
- Taskfile.yml with standard development tasks
- CodeQL security scanning workflow
- CLAUDE.md project development guidelines

### Changed
- Enhanced docsite with Mermaid diagram support
- Consolidated xDD library and benchmarking framework

## [0.1.0] - 2026-01

### Added
- Initial gauge crate - benchmarking and mutation testing framework
- xDD (Executable Domain-Driven Design) utilities for test generation
- Mutation tracker for measuring test quality
- Package config and CI/CD pipeline
- Comprehensive test suite for mutation analysis

[Unreleased]: https://github.com/KooshaPari/phenotype-gauge/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/KooshaPari/phenotype-gauge/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/KooshaPari/phenotype-gauge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/KooshaPari/phenotype-gauge/releases/tag/v0.1.0
