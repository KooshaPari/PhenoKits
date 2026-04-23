"""Workstream adapter."""
from __future__ import annotations
import os
from dataclasses import dataclass
from typing import Any
import httpx

@dataclass
class WorkstreamConfig:
    api_key: str; base_url: str = "https://api.workstream.io/v1"

@dataclass
class WorkstreamTask:
    id: str; title: str; status: str

class WorkstreamAdapter:
    def __init__(self, config: WorkstreamConfig | None = None) -> None:
        self.config = config or WorkstreamConfig(api_key=os.getenv("WORKSTREAM_API_KEY", ""))
        self._client = httpx.AsyncClient(base_url=self.config.base_url, headers={"Authorization": f"Bearer {self.config.api_key}"}, timeout=30.0)
    async def __aenter__(self) -> "WorkstreamAdapter": return self
    async def __aexit__(self, *args: Any) -> None: await self._client.aclose()
    async def list_tasks(self) -> list[WorkstreamTask]:
        response = await self._client.get("/tasks"); response.raise_for_status()
        return [WorkstreamTask(id=t["id"], title=t["title"], status=t["status"]) for t in response.json().get("tasks", [])]
