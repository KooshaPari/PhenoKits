"""Unified configuration using pydantic-settings."""
from __future__ import annotations
import os
from pathlib import Path
from typing import Any
from pydantic import Field, field_validator
from pydantic_settings import BaseSettings, SettingsConfigDict

class Config(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="PHENO_", config_file=".pheno.toml", extra="ignore")
    
    environment: str = Field(default="development")
    debug: bool = Field(default=False)
    version: str = Field(default="0.1.0")
    
    # Sub-configs
    llm_provider: str = Field(default="openai")
    llm_model: str = Field(default="gpt-4")
    cache_enabled: bool = Field(default=True)
    cache_path: Path = Field(default=Path(".cache"))
    
    @field_validator("llm_provider", mode="before")
    @classmethod
    def resolve_llm_provider(cls, v):
        return v or os.getenv("LLM_PROVIDER", "openai")
    
    def to_dict(self) -> dict[str, Any]:
        return self.model_dump()

_config: Config | None = None

def get_config() -> Config:
    global _config
    if _config is None:
        _config = Config()
    return _config
