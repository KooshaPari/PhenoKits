"""Pheno LLM - Unified LLM routing (LiteLLM)."""
import os

class LLMRouter:
    PROVIDERS = {"openai": "gpt-4", "anthropic": "claude-3", "gemini": "gemini-1.5"}
    
    def __init__(self):
        self._fallback = ["openai", "anthropic", "gemini"]
    
    def _get_api_key(self, provider):
        env_map = {"openai": "OPENAI_API_KEY", "anthropic": "ANTHROPIC_API_KEY", "gemini": "GEMINI_API_KEY"}
        return os.getenv(env_map.get(provider, ""))
    
    async def route(self, messages, model="gpt-4", provider="openai", **kwargs):
        api_key = self._get_api_key(provider)
        if not api_key:
            raise ValueError(f"No API key for {provider}")
        return {"model": model, "provider": provider, "messages": messages}

_router = None

def get_router():
    global _router
    if _router is None:
        _router = LLMRouter()
    return _router

async def route_llm(messages, model="gpt-4", provider="openai", **kwargs):
    return await get_router().route(messages, model, provider, **kwargs)

__all__ = ["LLMRouter", "route_llm", "get_router"]
