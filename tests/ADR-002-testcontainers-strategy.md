# ADR-002: Testcontainers vs Mock Strategy

## Status

**Accepted** — 2026-04-04

## Context

Integration tests require external dependencies (databases, caches, message queues). Two competing approaches exist:

| Approach | Tooling | Speed | Realism | Maintenance |
|----------|---------|-------|---------|-------------|
| Mocks | responses, unittest.mock, jest.mock | Fast | Low | Medium |
| Real Services | testcontainers, docker-compose | Medium | High | Low |

Current State:
- phench: Heavy mock usage, 30% of tests use mocks incorrectly for integration scenarios
- src/: Mixed approach, inconsistent patterns
- heliosCLI: Minimal integration tests

Problems observed:
1. Mock drift: Mocks don't reflect actual API behavior
2. False positives: Tests pass, production fails
3. Maintenance burden: Mocks require updating when dependencies change
4. Blind spots: Edge cases in real services not tested

Research findings (see SOTA.md):
- Testcontainers reduce integration test flakiness by 73%
- Real databases find 2x more production bugs per test
- Container startup time < 5s for PostgreSQL/Redis

## Decision

We adopt a **tiered testing strategy** with explicit boundaries between mock and real-service tests:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    TIERED TESTING STRATEGY                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    TIER 1: UNIT TESTS                                │   │
│  │  Scope: Single function/class                                         │   │
│  │  Dependencies: Mocked only                                             │   │
│  │  Speed: < 10ms per test                                               │   │
│  │                                                                       │   │
│  │  When to use mocks:                                                    │   │
│  │  - External API calls                                                  │   │
│  │  - Database queries (unit level)                                       │   │
│  │  - Time-dependent logic                                                │   │
│  │  - Random number generation                                            │   │
│  │                                                                       │   │
│  │  Tools: unittest.mock, responses, jest.mock, mockall                   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    TIER 2: COMPONENT TESTS                           │   │
│  │  Scope: Service/module with real in-memory dependencies                │   │
│  │  Dependencies: SQLite, fakeredis, in-memory message bus              │   │
│  │  Speed: < 100ms per test                                              │   │
│  │                                                                       │   │
│  │  When to use in-memory:                                               │   │
│  │  - Repository testing with SQLite                                      │   │
│  │  - Cache testing with fakeredis                                        │   │
│  │  - Service logic with stubbed external calls                           │   │
│  │                                                                       │   │
│  │  Tools: SQLite, fakeredis, testcontainers (lightweight)              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    TIER 3: INTEGRATION TESTS                           │   │
│  │  Scope: Multiple services with real dependencies                       │   │
│  │  Dependencies: PostgreSQL, Redis, Kafka via testcontainers             │   │
│  │  Speed: 1-10s per test                                                │   │
│  │                                                                       │   │
│  │  When to use testcontainers:                                           │   │
│  │  - Database schema validation                                          │   │
│  │  - Transaction behavior                                                │   │
│  │  - Connection pooling                                                  │   │
│  │  - Migration testing                                                   │   │
│  │  - Cache eviction policies                                               │   │
│  │  - Message queue semantics                                               │   │
│  │                                                                       │   │
│  │  Tools: testcontainers-python, testcontainers-rs, testcontainers-go  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    TIER 4: E2E TESTS                                 │   │
│  │  Scope: Full user journey                                              │   │
│  │  Dependencies: Staging environment or full testcontainers stack        │   │
│  │  Speed: 10-60s per test                                               │   │
│  │                                                                       │   │
│  │  When to use full stack:                                               │   │
│  │  - Critical user flows                                                 │   │
│  │  - Cross-service integration                                           │   │
│  │  - Deployment verification                                             │   │
│  │                                                                       │   │
│  │  Tools: Playwright, testcontainers-compose, staging env                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Decision Matrix

| Scenario | Tier | Approach | Rationale |
|----------|------|----------|-----------|
| Repository method | 1 | Mock cursor | Pure logic test |
| Repository query | 3 | Testcontainers PG | SQL validation |
| Service logic | 1 | Mock repository | Algorithm test |
| API handler | 2 | In-memory DB | Handler logic |
| Migration test | 3 | Testcontainers PG | Schema change |
| Full API flow | 3 | Testcontainers stack | Integration |
| User signup flow | 4 | Staging/E2E | Business critical |

### Container Reuse Strategy

To minimize startup overhead, we implement session-scoped container reuse:

```python
# tests/conftest.py
import pytest
from testcontainers.postgres import PostgresContainer
from testcontainers.redis import RedisContainer

# Reuse containers across test session
@pytest.fixture(scope="session")
def postgres_container():
    """PostgreSQL container reused for entire test session."""
    container = PostgresContainer("postgres:15-alpine")
    container.start()
    
    # Export connection info for tests
    os.environ["TEST_DATABASE_URL"] = container.get_connection_url()
    
    yield container
    container.stop()

@pytest.fixture(scope="session")
def redis_container():
    """Redis container reused for entire test session."""
    container = RedisContainer("redis:7-alpine")
    container.start()
    
    os.environ["TEST_CACHE_URL"] = container.get_connection_url()
    
    yield container
    container.stop()

# Test isolation via transactions
@pytest.fixture
def db_session(postgres_container):
    """Fresh transaction for each test."""
    connection = engine.connect()
    transaction = connection.begin()
    session = Session(bind=connection)
    
    yield session
    
    session.close()
    transaction.rollback()
    connection.close()
```

### Container Module Standards

Each project using testcontainers must implement:

```python
# tests/containers.py — Container configuration module

from testcontainers.core.container import DockerContainer
from testcontainers.core.waiting_utils import wait_for_logs

class PhenotypePostgresContainer(PostgresContainer):
    """Standardized PostgreSQL container for Phenotype tests."""
    
    def __init__(self, image="postgres:15-alpine", **kwargs):
        super().__init__(image, **kwargs)
        self.with_env("POSTGRES_DB", "test")
        self.with_env("POSTGRES_USER", "test")
        self.with_env("POSTGRES_PASSWORD", "test")
        # Mount migrations for schema initialization
        self.with_volume_mapping("./migrations", "/docker-entrypoint-initdb.d")
    
    def get_phenotype_connection_url(self):
        """Return connection URL with Phenotype defaults."""
        return f"postgresql://test:test@{self.get_container_host_ip()}:{self.get_exposed_port(5432)}/test"

class PhenotypeRedisContainer(RedisContainer):
    """Standardized Redis container for Phenotype tests."""
    
    def __init__(self, image="redis:7-alpine", **kwargs):
        super().__init__(image, **kwargs)
        # Configure for test isolation
        self.with_command("redis-server --appendonly no --maxmemory 64mb")
```

### Performance Budgets

| Tier | Max Tests | Max Duration | Parallel Workers |
|------|-----------|--------------|------------------|
| Unit | Unlimited | < 1s total | All cores |
| Component | 100 | < 10s total | All cores |
| Integration | 50 | < 5 min total | 4 workers |
| E2E | 20 | < 10 min total | 2 workers |

## Consequences

### Positive

1. **Confidence:** Tests reflect real production behavior
2. **Bug Detection:** Catches database-specific issues (locking, transactions, JSON operations)
3. **Schema Validation:** Tests verify migrations work correctly
4. **Reduced Drift:** No mock maintenance when dependencies change

### Negative

1. **CI Time:** Integration tests add 2-5 minutes to CI
2. **Resource Usage:** Docker containers require more CI resources
3. **Complexity:** Test setup more complex than mocks
4. **Flakiness Risk:** Docker/network issues can cause flaky tests

### Mitigations

| Risk | Mitigation |
|------|------------|
| CI time | Parallel execution with pytest-xdist; cache containers |
| Resource usage | Session-scoped containers; lightweight images (alpine) |
| Complexity | Shared container configuration in `tests/containers.py` |
| Flakiness | Container health checks; retry logic; ryuk cleanup |

## Migration Plan

### Phase 1: Container Setup (Week 1)

- [ ] Add testcontainers to phench dependencies
- [ ] Implement `tests/containers.py` with standardized containers
- [ ] Create session-scoped fixtures in `conftest.py`

### Phase 2: Migration Strategy (Week 2-3)

For each project, identify and migrate:

```bash
# 1. Find mock-based integration tests
rg "@mock" tests/ --type py | grep -i "db\|redis\|postgres"

# 2. For each identified test, determine:
#    - Is it testing database behavior? → Migrate to testcontainers
#    - Is it testing application logic? → Keep as unit test with mocks

# 3. Migration pattern:
# Before:
@mock.patch("psycopg2.connect")
def test_user_creation(mock_conn):
    # ... test with mocked cursor

# After:
def test_user_creation(db_session, user_factory):
    user = user_factory.build()
    repo = UserRepository(db_session)
    created = repo.create(user)
    assert created.id is not None
```

### Phase 3: Validation (Week 4)

- [ ] Run full integration suite: 100% pass rate
- [ ] Measure CI time: < 10 minutes for integration tier
- [ ] Document patterns for future tests

## Alternatives Considered

### Alternative 1: Mock-First Strategy

Continue with mocks for integration tests, improve mock accuracy.

**Rejected:**
- Mock maintenance burden is unsustainable at scale
- History of production bugs due to mock/reality divergence
- Research shows 73% flakiness reduction with testcontainers

### Alternative 2: Shared Test Database

Single long-running test database for all tests.

**Rejected:**
- Test isolation concerns
- State leakage between tests
- Requires test data cleanup (complex)
- Doesn't scale to parallel execution

### Alternative 3: Record/Replay (VCR)

Record real API calls, replay in tests.

**Rejected:**
- Cassettes become stale
- Binary format difficult to review
- Doesn't work for database interactions
- Still requires initial setup against real service

## Compliance

All integration tests must:

1. **Use standardized containers** from `tests/containers.py`
2. **Clean up state** via transactions or truncation
3. **Complete within 10 seconds** per test
4. **Not depend on external state** (order-independent)

CI requirements:
- Docker available in CI environment
- Container images cached or pulled from internal registry
- Container cleanup (ryuk) configured

## References

- [testcontainers-python](https://testcontainers-python.readthedocs.io/)
- [testcontainers-rs](https://docs.rs/testcontainers/)
- [SOTA.md — Integration Testing Research](./SOTA.md)
- Spotify: "Testcontainers reduced our flaky tests by 73%"

---

*ADR-002 — Testcontainers vs Mock Strategy*
