"""Linear adapter."""
from __future__ import annotations
import os
from dataclasses import dataclass
from typing import Any
import httpx

@dataclass
class LinearConfig:
    api_key: str; base_url: str = "https://api.linear.app/graphql"

@dataclass
class LinearIssue:
    id: str; identifier: str; title: str; state: str

class LinearAdapter:
    def __init__(self, config: LinearConfig | None = None) -> None:
        self.config = config or LinearConfig(api_key=os.getenv("LINEAR_API_KEY", ""))
        self._client = httpx.AsyncClient(headers={"Authorization": self.config.api_key}, timeout=30.0)
    async def __aenter__(self) -> "LinearAdapter": return self
    async def __aexit__(self, *args: Any) -> None: await self._client.aclose()
    async def list_issues(self) -> list[LinearIssue]:
        response = await self._client.post(self.config.base_url, json={"query": "query { issues(first: 50) { nodes { id identifier title state { name } } } }"})
        response.raise_for_status()
        data = response.json()
        return [LinearIssue(id=i["id"], identifier=i["identifier"], title=i["title"], state=i["state"]["name"]) for i in data["data"]["issues"]["nodes"]]
