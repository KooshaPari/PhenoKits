# Product Requirements Document — template-program-ops

**Product:** Phenotype Operations Templates
**Template Version:** 0.1.0
**Date:** 2026-04-02
**Status:** Foundation Only

## Purpose

Provide operations tooling templates that follow Phenotype platform conventions. Currently provides basic shell scaffolding; CLI patterns, logging, and scheduling templates are on the roadmap.

## Current State

The template currently provides:
- Basic shell script patterns
- Kitty-specs integration
- Taskfile.yml configuration
- Basic CI/CD workflow structure

## Target State

### Phase 1: Foundation (Current - v0.1.0) ✅
- [x] Basic shell scaffolding
- [x] Kitty-specs integration
- [x] Taskfile configuration

### Phase 2: CLI Tools (v0.2.0) 🚧
- [ ] phenotype-ops-cli template
- [ ] Typer CLI framework
- [ ] Rich output formatting
- [ ] Subcommand patterns

### Phase 3: Logging & Config (v0.3.0) 📋
- [ ] structlog integration
- [ ] TOML configuration
- [ ] Pydantic validation
- [ ] Secret management

### Phase 4: Scheduling (v0.4.0) 📋
- [ ] Task scheduler patterns
- [ ] Cron integration
- [ ] Retry mechanisms
- [ ] Error notification

## Problem Statement

Operations tooling requires:
- ✅ Basic shell setup (covered)
- ⏳ CLI interface patterns (pending)
- ⏳ Configuration management (pending)
- ⏳ Structured logging (pending)
- ⏳ Task scheduling (pending)

## User Stories

### US-OPS-001: Basic Project Setup ✅
**As an** operations engineer
**I want to** generate a basic operations project
**So that** I have a starting point

**Status:** Implemented (v0.1.0)

### US-OPS-002: Generate CLI Tool 🚧
**As a** DevOps engineer
**I want to** generate an operations CLI
**So that** I can automate tasks

**Status:** Roadmap (v0.2.0)

### US-OPS-003: Use Structured Logging 🚧
**As an** SRE
**I want to** have consistent logs
**So that** I can debug issues

**Status:** Roadmap (v0.3.0)

## Success Metrics

| Metric | Target | Current |
|--------|--------|---------|
| Project generation | < 30 seconds | ✅ |
| Taskfile tasks | Working | ✅ |
| Template completeness | 100% | ⏳ Pending |

## Constraints

- Bash/zsh compatibility
- Taskfile for orchestration
- Kitty-specs for specifications
