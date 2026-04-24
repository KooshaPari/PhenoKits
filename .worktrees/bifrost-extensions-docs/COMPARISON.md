# Comparison Matrix

## Feature Comparison

This document compares **Bifrost Extensions** with similar tools in the LLM gateway/extension layer space.

| Repository | Purpose | Key Features | Language/Framework | Maturity | Comparison |
|------------|---------|--------------|-------------------|----------|------------|
| **Bifrost Extensions (this repo)** | Clean extension layer for Bifrost LLM gateway | Plugin system, Multi-provider, Serverless deployment | Go | Stable | Clean extension pattern |
| [bifrost](https://github.com/bifrost) | LLM gateway upstream | Core routing, Provider abstraction | Go | Stable | Upstream core |
| [cliproxy](https://github.com/cliproxy) | CLI proxy for LLM | OpenAI-compatible, Provider routing | Go | Stable | Upstream dependency |
| [litellm](https://github.com/BerriAI/litellm) | LLM proxy | 100+ providers, OpenAI-compatible | Python | Stable | Industry standard proxy |
| [portkey](https://github.com/PortKey-AI/openapi) | LLM gateway | Observability, Retries, Load balancing | Python | Stable | Enterprise-focused |
| [玄天后](https://github.com/teasherg/proxy) | OpenAI-compatible proxy | Multi-provider, Simple setup | Go | Beta | Simpler alternative |
| [go-openai-proxy](https://github.com/linjiao/go-openai-proxy) | OpenAI proxy | Basic routing | Go | Beta | Minimal alternative |

## Detailed Feature Comparison

### Extension & Plugin System

| Feature | Bifrost Extensions | LiteLLM | Portkey | go-openai-proxy |
|---------|-------------------|---------|---------|-----------------|
| Plugin Architecture | ✅ | ❌ | ❌ | ❌ |
| Extension Layer | ✅ | ❌ | ❌ | ❌ |
| Zero Upstream Mods | ✅ | ❌ | ❌ | ❌ |
| Custom Providers | ✅ | ✅ | ✅ | ❌ |

### Deployment Options

| Feature | Bifrost Extensions | LiteLLM | Portkey | go-openai-proxy |
|---------|-------------------|---------|---------|-----------------|
| Fly.io | ✅ | ✅ | ✅ | ❌ |
| Vercel | ✅ | ✅ | ✅ | ❌ |
| Railway | ✅ | ✅ | ✅ | ❌ |
| Docker | ✅ | ✅ | ✅ | ✅ |
| Kubernetes | ✅ | ✅ | ✅ | ❌ |

### Configuration & Observability

| Feature | Bifrost Extensions | LiteLLM | Portkey |
|---------|-------------------|---------|---------|
| Viper Config (YAML) | ✅ | ❌ | ❌ |
| Environment Variables | ✅ | ✅ | ✅ |
| Redis Caching | ✅ | ✅ | ✅ |
| Structured Logging | ✅ | ✅ | ✅ |
| Prometheus Metrics | ✅ | ✅ | ✅ |

## Unique Value Proposition

Bifrost Extensions provides:

1. **Clean Extension Pattern**: Consumes upstream as Go modules without modifications
2. **Plugin Architecture**: Extensible without modifying core
3. **Easy Sync**: Zero modifications to upstream = easy to stay current
4. **Multi-Deployment**: Fly.io, Vercel, Railway, Render, Homebox

## Architecture Pattern

```
┌─────────────────────────────────────────────────────────────┐
│                    Bifrost Extensions                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Extension Layer (Plugins)                │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│  ┌─────────────────┐  ┌─────────────────┐                │
│  │  bifrost       │  │  cliproxy       │                │
│  │  (upstream)    │  │  (upstream)     │                │
│  └─────────────────┘  └─────────────────┘                │
└─────────────────────────────────────────────────────────────┘
```

vs LiteLLM (monolithic):
```
┌─────────────────────────────────────────────────────────────┐
│                         LiteLLM                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │
│  │  Provider A │  │  Provider B │  │  Provider C │       │
│  └─────────────┘  └─────────────┘  └─────────────┘       │
│         │               │               │                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Unified OpenAI-compatible API           │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## When to Use What

| Use Case | Recommended Tool |
|----------|-----------------|
| Need plugin extensibility | Bifrost Extensions |
| Quick setup, many providers | LiteLLM |
| Enterprise observability | Portkey |
| Minimal setup | go-openai-proxy |
| Part of Phenotype ecosystem | Bifrost Extensions |

## References

- Upstream: [bifrost](https://github.com/bifrost), [cliproxy](https://github.com/cliproxy)
- LiteLLM: [BerriAI/litellm](https://github.com/BerriAI/litellm)
- Portkey: [PortKey-AI/openapi](https://github.com/PortKey-AI/openapi)
