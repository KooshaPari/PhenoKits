# OmniRoute Specification

> Free AI Gateway — universal API proxy with smart routing, 60+ providers, MCP server, and A2A protocol

## Overview

OmniRoute provides a single API endpoint that routes requests to 60+ AI providers with automatic fallback, supporting chat completions, embeddings, image generation, video, audio, reranking, and web search.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                       OmniRoute Gateway                          │
│                                                                  │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│  │   Router     │ │   Provider   │ │   MCP Server │          │
│  │   Engine     │ │   Registry   │ │   (25 tools) │          │
│  └──────┬───────┘ └──────┬───────┘ └──────┬───────┘          │
│         └────────────────┼────────────────┘                     │
│                          │                                       │
│  ┌──────────────┐ ┌──────┴───────┐ ┌──────────────┐          │
│  │   A2A        │ │   Fallback   │ │   Electron   │          │
│  │   Protocol   │ │   Manager    │ │   Desktop    │          │
│  └──────────────┘ └──────────────┘ └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

## Components

| Component         | Description                                             |
| ----------------- | ------------------------------------------------------- |
| Router Engine     | Request routing with smart provider selection           |
| Provider Registry | 60+ provider adapters (OpenAI, Anthropic, Google, etc.) |
| Fallback Manager  | Automatic failover on provider errors                   |
| MCP Server        | 25 Model Context Protocol tools                         |
| A2A Protocol      | Agent-to-Agent communication                            |
| Desktop App       | Electron-based management UI                            |

## API Surface

```bash
POST /v1/chat/completions    # Chat completions
POST /v1/embeddings          # Embeddings
POST /v1/images/generations  # Image generation
POST /v1/audio/transcriptions # Audio transcription
POST /v1/rerank              # Reranking
GET  /v1/models              # List available models
```

## Provider Routing

```yaml
routing:
  strategy: smart # smart | round-robin | cost-optimized | latency-optimized
  fallback:
    enabled: true
    max_retries: 3
  providers:
    - name: openai
      priority: 1
      models: [gpt-4, gpt-3.5-turbo]
    - name: anthropic
      priority: 2
      models: [claude-3-sonnet, claude-3-haiku]
    - name: google
      priority: 3
      models: [gemini-pro]
```

## Performance Targets

| Metric              | Target         |
| ------------------- | -------------- |
| Request routing     | <10ms overhead |
| Provider failover   | <500ms         |
| Concurrent requests | 10K+           |
| Provider count      | 60+            |
