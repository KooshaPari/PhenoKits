# AGENTS.md — artifacts

## Project Overview

- **Name**: artifacts (Build Artifact Management)
- **Description**: Centralized artifact management system for storing, versioning, and distributing build outputs across the Phenotype ecosystem
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/artifacts`
- **Language Stack**: Python 3.12+, FastAPI, PostgreSQL
- **Published**: Private (Phenotype org)

## Quick Start

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/artifacts

# Install dependencies
pip install -r requirements.txt

# Set up database
python -m scripts.init_db

# Start development server
python -m uvicorn main:app --reload

# Run tests
pytest tests/
```

## Architecture

### Artifact System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        API Layer                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐   │
│  │   REST API      │  │   GraphQL       │  │   WebSocket     │   │
│  │   (FastAPI)     │  │   (Optional)    │  │   (Events)      │   │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘   │
└───────────┼───────────────────┼───────────────────┼──────────────┘
            │                   │                   │
            └───────────────────┼───────────────────┘
                                │
┌───────────────────────────────▼───────────────────────────────┐
│                     Service Layer                               │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │                   Business Logic                            │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │ │
│  │  │ Upload   │  │ Version  │  │ Metadata │  │ Search  │ │ │
│  │  │ Service  │  │ Manager  │  │ Service  │  │ Service │ │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │ │
│  └──────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
            │                   │                   │
┌───────────▼───────────────────▼───────────────────▼───────────┐
│                     Storage Layer                                │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐│
│  │   Object Store  │  │   Database      │  │   Cache         ││
│  │                 │  │                 │  │                 ││
│  │  • S3/MinIO     │  │  • PostgreSQL   │  │  • Redis        ││
│  │  • Local FS     │  │  • Metadata     │  │  • CDN          ││
│  └─────────────────┘  └─────────────────┘  └─────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### Artifact Lifecycle

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Upload  │───▶│ Validate │───▶│  Store   │───▶│ Version  │───▶│  Index   │
│          │    │          │    │          │    │          │    │          │
└──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘
     │                │              │              │              │
     ▼                ▼              ▼              ▼              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           Event Stream                                  │
│     UploadEvent    ValidationEvent   StoreEvent   VersionEvent  IndexEvent │
└─────────────────────────────────────────────────────────────────────────┘
```

### Storage Backend Abstraction

```
┌────────────────────────────────────────────────────────────────────┐
│                      Storage Port Interface                         │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    ArtifactStoragePort                        │  │
│  │                                                              │  │
│  │  + store(artifact: Artifact) -> URL                         │  │
│  │  + retrieve(id: UUID) -> Artifact                             │  │
│  │  + delete(id: UUID) -> bool                                   │  │
│  │  + list(query: Query) -> List<Artifact>                       │  │
│  │                                                              │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              │                                      │
│          ┌───────────────────┼───────────────────┐                  │
│          │                   │                   │                  │
│          ▼                   ▼                   ▼                  │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐           │
│  │ S3Storage    │   │ LocalStorage │   │ AzureStorage │           │
│  │ Adapter      │   │ Adapter      │   │ Adapter      │           │
│  └──────────────┘   └──────────────┘   └──────────────┘           │
│                                                                     │
└────────────────────────────────────────────────────────────────────┘
```

## Quality Standards

### Python Code Quality

- **Formatter**: `black` (line length 100)
- **Linter**: `ruff` with strict rules
- **Type Checker**: `mypy --strict`
- **Import Sorting**: `isort`
- **Security**: `bandit` for security checks

### Test Requirements

```bash
# Unit tests
pytest tests/unit/

# Integration tests
pytest tests/integration/

# API tests
pytest tests/api/ -v

# With coverage
pytest --cov=artifacts --cov-report=html --cov-report=term-missing
```

### Database Migrations

```bash
# Create migration
alembic revision --autogenerate -m "Add artifact tags"

# Run migrations
alembic upgrade head

# Rollback
alembic downgrade -1
```

## Git Workflow

### Branch Naming

Format: `<type>/<component>/<description>`

Types: `feat`, `fix`, `api`, `storage`, `perf`

Examples:
- `feat/api/add-graphql-endpoint`
- `fix/storage/s3-large-file-upload`
- `perf/index/add-metadata-indexing`

### Commit Messages

Format: `<type>(<scope>): <description>`

Examples:
- `feat(api): add batch upload endpoint`
- `fix(storage): resolve S3 multipart upload issues`
- `docs(api): update OpenAPI specification`

## File Structure

```
artifacts/
├── src/
│   ├── artifacts/
│   │   ├── __init__.py
│   │   ├── main.py            # FastAPI application
│   │   ├── config.py          # Configuration
│   │   ├── api/               # API routes
│   │   │   ├── __init__.py
│   │   │   ├── artifacts.py   # Artifact endpoints
│   │   │   ├── versions.py    # Version endpoints
│   │   │   └── search.py      # Search endpoints
│   │   ├── services/          # Business logic
│   │   │   ├── __init__.py
│   │   │   ├── upload.py
│   │   │   ├── storage.py
│   │   │   └── metadata.py
│   │   ├── models/            # Database models
│   │   │   ├── __init__.py
│   │   │   └── artifact.py
│   │   ├── ports/             # Interface definitions
│   │   └── adapters/          # Implementation adapters
│   └── scripts/
├── tests/
│   ├── unit/
│   ├── integration/
│   └── fixtures/
├── migrations/                # Alembic migrations
├── docs/                      # Documentation
├── docker/
├── docker-compose.yml
├── pyproject.toml
├── requirements.txt
└── AGENTS.md                  # This file
```

## CLI Commands

```bash
# Development server
python -m uvicorn src.artifacts.main:app --reload --port 8000

# Production server
gunicorn src.artifacts.main:app -w 4 -k uvicorn.workers.UvicornWorker

# Database commands
python -m scripts.db init
python -m scripts.db migrate
python -m scripts.db seed

# Maintenance
python -m scripts.cleanup --dry-run
python -m scripts.reindex
python -m scripts.validate
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ARTIFACTS_DATABASE_URL` | PostgreSQL connection | `postgresql://localhost/artifacts` |
| `ARTIFACTS_STORAGE_BACKEND` | Storage type | `local` |
| `ARTIFACTS_S3_BUCKET` | S3 bucket name | - |
| `ARTIFACTS_S3_ENDPOINT` | S3-compatible endpoint | - |
| `ARTIFACTS_REDIS_URL` | Redis cache URL | `redis://localhost:6379` |
| `ARTIFACTS_MAX_UPLOAD_SIZE` | Max upload size (bytes) | `1073741824` (1GB) |
| `ARTIFACTS_API_KEY` | API authentication | - |

### config.yaml

```yaml
api:
  title: "Artifacts API"
  version: "1.0.0"
  cors_origins:
    - "https://phenotype.dev"
  
storage:
  backend: s3  # local, s3, azure
  s3:
    bucket: phenotype-artifacts
    region: us-east-1
    endpoint: null  # For MinIO compatibility
  
  local:
    path: /data/artifacts
    
database:
  pool_size: 20
  max_overflow: 10
  
cache:
  ttl: 3600
  max_size: 10000
  
upload:
  max_size: 1GB
  chunk_size: 10MB
  allowed_types:
    - application/zip
    - application/x-tar
    - application/octet-stream
```

## Troubleshooting

### Database connection errors

```bash
# Check PostgreSQL
pg_isready -h localhost -p 5432

# Verify connection string
python -c "import os; print(os.getenv('ARTIFACTS_DATABASE_URL'))"

# Test connection
python -m scripts.db test
```

### Storage upload failures

```bash
# Check S3 credentials
aws s3 ls s3://$ARTIFACTS_S3_BUCKET

# Verify local storage path
ls -la $ARTIFACTS_STORAGE_PATH

# Check disk space
df -h
```

### API performance issues

```bash
# Enable profiling
python -m uvicorn main:app --reload --log-level debug

# Check database queries
ENABLE_SQL_LOGGING=1 python -m uvicorn main:app

# Monitor with Prometheus
curl http://localhost:8000/metrics
```

## API Reference

### REST Endpoints

| Method | Endpoint | Description |
|----------|----------|-------------|
| POST | `/api/v1/artifacts` | Upload new artifact |
| GET | `/api/v1/artifacts` | List artifacts |
| GET | `/api/v1/artifacts/{id}` | Get artifact metadata |
| GET | `/api/v1/artifacts/{id}/download` | Download artifact |
| DELETE | `/api/v1/artifacts/{id}` | Delete artifact |
| POST | `/api/v1/artifacts/{id}/versions` | Create new version |
| GET | `/api/v1/search` | Search artifacts |

### Upload Example

```bash
# Upload artifact
curl -X POST http://localhost:8000/api/v1/artifacts \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: multipart/form-data" \
  -F "file=@build.zip" \
  -F "metadata={\"project\":\"myapp\",\"version\":\"1.0.0\"}"
```

## Resources

- [FastAPI Documentation](https://fastapi.tiangolo.com/)
- [SQLAlchemy ORM](https://docs.sqlalchemy.org/)
- [Alembic Migrations](https://alembic.sqlalchemy.org/)
- [Phenotype Registry](https://github.com/KooshaPari/phenotype-registry)

## Agent Notes

**Critical Implementation Details:**
- Use streaming for large file uploads
- Implement checksum verification for integrity
- Cache frequently accessed metadata
- Use background tasks for heavy operations

**Known Gotchas:**
- S3 multipart uploads require tracking upload IDs
- Database transactions must handle concurrent access
- File descriptors must be properly closed
- Large files may timeout without streaming

**Testing Strategy:**
- Mock storage backends in unit tests
- Use test containers for integration tests
- Verify checksums match uploaded files
- Test concurrent upload scenarios
