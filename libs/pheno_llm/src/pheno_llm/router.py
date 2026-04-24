"""Unified LLM router using LiteLLM."""
from __future__ import annotations
import os
from typing import Any
import litellm
from litellm import acompletion

class LLMRouter:
    def __init__(self) -> None:
        self._providers = {"openai": "gpt-4", "anthropic": "claude-3", "gemini": "gemini-1.5", "deepseek": "deepseek-chat"}
        self._fallback_chain = ["openai", "anthropic", "gemini"]
    
    def _get_api_key(self, provider: str) -> str | None:
        env_map = {"openai": "OPENAI_API_KEY", "anthropic": "ANTHROPIC_API_KEY", "gemini": "GEMINI_API_KEY", "deepseek": "DEEPSEEK_API_KEY"}
        return os.getenv(env_map.get(provider, ""))
    
    async def route(self, messages: list[dict], model: str = "gpt-4", provider: str = "openai", **kwargs) -> Any:
        model_string = f"{provider}/{model}"
        api_key = self._get_api_key(provider)
        if not api_key: raise ValueError(f"No API key for {provider}")
        return await acompletion(model=model_string, messages=messages, api_key=api_key, **kwargs)
    
    async def route_with_fallback(self, messages: list[dict], model: str, providers: list[str] | None = None, **kwargs) -> Any:
        providers = providers or self._fallback_chain
        last_error = None
        for provider in providers:
            try: return await self.route(messages, model, provider, **kwargs)
            except Exception as e: last_error = e
        raise last_error or RuntimeError("All providers failed")

_default_router: LLMRouter | None = None
def get_router() -> LLMRouter:
    global _default_router
    if _default_router is None: _default_router = LLMRouter()
    return _default_router

async def route_llm(messages: list[dict], model: str = "gpt-4", provider: str = "openai", **kwargs) -> Any:
    return await get_router().route(messages, model, provider, **kwargs)
