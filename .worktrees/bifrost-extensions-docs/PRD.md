# Product Requirements Document — Bifrost Extensions

**Version:** 1.0.0
**Status:** Active
**Repository:** github.com/KooshaPari/bifrost-extensions

---

## Overview

Bifrost Extensions is a Go service that extends the [Bifrost LLM gateway](https://github.com/maximhq/bifrost) using a zero-fork architecture. It consumes the upstream `bifrost/core` module as a Go dependency and adds intelligent routing, content safety, context folding, OAuth proxy authentication, CLI agent integration, and serverless deployment tooling without modifying any upstream code.

The primary goal is to provide a production-quality LLM gateway with cost-aware multi-provider routing, OAuth-authenticated access to AI providers, and the ability to treat CLI coding agents (Claude Code, Codex, Cursor, Goose, etc.) as first-class Bifrost providers accessible via the standard OpenAI-compatible API.

---

## Epics and User Stories

### E1: Zero-Fork Extension Architecture

**E1.1: Module-based upstream consumption**
As a developer, I want to import Bifrost core and CLIProxyAPI as Go module dependencies so that I can receive upstream bug fixes and feature updates without maintaining a fork.

Acceptance Criteria:
- `go.mod` declares `github.com/maximhq/bifrost/core` as a direct dependency
- No vendored copies of upstream source files exist in this repository
- `go mod tidy && go build ./...` succeeds with zero errors

**E1.2: Plugin interface compliance**
As a developer, I want all extension behavior expressed as plugins implementing `schemas.Plugin` so that extensions are composable and do not couple to the routing core.

Acceptance Criteria:
- All plugins implement `GetName()`, `PreHook()`, `PostHook()`, `TransportInterceptor()`, `Cleanup()`
- `PreHook` supports three outcomes: pass-through with modified request, short-circuit with cached response, abort with error
- `PostHook` supports two outcomes: pass-through with modified response, replace with error
- All plugins are goroutine-safe (concurrent hook calls)

**E1.3: OpenAI-compatible HTTP server**
As an API consumer, I want the server to expose OpenAI-compatible chat and text completion endpoints so that existing OpenAI SDK clients work without modification.

Acceptance Criteria:
- `POST /v1/chat/completions` accepts standard OpenAI request bodies
- `POST /v1/completions` accepts standard OpenAI request bodies
- Response shapes match OpenAI API conventions
- Server built on Chi router with CORS middleware

---

### E2: Intelligent Multi-Provider Routing

**E2.1: Unified routing plugin**
As an operator, I want a single routing plugin that integrates multiple routing strategies (RouteLLM, SemanticRouter, ArchRouter, MIRT) so that the best routing approach is applied per request without running multiple separate services.

Acceptance Criteria:
- `IntelligentRouter` plugin (package `intelligentrouter`) consolidates RouteLLM, SemanticRouter, ArchRouter, and MIRT via feature flags in `Config`
- `RoutingDecision` struct is stored in request context via `PreHook` and read back in `PostHook` for outcome logging
- `RoutingDecision` fields populated: `SelectedModel`, `SelectedProvider`, `TaskType`, `RiskLevel`, `Confidence`, `CostEstimate`, `QuotaHeadroom`, `Reasoning`

**E2.2: Cost-aware endpoint selection**
As an operator, I want routing decisions to account for per-token cost estimates so that the cheapest adequate model is selected within a configurable ceiling.

Acceptance Criteria:
- `costengine.Engine` (package `costengine`) computes cost estimate for candidate endpoints before selection
- `Config.MaxCostPerRequest` enforces a hard ceiling (default $1.00)
- `Config.PreferSubscriptions` prefers subscription-bucket endpoints over pay-per-token when both qualify
- `Config.AllowScarceEndpoints` gates use of `scarce_premium` tier endpoints (default false)

**E2.3: Task classification**
As an operator, I want requests classified by task type so that routing rules select models optimized for the specific workload category.

Acceptance Criteria:
- `TaskType` enum covers: `tool_call`, `code_generation`, `reasoning`, `conversation`, `default`
- `RiskLevel` enum covers: `low`, `medium`, `high`
- Classification performed by ArchRouter (`katanemo/Arch-Router-1.5B`, configurable endpoint) or falls back to semantic rules from `Config.SemanticRulesPath`
- Classification result included in `RoutingDecision`

**E2.4: RouteLLM quality-cost optimization**
As an operator, I want RouteLLM matrix factorization routing to dynamically select strong vs. weak models based on query difficulty so that easy queries use cheap models and hard queries use capable models.

Acceptance Criteria:
- `Config.RouteLLMEnabled` enables/disables RouteLLM
- `Config.RouteLLMRouter` selects algorithm: `mf` (matrix factorization, default), `bert`, `sw_ranking`
- `Config.RouteLLMThreshold` (0.0–1.0, default 0.7) sets the difficulty threshold for strong-model selection
- Routing decision records which RouteLLM router was used

**E2.5: Outcome-driven learning**
As an operator, I want routing outcomes recorded and fed back into a learning subsystem so that routing quality improves over time without manual tuning.

Acceptance Criteria:
- `TieredLearningSystem` (package `plugins/learning`) records decision outcomes in `PostHook`
- `ThreePillarOptimizer` aggregates latency, cost, and error-rate signals to adjust routing weights
- Learning state persists across restarts via PostgreSQL (SQLC-generated queries in `db/`)

**E2.6: Smart fallback**
As an operator, I want automatic retry with alternative providers and models on failure so that transient outages do not surface to end users.

Acceptance Criteria:
- `smartfallback` plugin intercepts `PostHook` errors and re-submits with a configured fallback chain
- Fallback chain is `[]schemas.Fallback{Model, Provider}` pairs set on the request
- Max fallback depth configurable; exceeded depth returns the original error unchanged

---

### E3: Content Safety and Context Management

**E3.1: Content safety plugin**
As an operator, I want inbound requests and outbound responses scanned for prohibited content so that the gateway enforces content policies without requiring changes to upstream Bifrost.

Acceptance Criteria:
- `contentsafety` plugin (package `plugins/contentsafety`) implements `PreHook` and `PostHook`
- `PreHook` short-circuits requests that violate input policy (returns `PluginShortCircuit`)
- `PostHook` replaces responses that violate output policy with a safe fallback

**E3.2: Context folding plugin**
As an operator, I want long conversation histories compressed before being sent to models with small context windows so that token limits are not exceeded in multi-turn conversations.

Acceptance Criteria:
- `contextfolding` plugin (package `plugins/contextfolding`) implements `PreHook`
- Compression strategy selectable: truncate-oldest or SLM-summarize
- SLM summarizer endpoint configurable via `Config.SummarizerSLMURL` (default `http://localhost:9002`)
- Original message count and post-fold token estimate recorded for observability

**E3.3: Prompt adaptation**
As an operator, I want prompts automatically adapted for different provider conventions so that a single request can be routed to any provider without manual per-provider prompt engineering.

Acceptance Criteria:
- `promptadapter` plugin (package `plugins/promptadapter`) implements `PreHook`
- Adapter rules defined per target provider
- Role mapping is complete and reversible; no semantic content is dropped

---

### E4: CLI Agent Provider

**E4.1: CLI agent as Bifrost provider**
As a developer, I want to route LLM requests to a locally running CLI coding agent so that subscription-licensed CLI tools (Claude Code, Codex, Goose, Cursor, etc.) are accessible via the standard Bifrost API.

Acceptance Criteria:
- `agentcli.Provider` (package `providers/agentcli`) implements `schemas.Provider`
- Supported agent types: `claude`, `codex`, `goose`, `aider`, `gemini`, `copilot`, `amp`, `cursor`, `auggie`, `amazonq`, `opencode`
- Provider key pattern: `agentcli-<agent_type>` (e.g., `agentcli-claude`)
- `ChatCompletion` extracts the last user message, sends it to the CLI agent via agentapi HTTP, polls for completion, and returns the assembled text as a `BifrostResponse`

**E4.2: Agent readiness polling**
As a developer, I want the provider to wait for the CLI agent to reach a stable state before sending a message so that messages are not lost during agent startup or mid-task processing.

Acceptance Criteria:
- `waitForStable` polls `GET /status` until `status == "stable"` or `Config.MaxWaitTime` (default 5 min) is exceeded
- Poll interval configurable via `Config.PollInterval` (default 500 ms)
- Timeout returns HTTP 503 `BifrostError`

**E4.3: agentapi integration**
As a developer, I want the agentcli provider to integrate with the agentapi protocol so that CLI agent control (send message, get messages, get status, get screen) uses a standard interface.

Acceptance Criteria:
- `providers/agentcli/client.go` implements agentapi HTTP client methods: `getStatus`, `getMessages`, `sendMessage`
- Message baseline snapshot taken before send to identify new response messages
- Terminal dimensions configurable (`Config.TerminalWidth`, `Config.TerminalHeight`)

---

### E5: OAuth Proxy Authentication

**E5.1: PKCE OAuth2 for Claude**
As a CLI user, I want to authenticate with Anthropic Claude via OAuth2 PKCE so that I do not need to manually generate or manage API keys.

Acceptance Criteria:
- `ClaudeOAuth` implements `OAuthProvider` interface with PKCE S256 code challenge/verifier
- Auth URL: `https://claude.ai/oauth/authorize` with scopes `org:create_api_key`, `user:profile`, `user:inference`
- Token exchange: `POST https://console.anthropic.com/v1/oauth/token` with `application/json`
- Tokens persisted to `oauth_tokens.json` (file mode 0600) via `OAuthManager`

**E5.2: PKCE OAuth2 for OpenAI Codex**
As a CLI user, I want to authenticate with OpenAI Codex via OAuth2 PKCE so that subscription-licensed Codex access is available via the gateway.

Acceptance Criteria:
- `CodexOAuth` implements `OAuthProvider` with PKCE S256
- Auth URL: `https://auth.openai.com/authorize` with audience `https://api.openai.com/v1`
- Token exchange: `POST https://auth.openai.com/oauth/token` with `application/x-www-form-urlencoded`
- Scopes: `openid`, `profile`, `email`, `offline_access`

**E5.3: Token lifecycle management**
As an operator, I want tokens proactively refreshed before expiry so that authenticated sessions do not fail mid-request due to stale tokens.

Acceptance Criteria:
- `TokenData.NeedsRefresh()` returns true when token expires within 5 minutes of wall clock time
- `TokenData.IsExpired()` returns true when token is past `ExpiresAt`
- `OAuthManager.LoadTokens()` reads from `<cacheDir>/oauth_tokens.json`; returns empty map if file does not exist
- `OAuthManager.SaveTokens()` writes with `MarshalIndent` at mode 0600

---

### E6: CLI and Serverless Deployment

**E6.1: Cobra CLI**
As a developer, I want a single `bifrost` binary with subcommands so that server management, initialization, and deployment require no separate tooling.

Acceptance Criteria:
- `bifrost init` — scaffolds project configuration files
- `bifrost server` — starts the HTTP server with plugins loaded from YAML configuration
- `bifrost deploy fly|vercel|railway|render|homebox` — deploys to the named platform
- CLI framework: Cobra; configuration loading: Viper (YAML + environment variable override)

**E6.2: Serverless deployment targets**
As an operator, I want deployment manifests for multiple serverless platforms so that the gateway can be run without managing container infrastructure.

Acceptance Criteria:
- `fly.toml` present and valid for Fly.io deployment
- `vercel.json` present for Vercel serverless functions
- `railway.json` present for Railway deployment
- `render.yaml` present for Render deployment
- `bifrost deploy <platform>` executes the appropriate platform CLI

**E6.3: PostgreSQL and Redis persistence**
As an operator, I want routing state, learning outcomes, and session data persisted to PostgreSQL with Redis caching so that optimization state survives service restarts.

Acceptance Criteria:
- `db/` package provides type-safe SQLC-generated queries (`sqlc.yaml` defines generation config)
- Redis client (`go-redis/v9`) used for hot-path caching of routing weights and session state
- `docker-compose.yml` provides local PostgreSQL and Redis for development

---

## Non-Functional Requirements

| ID | Requirement |
|----|-------------|
| NFR-1 | Single binary distribution: `go build ./cmd/bifrost` produces a standalone executable |
| NFR-2 | All plugins must be goroutine-safe; concurrent `PreHook`/`PostHook` calls must not race |
| NFR-3 | `go vet ./...` and `go build ./...` must pass with zero errors and zero warnings |
| NFR-4 | No modifications to upstream `bifrost/core` or `CLIProxyAPI` source code |
| NFR-5 | OAuth token files must be stored with permission mode 0600 |
| NFR-6 | All server and plugin code must use `schemas.Logger` interface for structured logging |
| NFR-7 | All configuration must support YAML files and environment variable overrides via Viper |
| NFR-8 | File size hard limit: 500 lines per `.go` file; target 350 lines |

## E3: CLI and Deployment

### E3.1: Cobra CLI
**As** an operator, **I want** a CLI (`bifrost init`, `bifrost server`, `bifrost deploy`) **so that** I can manage the gateway from the command line.

### E3.2: Multi-Platform Deployment
**As** an operator, **I want** deployment support for Fly.io, Vercel, Railway, Render, and Homebox **so that** I can deploy to my preferred platform.

## E4: OAuth and Agent Integration

### E4.1: OAuth Proxy with PKCE
**As** a developer, **I want** OAuth proxy support with PKCE flow **so that** I can authenticate with Claude and other providers securely.

### E4.2: AgentAPI SSE Integration
**As** a developer, **I want** SSE event streaming from agent CLI providers **so that** I can integrate with agent workflows in real-time.
