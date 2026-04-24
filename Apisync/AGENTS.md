# AGENTS.md — Apisync

## Project Overview

- **Name**: Apisync
- **Description**: API Synchronization Platform — Automated API schema synchronization, version management, and multi-provider API gateway orchestration with OpenAPI/AsyncAPI support
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/Apisync`
- **Language Stack**: TypeScript, Python 3.12+, Node.js, Bun, PostgreSQL
- **Published**: Private (Phenotype org)

## Quick Start Commands

```bash
# Clone and setup
git clone https://github.com/KooshaPari/Apisync.git
cd Apisync

# Install dependencies (Bun preferred)
bun install

# Or with npm
npm install

# Setup database
bun run db:setup

# Run migrations
bun run db:migrate

# Start development server
bun dev

# Run tests
bun test
```

## Architecture

### Hexagonal Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         API Consumers                                │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   Web Dashboard │  │   CLI Tool      │  │   CI/CD Hooks   │         │
│  │   (Next.js)     │  │   (Bun/Node)    │  │   (GitHub)      │         │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘         │
└───────────┼────────────────────┼────────────────────┼────────────────┘
            │                    │                    │
            ▼                    ▼                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                           API Layer                                  │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │                    REST / GraphQL Gateway                      │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐            │   │
│  │  │   Sync     │  │   Schema   │  │   Gateway  │            │   │
│  │  │   Endpoint │  │   Endpoint │  │   Endpoint │            │   │
│  │  └────────────┘  └────────────┘  └────────────┘            │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Application Layer                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │  Sync Service   │  │  Schema Service │  │  Gateway Service│         │
│  │  (Orchestrate)  │  │  (Validate)     │  │  (Route)        │         │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘         │
└───────────┼────────────────────┼────────────────────┼────────────────┘
            │                    │                    │
            ▼                    ▼                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Domain Layer                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   API Entity    │  │   Schema Entity │  │   Sync Entity   │         │
│  │   (Aggregate)   │  │   (Value Obj)   │  │   (Aggregate)   │         │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘         │
└─────────────────────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Infrastructure Layer                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   PostgreSQL      │  │   Redis         │  │   External APIs   │         │
│  │   (Persistence)   │  │   (Cache)       │  │   (Providers)     │         │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘         │
└─────────────────────────────────────────────────────────────────────┘
```

### Sync Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    API Synchronization Flow                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐        │
│  │  Detect  │───▶│  Fetch   │───▶│ Compare  │───▶│  Sync    │        │
│  │  Change  │    │  Schema  │    │  Diff    │    │  Update  │        │
│  └──────────┘    └──────────┘    └──────────┘    └──────────┘        │
│       │                                              │                 │
│       │         ┌──────────────┐                   │                 │
│       └────────▶│  Webhook/    │◀──────────────────┘                 │
│                 │  Poll Trigger │                                      │
│                 └──────────────┘                                      │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Provider Integration

```
┌─────────────────────────────────────────────────────────────────────┐
│                      Provider Adapters                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────┐ │
│  │   OpenAPI    │  │   AsyncAPI   │  │   GraphQL    │  │   gRPC   │ │
│  │   Adapter    │  │   Adapter    │  │   Adapter    │  │  Adapter │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └────┬─────┘ │
│         │                  │                  │               │       │
│         └──────────────────┴──────────────────┴───────────────┘       │
│                                    │                                  │
│                                    ▼                                  │
│                        ┌─────────────────────┐                        │
│                        │   Schema Registry   │                        │
│                        │   (Unified Format)  │                        │
│                        └─────────────────────┘                        │
└─────────────────────────────────────────────────────────────────────┘
```

## Quality Standards

### TypeScript Code Quality

- **Formatter**: `biome format` (preferred) or `prettier`
- **Linter**: `biome lint` (preferred) or `eslint`
- **Type Checker**: `tsc --noEmit` (TypeScript 7 native/tsgo)
- **Tests**: `bun test` with coverage >75%
- **Schema Validation**: All API schemas validated against OpenAPI 3.1

### Python Code Quality (API Tools)

- **Formatter**: `ruff format` (preferred) or `black`
- **Linter**: `ruff check` (preferred) or `pylint`
- **Type Checker**: `mypy --strict`
- **Tests**: `pytest` with coverage >75%
- **Import Style**: Absolute imports, no relative parent imports

### Test Requirements

```bash
# TypeScript tests
bun test

# Python tests
pytest tests/

# API integration tests
bun test:integration

# Schema validation tests
bun test:schemas
```

## Git Workflow

### Branch Naming

Format: `<type>/<component>/<description>`

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `sync`

Examples:
- `feat/sync/add-webhook-triggers`
- `fix/schema/handle-nullable-arrays`
- `docs/api/add-provider-guide`
- `sync/openapi/v3.1-support`

### Commit Messages

Format: `<type>(<scope>): <description>`

Examples:
- `feat(sync): implement automatic webhook registration for providers`
- `fix(schema): resolve nullable type handling in OpenAPI 3.1`
- `docs(api): add comprehensive provider integration guide`
- `refactor(gateway): extract provider adapter interface`

## File Structure

```
Apisync/
├── src/
│   ├── api/                # API routes and controllers
│   │   ├── routes/         # Express/Fastify routes
│   │   ├── middleware/     # Auth, validation, logging
│   │   └── schemas/        # Request/response schemas
│   ├── application/        # Application services
│   │   ├── sync/           # Sync orchestration
│   │   ├── schema/         # Schema management
│   │   └── gateway/        # Gateway configuration
│   ├── domain/             # Domain layer
│   │   ├── entities/       # Domain entities
│   │   ├── value-objects/  # Value objects
│   │   └── repositories/   # Repository interfaces
│   ├── infrastructure/     # Infrastructure layer
│   │   ├── persistence/    # Database adapters
│   │   ├── cache/          # Redis adapters
│   │   └── providers/      # API provider integrations
│   └── tools/              # CLI and utility tools
├── tests/
│   ├── unit/               # Unit tests
│   ├── integration/        # Integration tests
│   └── fixtures/           # Test fixtures
├── docs/                   # Documentation
├── specs/                  # OpenAPI/AsyncAPI specs
├── scripts/                # Build and utility scripts
└── migrations/             # Database migrations
```

## CLI Commands

### Development

```bash
# Start development server
bun dev

# Start with hot reload
bun dev:watch

# Start specific service
bun dev:sync
bun dev:gateway
```

### Database Operations

```bash
# Run migrations
bun run db:migrate

# Rollback migration
bun run db:rollback

# Seed database
bun run db:seed

# Reset database
bun run db:reset

# Generate migration
bun run db:generate --name add_api_endpoints
```

### Sync Operations

```bash
# Sync all APIs
bun run sync:all

# Sync specific provider
bun run sync --provider stripe

# Sync specific API
bun run sync --api api-123

# Dry run sync
bun run sync --dry-run

# Force full sync (ignore checkpoints)
bun run sync --full
```

### Schema Operations

```bash
# Validate schema
bun run schema:validate --file ./api.yaml

# Convert schema format
bun run schema:convert --from openapi --to asyncapi

# Diff schemas
bun run schema:diff --left v1.yaml --right v2.yaml

# Generate SDK
bun run schema:sdk --schema ./api.yaml --lang typescript
```

### Provider Management

```bash
# List providers
bun run provider:list

# Add provider
bun run provider:add --name github --type openapi

# Test provider connection
bun run provider:test --id provider-123

# Refresh provider schemas
bun run provider:refresh --id provider-123
```

## Troubleshooting

### Database Connection Issues

```bash
# Database not connecting
# 1. Check PostgreSQL is running
pg_isready -h localhost -p 5432

# 2. Verify connection string
echo $DATABASE_URL

# 3. Test connection
bun run db:test

# 4. Check migrations status
bun run db:status
```

### Schema Validation Failures

```bash
# Schema not validating
# 1. Validate manually
bun run schema:validate --file ./problematic.yaml

# 2. Check OpenAPI version
head -20 ./problematic.yaml | grep openapi

# 3. Convert to latest version
bun run schema:upgrade --file ./problematic.yaml

# 4. Lint schema
bun run schema:lint --file ./problematic.yaml
```

### Sync Failures

```bash
# Sync failing
# 1. Check provider health
bun run provider:test --id failing-provider

# 2. Enable debug logging
LOG_LEVEL=debug bun run sync --api api-123

# 3. Check rate limits
bun run provider:limits --id failing-provider

# 4. Retry with backoff
bun run sync --api api-123 --retry --backoff
```

### Webhook Issues

```bash
# Webhooks not firing
# 1. Check webhook registrations
bun run webhook:list --api api-123

# 2. Test webhook delivery
bun run webhook:test --id webhook-123

# 3. View webhook logs
bun run webhook:logs --id webhook-123

# 4. Re-register webhooks
bun run webhook:register --api api-123
```

### Type Errors

```bash
# TypeScript errors
# 1. Clear cache
rm -rf node_modules .bun bun.lock
bun install

# 2. Type check
bun run typecheck

# 3. Check for circular deps
bun run deps:circular

# 4. Regenerate types
bun run types:generate
```

## Environment Variables

```bash
# Database
DATABASE_URL=postgresql://user:pass@localhost:5432/apisync
REDIS_URL=redis://localhost:6379

# API
API_PORT=3000
API_HOST=0.0.0.0
JWT_SECRET=your-jwt-secret

# Sync
SYNC_INTERVAL=300
SYNC_BATCH_SIZE=100
SYNC_WORKERS=4

# Providers
GITHUB_TOKEN=ghp_xxx
STRIPE_KEY=sk_xxx
```

## API Provider Support

| Provider | Type | Status | Notes |
|----------|------|--------|-------|
| GitHub | OpenAPI 3.0 | ✅ Active | Full sync support |
| Stripe | OpenAPI 3.0 | ✅ Active | Webhook events |
| Twilio | OpenAPI 3.0 | ✅ Active | SMS/Voice APIs |
| SendGrid | OpenAPI 3.0 | ✅ Active | Email APIs |
| AWS | CloudFormation | 📋 Planned | Multi-service |
| Azure | ARM Templates | 📋 Planned | Resource management |
| GCP | Discovery API | 📋 Planned | Service catalog |

## Integration Points

| System | Protocol | Purpose |
|--------|----------|---------|
| PhenoMCP | REST | Agent API access |
| HeliosApp | Events | Deployment sync |
| Portage | Webhooks | Build notifications |
| TheGent | REST | API key management |

## Governance Rules

### Mandatory Checks

1. **FR Traceability**
   - All tests MUST reference FR-XXX-NNN
   - Use: @pytest.mark.traces_to() / #[trace_to()] / tracesTo()

2. **AI Attribution**
   - .phenotype/ai-traceability.yaml MUST exist
   - MUST be updated on every AI-generated change

3. **CI/CD Compliance**
   - .github/workflows/traceability.yml MUST pass
   - No merges with drift > 90%

4. **Code Quality**
   - All code MUST have corresponding tests
   - Minimum 80% coverage for new code

### Prohibited Actions

- ❌ Delete without read first
- ❌ Modify without FR reference
- ❌ Skip validation on merge

### Validation

Run before any commit:
```bash
python3 validate_governance.py
```

Must pass all checks before PR.

---

Last Updated: 2026-04-05
Version: 1.0.0
