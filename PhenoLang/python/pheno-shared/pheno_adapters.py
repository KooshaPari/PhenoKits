"""Pheno Adapters - Unified API adapters (GitHub, Linear, Workstream)."""
import os
import httpx

class GitHubAdapter:
    def __init__(self, token=None):
        self.token = token or os.getenv("GITHUB_TOKEN", "")
        self._client = httpx.AsyncClient(
            base_url="https://api.github.com",
            headers={
                "Authorization": f"token {self.token}",
                "Accept": "application/vnd.github.v3+json"
            },
            timeout=30.0
        )
    
    async def list_prs(self, state="open"):
        response = await self._client.get("/pulls", params={"state": state})
        response.raise_for_status()
        return response.json()

class LinearAdapter:
    def __init__(self, api_key=None):
        self.api_key = api_key or os.getenv("LINEAR_API_KEY", "")
        self._client = httpx.AsyncClient(
            headers={"Authorization": self.api_key},
            timeout=30.0
        )
    
    async def list_issues(self):
        response = await self._client.post(
            "https://api.linear.app/graphql",
            json={"query": "query { issues(first: 50) { nodes { id identifier title } } }"}
        )
        response.raise_for_status()
        return response.json().get("data", {}).get("issues", {}).get("nodes", [])

class WorkStreamAdapter:
    def __init__(self, api_key=None):
        self.api_key = api_key or os.getenv("WORKSTREAM_API_KEY", "")
        self._client = httpx.AsyncClient(
            base_url="https://api.workstream.io/v1",
            headers={"Authorization": f"Bearer {self.api_key}"},
            timeout=30.0
        )
    
    async def list_tasks(self):
        response = await self._client.get("/tasks")
        response.raise_for_status()
        return response.json().get("tasks", [])

__all__ = ["GitHubAdapter", "LinearAdapter", "WorkStreamAdapter"]
