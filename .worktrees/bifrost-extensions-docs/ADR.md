# Architecture Decision Records — Bifrost Extensions

**Version:** 1.0.0
**Status:** Active

---

## ADR-001: Zero-Fork Architecture

**Status:** Accepted
**Date:** 2026-03-26
**Traces to:** FR-PLG, E1.1

**Context:**
The upstream `bifrost` project (`github.com/maximhq/bifrost`) is actively maintained. Maintaining a fork would require continuous manual merge effort as upstream evolves. The same applies to `CLIProxyAPI` (`github.com/router-for-me/CLIProxyAPI`). Both projects expose stable Go module APIs.

**Decision:**
Consume `github.com/maximhq/bifrost/core` and `CLIProxyAPI` as Go module dependencies declared in `go.mod`. All extension behavior is additive — new packages, new plugins, new providers — with zero modifications to upstream source files.

**Alternatives Considered:**
1. Fork both upstream repos and merge periodically. Rejected: merge burden is high; upstream authors are active and breaking changes are common.
2. Copy only needed types into this repo. Rejected: creates type divergence and breaks interface compatibility.

**Consequences:**
- Upstream bug fixes and new provider support are available via `go get`.
- Cannot change core routing behavior; must use plugin hooks for all interception.
- Local `replace` directives in `go.mod` allow development against local copies of `CLIProxyAPI` and `agentapi-plusplus`; these must be removed before publishing.

---

## ADR-002: Go + Cobra + Viper for CLI

**Status:** Accepted
**Date:** 2026-03-26
**Traces to:** FR-CLI-004, E6.1

**Context:**
The entire upstream stack (Bifrost core, CLIProxyAPI) is written in Go. A CLI tool with nested subcommands (`init`, `server`, `deploy fly|vercel|...`) requires a framework that handles flag inheritance, help generation, and shell completion.

**Decision:**
Go for all production code. Cobra (`spf13/cobra v1.10.1`) for CLI subcommand structure. Viper (`spf13/viper v1.21.0`) for configuration, supporting YAML files with environment variable overrides. Single binary output from `go build ./cmd/bifrost`.

**Alternatives Considered:**
1. Python with Click. Rejected: not compatible with the Go codebase; separate binary required.
2. Hand-rolled `flag` parsing. Rejected: does not support subcommand trees or shell completion.

**Consequences:**
- Single statically-linked binary; no runtime dependency on interpreters.
- Viper's YAML + env layering means all config values can be overridden in CI/CD without config file changes.
- Cobra's subcommand model means new deployment targets are added as new `cobra.Command` children of the `deploy` command.

---

## ADR-003: Plugin Architecture (PreHook / PostHook / TransportInterceptor)

**Status:** Accepted
**Date:** 2026-03-26
**Traces to:** FR-PLG-001 through FR-PLG-005, E1.2

**Context:**
All extension behavior (routing, safety, context folding, prompt adaptation, fallback) must intercept requests and responses at defined points without modifying the Bifrost routing core. Extensions must be independently composable and independently testable.

**Decision:**
All extensions implement `schemas.Plugin` from `bifrost/core/schemas`. The interface defines three interception points:
- `PreHook`: intercept before provider call; can modify request, short-circuit with a cached/computed response, or abort with error.
- `PostHook`: intercept after provider response; can modify response or replace with error.
- `TransportInterceptor`: intercept at HTTP transport layer; can modify headers and body.

`RoutingDecision` is stored in the request `context.Context` by `IntelligentRouter.PreHook` using a private typed context key, making it available to `PostHook` without additional state on the plugin struct.

**Alternatives Considered:**
1. Middleware chain (HTTP-level). Rejected: operates too late for request modification (provider is already selected); cannot short-circuit at the LLM routing layer.
2. Embedded sub-classing of Bifrost. Rejected: requires forking upstream.

**Consequences:**
- Clean separation of concerns; each plugin is independently unit-testable with a mock `BifrostRequest`.
- Plugin order matters; the server must document the configured plugin execution order.
- Context-based state passing (`routingDecisionKey`) requires discipline: all plugins that read routing context must handle the case where the key is absent.

---

## ADR-004: Chi HTTP Router

**Status:** Accepted
**Date:** 2026-03-26
**Traces to:** FR-SRV-004, E1.3

**Context:**
The service exposes OpenAI-compatible HTTP endpoints. A lightweight router with middleware composability is required. The server must support CORS (for browser-based clients) and structured logging.

**Decision:**
Chi (`go-chi/chi/v5 v5.2.2`) for HTTP routing. `go-chi/cors` for CORS middleware. Chi middleware (`middleware.Logger`, `middleware.Recoverer`) for request logging and panic recovery.

**Alternatives Considered:**
1. Gin. Rejected: heavier API; more magic; Bifrost core already uses Chi conventions.
2. Standard `net/http` `ServeMux`. Rejected: no middleware composability; pattern matching is insufficient for REST APIs.
3. Echo. Rejected: adds unnecessary abstraction; Chi is lighter and idiomatic.

**Consequences:**
- Handler functions receive `http.ResponseWriter` and `*http.Request` — standard Go interfaces, no framework lock-in.
- Middleware is composable via `router.Use(...)`.
- Chi's `context.Context` is the standard `context.Context`, so `RoutingDecision` values stored by plugins are accessible in handlers.

---

## ADR-005: Unified IntelligentRouter Consolidating Multiple Routing Strategies

**Status:** Accepted
**Date:** 2026-03-26
**Traces to:** FR-RTR-001 through FR-RTR-011, E2.1–E2.4

**Context:**
Multiple routing strategies exist as separate research projects: RouteLLM (difficulty-based model selection via matrix factorization), SemanticRouter (rule-based intent routing), ArchRouter (task classification via a 1.5B parameter model), and MIRT (multi-dimensional item response theory for quality-cost optimization). Running them as separate services adds operational complexity. Each strategy alone is insufficient — optimal routing requires combining cost awareness, task classification, and difficulty estimation.

**Decision:**
Consolidate all strategies into a single `IntelligentRouter` plugin (package `plugins/intelligentrouter`). Each strategy is a feature-flagged sub-component controlled by `Config` fields (`RouteLLMEnabled`, `MIRTEnabled`, `ArchRouterEndpoint`, `SemanticRulesPath`). The router produces a single `RoutingDecision` that combines all strategy outputs. The `costengine.Engine` enforces cost ceilings after strategy selection.

**Alternatives Considered:**
1. Separate plugin per strategy, chained in pipeline. Rejected: produces conflicting routing decisions; no unified cost awareness across strategies.
2. External routing service (sidecar). Rejected: adds network hop on every request; increases latency on the hot path.

**Consequences:**
- Feature flags allow progressive rollout: start with only `RouteLLMEnabled=true`, add ArchRouter as the local model becomes available.
- `Config.RouterSLMURL` and `Config.SummarizerSLMURL` decouple the SLM backend from the main server process.
- The router accumulates complexity over time; file size limits (500 LOC hard limit) must be enforced to keep sub-components in separate files (`router.go`, `decision.go`, `semantic.go`, `routellm.go`, `mirt.go`, `arch_router.go`).

---

## ADR-006: CLI Agents as First-Class Bifrost Providers via agentapi

**Status:** Accepted
**Date:** 2026-03-26
**Traces to:** FR-AGT-001 through FR-AGT-007, E4.1–E4.3

**Context:**
AI coding CLI tools (Claude Code, OpenAI Codex, Goose, Cursor, Aider, etc.) are licensed via subscription and provide capabilities not accessible via raw API (project context, file editing, agentic tool use). Making them addressable as LLM providers via a standard API allows the routing layer to select them like any other model.

**Decision:**
`agentcli.Provider` implements `schemas.Provider`. It communicates with a locally running CLI agent via the agentapi HTTP protocol (GET `/status`, GET `/messages`, POST `/messages`). The provider waits for the agent to reach `stable` status, sends the user message, and polls for a new response message to appear in the message list. Provider key: `agentcli-<agent_type>`.

**Alternatives Considered:**
1. Direct process spawn. Rejected: complex process management; no standard I/O protocol; varies per agent.
2. Screen scraping only (no agentapi). Rejected: fragile; no structured message extraction.

**Consequences:**
- Requires a separately running agentapi process wrapping the CLI agent.
- `Config.MaxWaitTime` (default 5 min) must be tuned per agent; slow agents (Aider, Cursor with large codebases) may need longer timeouts.
- Response is assembled from message list delta (messages after the pre-send snapshot) — this assumes CLI agents append responses as discrete messages, which is true for agentapi-wrapped agents.

---

## ADR-007: OAuth2 PKCE for CLI Agent Authentication

**Status:** Accepted
**Date:** 2026-03-26
**Traces to:** FR-OAU-001 through FR-OAU-009, E5.1–E5.3

**Context:**
Claude and Codex CLI tools support OAuth2 PKCE authentication flows that do not require the user to manually generate or copy API keys. The PKCE flow is the correct OAuth2 profile for CLI public clients (no client secret).

**Decision:**
`providers/oauthproxy` implements provider-specific `OAuthProvider` implementations (`ClaudeOAuth`, `CodexOAuth`) sharing the `GeneratePKCE()` helper (SHA-256 S256 challenge). `OAuthManager` handles multi-provider token storage in a single JSON file (`oauth_tokens.json`, mode 0600) in a configurable cache directory.

**Alternatives Considered:**
1. API key only. Rejected: user experience is poor; keys require manual generation from provider web console.
2. Device authorization grant. Rejected: not supported by Claude or Codex OAuth endpoints as of implementation.

**Consequences:**
- Token files at 0600 prevent other local users from reading credentials.
- `NeedsRefresh()` 5-minute window prevents mid-request token expiry under normal conditions; agents with long task durations may still hit expiry if the refresh window is too small.
- The Claude client ID (`9d1c250a-e61b-44d9-88ed-5944d1962f5e`) is the publicly known CLI client ID for Anthropic's OAuth flow; it is not a secret.

---

## ADR-008: EnhancedAccount Wrapping schemas.Account

**Status:** Accepted
**Date:** 2026-03-26
**Traces to:** E1.2, agentcli provider integration

**Context:**
Bifrost's `schemas.Account` interface provides `GetKeysForProvider`. Extension providers (e.g., `agentcli`) need to register keys for custom `ModelProvider` values (`agentcli-claude`, etc.) without modifying the base account.

**Decision:**
`account.EnhancedAccount` holds an in-memory map of `schemas.ModelProvider -> []schemas.Key` plus a `schemas.ProviderConfig` map. It accepts a fallback `schemas.Account` at construction and delegates to it when the in-memory map has no entry for the requested provider. Thread safety via `sync.RWMutex`.

**Consequences:**
- Zero changes to the upstream `Account` interface.
- Extension-managed keys take precedence over fallback keys.
- The `EnhancedAccount` must be populated before server startup; no dynamic key addition at runtime is supported in the current design.
