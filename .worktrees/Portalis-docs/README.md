# Portkey - LLM Gateway Abstractions

A library for building LLM-powered applications with consistent interfaces across different providers (OpenAI, Anthropic, Ollama, etc.).

## Features

- **Provider Abstraction**: Single interface for OpenAI, Anthropic, Ollama, and more
- **Hexagonal Architecture**: Clean separation between domain and infrastructure
- **Type Safety**: Full type hints with mypy support
- **Extensible**: Easy to add new providers via port implementations

## Installation

```bash
pip install portkey

# With specific providers
pip install "portkey[openai]"
pip install "portkey[anthropic]"
pip install "portkey[all]"
```

## Quick Start

```python
from portkey import Message, Response
from portkey.domain import CompletionRequest, Provider

# Simple completion
messages = [
    Message(role="user", content="Hello, world!"),
]

request = CompletionRequest(
    messages=messages,
    model="gpt-4",
    temperature=0.7,
)

# Response is a simple domain object
response: Response = await provider.complete(request)
print(response.content)
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  APPLICATION LAYER                                      │
│  • Ports (LLMProvider interface)                     │
│  • Use cases                                          │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│  DOMAIN LAYER (Pure, no dependencies)                  │
│  • Models (Message, Response, CompletionRequest)      │
│  • Errors (LLMError, RateLimitError, etc.)           │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│  INFRASTRUCTURE LAYER (Adapters)                      │
│  • OpenAIProvider                                     │
│  • AnthropicProvider                                  │
│  • OllamaProvider                                     │
│  • InMemoryCache, RedisCache                          │
└─────────────────────────────────────────────────────────┘
```

## Adding a New Provider

```python
from portkey.application.ports import LLMProvider
from portkey.domain import CompletionRequest, Response

class MyProvider(LLMProvider):
    @property
    def provider_name(self) -> str:
        return "my-provider"

    @property
    def supported_models(self) -> list[str]:
        return ["my-model-v1", "my-model-v2"]

    def complete(self, request: CompletionRequest) -> Response:
        # Call your LLM API
        return Response(
            content="response text",
            model=request.model,
            provider=Provider.OPENAI,  # or custom Provider
        )
```

## Comparison Matrix

| Feature | Portkey | LiteLLM | Portkey AI | BAML |
|---------|---------|---------|------------|------|
| Provider Abstraction | ✅ | ✅ | ✅ | ❌ |
| Open Source | ✅ | ✅ | ❌ | ✅ |
| Python Only | ✅ | ✅ | ❌ | ✅ |
| TypeScript Support | ❌ | ❌ | ✅ | ❌ |
| Caching | ✅ | ✅ | ✅ | ❌ |
| Retry Logic | ✅ | ✅ | ✅ | ❌ |
| Streaming | ✅ | ✅ | ✅ | ❌ |

## Supported Providers

| Provider | Status | Models |
|----------|--------|--------|
| OpenAI | Planned | GPT-4, GPT-3.5 |
| Anthropic | Planned | Claude 3, Claude 2 |
| Ollama | Planned | Llama 2, Mistral |
| Azure OpenAI | Planned | GPT-4, GPT-3.5 |
| Google Vertex | Planned | PaLM 2, Gemini |

## Development

```bash
# Install dependencies
pip install -e ".[all]"

# Run tests
pytest tests/

# Type check
mypy src/

# Lint
ruff check src/
```

## License

MIT OR Apache-2.0
