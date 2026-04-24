# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Feature comparison matrix with 9 similar tools in the AI agent governance space.
- VitePress documentation site scaffold and verification harness.

### Fixed
- CI: Added missing `scripts/quality-gate.sh` to resolve PR check failures.
- Linting: Fixed over 12,000 linting errors using `ruff`.

## [0.1.0] - 2026-03-25

### Added
- Initial release of AgentOps Policy Federation.
- Seven-scope hierarchical policy resolution (global, org, repo, path, command, actor, session).
- Multi-harness runtime integration: Codex, Cursor, Factory, Claude.
- Policy enforcement for exec, write, and network actions.
- Policy configuration compiler and installer (`policyctl` CLI).
- Headless ask delegation engine for policy ask decisions.
- Canonical repo write guard policy.
- Read-only shell commands and worktrees path allow rules.
- Unit tests for policy-federation modules (risk, delegate, learner, gap_detector).
- VitePress docsite scaffold.
- Spec documentation: PRD, ADR, Functional Requirements, Plan, trackers.
- Standardized AgilePlus governance in CLAUDE.md.
- Repository policies, issue templates, CODEOWNERS, security scanning, and CI workflows.

### Fixed
- Restored missing `_build_review_request` function and fixed test assertions.
- Added missing imports for `verify_audit_chain` and test helper functions.
- Removed unused variable in `gap_detector.py`.
- Added read-only shell commands to allow rules.

### Security
- Policy engine hardening: TOCTOU prevention, environment sanitization, write-bypass closure, audit chaining.
- Secret scanning integration.
- Dependabot configuration.

[Unreleased]: https://github.com/KooshaPari/policy-federation/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/KooshaPari/policy-federation/releases/tag/v0.1.0
