# Implementation Plan — Bifrost Extensions

**Version:** 1.1.0
**Last Updated:** 2026-03-27
**Status:** Active

---

## DAG Summary

```
P1 --> P2 --> P3 --> P4 --> P5 --> P6 --> P7
              |                    |
              +--> P5 (parallel) --+
```

Phases 5 and 6 can proceed in parallel after Phase 4 completes.
Phase 7 depends on all prior phases.

---

## Phase 1: Core Architecture (Done)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P1.1 | Define plugin interface (`schemas.Plugin`) | — | Done |
| P1.2 | Implement `EnhancedAccount` with fallback delegation | P1.1 | Done |
| P1.3 | Set up Viper configuration (YAML + env overlay) | — | Done |
| P1.4 | Chi HTTP server with OpenAI-compatible endpoints | P1.1 | Done |

References: `account/`, `config/`, `server/`, `go.mod`

---

## Phase 2: Plugins (Done)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P2.1 | Intelligent router plugin (`plugins/intelligentrouter`) | P1.1 | Done |
| P2.2 | Smart fallback plugin (`plugins/smartfallback`) | P1.1 | Done |
| P2.3 | Learning plugin (`plugins/learning`) | P2.1 | Done |

References: `plugins/`

---

## Phase 3: CLI (Done)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P3.1 | Cobra CLI framework (`cmd/bifrost`) | — | Done |
| P3.2 | `bifrost init` command | P3.1 | Done |
| P3.3 | `bifrost server` command | P3.1, P1.4 | Done |
| P3.4 | `bifrost deploy` multi-platform (Fly.io, Vercel, Railway, Render) | P3.1 | Done |

References: `cmd/`, `Makefile.cli`

---

## Phase 4: OAuth and Agent Integration (Done)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P4.1 | PKCE OAuth flow for Claude (`providers/oauthproxy`) | — | Done |
| P4.2 | Token store with refresh (`OAuthManager`) | P4.1 | Done |
| P4.3 | AgentCLI SSE streaming provider | — | Done |
| P4.4 | TUI screen capture | P4.3 | Done |

References: `providers/oauthproxy/`, `providers/agentcli/`

---

## Phase 5: Reliability and Observability (Active)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P5.1 | Structured logging via `log/slog` replacing `log` package | P1.4 | Pending |
| P5.2 | Prometheus metrics endpoint (`/metrics`) | P5.1 | Pending |
| P5.3 | OpenTelemetry distributed tracing | P5.1 | Pending |
| P5.4 | Health check endpoints (`/healthz`, `/readyz`) | P1.4 | Pending |
| P5.5 | Circuit breaker pattern for plugin failures | P2.1, P2.2 | Pending |
| P5.6 | Retry logic with exponential backoff on provider errors | P5.5 | Pending |
| P5.7 | Panic recovery and graceful plugin degradation | P5.5 | Pending |

References: `server/`, `plugins/`; traces to FR-SRV-005, NFR-6

---

## Phase 6: Test Coverage (Active / Parallel with P5)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P6.1 | Unit tests for all plugin `PreHook`/`PostHook` paths | P2.1–P2.3 | Pending |
| P6.2 | Unit tests for `IntelligentRouter` routing strategies | P2.1 | Pending |
| P6.3 | Unit tests for OAuth PKCE flow and token refresh | P4.1, P4.2 | Pending |
| P6.4 | Unit tests for CLI command handlers (all 7 top-level commands) | P3.1–P3.4 | Pending |
| P6.5 | Integration tests: server startup and OpenAI endpoint round-trips | P1.4, P6.1 | Pending |
| P6.6 | Integration tests: deploy command against mock platform APIs | P3.4, P6.4 | Pending |
| P6.7 | Coverage gate enforcement at 80% for `plugins/`, `server/`, `providers/` | P6.1–P6.6 | Pending |

References: `tests/`; traces to FR-PLG-001–FR-PLG-005

---

## Phase 7: Production Hardening (Planned)

| Task ID | Description | Depends On | Status |
|---------|-------------|------------|--------|
| P7.1 | Remove `replace` directives from `go.mod` for CI builds | P4.3 | Planned |
| P7.2 | Supply chain audit: `govulncheck`, `osv-scanner` in CI | P7.1 | Planned |
| P7.3 | Secret detection gate (gitleaks) in CI | — | Planned |
| P7.4 | CLI self-update mechanism (`go-selfupdate`) integration | P3.1 | Planned |
| P7.5 | GraphQL schema and resolver generation (`gqlgen`) | P1.4 | Planned |
| P7.6 | Validate `agentcli` provider against 5+ CLI agent types | P4.3, P4.4 | Planned |
| P7.7 | Release workflow: goreleaser + GitHub Releases | P7.1–P7.3 | Planned |

References: `go.mod`, `api/`, `slm/`, `slm-manager/`; traces to ADR-001, ADR-006

---

## Dependency DAG (Task-Level)

```
P1.1 --> P1.2, P1.4, P2.1, P2.2, P2.3
P1.3 --> P1.4
P1.4 --> P3.3, P5.4, P5.2, P6.5, P7.5
P2.1 --> P2.3, P5.5, P6.1, P6.2
P2.2 --> P5.5, P6.1
P3.1 --> P3.2, P3.3, P3.4, P7.4
P3.4 --> P6.6
P4.1 --> P4.2, P6.3
P4.3 --> P4.4, P7.1, P7.6
P5.1 --> P5.2, P5.3
P5.5 --> P5.6, P5.7
P6.1 --> P6.5, P6.7
P6.4 --> P6.6, P6.7
P6.5 --> P6.7
P6.6 --> P6.7
P7.1 --> P7.2, P7.7
P7.2 --> P7.7
P7.3 --> P7.7
```
