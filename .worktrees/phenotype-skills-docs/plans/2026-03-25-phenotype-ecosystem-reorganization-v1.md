# Phenotype Ecosystem Reorganization Plan

**Date**: 2026-03-25
**Status**: DRAFT
**Version**: 1.0

---

## Executive Summary

This plan reorganizes the Phenotype ecosystem into a clear two-tier structure:

1. **Phenotype-Domain Repos** (`phenotype-*`) - Products/systems that are specifically Phenotype
2. **Generic-Library Repos** (`*-kit`, `*-lib`, `*-core`, etc.) - Marketable utilities useful to any developer

---

## Current State Audit

### 20 Phenotype Repos

| Repo | Type | Status | Content |
|------|------|--------|---------|
| phenotype-agent-core | Domain | Ready | Agent orchestration |
| phenotype-task-engine | Domain | Ready | Task orchestration |
| phenotype-docs-engine | Domain | Ready | Documentation generation |
| phenotype-research-engine | Domain | Ready | Research engine |
| phenotype-evaluation | Domain | Ready | Benchmark framework |
| phenotype-agents | Domain | Ready | Agent implementations |
| phenotype-config | Domain | Ready | Configuration schemas |
| phenotype-skills | Domain | Ready | Skill modules |
| phenotype-design | Domain | Ready | Design documents |
| phenotypeActions | Domain | Ready | CI/CD workflows |
| phenotype-go-kit | **Generic** | Ready | Go toolkit (logctx, ringbuffer, waitfor, registry) |
| phenotype-infrakit | **Generic** | Ready | Rust crates (event-sourcing, cache, policy, state-machine) |
| phenotype-xdd-lib | **Generic** | NEEDS README | Rust xDD testing library |
| phenotype-auth-ts | **Generic** | NEEDS README | TypeScript auth patterns |
| phenotype-config-ts | **Generic** | NEEDS README | TypeScript config with Zod |
| phenotype-middleware-py | **Generic** | NEEDS README | Python middleware patterns |
| phenotype-logging-zig | **Generic** | NEEDS README | Zig structured logging |
| phenotype-cli-core | **Generic** | NEEDS README | Go CLI core |
| phenotype-colab-extensions | Fork Extension | Ready | Colab fork customizations |
| phenotype-cli-extensions | Fork Extension | Ready | helios-cli fork customizations |

---

## Proposed Reorganization

### Tier 1: Phenotype Products (phenotype-*)

These are the core products/systems that define Phenotype.

```
phenotype/
├── Agent Platform
│   ├── phenotype-agent-core      # Core agent models & orchestration
│   ├── phenotype-task-engine     # Task orchestration
│   ├── phenotype-agents         # Agent implementations
│   └── phenotype-skills         # Extracted skill modules
│
├── Engines
│   ├── phenotype-docs-engine     # Documentation generation
│   ├── phenotype-research-engine # Research & investigation
│   └── phenotype-evaluation     # Benchmark orchestration
│
├── Infrastructure
│   ├── phenotype-config          # Configuration schemas
│   └── phenotypeActions         # CI/CD workflows
│
├── Design
│   └── phenotype-design          # Design documents & specs
│
└── Extensions (forks)
    ├── phenotype-colab-extensions # colab fork customizations
    └── phenotype-cli-extensions   # helios-cli fork customizations
```

### Tier 2: Generic Libraries (*.kit, *.lib, *.core)

These are **marketable to other developers** - not Phenotype-specific.

```
generic/
├── Go Toolkit
│   └── go-kit/  (rename from phenotype-go-kit)
│       ├── logctx/       # Context-scoped logging
│       ├── ringbuffer/    # Generic circular buffer
│       ├── waitfor/      # Polling with backoff
│       └── registry/     # Thread-safe registry
│
├── Rust Crates (infrakit)
│   ├── event-sourcing/    # Event store with hash chains
│   ├── cache-adapter/    # Two-tier LRU cache
│   ├── policy-engine/    # Rule-based evaluation
│   └── state-machine/    # Generic FSM
│
├── TypeScript Libraries
│   ├── auth-ts/          # OAuth2/OIDC patterns (rename from phenotype-auth-ts)
│   ├── config-ts/        # Zod-based config (rename from phenotype-config-ts)
│   └── xdd-lib-ts/      # xDD utilities for TypeScript
│
├── Python Libraries
│   └── middleware-py/    # Hexagonal middleware patterns
│
├── Zig Libraries
│   └── logging-zig/      # Structured logging
│
└── CLI Core
    └── cli-core-go/     # Go CLI patterns (rename from phenotype-cli-core)
```

---

## xDD Methodology Encyclopedia (200+ Practices)

### Development Methodologies (+15 = 30 total)

| Acronym | Full Name | Description |
|---------|-----------|-------------|
| TDD | Test-Driven Development | Write tests before code |
| BDD | Behavior-Driven Development | Tests as specifications |
| DDD | Domain-Driven Design | Ubiquitous language & bounded contexts |
| ATDD | Acceptance Test-Driven Development | Customer-facing tests |
| SDD | Specification-Driven Development | Formal specs as tests |
| CDD | Contract-Driven Development | API contract testing |
| IDD | Integration-Driven Development | Test integrations first |
| MDD | Model-Driven Development | Models as primary artifact |
| FDD | Feature-Driven Development | Feature-based iterations |
| RDD | Risk-Driven Development | Test based on risk |
| PDD | Property-Driven Development | Property-based testing |
| ADD | Assumption-Driven Development | Test assumptions |
| EDBD | Example-Driven Development | Examples as specs |
| QBD | Quality-Based Development | Quality metrics driven |
| UDD | Usage-Driven Development | Based on usage patterns |
| BldD | Build-Driven Development | Build system testing |
| ODD | Outcome-Driven Development | Outcome-based specs |
| VDD | Verification-Driven Development | Verification focus |
| WDD | Workflow-Driven Development | Workflow-based testing |

### Design Principles (+20 = 40 total)

| Principle | Description |
|-----------|-------------|
| DRY | Don't Repeat Yourself |
| KISS | Keep It Simple, Stupid |
| YAGNI | You Aren't Gonna Need It |
| SOLID | Single, Open/Closed, Liskov, Interface Seg, Dependency Inversion |
| GRASP | General Responsibility Assignment Software Patterns |
| LoD | Law of Demeter |
| SoC | Separation of Concerns |
| CoC | Convention over Configuration |
| PoLA | Principle of Least Astonishment |
| ADP | Acyclic Dependencies Principle |
| SDP | Stable Dependencies Principle |
| CCP | Common Closure Principle |
| CRP | Common Reuse Principle |
| REP | Reuse/Release Equivalence Principle |
| SAP | Stable Abstractions Principle |
| High Cohesion | Related things together |
| Low Coupling | Minimal dependencies |
| PGA | Prepare for the General Approach |
| PGE | Protect the Essential Complexity |
| DoD | Definition of Done |
| MoSCoW | Must/Should/Could/Won't |
| INVEST | Independent, Negotiable, Valuable, Estimable, Small, Testable |
| DEEP | Distinct, Evolutionary, Evidence-based, Professional |
| SUCCINCT | Single, Understandable, Consistent, Coherent, Usable, Narrow, Testable |
| SOLID-T | SOLID + Traceability |
| RACTER | Responsibility Assignment Principles |
| CRAP | Change Risk Analysis and Predictions |
| CUPID | Composable, Unix-style, Predictable, Idiomatic, Domain-based |
| STUPID | Singleton, Tight Coupling, Untestable, Premature Optimization, Indescriptive Naming, Duplication |
| KILLER | Keep Interfaces Legible, Leverage ER |
| SLAP | Single Level of Abstraction Principle |
| TLDR | Too Long; Didn't Read |
| Favor | Favor composition over inheritance |
| Law of Leaky Abstractions | All non-trivial abstractions are leaky |
| Murphy's Law | What can go wrong, will |

### Architecture Patterns (+15 = 30 total)

| Pattern | Description |
|---------|-------------|
| Clean | Layered architecture (entities, use cases, interfaces, infrastructure) |
| Hexagonal | Ports and Adapters - business logic isolated from external concerns |
| Onion | Similar to Hexagonal with more explicit layering |
| CQRS | Command Query Responsibility Segregation |
| EDA | Event-Driven Architecture |
| Event Sourcing | Store events instead of state |
| Microservices | Small, independent services |
| Modular | Modular monolith with clear boundaries |
| Plugin | Extensible via plugins |
| Pipe-Filter | Data processing pipeline |
| Pub/Sub | Publisher/Subscriber messaging |
| Saga | Distributed transaction pattern |
| Strangler | Incrementally replace legacy systems |
| Sidecar | Deploy helpers alongside main service |
| Microkernel | Minimal core + plugins |
| Space-Based | Distributed memory & processing |
| Service Mesh | Infrastructure for service communication |
| BFF | Backend for Frontend |
| Anti-Corruption | Translate between domains |
| Circuit Breaker | Prevent cascading failures |
| Bulkhead | Isolate failures |
| Retry | Automatic retry with backoff |
| Timeout | Fail fast pattern |
| Observer | Event notification |
| Mediator | Centralized coordination |
| Facade | Simple interface to complex system |
| Proxy | Surrogate or placeholder |
| Decorator | Attach responsibilities dynamically |
| Strategy | Interchangeable algorithms |

### Quality Assurance (+15 = 30 total)

| Method | Description |
|--------|-------------|
| Mutation | Mutate code to verify test quality |
| Property-Based | Test properties rather than examples |
| Contract | Verify API contracts |
| Chaos | Random failure injection |
| Fuzz | Random input testing |
| Security | Security scanning |
| Coverage | Code coverage analysis |
| Complexity | Cyclomatic complexity metrics |
| Benchmark | Performance testing |
| Load | Stress testing |
| Smoke | Basic sanity tests |
| Sanity | Quick subset of tests |
| Regression | Catch regressions |
| E2E | End-to-end testing |
| Integration | Component integration |
| Unit | Isolated component |
| Snapshot | Visual regression |
| Visual Regression | UI screenshot comparison |
| Accessibility | a11y compliance |
| Cross-Browser | Browser compatibility |
| Cross-Platform | Platform compatibility |
| i18n | Internationalization |
| Penetration | Security attack simulation |
| Mutation Coverage | Measure mutation score |
| Fault Injection | Introduce failures |
| Stress | Beyond normal capacity |
| Endurance | Long-duration testing |
| Scalability | Growth testing |
| Performance | Response time metrics |

### Process Methodologies (+15 = 30 total)

| Method | Description |
|--------|-------------|
| DevOps | Development + Operations |
| CI/CD | Continuous Integration/Delivery |
| Agile | Iterative development |
| Scrum | Sprint-based framework |
| Kanban | Flow-based delivery |
| DevSecOps | Security integrated |
| GreenOps | Environmentally conscious |
| AIOps | AI-assisted operations |
| ChatOps | Collaboration via chat |
| Value Stream | End-to-end value flow |
| Kaizen | Continuous improvement |
| PDCA | Plan-Do-Check-Act |
| SIPOC | Suppliers-Inputs-Process-Outputs-Customers |
| VSM | Value Stream Mapping |
| RCA | Root Cause Analysis |
| Sprint Retrospective | Team improvement |
| Stand-up | Daily sync |
| Sprint Planning | Plan sprint work |
| Sprint Review | Demo completed work |
| Backlog Grooming | Refine future work |
| GitOps | Git as source of truth |
| MLOps | ML deployment & operations |
| SRE | Site Reliability Engineering |
| FinOps | Cloud cost optimization |
| Platform Eng | Internal developer platforms |
| Release Engineering | Controlled deployments |
| Incident Management | Handle production issues |
| Change Management | Controlled changes |
| Capacity Planning | Resource forecasting |
| SLO/SLI/SLA | Reliability contracts |

### Documentation (+10 = 20 total)

| Type | Description |
|------|-------------|
| ADRs | Architecture Decision Records |
| RFCs | Request for Comments |
| Design Docs | Technical designs |
| Runbooks | Operational procedures |
| SpecDD | Specification-driven development |
| API Docs | Interface documentation |
| Code Comments | Inline documentation |
| README | Project overview |
| Contributing Guides | How to contribute |
| Style Guides | Coding standards |
| Best Practices | Recommended approaches |
| Troubleshooting | Problem resolution guides |
| Onboarding | New member guides |
| Security Policies | Security guidelines |
| Incident Reports | Post-mortem analysis |
| User Guides | End-user documentation |
| API References | Detailed API specs |
| Architecture Diagrams | Visual architecture |
| Decision Trees | Decision flowcharts |
| Glossaries | Terminology definitions |

### Emerging Practices (+10 = 20 total)

| Practice | Description |
|----------|-------------|
| AI-DD | AI-assisted development |
| Prompt-Driven | Prompts as specifications |
| StoryDD | User story driven |
| TraceDD | Trace-based development |
| Mobile-First | Mobile design priority |
| Cloud-Native | Cloud-optimized design |
| Container-First | Container-native |
| Serverless-First | Serverless architecture |
| Edge-Computing | Distributed edge |
| Quantum-Safe | Quantum-resistant |
| Responsible AI | Ethical AI development |
| Model Governance | ML model management |
| Technical Writing | Clear technical prose |
| Knowledge Mgmt | Knowledge capture |
| Green IT | Sustainable computing |
| Carbon-Aware | Energy-conscious |
| Digital Twins | Virtual representations |
| Platform Eng | Internal platforms |
| Developer Experience | DX improvement |
| Design Systems | Component libraries |

---

## Action Items

### Phase 1: Documentation (This Session)

- [ ] Complete README for phenotype-xdd-lib
- [ ] Complete README for phenotype-auth-ts
- [ ] Complete README for phenotype-config-ts
- [ ] Complete README for phenotype-middleware-py
- [ ] Complete README for phenotype-logging-zig
- [ ] Complete README for phenotype-cli-core

### Phase 2: Renaming (If Desired)

Consider renaming generic libraries to remove `phenotype-` prefix:

| Current | Proposed | Rationale |
|---------|----------|-----------|
| phenotype-go-kit | go-kit | Generic Go utilities |
| phenotype-infrakit | infrakit | Generic Rust infra |
| phenotype-auth-ts | auth-patterns-ts | Generic auth |
| phenotype-config-ts | config-ts | Generic config |
| phenotype-cli-core | cli-core | Generic CLI |

### Phase 3: Extract More Libraries

From thegent:
- `observability-kit` - Logging, metrics, tracing
- `cli-patterns` - CLI architecture patterns

From portage:
- `benchmark-framework` - Generic benchmark orchestration

From heliosApp:
- `ui-components` - Shared React components
- `desktop-runtime` - Desktop integration layer

---

## Rollback Plan

If reorganization causes issues:
1. Keep old repo names as aliases
2. Don't delete source repos until new structure is validated
3. Use git tags for version tracking
4. Maintain changelog in both locations during transition

---

## Success Metrics

- All 20 repos have complete README
- All generic libraries are independently consumable
- Clear dependency graph between repos
- No circular dependencies
- CI/CD working on all repos
- Documentation coverage > 80%

---

**Document Version**: 1.0
**Last Updated**: 2026-03-25
**Next Review**: 2026-04-01
