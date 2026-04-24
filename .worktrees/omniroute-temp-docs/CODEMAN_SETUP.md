# OmniRoute Setup for Codeman

## Summary

Fixed the "Worker" combo and created optimized combos for reliable AI routing.

## Changes Made

### 1. Fixed Worker Combo
**Before:** 23 models including broken providers  
**After:** 10 working models

**Removed:**
- `opencode-zen/*` - credits_exhausted
- `nvidia/*` - unavailable
- `if/deepseek-v3.2` - unknown provider

**Remaining:**
- gemini-cli/gemini-3-flash-preview
- gh/oswe-vscode-prime
- gh/gpt-5-mini
- cx/gpt-5.1-codex-mini
- kc/openrouter/free
- kg/openrouter/free
- openrouter/openrouter/free
- kg/qwen/qwen3.6-plus-preview:free
- kg/stepfun/step-3.5-flash:free
- gemini/gemini-3-flash-preview

### 2. Created FreeTier Combo
10 free-tier models from working providers:
- Gemini CLI & Gemini (free tier)
- Kilo Gateway -> OpenRouter free
- KiloCode -> OpenRouter free
- Direct OpenRouter free
- Kilo -> Qwen free
- Kilo -> Stepfun free
- GitHub Copilot free models

### 3. Created Codeman Combo
9 reliable coding models:
- gemini/gemini-3.1-pro
- gemini/gemini-3-flash-preview
- cx/gpt-5.4
- cx/gpt-5.4-mini
- opencode-go/minimax-m2.7
- minimax/minimax-m2.7-highspeed
- groq/llama-4-scout
- openrouter/anthropic/claude-3.5-sonnet
- gh/gpt-5.1-codex

## API Access

### Your API Key
```
ork_codeman_ab58897afde6452a
```

### Endpoints
- Dashboard: http://localhost:20128/dashboard
- API Base: http://localhost:20128/v1
- Chat Completions: http://localhost:20128/v1/chat/completions

### Usage

#### cURL
```bash
curl http://localhost:20128/v1/chat/completions \
  -H 'Authorization: Bearer ork_codeman_ab58897afde6452a' \
  -H 'Content-Type: application/json' \
  -d '{"model":"Codeman","messages":[{"role":"user","content":"Hello"}]}'
```

#### Environment Variables
```bash
export OPENAI_API_KEY='ork_codeman_ab58897afde6452a'
export OPENAI_BASE_URL='http://localhost:20128/v1'
export OPENAI_MODEL='Codeman'
```

#### Claude Code
Add to `~/.claude/config.json`:
```json
{
  "apiKey": "ork_codeman_ab58897afde6452a",
  "baseUrl": "http://localhost:20128/v1",
  "model": "Codeman"
}
```

## Available Providers (Active)

| Provider | Type | Status |
|----------|------|--------|
| openrouter | apikey | active |
| claude | oauth | active |
| kilocode | oauth | active |
| codex | oauth | active |
| github | oauth | active |
| antigravity | oauth | active |
| kimi-coding | oauth | active |
| qwen | oauth | active |
| opencode-go | apikey | active |
| gemini | apikey | active |
| kiro | oauth | active |
| minimax | apikey | active |
| groq | apikey | active |
| kilo-gateway | apikey | active |

## Broken Providers (Excluded)

| Provider | Status |
|----------|--------|
| opencode-zen | credits_exhausted |
| nvidia | unavailable |

## Combo Selection Guide

| Combo | Use Case | Cost |
|-------|----------|------|
| **Codeman** | Reliable coding tasks | Varies (OAuth + API keys) |
| **FreeTier** | Zero-cost experimentation | Free |
| **Worker** | Load-balanced parallel tasks | Varies |
| **Main** | Balanced performance | Varies |

---

*Setup completed: 2026-04-02*
*OmniRoute version: 3.4.1*
