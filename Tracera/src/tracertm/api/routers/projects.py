"""Project API endpoints."""

from __future__ import annotations

import hashlib
import json
import logging
from typing import Annotated, Any

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel
from sqlalchemy import delete
from sqlalchemy.ext.asyncio import AsyncSession

from tracertm.api.deps import auth_guard, get_cache_service, get_db
from tracertm.api.security import (
    _maybe_await,
    ensure_project_access,
    ensure_write_permission,
)
from tracertm.models.item import Item
from tracertm.models.project import Project
from tracertm.repositories import item_repository, link_repository, project_repository
from tracertm.services.cache_service import CacheService

logger = logging.getLogger(__name__)
router = APIRouter(tags=["projects"])


class CreateProjectRequest(BaseModel):
    """Request model for create project endpoint."""

    name: str
    description: str | None = None
    metadata: dict[str, Any] | None = None


class UpdateProjectRequest(BaseModel):
    """Request model for update project endpoint."""

    name: str | None = None
    description: str | None = None
    metadata: dict[str, Any] | None = None


class ImportRequest(BaseModel):
    """Request model for import endpoint."""

    format: str
    data: str


async def _ensure_default_account_for_user(db: AsyncSession, user_id: str) -> str:
    """Ensure the user has a personal account for project ownership."""
    from tracertm.models.account_user import AccountRole
    from tracertm.repositories.account_repository import AccountRepository

    account_repo = AccountRepository(db)
    accounts = await account_repo.list_by_user(user_id)
    if accounts:
        return accounts[0].id
    slug = "personal-" + hashlib.md5(user_id.encode(), usedforsecurity=False).hexdigest()[:12]
    account = await account_repo.create(
        name="My Workspace",
        slug=slug,
        account_type="personal",
    )
    await account_repo.add_user(account.id, user_id, AccountRole.OWNER)
    await db.commit()
    return account.id


@router.get("/api/v1/projects")
async def list_projects(
    skip: int = 0,
    limit: int = 100,
    claims: Annotated[dict[str, Any], Depends(auth_guard)] = None,
    db: Annotated[AsyncSession, Depends(get_db)] = None,
    cache: Annotated[CacheService, Depends(get_cache_service)] = None,
) -> dict[str, Any]:
    """List all projects."""
    user_id = claims.get("sub") if isinstance(claims, dict) else None
    if user_id:
        await _ensure_default_account_for_user(db, user_id)

    cache_key = cache._generate_key("projects", user_id=user_id or "", skip=skip, limit=limit)
    cached = await cache.get(cache_key)
    if cached is not None:
        return cached

    repo = project_repository.ProjectRepository(db)
    projects = await repo.get_all()
    result = {
        "total": len(projects),
        "projects": [
            {
                "id": str(project.id),
                "name": project.name,
                "description": getattr(project, "description", None),
                "metadata": getattr(project, "metadata", {}) or {},
                "created_at": project.created_at.isoformat()
                if getattr(project, "created_at", None)
                else None,
            }
            for project in projects[skip : skip + limit]
        ],
    }
    await cache.set(cache_key, result, cache_type="projects")
    return result


@router.get("/api/v1/projects/{project_id}")
async def get_project(
    project_id: str,
    claims: Annotated[dict[str, Any], Depends(auth_guard)],
    db: Annotated[AsyncSession, Depends(get_db)],
    cache: Annotated[CacheService, Depends(get_cache_service)],
) -> dict[str, Any]:
    """Get a specific project."""
    ensure_project_access(project_id, claims)
    cache_key = cache._generate_key("project", project_id=project_id)
    cached = await cache.get(cache_key)
    if cached is not None:
        return cached

    repo = project_repository.ProjectRepository(db)
    project = await repo.get_by_id(project_id)
    if not project:
        raise HTTPException(status_code=404, detail="Project not found")

    project_metadata = getattr(project, "project_metadata", None) or getattr(project, "metadata", None) or {}
    description = getattr(project, "description", None)
    if not description and isinstance(project_metadata, dict):
        description = project_metadata.get("description")

    result = {
        "id": str(project.id),
        "name": project.name,
        "description": description,
        "metadata": project_metadata,
        "created_at": project.created_at.isoformat() if getattr(project, "created_at", None) else None,
        "updated_at": project.updated_at.isoformat() if getattr(project, "updated_at", None) else None,
    }
    await cache.set(cache_key, result, cache_type="project")
    return result


@router.post("/api/v1/projects")
async def create_project(
    request: CreateProjectRequest,
    claims: Annotated[dict[str, Any], Depends(auth_guard)],
    db: Annotated[AsyncSession, Depends(get_db)],
    cache: Annotated[CacheService, Depends(get_cache_service)],
) -> dict[str, Any]:
    """Create a new project."""
    ensure_write_permission(claims, action="create_project")
    user_id = claims.get("sub") if isinstance(claims, dict) else None
    account_id = await _ensure_default_account_for_user(db, user_id) if user_id else None

    repo = project_repository.ProjectRepository(db)
    try:
        project = await repo.create(
            name=request.name,
            description=request.description,
            metadata=request.metadata,
            account_id=account_id,
        )
        await db.commit()
        await cache.clear_prefix("projects")
        return {
            "id": str(project.id),
            "name": project.name,
            "description": project.description,
            "metadata": project.metadata,
        }
    except Exception as exc:
        await db.rollback()
        logger.exception("Error creating project")
        raise HTTPException(status_code=500, detail=str(exc)) from exc


@router.put("/api/v1/projects/{project_id}")
async def update_project(
    project_id: str,
    request: UpdateProjectRequest,
    claims: Annotated[dict[str, Any], Depends(auth_guard)],
    db: Annotated[AsyncSession, Depends(get_db)],
) -> dict[str, Any]:
    """Update a project."""
    ensure_project_access(project_id, claims)
    ensure_write_permission(claims, action="update_project")
    repo = project_repository.ProjectRepository(db)
    project = await repo.update(
        project_id=project_id,
        name=request.name,
        description=request.description,
        metadata=request.metadata,
    )
    if not project:
        raise HTTPException(status_code=404, detail="Project not found")
    return {
        "id": str(project.id),
        "name": project.name,
        "description": project.description,
        "metadata": project.metadata,
    }


@router.delete("/api/v1/projects/{project_id}")
async def delete_project(
    project_id: str,
    claims: Annotated[dict[str, Any], Depends(auth_guard)],
    db: Annotated[AsyncSession, Depends(get_db)],
) -> dict[str, Any]:
    """Delete a project and its direct items/links."""
    ensure_project_access(project_id, claims)
    ensure_write_permission(claims, action="delete_project")

    repo = project_repository.ProjectRepository(db)
    project = await _maybe_await(repo.get_by_id(project_id))
    if not project:
        raise HTTPException(status_code=404, detail="Project not found")

    link_repo = link_repository.LinkRepository(db)
    item_repo = item_repository.ItemRepository(db)
    links = await _maybe_await(link_repo.get_by_project(project_id))
    await _maybe_await(item_repo.list_all(project_id))
    for link in links:
        await _maybe_await(link_repo.delete(str(link.id)))

    await _maybe_await(db.execute(delete(Item).where(Item.project_id == project_id)))
    await _maybe_await(db.execute(delete(Project).where(Project.id == project_id)))
    await _maybe_await(db.commit())
    return {"success": True, "message": "Project deleted successfully"}


@router.get("/api/v1/projects/{project_id}/export")
async def export_project(
    project_id: str,
    format: str = "json",
    claims: Annotated[dict[str, Any], Depends(auth_guard)] = None,
    db: Annotated[AsyncSession, Depends(get_db)] = None,
) -> Any:
    """Export project data to json, csv, markdown, or full canonical JSON."""
    ensure_project_access(project_id, claims)
    if format == "full":
        from tracertm.services.export_service import ExportService

        service = ExportService(db)
        return json.loads(await service.export_to_json(project_id))

    from tracertm.services.export_import_service import ExportImportService

    service = ExportImportService(db)
    exporters = {
        "json": service.export_to_json,
        "csv": service.export_to_csv,
        "markdown": service.export_to_markdown,
    }
    if format not in exporters:
        raise HTTPException(
            status_code=400,
            detail=f"Unsupported format: {format}. Supported formats: json, csv, markdown, full",
        )
    result = await exporters[format](project_id)
    if "error" in result:
        raise HTTPException(status_code=404, detail=result["error"])
    return result


@router.post("/api/v1/projects/{project_id}/import")
async def import_project(
    project_id: str,
    request: ImportRequest,
    claims: Annotated[dict[str, Any], Depends(auth_guard)],
    db: Annotated[AsyncSession, Depends(get_db)],
) -> Any:
    """Import project data into an existing project."""
    ensure_project_access(project_id, claims)
    ensure_write_permission(claims, action="import_project")
    from tracertm.services.export_import_service import ExportImportService

    service = ExportImportService(db)
    importers = {
        "json": service.import_from_json,
        "csv": service.import_from_csv,
    }
    if request.format not in importers:
        raise HTTPException(
            status_code=400,
            detail=f"Unsupported format: {request.format}. Supported formats: json, csv",
        )
    result = await importers[request.format](project_id, request.data)
    if "error" in result:
        raise HTTPException(status_code=400, detail=result["error"])
    return result


@router.post("/api/v1/import")
async def import_full_project(
    body: dict[str, Any],
    claims: Annotated[dict[str, Any], Depends(auth_guard)],
    db: Annotated[AsyncSession, Depends(get_db)],
) -> Any:
    """Import a full project from canonical JSON."""
    ensure_write_permission(claims, action="import_project")
    from tracertm.services.import_service import ImportService

    service = ImportService(db)
    result = await service.import_from_json(json.dumps(body))
    await db.commit()
    return result
