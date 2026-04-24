# Software Engineering Methodologies, Patterns & Practices Encyclopedia

**Version:** 1.0.0
**Last Updated:** 2026-03-25
**Purpose:** Comprehensive reference for all xDD methodologies, patterns, and best practices across the Phenotype ecosystem

---

## Table of Contents

1. [Development Methodologies (xDD)](#1-development-methodologies-xdd)
2. [Design Principles](#2-design-principles)
3. [Architectural Patterns](#3-architectural-patterns)
4. [Quality Assurance Methods](#4-quality-assurance-methods)
5. [Process & Delivery](#5-process--delivery)
6. [Documentation Approaches](#6-documentation-approaches)
7. [Emerging AI-Driven Methods](#7-emerging-ai-driven-methods)
8. [Code Organization Patterns](#8-code-organization-patterns)
9. [Repository Structure](#9-repository-structure)
10. [Polyrepo & Microservice Patterns](#10-polyrepo--microservice-patterns)
11. [Plugin & Library Architecture](#11-plugin--library-architecture)
12. [Code Quality Metrics](#12-code-quality-metrics)

---

## 1. Development Methodologies (xDD)

### Core TDD Family

| Method | Full Name | Description | When to Use |
|--------|-----------|-------------|-------------|
| **TDD** | Test-Driven Development | Write tests before code: Red → Green → Refactor | All production code |
| **BDD** | Behavior-Driven Development | Test behavior via human-readable specs (Gherkin) | Business-facing features |
| **ATDD** | Acceptance-Test Driven Development | Tests define acceptance criteria | User stories, requirements |
| **SDD** | Specification-Driven Development | Formal specs drive implementation | Mission-critical systems |
| **FDD** | Feature-Driven Development | Features as primary development unit | Large enterprise projects |
| **CDD** | Consumer-Driven Development | API consumers write contract tests | Service-oriented systems |
| **MDD** | Model-Driven Development | Generate code from domain models | MDA, EMF-based projects |
| **RDD** | Responsibility-Driven Development | Focus on object responsibilities | OOP design |
| **DDD** | Domain-Driven Design | Ubiquitous language, bounded contexts | Complex business domains |
| **ADD** | Attribute-Driven Design | Design based on quality attributes | Non-functional requirements |
| **IDD** | Interaction-Driven Development | UI/UX interactions drive development | Frontend-heavy apps |

### Testing Variants

| Method | Full Name | Description |
|--------|-----------|-------------|
| **Property-Based Testing (PBT)** | - | Generate random inputs to test invariants (proptest, QuickCheck) |
| **Mutation Testing** | - | Mutate code to verify test effectiveness |
| **Contract Testing** | - | Verify API contracts between services |
| **Snapshot Testing** | - | Compare output against stored snapshots |
| **Fuzz Testing** | - | Random malformed input generation |
| **Golden Testing** | - | Compare against expected golden files |
| **Chaos Testing** | - | Deliberately break things to test resilience |
| **Property-Based BDD** | - | BDD + PBT combined |

### Specification & Documentation Methods

| Method | Full Name | Description |
|--------|-----------|-------------|
| **SpecDD** | Specification-Driven Development | Formal specs as source of truth |
| **DocOps** | Documentation Operations | Documentation as code |
| **Design by Contract (DbC)** | - | Pre/post conditions, invariants |
| **Living Documentation** | - | Documentation generated from code |
| **Readme-Driven Development (RDD)** | - | Write README before code |
| **Changelog-Driven Development** | - | Maintain changelog as you go |
| **Decision Logging** | - | Record why decisions were made |

---

## 2. Design Principles

### SOLID Principles

| Principle | Description | Application |
|-----------|-------------|-------------|
| **S**ingle Responsibility | One reason to change | Every class does one thing |
| **O**pen/Closed | Open for extension, closed for modification | Use inheritance, interfaces |
| **L**iskov Substitution | Subtypes must be substitutable | Honor parent contracts |
| **I**nterface Segregation | Small, focused interfaces | Don't force unused methods |
| **D**ependency Inversion | Depend on abstractions | Inject dependencies |

### Other Core Principles

| Principle | Description | Mnemonic |
|-----------|-------------|----------|
| **DRY** | Don't Repeat Yourself | One source of truth |
| **KISS** | Keep It Simple, Stupid | Prefer simplicity |
| **YAGNI** | You Aren't Gonna Need It | Don't build speculatively |
| **WET** | Write Everything Twice | Anti-DRY (avoid) |
| **SOLID** | See above | See above |
| **GRASP** | General Responsibility Assignment Software Patterns | 9 patterns for OOD |
| **PoLA** | Principle of Least Astonishment | Expected behavior |
| **LoD** | Law of Demeter | Talk only to friends |
| **SoC** | Separation of Concerns | Modularize |
| **CoC** | Convention over Configuration | Sensible defaults |
| **SRP** | Single Responsibility Principle | See SOLID |
| **ISP** | Interface Segregation Principle | See SOLID |
| **DIP** | Dependency Inversion Principle | See SOLID |
| **CCP** | Common Closure Principle | Classes change together |
| **CRP** | Common Reuse Principle | Classes used together stay together |
| **SAP** | Stable Abstractions Principle | Stable = abstract |
| **ADP** | Acyclic Dependencies Principle | No circular deps |
| **SDP** | Stable Dependencies Principle | Depend on stable |
| **REP** | Reuse/Release Equivalence Principle | Granular releases |

### Design Smells (Anti-Patterns)

| Smell | Description |
|-------|-------------|
| **God Object** | Class doing too much |
| **Shotgun Surgery** | One change requires many modifications |
| **Feature Envy** | Class overly using another class |
| **Data Clump** | Group of variables always together |
| **Primitive Obsession** | Overuse of primitives over objects |
| **Long Parameter List** | Too many parameters |
| **Divergent Change** | Class changes for multiple reasons |
| **Parallel Inheritance** | Two hierarchies that evolve together |
| **Message Chain** | Long chain of method calls |
| **Middle Man** | Class doing nothing but delegation |
| **Inappropriate Intimacy** | Classes knowing too much about each other |
| **Refused Bequest** | Unused inherited methods |
| **Speculative Generality** | "I might need this" code |
| **Temporary Field** | Instance variable only sometimes used |

---

## 3. Architectural Patterns

### Core Architecture Styles

| Pattern | Description | Use Case |
|---------|-------------|----------|
| **Clean Architecture** | Layers: Domain, Application, Interface, Infrastructure | Enterprise apps |
| **Hexagonal Architecture** | Ports & Adapters, Inside-Out design | Framework-agnostic |
| **Onion Architecture** | Core, Services, Interfaces concentric layers | Testable domains |
| **CQRS** | Command Query Responsibility Segregation | Read/write scaling |
| **Event Sourcing** | Store events, not state | Audit trails, replay |
| **EDA** | Event-Driven Architecture | Real-time, microservices |
| **Microservices** | Small, independent services | Scalable systems |
| **Monolith** | Single deployable unit | Simple apps |
| **Modular Monolith** | Monolith with clear modules | Gradual migration |
| **Service-Oriented (SOA)** | Services with ESB | Enterprise integration |
| **Serverless** | Function-as-a-Service | Event-driven, scale-to-zero |
| **Edge Computing** | Process at network edge | IoT, latency-critical |

### Layered Architecture

```
┌─────────────────────────────────────────┐
│           Presentation Layer             │
│  (UI, API Gateway, Controllers, Views)  │
├─────────────────────────────────────────┤
│           Application Layer              │
│    (Use Cases, Commands, Queries,       │
│     Application Services, Orchestration)  │
├─────────────────────────────────────────┤
│             Domain Layer                 │
│  (Entities, Value Objects, Aggregates,   │
│   Domain Services, Events, Interfaces)   │
├─────────────────────────────────────────┤
│         Infrastructure Layer              │
│  (Repositories, External Services,       │
│   Frameworks, DB, Messaging, Caching)   │
└─────────────────────────────────────────┘
```

### Hexagonal Architecture (Ports & Adapters)

```
                    ┌─────────────────┐
                    │   Primary/Driving│
                    │   Adapters       │
                    │  (REST, GraphQL, │
                    │   CLI, UI)       │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │     Ports      │
                    │  (Inbound)     │
                    │                │
                    │ ┌────────────┐ │
                    │ │   Domain   │ │
                    │ │   Core     │ │
                    │ │            │ │
                    │ │ - Entities │ │
                    │ │ - Services │ │
                    │ │ - Events   │ │
                    │ │ - Ports    │ │
                    │ └────────────┘ │
                    │                │
                    │     Ports      │
                    │  (Outbound)    │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │Secondary/Driven │
                    │    Adapters     │
                    │ (DB, Cache,     │
                    │  External APIs) │
                    └─────────────────┘
```

---

## 4. Quality Assurance Methods

### Testing Pyramid

```
                    ▲
                   ╱ ╲
                  ╱   ╲
                 ╱  E2E╲
                ╱───────╲
               ╱         ╲
              ╱  Integr. ╲
             ╱───────────╲
            ╱             ╲
           ╱    Unit      ╲
          ╱───────────────╲
```

### QA Practices

| Practice | Description |
|----------|-------------|
| **Shift-Left Testing** | Test earlier in lifecycle |
| **Continuous Testing** | Automated tests in CI/CD |
| **Shift-Right Testing** | Test in production |
| **Canary Testing** | Gradual rollout to subset |
| **A/B Testing** | Compare two versions |
| **Blue-Green Deploy** | Zero-downtime deployment |
| **Chaos Engineering** | Deliberately break production |
| **Synthetic Monitoring** | Proactive production testing |
| **Contract Testing** | API contract verification |
| **Semantic Testing** | Meaningful test names |

### Code Review Practices

| Practice | Description |
|----------|-------------|
| **Peer Review** | Another developer reviews |
| **Pair Programming** | Two developers, one machine |
| **Mob Programming** | Whole team, one machine |
| **Code Walkthrough** | Author-led review meeting |
| **Static Analysis** | Automated code analysis |
| **Linting** | Style and pattern enforcement |
| **Pre-commit Reviews** | Pre-commit hooks, CI gates |

---

## 5. Process & Delivery

### Development Processes

| Process | Description |
|---------|-------------|
| **DevOps** | Dev + Ops collaboration |
| **GitOps** | Git as single source of truth |
| **Agile** | Iterative, adaptive |
| **Scrum** | Sprints, roles, ceremonies |
| **Kanban** | Visual workflow, WIP limits |
| **SAFe** | Scaled Agile Framework |
| **LeSS** | Large-Scale Scrum |
| **XP** | Extreme Programming |
| **Lean** | Eliminate waste |
| **Six Sigma** | Data-driven process improvement |
| **Continuous Integration (CI)** | Frequent integration |
| **Continuous Delivery (CD)** | Automated delivery pipeline |
| **Continuous Deployment** | Auto-deploy to production |
| **DevSecOps** | Security in DevOps |
| **GitHub Flow** | Simple branching for teams |
| **Trunk-Based Development** | Short-lived branches |
| **GitFlow** | Feature, develop, release branches |

### Delivery Metrics

| Metric | Description |
|--------|-------------|
| **DORA Metrics** | Deployment Frequency, Lead Time, MTTR, Change Failure Rate |
| **Cycle Time** | Time from start to done |
| **Lead Time** | Time from request to production |
| **MTTR** | Mean Time To Recovery |
| **MTBF** | Mean Time Between Failures |
| **SLI/SLO/SLA** | Service Level Indicators/Objectives/Agreements |
| **Escape Velocity** | Issues found in production |
| **Defect Density** | Bugs per LOC |

---

## 6. Documentation Approaches

### Documentation Types

| Type | Description | Tool |
|------|-------------|------|
| **ADRs** | Architecture Decision Records | adr-tools |
| **RFCs** | Request for Comments | md RFC |
| **Design Docs** | Technical design specs | Markdown |
| **Runbooks** | Operational procedures | Wiki/MD |
| **API Docs** | API specifications | OpenAPI/Swagger |
| **Code Comments** | Inline documentation | JSDoc, Rustdoc |
| **README** | Project overview | MD |
| **CHANGELOG** | Version history | Keep a Changelog |
| **CONTRIBUTING** | Contribution guide | MD |
| **Architecture Diagrams** | Visual architecture | Mermaid, PlantUML, C4 |

### Specification Methods

| Method | Description |
|--------|-------------|
| **OpenAPI/Swagger** | REST API specification |
| **AsyncAPI** | Async API specification |
| **GraphQL Schema** | GraphQL type system |
| **Protobuf/Thrift** | IDL for RPC |
| **JSON Schema** | JSON structure validation |
| **RAML** | RESTful API Modeling Language |
| **OAS** | OpenAPI Specification |

---

## 7. Emerging AI-Driven Methods

| Method | Description |
|--------|-------------|
| **AI-DD** | AI-assisted Development |
| **Prompt-Driven Development (PDD)** | Prompts as specs |
| **StoryDD** | AI generates user stories |
| **TraceDD** | AI traces requirements to code |
| **AI Pair Programming** | Copilot, Cursor, etc. |
| **Automated Code Review** | AI-powered review |
| **AI-Generated Tests** | LLMs write tests |
| **Semantic Search** | Natural language code search |
| **AI Refactoring** | LLM-assisted refactoring |
| **Documentation Generation** | Auto-generate docs |
| **Test Case Generation** | AI generates edge cases |

---

## 8. Code Organization Patterns

### Top-Level Directory Structure

```
project/
├── src/                    # Source code (language-specific layout)
├── lib/                    # Libraries (shared code)
├── bin/                    # Executables
├── cmd/                    # Command-line applications
├── internal/               # Private application code
├── pkg/                    # Public packages
├── api/                    # API definitions (protobuf, OpenAPI)
├── configs/                # Configuration files
├── scripts/                # Build, deployment, utility scripts
├── tests/                  # Integration/e2e tests
├── testdata/               # Test fixtures
├── docs/                   # Documentation
├── website/                # GitHub Pages, Docusaurus
├── examples/               # Example code
├── third_party/             # External dependencies
├── tools/                  # Tools used by this project
├── Makefile                # Build automation
├── README.md               # Project overview
├── LICENSE                 # License
├── CHANGELOG.md            # Version history
├── CONTRIBUTING.md         # Contribution guide
├── .github/                # GitHub Actions, templates
└── .gitignore
```

### Layered Architecture Layout

```
src/
├── domain/                 # Core business logic (framework-agnostic)
│   ├── entities/           # Domain entities
│   ├── value_objects/      # Immutable value types
│   ├── aggregates/         # Aggregate roots
│   ├── services/           # Domain services
│   ├── events/             # Domain events
│   ├── exceptions/         # Domain exceptions
│   └── ports/              # Port interfaces (inbound & outbound)
│
├── application/           # Use cases, application services
│   ├── commands/          # Write operations
│   ├── queries/           # Read operations
│   ├── handlers/          # Command/query handlers
│   ├── services/         # Application services
│   ├── dtos/             # Data transfer objects
│   └── mappers/          # Entity <-> DTO mappers
│
├── infrastructure/         # External concerns
│   ├── persistence/       # Database implementations
│   │   ├── repositories/
│   │   ├── mappers/
│   │   └── schemas/
│   ├── messaging/         # Message queue implementations
│   ├── external/         # External service clients
│   └── framework/        # Framework-specific code
│
├── interface/             # Adapters (inbound)
│   ├── controllers/      # API controllers
│   ├── presenters/       # Response formatting
│   ├── gateways/         # Interface adapters
│   └── handlers/         # Event handlers
│
└── config/               # Configuration
```

### Hexagonal Architecture Layout

```
src/
├── domain/                    # Core domain (no dependencies)
│   ├── model/                 # Entities, value objects
│   ├── service/               # Domain services
│   ├── event/                 # Domain events
│   └── port/                  # Port interfaces
│       ├── inbound/           # Primary ports (driving)
│       └── outbound/          # Secondary ports (driven)
│
├── application/               # Use cases
│   ├── service/               # Application services
│   ├── command/               # Command handlers
│   └── query/                 # Query handlers
│
├── adapter/                   # Adapters (implement ports)
│   ├── primary/              # Driving adapters
│   │   ├── rest/             # REST API
│   │   ├── graphql/          # GraphQL API
│   │   ├── cli/              # CLI
│   │   └── ui/               # UI adapters
│   │
│   └── secondary/            # Driven adapters
│       ├── persistence/       # DB repositories
│       ├── cache/             # Caching
│       ├── messaging/         # Message queues
│       └── external/          # External services
│
└── infrastructure/            # Framework, config
```

### Microservices Layout

```
services/
├── user-service/
│   ├── src/
│   ├── tests/
│   ├── Dockerfile
│   └── Makefile
│
├── order-service/
│   ├── src/
│   ├── tests/
│   ├── Dockerfile
│   └── Makefile
│
├── payment-service/
│   ├── src/
│   ├── tests/
│   ├── Dockerfile
│   └── Makefile
│
├── api-gateway/
│   ├── src/
│   └── nginx.conf
│
└── service-mesh/
    └── istio/
```

---

## 9. Repository Structure

### Monorepo

```
monorepo/
├── packages/
│   ├── package-a/
│   ├── package-b/
│   └── shared/
├── apps/
│   ├── web/
│   └── mobile/
├── infrastructure/
│   ├── terraform/
│   └── k8s/
├── docs/
├── scripts/
├── tooling/
├── package.json             # Workspace root
└── pnpm-workspace.yaml
```

### Polyrepo

```
org/
├── frontend/
├── backend/
├── shared-libs/
│   ├── utils/
│   ├── types/
│   └── ui-components/
├── infrastructure/
└── docs/
```

### Repository Types

| Type | Pros | Cons | Best For |
|------|------|------|----------|
| **Monorepo** | Atomic changes, shared deps, easy refactor | Scale challenges, CI complexity | Small-medium teams |
| **Polyrepo** | Independent deploys, clear ownership | Duplication, coordination | Large orgs, multiple teams |
| **Modular Monolith** | Monolith benefits + modularity | Scale challenges | Starting out, gradual migration |

---

## 10. Polyrepo & Microservice Patterns

### Service Patterns

| Pattern | Description |
|---------|-------------|
| **Strangler Fig** | Incrementally replace monolith |
| **Anti-Corruption Layer** | Translate between systems |
| **Backends for Frontends (BFF)** | API tailored per frontend |
| **API Gateway** | Single entry point |
| **Sidecar** | Co-located helper container |
| **Ambassador** | Sidecar for external services |
| **Adapter** | Protocol translation |
| **Circuit Breaker** | Prevent cascade failures |
| **Bulkhead** | Isolate failures |
| **Retry** | Automatic retry with backoff |
| **Timeout** | Fail fast on slow responses |
| **Dead Letter Queue** | Failed message handling |
| **Saga** | Distributed transactions |
| **Outbox** | Reliable event publishing |
| **Compensating Transaction** | Undo failed operations |
| **Claim Check** | Store large messages externally |

### Communication Patterns

| Pattern | Description |
|---------|-------------|
| **Request/Response** | Synchronous RPC |
| **Publish/Subscribe** | Event-based |
| **Point-to-Point** | Direct messaging |
| **Message Queue** | Async via queue |
| **Event Streaming** | Kafka, Pulsar |
| **gRPC** | High-performance RPC |
| **GraphQL** | Query-based API |
| **Webhook** | HTTP callbacks |

---

## 11. Plugin & Library Architecture

### Plugin Patterns

| Pattern | Description |
|---------|-------------|
| **Plugin Architecture** | Dynamic loading |
| **Extension Points** | Defined hook locations |
| **Sandbox** | Isolated execution |
| **Feature Flags** | Toggle features |
| **Strategy** | Interchangeable algorithms |
| **Observer** | Event notification |
| **Decorator** | Dynamic behavior |
| **Middleware** | Pipeline processing |
| **Pipeline** | Sequential processing |

### Library Patterns

| Pattern | Description |
|---------|-------------|
| **Facade** | Simple interface to complex system |
| **Builder** | Object construction |
| **Factory** | Object creation |
| **Singleton** | Single instance (use sparingly) |
| **Registry** | Type/object registration |
| **Service Locator** | Runtime service resolution |
| **Dependency Injection** | Injected dependencies |
| **Composition Root** | DI container setup |

---

## 12. Code Quality Metrics

### Code Metrics

| Metric | Description | Target |
|--------|-------------|--------|
| **LOC** | Lines of Code | Minimize, high ratio |
| **Cyclomatic Complexity** | Decision points | < 10 per function |
| **Cognitive Complexity** | Human-understandable | Low |
| **Coupling** | Dependencies between modules | Low |
| **Cohesion** | Relatedness within module | High |
| **Churn** | Frequency of file changes | Monitor hotspots |
| **Coverage** | Test coverage percentage | > 80% |
| **Duplication** | Repeated code | < 5% |
| **Comment Ratio** | Comments to code | 20-30% |

### Maintainability Indicators

| Indicator | Tool |
|-----------|------|
| **Code Climate** | GPA, issues |
| **SonarQube** | Technical debt |
| **ESLint/TSLint** | Linting |
| **Ruff/Flake8** | Python linting |
| **golangci-lint** | Go linting |
| **Clippy** | Rust linting |
| **Prettier** | Formatting |
| **Ruff** | Python formatting |

---

## Appendix A: Quick Reference Card

```
┌─────────────────────────────────────────────────────────────────┐
│                    xDD METHODOLOGIES QUICK REF                 │
├─────────────────────────────────────────────────────────────────┤
│ TDD│BDD│ATDD│SDD│FDD│CDD│MDD│RDD│DDD│ADD│IDD│Property│      │
│ Dev│Beh.│Accep│Spec│Feat│Cons│Modl│Resp│Domn│Attr│Intn│Prop  │
├─────────────────────────────────────────────────────────────────┤
│ SOLID: S│O│D│L│I  │  DRY│KISS│YAGNI│GRASP│PoLA│LoD│SoC│CoC   │
│ 5 Princ│ple│s   │  Don't│Keep│Youre│8 Pat│Least│Talk│Sepa│Conv│
├─────────────────────────────────────────────────────────────────┤
│ ARCHITECTURE: Clean│Hexagonal│Onion│CQRS│EDA│Microservices    │
│                │Ports   │Layers│Command│Event│Small           │
├─────────────────────────────────────────────────────────────────┤
│ QUALITY: Mutation│Contract│Chaos│Fuzz│Snapshot│Property        │
│              │Test │Testing│Engin│Test │Test   │Test          │
├─────────────────────────────────────────────────────────────────┤
│ PROCESS: Agile│Scrum│Kanban│DevOps│GitOps│CI/CD│Lean│XP      │
│           │Iter│Sprnt│Flow │Dev+│Git  │Auto  │Waste│Pair       │
│           │    │     │     │Ops │Src  │Deliv │    │Prog        │
├─────────────────────────────────────────────────────────────────┤
│ DOCS: ADR│RFC│DesignDoc│Runbook│API│README│CHANGELOG│SpecDD   │
│       │Arc│Req For│Tech  │Op  │Open│Proj  │Vers  │Spec         │
│       │Dec│Commnt │Spec  │Man │API │Info  │Hist  │Driven       │
└─────────────────────────────────────────────────────────────────┘
```

---

## Appendix B: Pattern Selection Guide

```
┌────────────────────────────────────────────────────────────────┐
│                    WHEN TO USE WHICH PATTERN                   │
├────────────────────────────────────────────────────────────────┤
│ COMPLEX DOMAIN                                              │
│  → DDD + Hexagonal + Clean Architecture                       │
│  → Event Sourcing for audit/replay                            │
│  → CQRS for read/write separation                            │
├────────────────────────────────────────────────────────────────┤
│ SIMPLE CRUD APP                                              │
│  → Layered Architecture                                       │
│  → TDD for core logic                                         │
│  → Simple Repository pattern                                   │
├────────────────────────────────────────────────────────────────┤
│ MICROSERVICES                                                │
│  → API Gateway + BFF                                          │
│  → Circuit Breaker + Retry                                     │
│  → Saga for distributed transactions                          │
│  → Event Sourcing for async communication                      │
├────────────────────────────────────────────────────────────────┤
│ FRONTEND-HEAVY                                               │
│  → Feature-Sliced Design                                       │
│  → Component-driven development                               │
│  → Storybook for component docs                               │
│  → BDD for integration                                        │
├────────────────────────────────────────────────────────────────┤
│ LIBRARY/SDK                                                  │
│  → Semantic Versioning                                        │
│  → API-first design                                           │
│  → Consumer-driven contracts                                  │
│  → Comprehensive tests                                       │
├────────────────────────────────────────────────────────────────┤
│ DATA-INTENSIVE                                               │
│  → Event Sourcing + CQRS                                      │
│  → Read replicas                                               │
│  → Batch processing patterns                                  │
├────────────────────────────────────────────────────────────────┤
│ REAL-TIME                                                    │
│  → Event-Driven Architecture                                   │
│  → WebSocket adapters                                         │
│  → Saga for orchestration                                     │
└────────────────────────────────────────────────────────────────┘
```

---

## Appendix C: Anti-Patterns to Avoid

| Anti-Pattern | Description | Solution |
|--------------|-------------|----------|
| **Spaghetti Code** | Tangled logic | Refactor to layered |
| **Golden Hammer** | One tool for all | Match tool to problem |
| **Not Invented Here** | Don't use existing | Evaluate existing first |
| **Premature Optimization** | Optimize early | Profile first |
| **Over-Engineering** | Unnecessary complexity | YAGNI, KISS |
| **Under-Engineering** | Too little design | DDD, Clean Arch |
| **Copy-Paste Programming** | Duplicate code | Extract to shared |
| **Magic Numbers** | Unnamed constants | Use constants/enums |
| **Parallel Worlds** | Duplicated logic | Single source |
| ** Schweitzerian Problem** | Deferred cleanup | Keep code clean |
| **Lava Flow** | Dead code accumulation | Remove dead code |
| **Brain Overload** | Too much in head | Documentation |

---

## Appendix D: Extended xDD & Process Catalog (60+ Named Practices)
## Appendix D: Extended xDD & Process Catalog (100+ Named Practices)

Additional named methodologies, processes, and "xDD" style labels used across industry and Phenotype docs. Use with judgment; combine with **Hexagonal + DDD + TDD/BDD/SDD** as appropriate.

| Tag | Name / Notion | One-line Use |
|-----|---------------|--------------|
| **TDD** | Test-Driven Development | Red → Green → Refactor cycle |
| **BDD** | Behavior-Driven Development | Human-readable behavior specs |
| **ATDD** | Acceptance-Test Driven Development | Tests define acceptance criteria |
| **SDD** | Specification-Driven Development | Formal specs drive implementation |
| **FDD** | Feature-Driven Development | Features as primary unit |
| **CDD** | Consumer-Driven Development | API consumers write contracts |
| **MDD** | Model-Driven Development | Code generation from models |
| **RDD** | Responsibility-Driven Development | Object responsibilities focus |
| **DDD** | Domain-Driven Design | Ubiquitous language, bounded contexts |
| **ADD** | Attribute-Driven Design | Quality attribute focus |
| **IDD** | Interaction-Driven Development | UI/UX interactions drive code |
| **EDD** | Event-Driven Development | Design around domain events |
| **VDD** | Vulnerability-Driven Development | Vuln scan and threat model priority |
| **HDD** | Hypothesis-Driven Development | Metric-validated hypotheses |
| **PDD** | Pain-Driven Development | Developer/customer pain removal |
| **LDD** | Log-Driven Development | Structured logs guide fixes |
| **IFD** | Interface-First Design | Contracts before implementations |
| **ODD** | Observability-Driven Development | SLOs, metrics, traces as design inputs |
| **XDD** | *eX*-Driven Development (umbrella) | Any discipline where X is the driver |
| **NbDD** | Notebook-Driven Development | Jupyter/observable notebooks |
| **DDD-lite** | Pragmatic DDD | Bounded contexts without full ES/CQRS |
| **SpecDD** | Specification-Driven Development | Formal specs as source of truth |
| **DocDD** | Documentation-Driven Development | Docs before code |
| **ReadmeDD** | Readme-Driven Development | Write README first |
| **ContractDD** | Contract-Driven Development | Consumer contracts first |
| **API-DD** | API-Driven Development | API design before implementation |
| **CodeDD** | Code Generation-Driven | Templates generate boilerplate |
| **TraceDD** | Traceability-Driven Development | Requirements to code mapping |
| **StoryDD** | Story-Driven Development | User stories drive features |
| **IncidentDD** | Incident-Driven Development | Past incidents inform design |
| **DebtDD** | Technical Debt-Driven | Debt prioritization guides work |
| **AI-DD** | AI-Assisted Development | LLM augments dev workflow |
| **PromptDD** | Prompt-Driven Development | Prompts as executable specs |
| **CopilotDD** | Copilot-Driven Development | AI pair programming as norm |
| **ReviewDD** | Review-Driven Development | Code review as quality gate |
| **MetricsDD** | Metrics-Driven Development | Data guides decisions |
| **CostDD** | Cost-Driven Development | Cloud cost as factor |
| **PerfDD** | Performance-Driven Development | Benchmarks guide optimization |
| **SecDD** | Security-Driven Development | Threat models guide design |
| **A11yDD** | Accessibility-Driven Development | A11y from start |
| **I18nDD** | Internationalization-Driven | i18n from architecture |
| **GreenDD** | Green/Sustainable Development | Energy efficiency in code |
| **CQRS** | Command Query Responsibility Segregation | Split read/write models |
| **ES** | Event Sourcing | Events as source of truth |
| **EDA** | Event-Driven Architecture | Events trigger processing |
| **SEDA** | Staged Event-Driven Architecture | Multi-stage event processing |
| **KEDA** | Kubernetes Event-Driven Autoscaling | Event-driven scaling |
| **API-first** | API-First Design | OpenAPI/Proto before impl |
| **Mobile-first** | Mobile-First UX | Constraints-first responsive |
| **Security-first** | Security-First | Threat modeling early |
| **Privacy-by-Design** | Privacy-by-Design | Data minimization |
| **Shift-left** | Shift-Left Testing | Tests earlier in lifecycle |
| **Shift-right** | Shift-Right Testing | Production experimentation |
| **SRE** | Site Reliability Engineering | Error budgets, toil reduction |
| **DevSecOps** | DevSecOps | Security in CI/CD |
| **GitOps** | GitOps | Git as infra source of truth |
| **NoOps** | NoOps (managed) | Managed services preference |
| **FinOps** | FinOps | Cloud cost visibility |
| **Platform Eng** | Platform Engineering | IDP and golden paths |
| **IDP** | Internal Developer Platform | Self-service tooling |
| **SBO** | Service-Based Organization | Ownership maps to services |
| **Team Topologies** | Team Topologies | Stream-aligned, platform teams |
| **Wardley Mapping** | Wardley Mapping | Value chain evolution |
| **C4 Model** | C4 Model | Context/Container/Component/Code |
| **ARCHIMATE** | ArchiMate | Enterprise architecture notation |
| **TOGAF** | TOGAF | ADM phases for enterprise governance |
| **Zachman** | Zachman Framework | Multi-dimensional enterprise views |
| **4+1 Views** | 4+1 Architectural Views | Logical, process, dev, physical, scenarios |
| **12-Factor** | Twelve-Factor App | Cloud-native app principles |
| **Well-Architected** | AWS Well-Architected | Cloud review pillars |
| **CIS** | CIS Benchmarks | Hardening baselines |
| **STRIDE** | STRIDE Threat Model | Spoofing, tampering, etc. |
| **PASTA** | PASTA Threat Modeling | Risk-centric threat modeling |
| **LINDDUN** | LINDDUN Privacy | Privacy threat modeling |
| **OWASP ASVS** | ASVS | Application security verification |
| **OWASP SAMM** | SAMM | Software assurance maturity |
| **NIST SSDF** | Secure SDLC | NIST secure software dev |
| **SLSA** | Supply-chain Levels | Artifact provenance levels |
| **Sigstore** | Sigstore / Cosign | Signing and OCI provenance |
| **SBOM** | Software Bill of Materials | SPDX/CycloneDX inventories |
| **OpenTelemetry** | OpenTelemetry | Traces, metrics, logs standard |
| **eBPF** | eBPF Observability | Kernel-level introspection |
| **Chaos Eng.** | Chaos Engineering | Steady-state experiments |
| **Game Days** | Game Days | Rehearsed incident response |
| **Blameless PM** | Blameless Postmortems | Learning from incidents |
| **SLI/SLO/SLA** | Reliability Metrics | Indicators, objectives, agreements |
| **Error Budgets** | Error Budget Policy | Velocity vs reliability balance |
| **Feature Flags** | Feature Toggles | Controlled rollout |
| **Blue/Green** | Blue/Green Deploy | Zero-downtime cutover |
| **Canary** | Canary Releases | Gradual traffic shifting |
| **Progressive Deliv.** | Progressive Delivery | Flags + metrics + automation |
| **Trunk-Based Dev** | Trunk-Based Development | Short-lived branches |
| **Ship/Show/Ask** | Ship/Show/Ask | Risk-based release patterns |
| **Shape Up** | Shape Up (Basecamp) | Betting table, cycles, pitches |
| **Lean Startup** | Build-Measure-Learn | Validated learning |
| **OKRs** | Objectives & Key Results | Outcome alignment |
| **North Star Metric** | North Star Metric | Single primary product metric |
| **WCAG** | Web Accessibility | A11y conformance |
| **Performance Budget** | Performance Budget | UX performance limits |
| **SemVer** | Semantic Versioning | API version compatibility |
| **Conventional Commits** | Conventional Commits | Machine-readable changelog |
| **OpenAPI** | OpenAPI | HTTP API contract |
| **AsyncAPI** | AsyncAPI | Event/API async contracts |
| **GraphQL** | GraphQL | Client-driven schema |
| **gRPC** | gRPC / Protobuf | Efficient RPC contracts |
| **Saga** | Saga Pattern | Distributed transaction orchestration |
| **Outbox** | Outbox Pattern | Reliable event publishing |
| **Idempotency** | Idempotent Operations | Safe retries |
| **DDD Strategic** | Strategic DDD | Context maps, bounded contexts |
| **DDD Tactical** | Tactical DDD | Entities, aggregates, value objects |
| **Clean Arch.** | Clean Architecture | Dependency rule inward |
| **Hexagonal** | Ports & Adapters | Testable boundaries |
| **Onion Arch.** | Onion Architecture | Layered dependency inversion |
| **Microkernel** | Microkernel / Plugin | Core + plugins |
| **Extensibility** | Extensibility Points | Plugin seams without core churn |
| **ADR** | Architecture Decision Records | Documented decisions |
| **RFC** | Request for Comments | Design discussion |
| **PRD** | Product Requirements Doc | Product feature specs |
| **Spec** | Specification | Detailed requirements |
| **Design Review** | Design Review | Architecture validation |
| **Code Review** | Code Review | Peer quality check |
| **Pair Programming** | Pair Programming | Two devs, one machine |
| **Mob Programming** | Mob Programming | Whole team, one machine |
| **Ensemble** | Ensemble Programming | Collaborative coding |
| **Sprint** | Sprint | Time-boxed iteration |
| **Kanban** | Kanban | Visual workflow management |
| **Backlog** | Backlog Grooming | Story refinement |
| **Retrospective** | Retrospective | Team process improvement |
| **Standup** | Daily Standup | Daily sync |
| **Planning** | Sprint Planning | Commitment setting |
| **Refinement** | Backlog Refinement | Story acceptance criteria |
| **Desk Check** | Desk Check | Quick review |
| **Debugging** | Debugging | Issue investigation |
| **Profiling** | Profiling | Performance analysis |
| **Benchmarking** | Benchmarking | Performance comparison |
| **Load Testing** | Load Testing | Capacity verification |
| **Stress Testing** | Stress Testing | Breaking point finding |
| **Soak Testing** | Soak Testing | Long-duration reliability |
| **Smoke Testing** | Smoke Testing | Basic functionality check |
| **Sanity Testing** | Sanity Testing | Core feature verification |
| **Regression Testing** | Regression Testing | Existing feature protection |
| **End-to-End Testing** | E2E Testing | Full flow verification |
| **Integration Testing** | Integration Testing | Component interaction |
| **Unit Testing** | Unit Testing | Individual component test |
| **Mutation Testing** | Mutation Testing | Test quality verification |
| **Property-Based Testing** | Property-Based Testing | Random input generation |
| **Fuzz Testing** | Fuzz Testing | Random malformed input |
| **Snapshot Testing** | Snapshot Testing | Output comparison |
| **Golden Testing** | Golden Testing | Expected file comparison |
| **Contract Testing** | Contract Testing | API contract verification |
| **Semantic Testing** | Semantic Testing | Meaningful test names |
| **Synthetic Monitoring** | Synthetic Monitoring | Proactive production testing |
| **Incident Response** | Incident Response | Production issue handling |
| **Blameless Review** | Blameless Review | Learning without blame |
| **Runbook** | Runbook | Operational procedures |
| **Postmortem** | Postmortem | Incident analysis document |
| **On-call** | On-call | Production support rotation |
| **SLO** | Service Level Objective | Reliability target |
| **Error Budget** | Error Budget | Allowed failure budget |
| **Toil Reduction** | Toil Reduction | Automation of repetitive work |
| **Self-Healing** | Self-Healing | Automatic recovery |
| **Auto-scaling** | Auto-scaling | Dynamic capacity management |
| **Circuit Breaker** | Circuit Breaker | Failure isolation |
| **Bulkhead** | Bulkhead | Failure isolation pattern |
| **Retry** | Retry Pattern | Automatic retry with backoff |
| **Timeout** | Timeout Pattern | Fail fast on slow responses |
| **Dead Letter Queue** | Dead Letter Queue | Failed message handling |
| **Compensating Transaction** | Compensating Transaction | Undo failed operations |
| **Claim Check** | Claim Check | Store large messages externally |
| **Anti-Corruption Layer** | Anti-Corruption Layer | System translation |
| **Strangler Fig** | Strangler Fig | Incremental replacement |
| **Sidecar** | Sidecar Pattern | Co-located helper |
| **Ambassador** | Ambassador Pattern | External service proxy |
| **Adapter** | Adapter Pattern | Protocol translation |
| **Facade** | Facade Pattern | Simple interface |
| **Decorator** | Decorator Pattern | Dynamic behavior |
| **Proxy** | Proxy Pattern | Surrogate object |
| **Bridge** | Bridge Pattern | Abstraction/implementation split |
| **Composite** | Composite Pattern | Tree structures |
| **Flyweight** | Flyweight Pattern | Shared state |
| **State** | State Pattern | Object state transitions |
| **Strategy** | Strategy Pattern | Interchangeable algorithms |
| **Template Method** | Template Method Pattern | Algorithm skeleton |
| **Observer** | Observer Pattern | Event notification |
| **Mediator** | Mediator Pattern | Centralized communication |
| **Chain of Responsibility** | Chain of Responsibility | Request passing |
| **Command** | Command Pattern | Request as object |
| **Iterator** | Iterator Pattern | Sequential access |
| **Visitor** | Visitor Pattern | Operation on object structure |
| **Interpreter** | Interpreter Pattern | Language interpretation |
| **Memento** | Memento Pattern | State capture/restore |
| **Builder** | Builder Pattern | Object construction |
| **Factory Method** | Factory Method Pattern | Object creation |
| **Abstract Factory** | Abstract Factory Pattern | Related object creation |
| **Prototype** | Prototype Pattern | Object cloning |
| **Singleton** | Singleton Pattern | Single instance |
**Org-wide standard:** see [PHENOTYPE_ORG_WIDE_ENGINEERING_STANDARD.md](./PHENOTYPE_ORG_WIDE_ENGINEERING_STANDARD.md).

---

## See Also

- [Architecture Decision Records (ADRs)](./adrs/)
- [Phenotype org–wide engineering standard](./PHENOTYPE_ORG_WIDE_ENGINEERING_STANDARD.md)
- [Architecture decision tree](./architecture-decision-tree.md)
- [AgilePlus mandate](./agileplus-mandate.md)
- [Release branch governance](./release-branch-governance.md)
- `thegent/docs/governance/23_ARCHITECTURAL_GOVERNANCE.md` (hexagonal, polyrepo, XDD)
