# Archived Repo Registry

**Status:** Living index | **Last updated:** 2026-04-25 | **Scope:** KooshaPari org

User policy: **STRICTLY DO NOT DELETE NOR UNARCHIVE** any archived repo. This registry tags each so agents stop investigating them and know where the heritage went.

## Successor Map (archived → canonical)

| Archived | Canonical successor | Note |
|----------|---------------------|------|
| `tehgent` | `thegent` | Stub renamed/replaced; ~14KB only. |
| `pheno-sdk` | `phenosdk` family (phenosdk-decompose-llm, etc.) | Empty placeholder; family is canonical. |
| `helios-cli-backup` | `HexaKit/helios-cli` | Explicit "DEPRECATED" backup of pre-HexaKit move. |
| `agentapi` | `agentapi-plusplus` (Go) | Empty stub; Go rewrite is live. |
| `KodeVibeGo` | `HexaKit` (per repo description) | Consolidated into HexaKit lint/scan paths. |
| `Authvault` | `AuthKit` | Authkit→authvault rename in progress; Authvault archive holds OAuth2/JWT/RBAC heritage. |

## Useful Heritage (mine before reuse, do not revive)

| Repo | What's worth grepping | Target consumer |
|------|----------------------|-----------------|
| `KommandLineAutomation` | Rust CLI-recording spike (~41KB) | Future demo/asciinema work |
| `localbase3` | `/localbase-chain` Go module — local-first DB experiment | Future local-first storage in Phenotype |
| `atoms.tech` | Real Next.js product (107MB, 37+ PRs, table UX, security hardening) | Reference for projects-landing Next.js scaffolding |
| `Logify` | Structured logging w/ zero-cost abstraction (Rust) | phenotype-shared if logging needs evolve |
| `Eventra` | CQRS + Event Sourcing framework (Rust) | phenotype-event-sourcing extension |
| `phenoXddLib` | xDD utilities — property testing, contract verify, mutation cov | thegent xDD governance |

## Name-Reserved Candidates (zero-content but useful names)

| Repo | Suggested future use |
|------|----------------------|
| `Servion` | Service registry/discovery extraction from AgilePlus or heliosCLI |
| `Guardrail` | Resilience patterns (rate-limit/circuit-break/bulkhead) — overlaps `phenotype-policy-engine` |
| `Cryptora` | Future cryptography utility crate |
| `Diffuse` | Unified diff/patch library (if extracted from agileplus-graph or similar) |
| `forge` | CLI task runner / build orchestrator — note the lowercase; likely reserved deliberately |

## Pure Dead-Weight (skip in audits)

`acp`, `PriceyApp` (upstream fork at mobile-next), `AtomsBot`, `argisexec`, `router-docs`, `phenotype-colab-extensions`, `KaskMan`, `ccusage` (upstream fork at ryoppippi), `vibe-kanban` (upstream fork at BloopAI), `KWatch`, `slickport`, `chatta`, `Prismal`, `kwality`, `KlipDot`, `AppGen`, `phenoForge`, `Quillr`, `Zerokit`, `Settly`, `phenotype-dep-guard`, `phenoRouterMonitor`, `worktree-manager`, `Synthia` (upstream fork BeehiveInnovations).

## Inert PRs in Archived Repos

Per `memory/reference_archived_repos_locked.md`: archived repos block `gh pr close` and `gh pr comment`. Known inert PRs (cannot be closed): worktree-manager #9–#16, Settly #1–#6, KodeVibeGo #41, phenoXddLib #7. **Skip in fleet PR-count audits.**

## Audit Process

When auditing an archived repo:
1. Check this registry first — if listed, skip detailed investigation.
2. If not listed, fetch `gh repo view` for diskUsage + last commit; if <100KB and no description, mark dead-weight here.
3. If real code, grep before any extraction; record successor mapping.
4. Never run `gh repo delete` or `gh repo edit --visibility` on these.

## References

- `worklogs/ARCHITECTURE.md` — extraction candidates from heritage repos
- `memory/reference_archived_repos_locked.md` — write-lock behavior
