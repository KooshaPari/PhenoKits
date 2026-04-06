# Portkey - Feature Comparison Matrix

## Overview

Portkey is an LLM gateway abstraction library providing consistent interfaces across multiple LLM providers.

## Comparison with Alternatives

| Feature | Portkey | LiteLLM | Portkey.ai | Instructor | BAML |
|---------|---------|---------|------------|------------|------|
| **License** | MIT | MIT | Proprietary | Apache 2.0 | BSL |
| **Language** | Python | Python | API | Python | DSL/TS |
| **Open Source** | ✅ | ✅ | ❌ | ✅ | ✅ |
| **Provider Abstraction** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Structured Outputs** | Planned | ✅ | ✅ | ✅ | ✅ |
| **Caching** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Retry Logic** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Rate Limiting** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Tracing/Monitoring** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Streaming** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Function Calling** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Multi-Modal** | Planned | ✅ | ✅ | ✅ | ❌ |
| **Fine-tuning** | ❌ | ❌ | ❌ | ❌ | ❌ |

## Detailed Feature Comparison

### Provider Support

| Provider | Portkey | LiteLLM | Portkey.ai |
|----------|---------|---------|------------|
| OpenAI | ✅ | ✅ | ✅ |
| Anthropic | ✅ | ✅ | ✅ |
| Ollama | ✅ | ✅ | ❌ |
| Azure OpenAI | ✅ | ✅ | ✅ |
| Google Vertex | ✅ | ✅ | ✅ |
| AWS Bedrock | ✅ | ✅ | ✅ |
| Cohere | ✅ | ✅ | ❌ |
| Mistral AI | ✅ | ✅ | ❌ |
| Groq | ✅ | ✅ | ❌ |

### Architecture

| Aspect | Portkey | LiteLLM | Instructor |
|--------|---------|---------|------------|
| Hexagonal Architecture | ✅ | ❌ | ❌ |
| Domain-Driven Design | ✅ | ❌ | ❌ |
| Type Safety | Full | Partial | Full |
| Async Support | ✅ | ✅ | ✅ |
| Sync Support | ✅ | ✅ | ✅ |

### Use Cases

| Use Case | Portkey | LiteLLM | Instructor |
|----------|---------|---------|------------|
| Simple Chat | ✅ | ✅ | ✅ |
| Structured Output | ✅ | ✅ | ✅ |
| Multi-Provider Fallback | ✅ | ✅ | ❌ |
| Cost Optimization | ✅ | ✅ | ❌ |
| Observability | ✅ | ✅ | ❌ |

## Unique Value Proposition

### Why Portkey?

1. **Clean Architecture**: Built on hexagonal/clean architecture principles with clear separation of concerns
2. **Domain-Driven**: Models are pure domain objects with no provider-specific logic
3. **Extensible**: Easy to add custom providers or extend existing ones
4. **Type-Safe**: Full type hints with strict mypy configuration
5. **Phenotype Ecosystem**: Designed to integrate with Phenotype's polyrepo system

### When to Use Alternatives

- **LiteLLM**: If you need extensive built-in integrations with observability platforms
- **Instructor**: If you primarily need structured output parsing (Pydantic)
- **Portkey.ai**: If you prefer a managed API over self-hosted

## Roadmap

| Feature | Status |
|---------|--------|
| Core abstractions | ✅ Implemented |
| OpenAI adapter | 🔨 Planned |
| Anthropic adapter | 🔨 Planned |
| Ollama adapter | 🔨 Planned |
| Redis cache | 🔨 Planned |
| Structured output | 🔨 Planned |
| Streaming | 🔨 Planned |
| Multi-provider fallback | 🔨 Planned |

## References

- [LiteLLM](https://github.com/BerriAI/litellm)
- [Portkey.ai](https://portkey.ai/)
- [Instructor](https://github.com/jxnl/instructor)
- [BAML](https://github.com/BoundaryML/baml)
