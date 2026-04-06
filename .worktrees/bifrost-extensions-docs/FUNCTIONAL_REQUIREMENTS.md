# Functional Requirements — Bifrost Extensions

**Version:** 1.0.0
**Status:** Active
**Traces to:** PRD.md epics E1–E6

---

## FR-PLG: Plugin System

- **FR-PLG-001:** Every plugin SHALL implement the `schemas.Plugin` interface: `GetName() string`, `PreHook(*context.Context, *BifrostRequest) (*BifrostRequest, *PluginShortCircuit, error)`, `PostHook(*context.Context, *BifrostResponse, *BifrostError) (*BifrostResponse, *BifrostError, error)`, `TransportInterceptor(*context.Context, string, map[string]string, map[string]any) (map[string]string, map[string]any, error)`, `Cleanup() error`. Traces to: E1.2
- **FR-PLG-002:** `PreHook` SHALL support exactly three outcome paths: (a) return modified request to continue, (b) return `PluginShortCircuit` to bypass provider with a pre-computed response, (c) return a non-nil `error` to abort. Traces to: E1.2
- **FR-PLG-003:** `PostHook` SHALL support exactly two outcome paths: (a) return modified response to continue, (b) return modified `BifrostError` to replace response with error. Traces to: E1.2
- **FR-PLG-004:** All plugin implementations SHALL be goroutine-safe; concurrent hook calls on the same plugin instance MUST NOT cause data races. Traces to: NFR-2
- **FR-PLG-005:** Plugin `Cleanup()` SHALL release all resources (connections, goroutines) and SHALL NOT block indefinitely. Traces to: E1.2

---

## FR-SRV: HTTP Server

- **FR-SRV-001:** The server SHALL expose `POST /v1/chat/completions` accepting an OpenAI-format chat completion request body. Traces to: E1.3
- **FR-SRV-002:** The server SHALL expose `POST /v1/completions` accepting an OpenAI-format text completion request body. Traces to: E1.3
- **FR-SRV-003:** Response bodies SHALL conform to OpenAI API response shape (`id`, `object`, `created`, `model`, `choices`). Traces to: E1.3
- **FR-SRV-004:** The server SHALL be implemented using the Chi router (`go-chi/chi/v5`) with CORS middleware enabled. Traces to: E1.3
- **FR-SRV-005:** The server SHALL accept a `schemas.Logger` instance at construction and use it for all log output. Traces to: NFR-6

---

## FR-RTR: Intelligent Routing

- **FR-RTR-001:** `IntelligentRouter.PreHook` SHALL extract task features from the incoming `BifrostRequest` and produce a `RoutingDecision` before any provider call. Traces to: E2.1
- **FR-RTR-002:** `RoutingDecision` SHALL contain: `SelectedModel`, `SelectedProvider`, `SelectedEndpointID`, `TaskType`, `RiskLevel`, `Confidence`, `Alternatives`, `FallbackEndpoints`, `ContextStrategy`, `Reasoning`, `CostEstimate`, `QuotaHeadroom`. Traces to: E2.1
- **FR-RTR-003:** The `RoutingDecision` SHALL be stored in the request context using a private `contextKey` and read back in `PostHook` for outcome logging. Traces to: E2.1
- **FR-RTR-004:** `Config.RouteLLMEnabled` SHALL gate use of the RouteLLM router; when true, `Config.RouteLLMRouter` selects the algorithm (`mf`, `bert`, `sw_ranking`) and `Config.RouteLLMThreshold` (0.0–1.0) determines strong-model selection. Traces to: E2.4
- **FR-RTR-005:** `Config.MIRTEnabled` SHALL gate use of the MIRT quality-cost optimizer. Traces to: E2.1
- **FR-RTR-006:** `Config.ArchRouterEndpoint` and `Config.ArchRouterModel` SHALL configure the local Arch-Router service used for task classification. Traces to: E2.3
- **FR-RTR-007:** `TaskType` SHALL be one of: `tool_call`, `code_generation`, `reasoning`, `conversation`, `default`. Traces to: E2.3
- **FR-RTR-008:** `RiskLevel` SHALL be one of: `low`, `medium`, `high`. Traces to: E2.3
- **FR-RTR-009:** `Config.MaxCostPerRequest` (default 1.00 USD) SHALL act as a hard ceiling; endpoints exceeding this estimate SHALL be excluded from selection. Traces to: E2.2
- **FR-RTR-010:** `Config.PreferSubscriptions` (default true) SHALL prefer subscription-bucket endpoints over pay-per-token endpoints of equal quality. Traces to: E2.2
- **FR-RTR-011:** `Config.AllowScarceEndpoints` (default false) SHALL gate whether `scarce_premium` tier endpoints may be selected. Traces to: E2.2

---

## FR-LRN: Learning Subsystem

- **FR-LRN-001:** `TieredLearningSystem.RecordOutcome` SHALL be called from `IntelligentRouter.PostHook` with the `RoutingDecision` and response/error outcome. Traces to: E2.5
- **FR-LRN-002:** `ThreePillarOptimizer` SHALL aggregate latency, cost, and error-rate signals across recorded outcomes and adjust routing weights accordingly. Traces to: E2.5
- **FR-LRN-003:** Learning state SHALL persist across service restarts via PostgreSQL using SQLC-generated queries in the `db/` package. Traces to: E2.5

---

## FR-FBK: Smart Fallback

- **FR-FBK-001:** `smartfallback` plugin `PostHook` SHALL detect a non-nil `BifrostError` and re-submit the request with the next fallback from the configured chain. Traces to: E2.6
- **FR-FBK-002:** The fallback chain SHALL be expressed as `[]schemas.Fallback` with `Model` and `Provider` fields set on the original request. Traces to: E2.6
- **FR-FBK-003:** When all fallbacks are exhausted, the plugin SHALL return the original error unchanged. Traces to: E2.6

---

## FR-SAF: Content Safety

- **FR-SAF-001:** `contentsafety` plugin `PreHook` SHALL evaluate inbound request content against configured policy rules and return a `PluginShortCircuit` refusal when policy is violated. Traces to: E3.1
- **FR-SAF-002:** `contentsafety` plugin `PostHook` SHALL evaluate outbound response content and replace violating responses with a safe fallback `BifrostResponse`. Traces to: E3.1

---

## FR-CTX: Context Folding

- **FR-CTX-001:** `contextfolding` plugin `PreHook` SHALL reduce conversation history length when the estimated token count exceeds a configurable threshold. Traces to: E3.2
- **FR-CTX-002:** Compression strategy SHALL be configurable: `truncate-oldest` removes oldest messages first; `slm-summarize` calls the SLM summarizer endpoint at `Config.SummarizerSLMURL`. Traces to: E3.2
- **FR-CTX-003:** The plugin SHALL record original message count and post-fold message count in the request context for observability. Traces to: E3.2

---

## FR-PRM: Prompt Adaptation

- **FR-PRM-001:** `promptadapter` plugin `PreHook` SHALL rewrite prompt structure (system prompt placement, role normalization) according to per-provider adapter rules. Traces to: E3.3
- **FR-PRM-002:** No semantic content SHALL be lost during adaptation; role mapping SHALL be complete and reversible. Traces to: E3.3

---

## FR-AGT: CLI Agent Provider

- **FR-AGT-001:** `agentcli.Provider` SHALL implement `schemas.Provider` and return a provider key of the form `agentcli-<agent_type>`. Traces to: E4.1
- **FR-AGT-002:** `ChatCompletion` SHALL extract the last user-role message from `req.ChatRequest.Input` (iterating in reverse). Traces to: E4.1
- **FR-AGT-003:** `ChatCompletion` SHALL return HTTP 400 `BifrostError` when no user message is found. Traces to: E4.1
- **FR-AGT-004:** `waitForStable` SHALL poll `GET <baseURL>/status` at `Config.PollInterval` intervals until `status == "stable"` or `Config.MaxWaitTime` is exceeded. Traces to: E4.2
- **FR-AGT-005:** Timeout in `waitForStable` SHALL return a 503 `BifrostError` with a descriptive message. Traces to: E4.2
- **FR-AGT-006:** Supported `AgentType` values SHALL include: `claude`, `codex`, `goose`, `aider`, `gemini`, `copilot`, `amp`, `cursor`, `auggie`, `amazonq`, `opencode`. Traces to: E4.1
- **FR-AGT-007:** The response assembled from CLI agent output SHALL be returned as a `BifrostChatResponse` with `object = "chat.completion"` and `finish_reason = "stop"`. Traces to: E4.1

---

## FR-OAU: OAuth Proxy

- **FR-OAU-001:** `GeneratePKCE()` SHALL generate a 32-byte random code verifier (base64url encoded) and a SHA-256 code challenge (base64url encoded, no padding). Traces to: E5.1, E5.2
- **FR-OAU-002:** `ClaudeOAuth.GenerateAuthURL` SHALL produce a URL targeting `https://claude.ai/oauth/authorize` with parameters: `response_type=code`, `client_id`, `redirect_uri`, `scope`, `code_challenge`, `code_challenge_method=S256`, `state`. Traces to: E5.1
- **FR-OAU-003:** `ClaudeOAuth.ExchangeCode` SHALL POST to `https://console.anthropic.com/v1/oauth/token` with `application/json` body containing `code`, `grant_type=authorization_code`, `client_id`, `redirect_uri`, `code_verifier`. Traces to: E5.1
- **FR-OAU-004:** `CodexOAuth.GenerateAuthURL` SHALL produce a URL targeting `https://auth.openai.com/authorize` with audience `https://api.openai.com/v1`. Traces to: E5.2
- **FR-OAU-005:** `CodexOAuth.ExchangeCode` SHALL POST to `https://auth.openai.com/oauth/token` with `application/x-www-form-urlencoded` body. Traces to: E5.2
- **FR-OAU-006:** `TokenData.NeedsRefresh()` SHALL return true when `time.Now().Add(5 * time.Minute).After(ExpiresAt)`. Traces to: E5.3
- **FR-OAU-007:** `TokenData.IsExpired()` SHALL return true when `time.Now().After(ExpiresAt)`. Traces to: E5.3
- **FR-OAU-008:** `OAuthManager.SaveTokens` SHALL write to `<cacheDir>/oauth_tokens.json` with file permission 0600. Traces to: E5.3, NFR-5
- **FR-OAU-009:** `OAuthManager.LoadTokens` SHALL return an empty map (not an error) when the token file does not exist. Traces to: E5.3

---

## FR-CLI: CLI

- **FR-CLI-001:** The `bifrost` binary SHALL expose an `init` subcommand that scaffolds project configuration files. Traces to: E6.1
- **FR-CLI-002:** The `bifrost` binary SHALL expose a `server` subcommand that starts the HTTP server using plugins loaded from the YAML configuration file. Traces to: E6.1
- **FR-CLI-003:** The `bifrost` binary SHALL expose a `deploy` subcommand with platform targets: `fly`, `vercel`, `railway`, `render`, `homebox`. Traces to: E6.2
- **FR-CLI-004:** CLI framework SHALL be Cobra (`spf13/cobra`); configuration loading SHALL use Viper (`spf13/viper`) supporting YAML files and environment variable overrides. Traces to: E6.1

---

## FR-PRS: Persistence

- **FR-PRS-001:** The `db/` package SHALL provide type-safe PostgreSQL access via SQLC-generated Go code (`sqlc.yaml` defines generation configuration). Traces to: E6.3
- **FR-PRS-002:** Redis SHALL be used (via `go-redis/v9`) for hot-path caching of routing weights and session state. Traces to: E6.3
- **FR-PRS-003:** A `docker-compose.yml` SHALL provide local PostgreSQL and Redis instances for development. Traces to: E6.3

## FR-CFG: Configuration

- **FR-CFG-001:** Configuration SHALL support YAML files and environment variable overrides via Viper.
- **FR-CFG-002:** The `EnhancedAccount` SHALL support per-provider key and config overrides with fallback.

## FR-AUTH: OAuth

- **FR-AUTH-001:** OAuth proxy SHALL support PKCE flow for Claude authentication.
- **FR-AUTH-002:** Token store SHALL persist tokens to JSON file with expiration tracking.
- **FR-AUTH-003:** Background token refresh SHALL occur before expiration.

## FR-AGT: Agent Integration

- **FR-AGT-001:** AgentCLI provider SHALL support SSE event streaming.
- **FR-AGT-002:** TUI screen capture SHALL be available via agent client.

## FR-SRV: Server

- **FR-SRV-001:** HTTP server SHALL expose OpenAI-compatible endpoints (`/v1/chat/completions`).
- **FR-SRV-002:** Server SHALL use Chi router with middleware support.

---

## FR-SLM: Local SLM Integration

- **FR-SLM-001:** `RouteRequest` struct SHALL contain: `Conversation` ([]Message), `Role` (string), `RiskLevel` (string — "low"/"medium"/"high"), `TaskSummary` (string), `Candidates` ([]RouteCandidate), `Policies` ([]string), `Limits` ([]string), `EstimatedTokensIn` (int), `EstimatedTokensOut` (int). Traces to: E2.3
- **FR-SLM-002:** `RouteCandidate` struct SHALL contain: `EndpointID`, `ModelName`, `Qualities` (map[string]float64), `Traits` (map[string]float64), `Cost` (CandidateCost), `LatencyMS` (int), `QuotaHeadroom` (float64 0.0–1.0), `BillingNotes` (string). Traces to: E2.3
- **FR-SLM-003:** `RouteResponse` SHALL contain: `RouteID`, `PrimaryEndpointID` (string), `FallbackEndpointIDs` ([]string); the SLM router client SHALL POST to `Config.ArchRouterEndpoint` at `/v1/route`. Traces to: E2.3
- **FR-SLM-004:** The SLM summarizer client SHALL POST conversation history to `Config.SummarizerSLMURL` at `/v1/summarize` and return a condensed context string for inclusion in subsequent requests. Traces to: E3.2
- **FR-SLM-005:** The SLM validator client SHALL POST generated responses to a configured validator endpoint at `/v1/validate` to perform content safety checks before returning responses to callers. Traces to: E4.1
- **FR-SLM-006:** All SLM client HTTP calls SHALL use a shared `*http.Client` with a configurable timeout (default 10 s); timeouts SHALL return an error to the caller rather than blocking indefinitely. Traces to: NFR-3
