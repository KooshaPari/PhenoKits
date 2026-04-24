"""GitHub adapter."""
from __future__ import annotations
import os
from dataclasses import dataclass
from typing import Any
import httpx

@dataclass
class GitHubConfig:
    token: str; base_url: str = "https://api.github.com"; owner: str | None = None; repo: str | None = None

@dataclass
class PullRequest:
    number: int; title: str; state: str; url: str

class GitHubAdapter:
    def __init__(self, config: GitHubConfig | None = None) -> None:
        self.config = config or GitHubConfig(token=os.getenv("GITHUB_TOKEN", ""))
        self._client = httpx.AsyncClient(base_url=self.config.base_url, headers={"Authorization": f"token {self.config.token}", "Accept": "application/vnd.github.v3+json"}, timeout=30.0)
    async def __aenter__(self) -> "GitHubAdapter": return self
    async def __aexit__(self, *args: Any) -> None: await self._client.aclose()
    async def list_prs(self, state: str = "open") -> list[PullRequest]:
        response = await self._client.get(f"/repos/{self.config.owner}/{self.config.repo}/pulls", params={"state": state})
        response.raise_for_status()
        return [PullRequest(number=pr["number"], title=pr["title"], state=pr["state"], url=pr["html_url"]) for pr in response.json()]
