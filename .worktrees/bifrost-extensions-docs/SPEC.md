# SPEC.md — bifrost-extensions

## System Architecture

```
┌─────────────────────────────────────────────────────┐
│                    CLI (Cobra)                       │
│              bifrost init / server / deploy          │
├─────────────────────────────────────────────────────┤
│                  HTTP Server (Chi)                   │
│         OpenAI-compatible /v1/* endpoints            │
├──────────┬──────────┬───────────────────────────────┤
│ Plugins  │ Providers│         Config (Viper)         │
│          │          │     YAML + env variables       │
├──────────┤          ├───────────────────────────────┤
│IntelRouter│ AgentCLI │    PostgreSQL (migrations)    │
│Learning  │ OAuthProxy│    Redis (caching)            │
│SmartFall │          │                               │
├──────────┴──────────┴───────────────────────────────┤
│            Bifrost Core (schemas.Plugin)             │
│           Upstream module — never forked             │
└─────────────────────────────────────────────────────┘
```

## Components

| Component | Location | Purpose |
|-----------|----------|---------|
| CLI | `cmd/bifrost-enhanced/` | Cobra entrypoint, subcommands |
| Server | `server/` | Chi router, OpenAI-compatible API |
| Intelligent Router | `plugins/intelligentrouter/` | RouteLLM, Arch-Router, MIRT scoring |
| Learning | `plugins/learning/` | Latency/tokens/success tracking, pattern detection |
| Smart Fallback | `plugins/smartfallback/` | Exponential backoff, budget-aware fallbacks |
| AgentCLI Provider | `providers/agentcli/` | Wraps agentapi HTTP for Claude, Cursor, Goose |
| OAuth Proxy | `providers/oauthproxy/` | PKCE OAuth2 proxy through CLIProxyAPI |
| Account | `account/` | EnhancedAccount implementing schemas.Account |
| Config | `config/` | Viper-based YAML + env loading |

## Plugin Interface

```go
type Plugin interface {
    GetName() string
    TransportInterceptor(ctx, url, headers, body) (headers, body, error)
    PreHook(ctx, req) (req, *PluginShortCircuit, error)
    PostHook(ctx, resp, err) (resp, err, error)
    Cleanup() error
}
```

## Data Models

```go
type BifrostRequest struct {
    RequestType  RequestType
    ChatRequest  *BifrostChatRequest
    // union — exactly one field set
}

type BifrostResponse struct {
    ChatResponse *BifrostChatResponse
    // union
}

type BifrostError struct {
    StatusCode     *int
    Error          *ErrorField
    AllowFallbacks *bool
}

type RoutingDecision struct {
    Provider   ModelProvider
    Model      string
    Strategy   string
    Confidence float64
}
```

## API Surface

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/v1/chat/completions` | POST | OpenAI-compatible chat |
| `/v1/completions` | POST | Text completion |
| `/health` | GET | Health check |
| `/metrics` | GET | Prometheus metrics |

## Extension Points

- **New routing strategy**: add file in `plugins/intelligentrouter/`, wire into `decision.go`
- **New CLI agent**: add `AgentType` constant in `providers/agentcli/`
- **New OAuth provider**: add `ProviderType` in `providers/oauthproxy/`, update `getEndpoint()`

## Performance Targets

| Metric | Target |
|--------|--------|
| Routing decision latency | < 10ms (local), < 100ms (remote classifier) |
| Request overhead (plugin chain) | < 5ms per plugin |
| Fallback activation | < 50ms after primary failure |
| Cold start | < 2s with 3 plugins loaded |
| Concurrent requests | 1000+ sustained |

## Constraints

- Zero modifications to upstream `bifrost` and `cliproxy` modules
- All plugins implement `schemas.Plugin` interface
- Config via Viper (YAML + env vars), no hardcoded secrets
- PostgreSQL migrations required before first run
- Go 1.24+, tested on Linux and macOS
