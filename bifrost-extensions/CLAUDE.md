<!-- Base: platforms/thegent/dotfiles/governance/CLAUDE.base.md -->
<!-- Last synced: 2026-04-24 -->

# bifrost-extensions — CLAUDE.md

Extends thegent governance base. See `platforms/thegent/dotfiles/governance/CLAUDE.base.md` for canonical definitions.

## Project Overview

- **Name**: bifrost-extensions
- **Description**: OpenAI-compatible HTTP server extending Bifrost core with content safety, context folding, and tool routing plugins
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/bifrost-extensions`
- **Language Stack**: Go (1.25+)
- **Published**: Internal (Phenotype org)
- **Repository**: https://github.com/kooshapari/bifrost-extensions

## AgilePlus Mandate

All work MUST be tracked in AgilePlus:
- Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- CLI: `cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>`

## Work Requirements

1. **Check for AgilePlus spec before implementing**
2. **Create spec for new work**: `agileplus specify --title "<feature>" --description "<desc>"`
3. **Update work package status**: `agileplus status <feature-id> --wp <wp-id> --state <state>`
4. **No code without corresponding AgilePlus spec**

---

## Project

Go HTTP server integrating with Bifrost core (v1.2.30+) to provide OpenAI-compatible endpoints with pluggable extensions:
- **Content Safety**: Detoxify + GoEmotions models for toxicity pre-screening and response safety post-processing
- **Context Folding**: Semantic context compression for long-context LLM calls
- **Tool Routing**: Dynamic tool selector and cost-aware request dispatcher

## Stack

- **Language**: Go (1.25+)
- **Build**: `go build ./...`
- **Test**: `go test ./...`
- **Runtime**: HTTP/1.1 + WebSocket support
- **Dependencies**: chi/v5 (routing), pgx/v5 (PostgreSQL), gqlgen (GraphQL), hatchet (workflow), NATS (messaging)

## Structure

```
.
├── api/              # OpenAI-compatible API definitions (gRPC/Connect)
├── bifrost/          # Bifrost core integration
├── cmd/              # CLI entrypoints
├── config/           # Configuration management (YAML/TOML loaders)
├── costengine/       # Cost calculation and token accounting
├── db/               # PostgreSQL migrations and query builders
├── infra/            # Kubernetes/Helm charts, Docker compose
├── plugins/          # Extensible plugin system
│  ├── contentsafety/ # Content moderation plugin
│  ├── contextfolding/# Context compression plugin
│  └── toolrouter/    # Tool routing and selection plugin
├── providers/        # LLM provider integrations
├── server/           # HTTP server and request handlers
└── slm/              # Small Language Model integrations
```

## Conventions

- **Package layout**: Subdirectories = separate packages (no root-level package conflicts)
- **Error handling**: Explicit error returns; no silent fallbacks
- **Testing**: Table-driven tests (`*_test.go` files in package directories)
- **Dependencies**: Prefer Go standard library; wrap OSS (document with `// wraps: <lib-name> <version>`)
- **Versioning**: CalVer (v2026.04.0, v2026.03B, etc.)

## Build & Test

```bash
# Build all packages
go build ./...

# Run tests
go test ./...

# Run tests with coverage
go test -cover ./...

# Lint with golangci-lint (if installed)
golangci-lint run ./...
```

## Versioning (CalVer)

Bifrost-extensions uses CalVer for releases aligned with quarterly waves:
- **Format**: `v202X.WW[A-Z].PATCH` (e.g., v2026.02A, v2026.05.0)
- **Timeline**: Releases tagged on completion of quarterly feature waves
- **Current range**: v2026.02A (Feb wave A) → v2026.05.0 (May release)
- **Release process**: See `.github/workflows/` for automated tagging and changelog generation

## Documentation

- **SPEC.md**: Complete functional specification
- **PLAN.md**: Implementation plan and roadmap
- **CHARTER.md**: Project charter and governance
- **ADR.md**: Architecture decision records
- **AGENTS.md**: AI agent instructions
- **PRD.md**: Product requirements definition

---

## Disk & Coordination Policies

See `~/.claude/CLAUDE.md` for:
- Phenotype Disk Budget Policy
- Multi-Session Coordination Protocol
- Long Push Pattern (nohup for >3 min pushes)

---

## Local Quality Checks

From this repository root:

```bash
# Build validation
go build ./...

# Test suite
go test ./...

# Lint (if golangci-lint is available)
golangci-lint run ./... || echo "Install: brew install golangci-lint"
```

## Git Integration & Worktrees

- **Canonical**: `/Users/kooshapari/CodeProjects/Phenotype/repos/bifrost-extensions` (main branch)
- **Worktrees**: Feature work in `/Users/kooshapari/CodeProjects/Phenotype/repos/bifrost-extensions-wtrees/<topic>/`
- **Branch check**: `git status --short --branch` should show `main` in canonical folder
- **Merge protocol**: Pull to canonical only after feature is complete and quality gates pass

---

## Governance Reference

See thegent governance base for:
- CI completeness policy and pre-merge validation
- Phenotype Git and Delivery Workflow Protocol
- Phenotype Org Cross-Project Reuse Protocol
- Phenotype Long-Term Stability and Non-Destructive Change Protocol
- Worktree Discipline guidelines

Location: `platforms/thegent/dotfiles/governance/CLAUDE.base.md`

## Scripting Language Hierarchy

Go is the canonical language for this project. For new tools or utilities:
- **Go** — default (standard lib + stdlib patterns)
- **Bash** — only as ≤5-line glue with a top-of-file justification comment (e.g., "Bash for shell-integration wrapper before exec'ing compiled binary")

For more context: `docs/governance/scripting_policy.md` (Phenotype org root).
