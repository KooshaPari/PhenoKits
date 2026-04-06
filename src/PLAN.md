# PLAN.md - src/

## Overview

Core source code and shared libraries for the Phenotype ecosystem. Contains reusable components, MCP servers, and foundational utilities.

| | |
|---|---|
| **Project Type** | Shared Library + MCP Server |
| **Stack** | Python 3.10+ + MCP SDK + Pydantic |
| **Priority** | P0 |

---

## Phases

### Phase 1: DinoforgeMcp Core (Weeks 1-2)

| Deliverable | Description | Owner |
|-------------|-------------|-------|
| MCP server core | JSON-RPC 2.0 handlers | Dev |
| Stdio transport | Standard I/O transport | Dev |
| Tool registry | Dynamic tool registration | Dev |
| Pydantic models | Input/output validation | Dev |
| Package structure | pip installable package | Dev |

### Phase 2: Transport Expansion (Weeks 3-4)

| Deliverable | Description | Owner |
|-------------|-------------|-------|
| HTTP transport | RESTful MCP transport | Dev |
| WebSocket transport | Real-time streaming | Dev |
| Transport abstraction | Swappable transport layer | Dev |
| Connection pooling | Efficient resource use | Dev |
| Auth middleware | Token-based auth | Dev |

### Phase 3: Shared Libraries (Weeks 5-6)

| Deliverable | Description | Owner |
|-------------|-------------|-------|
| Config manager | Unified configuration | Dev |
| Logging context | Structured logging | Dev |
| Error types | Standardized errors | Dev |
| Utils/helpers | Common utilities | Dev |
| Documentation | API docs, examples | Docs |

---

## Timeline

```
Week:  1  2  3  4  5  6
       [==DinoforgeMcp==]
             [==Transports==]
                   [==Shared Libs==]
```

| Phase | Duration | Key Milestone |
|-------|----------|---------------|
| Phase 1 | 2 weeks | MCP server responding to stdio requests |
| Phase 2 | 2 weeks | HTTP + WebSocket transports functional |
| Phase 3 | 2 weeks | Shared libs consumed by 2+ projects |

**Total Duration: 6 weeks**

---

## Key Deliverables

| Deliverable | Phase | Success Criteria |
|-------------|-------|------------------|
| MCP server core | 1 | JSON-RPC 2.0 compliant |
| Stdio transport | 1 | CLI integration works |
| Tool definitions | 1 | JSON Schema validation |
| pip package | 1 | `pip install dinoforge-mcp` |
| HTTP transport | 2 | REST endpoints functional |
| WebSocket transport | 2 | Bidirectional streaming |
| Auth system | 2 | Token validation |
| Config manager | 3 | Hierarchical config |
| Logging context | 3 | Correlation IDs |
| Error handling | 3 | Typed errors, stack traces |

---

## Resource Estimate

| Role | Hours | Rate | Cost |
|------|-------|------|------|
| Python Dev | 180h | $100/hr | $18,000 |
| Protocol Engineer | 80h | $120/hr | $9,600 |
| QA Engineer | 60h | $80/hr | $4,800 |
| Tech Writer | 40h | $75/hr | $3,000 |
| **Total** | **360h** | - | **$35,400** |

---

## Integration Points

| Consumer | Integration | Timeline |
|----------|-------------|----------|
| agent-wave | MCP Client | Phase 1 |
| phenotype-cli | CLI stdio | Phase 1 |
| heliosApp | HTTP transport | Phase 2 |
| template-commons | Template ref | Phase 3 |

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Request latency (p50) | < 10ms | Round-trip |
| Request latency (p99) | < 50ms | Round-trip |
| Throughput | > 1000 req/s | Concurrent |
| Memory footprint | < 128MB | Per instance |
| Startup time | < 500ms | Import to ready |

---

## Risks & Mitigations

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| MCP spec changes | Medium | Abstraction layer |
| Transport bugs | Medium | Extensive testing per transport |
| Integration failures | Low | Early integration tests |

---

*Created: 2026-04-04*
