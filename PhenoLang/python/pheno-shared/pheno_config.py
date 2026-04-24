"""Pheno Config - Unified configuration (pydantic-settings)."""
from pydantic_settings import BaseSettings
from pydantic import Field

class Config(BaseSettings):
    model_config = {"env_prefix": "PHENO_", "extra": "ignore"}
    environment: str = Field(default="development")
    debug: bool = Field(default=False)
    version: str = Field(default="0.1.0")
    llm_provider: str = Field(default="openai")
    llm_model: str = Field(default="gpt-4")
    cache_enabled: bool = Field(default=True)
    cache_path: str = Field(default=".cache")

_config = None

def get_config():
    global _config
    if _config is None:
        _config = Config()
    return _config

__all__ = ["Config", "get_config"]
