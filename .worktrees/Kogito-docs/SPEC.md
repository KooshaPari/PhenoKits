# Kogito — Technical Specification

## Architecture

```
┌─────────────────────────────────────────────┐
│               CLI (Cobra)                    │
│          bifrost init | server | deploy      │
├─────────────────────────────────────────────┤
│              API Routes                      │
│          gin/fiber HTTP handlers              │
├────────────┬────────────────────────────────┤
│ Services   │  Plugin System                 │
│ business   │  extensible plugin architecture│
│ logic      │  with lifecycle hooks          │
├────────────┴────────────────────────────────┤
│         Configuration (Viper)                │
│        YAML + env vars + secrets             │
├──────────┬──────────┬───────────────────────┤
│PostgreSQL│  Redis   │  Bifrost (upstream)   │
│ migrations│ cache   │  as Go module         │
└──────────┴──────────┴───────────────────────┘
```

## Design Principles

- Clean extension layer pattern over upstream `bifrost` and `cliproxy`
- Zero modifications to upstream repositories
- Plugin-based extensibility for custom providers
- Consumes upstream as Go modules, stays in sync with main developers

## Components

| Component | Location | Responsibility |
|-----------|----------|---------------|
| CLI | `cmd/` | Cobra-based command interface |
| API | `api/` | HTTP route handlers |
| Services | `services/` | Business logic layer |
| Config | `config/` | Viper-based configuration |
| Database | `db/` | PostgreSQL migrations and queries |
| Plugins | `plugins/` | Plugin implementations |
| Docs | `docs/` | Architecture, CLI, deployment guides |

## Data Models

```go
type Config struct {
    Server   ServerConfig   `yaml:"server"`
    Database DatabaseConfig `yaml:"database"`
    Redis    RedisConfig    `yaml:"redis"`
    Plugins  []PluginConfig `yaml:"plugins"`
}

type ServerConfig struct {
    Host     string `yaml:"host"`
    Port     int    `yaml:"port"`
    TLS      bool   `yaml:"tls"`
    LogLevel string `yaml:"log_level"`
}

type DatabaseConfig struct {
    URL          string `yaml:"url"`
    MaxConns     int    `yaml:"max_conns"`
    MigrationsDir string `yaml:"migrations_dir"`
}

type PluginConfig struct {
    Name    string `yaml:"name"`
    Enabled bool   `yaml:"enabled"`
    Path    string `yaml:"path"`
}
```

## API Design

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/health` | Health check |
| GET | `/ready` | Readiness probe |
| POST | `/v1/chat/completions` | LLM gateway proxy |
| GET | `/v1/models` | List available models |
| POST | `/v1/plugins/:name/execute` | Plugin execution |
| GET | `/v1/config` | Current configuration |

## Deployment Targets

| Platform | Command | Notes |
|----------|---------|-------|
| Fly.io | `bifrost deploy fly` | Primary target |
| Vercel | `bifrost deploy vercel` | Serverless |
| Railway | `bifrost deploy railway` | Container |
| Render | `bifrost deploy render` | Container |
| Local | `bifrost server` | Development |

## Performance Targets

| Metric | Target |
|--------|--------|
| Startup time | <2s |
| Request latency (proxy) | <50ms overhead |
| DB migration | <10s for full schema |
| Plugin load time | <500ms per plugin |
| Memory baseline | <64MB |
| Concurrency | 1000+ connections |
