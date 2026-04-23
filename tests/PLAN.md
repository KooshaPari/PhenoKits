# PLAN.md - tests/

## Overview

Shared test utilities, fixtures, and testing infrastructure for the Phenotype ecosystem.

| | |
|---|---|
| **Project Type** | Testing Infrastructure |
| **Stack** | Python + JS + Rust (multi-language) |
| **Priority** | P1 |

---

## Phases

### Phase 1: Core Infrastructure (Weeks 1-2)

| Deliverable | Description | Owner |
|-------------|-------------|-------|
| Fixture system | Reusable test data fixtures | Dev |
| Factory classes | Object generation (factory-boy) | Dev |
| Mock registry | Pre-configured service mocks | Dev |
| Test helpers | Custom assertions, utils | Dev |
| pytest plugins | Shared pytest fixtures | Dev |

### Phase 2: Integration Tools (Weeks 3-4)

| Deliverable | Description | Owner |
|-------------|-------------|-------|
| Testcontainers | Container-based integration | Dev |
| Database fixtures | Postgres, Redis, SQLite | Dev |
| HTTP mocking | responses / msw integration | Dev |
| Async support | pytest-asyncio config | Dev |
| Coverage tools | pytest-cov, unified reports | Dev |

### Phase 3: Advanced Testing (Weeks 5-6)

| Deliverable | Description | Owner |
|-------------|-------------|-------|
| Snapshot testing | Visual/API snapshots | Dev |
| Contract tests | Pact consumer/provider | Dev |
| Property tests | Hypothesis integration | Dev |
| Parallel runner | pytest-xdist optimization | Dev |
| CI integration | GitHub Actions helpers | Dev |

---

## Timeline

```
Week:  1  2  3  4  5  6
       [==Core Infrastructure==]
             [==Integration Tools==]
                   [==Advanced Testing==]
```

| Phase | Duration | Key Milestone |
|-------|----------|---------------|
| Phase 1 | 2 weeks | Fixtures used in 3+ projects |
| Phase 2 | 2 weeks | Integration tests with containers |
| Phase 3 | 2 weeks | Contract tests in CI |

**Total Duration: 6 weeks**

---

## Key Deliverables

| Deliverable | Phase | Success Criteria |
|-------------|-------|------------------|
| Fixture library | 1 | 20+ reusable fixtures |
| Factory classes | 1 | User, Project, Org factories |
| Mock adapters | 1 | HTTP, DB, Cache mocks |
| pytest fixtures | 1 | Auto-loaded in consumer projects |
| Testcontainers | 2 | Postgres, Redis helpers |
| DB fixtures | 2 | Migrations, seed data |
| HTTP mocks | 2 | responses/msw wrappers |
| Async helpers | 2 | aiohttp, asyncio fixtures |
| Snapshot testing | 3 | syrupy integration |
| Contract tests | 3 | Pact broker integration |
| Property tests | 3 | Hypothesis strategies |
| Parallel runner | 3 | 4-8 workers configured |

---

## Resource Estimate

| Role | Hours | Rate | Cost |
|------|-------|------|------|
| QA Engineer | 200h | $85/hr | $17,000 |
| Python Dev | 80h | $100/hr | $8,000 |
| DevOps | 60h | $110/hr | $6,600 |
| **Total** | **340h** | - | **$31,600** |

---

## Supported Languages

| Language | Tools | Status |
|----------|-------|--------|
| Python | pytest, factory-boy | Phase 1 |
| JavaScript | jest, msw | Phase 2 |
| Rust | cargo test, mockall | Phase 2 |
| Go | testify, gomock | Phase 3 |

---

## Consumer Projects

| Project | Fixtures Used | Timeline |
|---------|---------------|----------|
| phench | All | Phase 1 |
| src/ | Core fixtures | Phase 1 |
| omniroute-temp | Mocks, containers | Phase 2 |
| template-domain | Full suite | Phase 3 |

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Fixture setup | < 50ms | Per fixture |
| Mock response | < 1ms | Latency |
| Unit test time | < 100ms | Per test |
| Integration test | < 5s | Per suite |
| Coverage target | > 80% | Line coverage |

---

## Risks & Mitigations

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| Fixture maintenance | High | Automated generation |
| Language coverage gaps | Medium | Prioritize by usage |
| Container startup slow | Medium | Image optimization |

---

*Created: 2026-04-04*
