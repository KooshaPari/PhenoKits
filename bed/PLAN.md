# Bed - Project Plan

**Document ID**: PLAN-BED-001  
**Version**: 1.0.0  
**Created**: 2026-04-05  
**Status**: Draft  
**Project Owner**: Phenotype Infrastructure Team  
**Review Cycle**: Monthly

---

## 1. Project Overview & Objectives

### 1.1 Vision Statement

Bed is Phenotype's development environment orchestration system - a declarative, reproducible infrastructure-as-code solution for managing development environments across the entire Phenotype ecosystem. It provides consistent, version-controlled environments that can be spun up locally, in CI/CD, or in the cloud.

### 1.2 Mission Statement

To eliminate "works on my machine" by providing deterministic, reproducible development environments that can be defined in code, shared across teams, and deployed anywhere with a single command.

### 1.3 Core Objectives

| Objective ID | Description | Success Criteria | Priority |
|--------------|-------------|------------------|----------|
| OBJ-001 | Declarative environment definitions | YAML/TOML environment specs | P0 |
| OBJ-002 | Reproducible builds | Same input = same environment | P0 |
| OBJ-003 | Multi-runtime support | Docker, VM, native, WASM | P0 |
| OBJ-004 | CI/CD integration | GitHub Actions, GitLab CI | P1 |
| OBJ-005 | Remote development | Cloud-based environments | P2 |
| OBJ-006 | Team synchronization | Shared environment registry | P1 |
| OBJ-007 | Health monitoring | Environment health checks | P1 |
| OBJ-008 | Cost optimization | Auto-shutdown, resource limits | P2 |

### 1.4 Problem Statement

Development environment inconsistencies cause:
- Wasted time debugging environment issues
- Onboarding friction for new team members
- CI/CD failures due to environment differences
- Difficulty reproducing production issues locally
- "Works on my machine" syndrome
- Knowledge silos around environment setup

### 1.5 Target Users

1. **Software Engineers**: Setting up local development
2. **DevOps Engineers**: Managing CI/CD environments
3. **Platform Engineers**: Maintaining shared infrastructure
4. **New Hires**: Onboarding to projects
5. **QA Engineers**: Reproducing issues

---

## 2. Architecture Strategy

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Bed Ecosystem                            │
├─────────────────────────────────────────────────────────────┤
│  CLI (bed)                     Web UI (bed-web)             │
│  ─────────                     ────────────────             │
│  - up/down                     - Environment dashboard      │
│  - status                      - Team sharing               │
│  - sync                        - Resource monitoring        │
│  - logs                        - Cost tracking              │
└─────────────────────────────────────────────────────────────┘
                           │
              ┌────────────┴────────────┐
              │       Bed Daemon        │
              │   (State Management)    │
              └────────────┬────────────┘
                           │
     ┌──────────┬──────────┼──────────┬──────────┐
     │          │          │          │          │
┌────▼────┐┌────▼────┐┌────▼────┐┌────▼────┐┌────▼────┐
│ Docker  ││   VM    ││  Cloud  ││  Local  ││  WASM   │
│Runtime  ││Runtime  ││Runtime  ││Runtime  ││Runtime  │
└─────────┘└─────────┘└─────────┘└─────────┘└─────────┘
```

### 2.2 Components

| Component | Technology | Purpose |
|-----------|------------|---------|
| bed-cli | Rust | Command-line interface |
| bed-daemon | Rust | Background state management |
| bed-web | TypeScript/React | Web dashboard |
| bed-runtime-docker | Docker API | Container orchestration |
| bed-runtime-vm | QEMU/Lima | VM management |
| bed-runtime-cloud | AWS/GCP/Azure | Cloud environments |
| bed-runtime-local | Native | Local process management |
| bed-runtime-wasm | Wasmtime | WASM sandboxing |

### 2.3 Configuration Schema

```yaml
# bed.yaml - Environment definition
name: phenotype-dev
version: "1.0"
runtime: docker

services:
  postgres:
    image: postgres:15
    env:
      POSTGRES_DB: phenotype
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready"]
      interval: 5s
  
  redis:
    image: redis:7
    ports:
      - "6379:6379"
  
  api:
    build: ./api
    depends_on:
      - postgres
      - redis
    env:
      DATABASE_URL: postgres://postgres@postgres/phenotype
    volumes:
      - ./api:/app:cached

features:
  - name: monitoring
    services: [grafana, prometheus]
  
  - name: tracing
    services: [jaeger]

profiles:
  development:
    features: []
  
  production-like:
    features: [monitoring, tracing]
```

---

## 3. Implementation Phases

### 3.1 Phase 0: Core Foundation (Weeks 1-4)

| Week | Deliverable | Owner |
|------|-------------|-------|
| 1 | CLI structure, config parsing | Core Team |
| 2 | Docker runtime implementation | Docker Team |
| 3 | State management, health checks | Core Team |
| 4 | Basic sync, logs | Core Team |

### 3.2 Phase 1: Runtime Expansion (Weeks 5-10)

| Week | Deliverable | Owner |
|------|-------------|-------|
| 5-6 | VM runtime (Lima) | VM Team |
| 7-8 | Local runtime | Core Team |
| 9-10 | WASM runtime | WASM Team |

### 3.3 Phase 2: Cloud & Sharing (Weeks 11-16)

| Week | Deliverable | Owner |
|------|-------------|-------|
| 11-12 | Cloud runtime (AWS) | Cloud Team |
| 13-14 | Environment registry | Platform Team |
| 15-16 | Team sync, Web UI | Frontend Team |

### 3.4 Phase 3: Production Features (Weeks 17-24)

| Week | Deliverable | Owner |
|------|-------------|-------|
| 17-18 | CI/CD integration | DevOps Team |
| 19-20 | Cost tracking | Platform Team |
| 21-22 | Auto-shutdown | Platform Team |
| 23-24 | Production hardening | SRE Team |

---

## 4. Technical Stack Decisions

| Layer | Technology | Rationale |
|-------|------------|-----------|
| CLI | Rust | Performance, binary size |
| Config | YAML/TOML | Familiar, readable |
| Docker | Docker API | Industry standard |
| VMs | Lima/QEMU | macOS native, lightweight |
| Cloud | Pulumi/TF | Existing IaC |
| Web UI | React | Component ecosystem |
| State | SQLite | Embedded, reliable |
| API | GraphQL | Flexible queries |

---

## 5. Risk Analysis & Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Docker Desktop licensing | Medium | High | Podman fallback |
| VM performance issues | Medium | Medium | Native mode fallback |
| Cloud cost overruns | Medium | High | Limits, auto-shutdown |
| Config format wars | Low | Medium | Auto-migrate formats |
| Runtime conflicts | Medium | Medium | Sandboxing |

---

## 6. Resource Requirements

### 6.1 Team

| Role | Count | Allocation |
|------|-------|------------|
| Rust Developer | 2 | 100% |
| DevOps Engineer | 1 | 50% |
| Frontend Developer | 1 | 50% |
| Platform Engineer | 1 | 25% |

### 6.2 Infrastructure

| Resource | Cost/Month |
|------------|------------|
| CI runners | $300 |
| Test environments | $200 |
| Cloud dev environments | $500 |

---

## 7. Timeline & Milestones

| Milestone | Date | Criteria |
|-----------|------|----------|
| MVP | Week 4 | Docker runtime, basic CLI |
| Beta | Week 10 | All local runtimes |
| V1.0 | Week 16 | Cloud, sharing |
| V2.0 | Week 24 | CI/CD, cost optimization |

---

## 8. Dependencies & Blockers

| Dependency | Required By | Status |
|------------|-------------|--------|
| Docker API client | Week 1 | Available |
| Lima integration | Week 5 | Research |
| AWS SDK | Week 11 | Available |
| Pulumi provider | Week 11 | Available |

---

## 9. Testing Strategy

| Type | Target | Tools |
|------|--------|-------|
| Unit | 80% | cargo test |
| Integration | 70% | Docker testcontainers |
| E2E | 50% | Bats (bash testing) |
| Runtime tests | All | CI matrix |

---

## 10. Deployment Plan

| Phase | Channel | Criteria |
|-------|---------|----------|
| Alpha | Internal | MVP complete |
| Beta | crates.io | 3 teams using |
| Stable | Homebrew | 10 teams using |

---

## 11. Rollback Procedures

| Scenario | Action |
|----------|--------|
| CLI bug | `cargo install bed --version <last>` |
| Environment broken | `bed down && bed up` |
| Config error | `bed doctor` for diagnosis |

---

## 12. Post-Launch Monitoring

| Metric | Target |
|--------|--------|
| Environment startup time | <30s |
| Sync time | <10s |
| Active environments | Track |
| Cost per environment | <$50/mo |

---

**Document Control**

- **Status**: Draft
- **Next Review**: 2026-05-05
