# SPEC.md — Planify

## System Architecture

```
┌─────────────────────────────────────────────────────┐
│                  React Frontend                      │
│         apps/web/ — React Router + TypeScript        │
├──────────┬──────────┬──────────────┬────────────────┤
│ Admin    │ Space    │ Live Collab  │ Proxy          │
│ apps/    │ apps/    │ apps/        │ apps/          │
│ admin/   │ space/   │ live/        │ proxy/         │
├──────────┴──────────┴──────────────┴────────────────┤
│              Django REST API (apps/api/)             │
│        PostgreSQL 14 + Redis + Celery               │
├─────────────────────────────────────────────────────┤
│         Phenotype Extensions (planned)               │
│   AgilePlus Module  |  Worktrees API                │
└─────────────────────────────────────────────────────┘
```

## Components

| Component | Location | Stack | Purpose |
|-----------|----------|-------|---------|
| Web App | `apps/web/` | React, TypeScript | Issue tracking, cycles, views |
| API Server | `apps/api/` | Django, Python | REST API, auth, business logic |
| Admin Panel | `apps/admin/` | TypeScript | Instance administration |
| Space | `apps/space/` | TypeScript | Public project spaces |
| Live | `apps/live/` | TypeScript | Real-time collaboration |
| Proxy | `apps/proxy/` | TypeScript | Request proxying |
| Shared UI | `packages/ui/` | TypeScript | Component library |
| Types | `packages/types/` | TypeScript | Shared type definitions |
| Hooks | `packages/hooks/` | TypeScript | React hooks |

## Core Data Models

```python
# Django models (apps/api/)
class Project:
    id: UUID
    name: str
    identifier: str
    workspace: Workspace

class Issue:
    id: UUID
    project: Project
    title: str
    description: RichText
    state: State
    priority: int
    assignee: User | None
    cycle: Cycle | None
    module: Module | None

class Cycle:
    id: UUID
    project: Project
    name: str
    start_date: date
    end_date: date
    status: CycleStatus

class Module:
    id: UUID
    project: Project
    name: str
    lead: User
    members: list[User]
```

## API Surface (Core)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/workspaces/{id}/projects/` | GET/POST | Project CRUD |
| `/api/projects/{id}/issues/` | GET/POST | Issue CRUD with filters |
| `/api/projects/{id}/cycles/` | GET/POST | Cycle management |
| `/api/projects/{id}/modules/` | GET/POST | Module management |
| `/api/projects/{id}/views/` | GET/POST | Saved view filters |
| `/api/projects/{id}/pages/` | GET/POST | Documentation pages |

## Phenotype Extensions (Planned)

### AgilePlus Module
```
packages/agileplus/          # Shared types/logic
apps/api/agileplus/          # Django models + endpoints
apps/web/components/agileplus/  # React components
```

### Worktrees API
```
apps/api/worktrees/          # Git-backed branch management
apps/web/worktrees/          # UI for worktree operations
```

## Performance Targets

| Metric | Target |
|--------|--------|
| API response (p95) | < 200ms |
| Page load (LCP) | < 2.5s |
| WebSocket latency | < 50ms |
| Issue list (1000 items) | < 500ms with pagination |
| Search across projects | < 1s |

## Infrastructure

| Service | Version | Purpose |
|---------|---------|---------|
| PostgreSQL | 14+ | Primary datastore |
| Redis | 7+ | Cache, Celery broker |
| Celery | 5+ | Async task processing |
| Docker Compose | - | Local development |

## Constraints

- Fork of makeplane/plane; track upstream changes
- Django backend in `apps/api/`, React frontend in `apps/web/`
- PostgreSQL 14 required; no SQLite support
- Environment variables in `.env.example`
- Phenotype extensions isolated from upstream code
- pnpm for frontend package management
