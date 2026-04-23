# xDD Methodology Catalog: Extended Edition

## Complete Reference: 150+ Development-Driven Methodologies

Status: Living document for Phenotype ecosystem architecture governance
Version: 2.2.0 | Updated: 2026-03-25

---

## Table of Contents

1. [Development Methodologies (25)](#development-methodologies-25)
2. [Design Principles (30)](#design-principles-30)
3. [Architectural Patterns (25)](#architectural-patterns-25)
4. [Quality Assurance (25)](#quality-assurance-25)
5. [Process & DevOps (25)](#process--devops-25)
6. [Documentation (15)](#documentation-15)
7. [AI-Augmented (20)](#ai-augmented-20)
8. [Data & Persistence (15)](#data--persistence-15)
9. [Team & Collaboration (10)](#team--collaboration-10)
10. [Security & Compliance (10)](#security--compliance-10)
11. [Performance & Optimization (10)](#performance--optimization-10)
12. [Legacy & Modernization (10)](#legacy--modernization-10)

---

## Development Methodologies (25)

| Acronym | Full Name | Description | When to Apply |
|---------|-----------|-------------|--------------|
| **TDD** | Test-Driven Development | Write failing tests before code, then make them pass | All production code |
| **BDD** | Behavior-Driven Development | Test from user behavior perspective using Gherkin | User-facing features |
| **DDD** | Domain-Driven Design | Model based on business domain bounded contexts | Complex domains |
| **ATDD** | Acceptance Test-Driven Development | Define acceptance criteria first | User stories |
| **SDD** | Schema-Driven Development | Define schemas/contracts first | API design |
| **FDD** | Feature-Driven Development | Build features incrementally | Large projects |
| **CDD** | Contract-Driven Development | Test service contracts | Microservices |
| **IDD** | Intent-Driven Development | Start with user intent | User story mapping |
| **MDD** | Model-Driven Development | Generate code from models | Code generation |
| **RDD** | Resource-Driven Development | Design REST APIs around resources | REST services |
| **ADD** | Architecture-Driven Development | Start with architecture design | System design |
| **ODD** | Ontology-Driven Development | Use semantic ontologies | Knowledge systems |
| **LDD** | Legacy-Driven Development | Modernize incrementally | Brownfield projects |
| **PDD** | Pattern-Driven Development | Apply design patterns | Architectural work |
| **QDD** | Query-Driven Development | Design around data access | Data-intensive apps |
| **SDD** | Specification-Driven Development | Detailed specs before code | Complex features |
| **VDD** | Verification-Driven Development | Formal verification integration | Safety-critical systems |
| **VDD** | Value-Driven Development | Business value prioritization | MVP development |
| **VDD** | Velocity-Driven Development | Sprint velocity optimization | Agile teams |
| **EDDD** | Event-Driven Domain Design | DDD + Event Sourcing | Event-centric domains |
| **LADD** | Lifecycle-Aware Development | Handle component lifecycles | Android, UI frameworks |
| **AIDD** | Agent-Intent-Driven Development | Agent orchestration of intent | Multi-agent systems |
| **HyDD** | Hybrid-Driven Development | Mix methodologies per module | Polyglot projects |
| **DevDD** | Developer Experience-Driven | DX as primary metric | Framework/tooling |
| **UXDD** | User Experience-Driven Development | UX research integration | Consumer apps |

---

## Design Principles (30)

### Core Principles (10)

| Acronym | Full Name | Description | Example |
|---------|-----------|-------------|---------|
| **DRY** | Don't Repeat Yourself | Single source of truth | Extract common functions |
| **KISS** | Keep It Simple, Stupid | Prefer simplicity | Simple over clever |
| **YAGNI** | You Aren't Gonna Need It | No premature features | Don't over-engineer |
| **SOLID** | Single, O/C/L/I/D | Object-oriented principles | See below |
| **SRP** | Single Responsibility Principle | One reason to change | One class, one job |
| **OCP** | Open/Closed Principle | Open for extension | Use interfaces |
| **LSP** | Liskov Substitution Principle | Subtype contracts | Duck typing |
| **ISP** | Interface Segregation Principle | Fine-grained interfaces | Many small interfaces |
| **DIP** | Dependency Inversion Principle | Depend on abstractions | Use traits/interface |
| **CRP** | Composite Reuse Principle | Favor composition over inheritance | Mixins, traits |

### Coupling & Cohesion (10)

| Acronym | Full Name | Description | Example |
|---------|-----------|-------------|---------|
| **GRASP** | General Responsibility Assignment | Responsibility patterns | Expert, Creator, Low Coupling |
| **LoD** | Law of Demeter | Minimize coupling | Tell, don't ask |
| **SoC** | Separation of Concerns | Modular design | UI vs Business Logic |
| **CCP** | Common Closure Principle | Package by release | Changes together |
| **REP** | Reuse-Release Equivalence Principle | Version together | Release atomic |
| **ADP** | Acyclic Dependencies Principle | No circular deps | Dependency graph |
| **SDP** | Stable Dependencies Principle | Depend on stable | Stability ranking |
| **SAP** | Stable Abstractions Principle | Stability = abstraction | Stable = abstract |
| **HighC** | High Cohesion | Related stuff together | Cohesive modules |
| **LowC** | Low Coupling | Minimize dependencies | Loose coupling |

### Cognitive & UX Principles (10)

| Acronym | Full Name | Description | Example |
|---------|-----------|-------------|---------|
| **CoC** | Convention over Configuration | Sensible defaults | Rails conventions |
| **PoLA** | Principle of Least Astonishment | Predictable behavior | Do what users expect |
| **POLP** | Principle of Least Privilege | Minimal permissions | Security-first |
| **POLA** | Principle of Least Authority | Capability-based security | Token-based auth |
| **CQS** | Command Query Separation | Mutate vs query分开 | Read vs write |
| **RAII** | Resource Acquisition Is Initialization | Scope-based resource mgmt | RAII in Rust/C++ |
| **EAFP** | Easier to Ask Forgiveness than Permission | Duck typing, try/catch | Python style |
| **LBYL** | Look Before You Leap | Check preconditions | Java style |
| **CUR** | Cognitive Use Reduction | Minimize mental load | Simple APIs |
| **CH** | Command Hierarchy | Clear command structure | CLI design |

---

## Architectural Patterns (25)

### Core Architecture (10)

| Acronym | Full Name | Description | Use Case |
|---------|-----------|-------------|----------|
| **Hex** | Hexagonal Architecture | Ports and Adapters | Framework independence |
| **Onion** | Onion Architecture | Layered domain-centric | Complex business rules |
| **Clean** | Clean Architecture | Use case-centric layers | Enterprise apps |
| **CQRS** | Command Query Responsibility Segregation | Separate read/write | High-scale systems |
| **EDA** | Event-Driven Architecture | Async event processing | Loose coupling |
| **SAGA** | Saga Pattern | Distributed transactions | Microservices |
| **ES** | Event Sourcing | Store events not state | Audit trails |
| **ECS** | Entity-Component-System | Game engine pattern | Games, simulations |
| **Micro** | Microservices | Small autonomous services | Scalable systems |
| **ModMono** | Modular Monolith | Monolith with boundaries | Start simple |

### Integration Patterns (8)

| Acronym | Full Name | Description | Use Case |
|---------|-----------|-------------|----------|
| **SA** | Service Aggregator | API composition | API gateway |
| **BFF** | Backend for Frontend | Frontend-specific APIs | Multiple clients |
| **Strangler** | Strangler Fig | Incremental migration | Legacy modernization |
| **ACL** | Anti-Corruption Layer | Legacy integration | Legacy systems |
| **MDA** | Model-Driven Architecture | CIM-PIM-PSM | MDA/MDD |
| **P2P** | Point-to-Point | Direct service communication | Simple systems |
| **PubSub** | Publish-Subscribe | Event-based communication | Event systems |
| **SC** | Service Mesh | Infrastructure for service comm | Kubernetes |

### Modern Patterns (7)

| Acronym | Full Name | Description | Use Case |
|---------|-----------|-------------|----------|
| **MicroFE** | Micro Frontends | Frontend decomposition | Web apps |
| **SPA** | Serverless Pattern Architecture | Function-as-a-Service | Event-driven |
| **Edge** | Edge Computing | Process at edge | IoT, CDN |
| **CDN** | Content Delivery Network | Distributed content | Static assets |
| **MultiC** | Multi-Tenancy | Shared infrastructure | SaaS |
| **Cell** | Cell-Based Architecture | Autonomous cell units | High isolation |
| **AutoSC** | Auto-Scaling Clusters | Dynamic scaling | Cloud-native |

---

## Quality Assurance (25)

### Testing Types (15)

| Acronym | Full Name | Description | Tooling |
|---------|-----------|-------------|---------|
| **PBT** | Property-Based Testing | Generate test cases | proptest, fast-check |
| **Muta** | Mutation Testing | Verify test quality | mutest, Stryker |
| **Contract** | Contract Testing | Verify API contracts | Pact, Testcontainers |
| **ShiftL** | Shift-Left Testing | Test earlier | CI integration |
| **Chaos** | Chaos Engineering | Resilience testing | Chaos Monkey |
| **Perf** | Performance Testing | Load/stress testing | k6, JMeter |
| **Sec** | Security Testing | Vulnerability scanning | SonarQube, Semgrep |
| **Fuzz** | Fuzz Testing | Random input testing | AFL, libFuzzer |
| **SBT** | Snapshot-Based Testing | Visual regression | Jest, Playwright |
| **Golden** | Golden Path Testing | Happy path validation | E2E tests |
| **Edge** | Edge Case Testing | Boundary conditions | Boundary analysis |
| **Smoke** | Smoke Testing | Basic sanity checks | Quick CI |
| **Sanity** | Sanity Testing | Build verification | Post-deploy |
| **Regress** | Regression Testing | Prevent regressions | Test suites |
| **E2E** | End-to-End Testing | Full flow testing | Playwright, Cypress |

### Quality Metrics (10)

| Acronym | Full Name | Description | Metric |
|---------|-----------|-------------|--------|
| **Cov** | Code Coverage | % code exercised | 80%+ target |
| **Complex** | Cyclomatic Complexity | Code path complexity | McCabe |
| **CBC** | Coupling Between Components | Dependency analysis | Low CBC |
| **RFC** | Response for a Class | Method call depth | < 10 typical |
| **LCOM** | Lack of Cohesion | Cohesion metric | < 5 |
| **DIT** | Depth of Inheritance Tree | Inheritance depth | < 6 |
| **NOC** | Number of Children | Subclass count | Low |
| **ABM** | Abstraction-Fan-out | Outgoing dependencies | Low |
| **WMC** | Weighted Methods per Class | Complexity sum | < 50 |
| **ATFC** | Adjusted Function Point | Size measurement | Sizing |

---

## Process & DevOps (25)

### CI/CD Pipeline (10)

| Acronym | Full Name | Description | Key Practices |
|---------|-----------|-------------|---------------|
| **CI** | Continuous Integration | Frequent integration | Automated builds/tests |
| **CD** | Continuous Delivery | Automated delivery | Pipeline automation |
| **CDE** | Continuous Deployment | Automated release | Zero-downtime deploys |
| **GitOps** | Git-based Operations | Git as source of truth | ArgoCD, Flux |
| **DevOps** | Dev + Ops collaboration | Shared responsibility | CI/CD, monitoring |
| **SRE** | Site Reliability Engineering | Ops as software | SLOs, error budgets |
| **IaC** | Infrastructure as Code | Codified infra | Terraform, Pulumi |
| **FinOps** | Financial Operations | Cloud cost management | Cost optimization |
| **ShiftR** | Shift-Right Testing | Production testing | Canary, feature flags |
| **Progressive** | Progressive Delivery | Gradual rollouts | Blue-green, canary |

### Project Management (10)

| Acronym | Full Name | Description | Key Practices |
|---------|-----------|-------------|---------------|
| **Agile** | Agile Methodology | Iterative development | Sprints, standups |
| **Scrum** | Scrum Framework | Sprint-based delivery | Roles, ceremonies |
| **Kanban** | Kanban Method | Flow-based delivery | WIP limits, boards |
| **XP** | Extreme Programming | Engineering practices | TDD, pair programming |
| **SAFe** | Scaled Agile Framework | Large-scale Agile | Agile at scale |
| **LeSS** | Large-Scale Scrum | Simplified scaling | Less rules |
| **Nexus** | Nexus Framework | Scaled Scrum | 3-9 teams |
| **Spotify** | Spotify Model | Autonomous squads | Tribes, chapters |
| **ShapeUp** | Shape Up Methodology | 6-week cycles | Appetite-based |
| **AgileUP** | Agile Unified Process | Iterative incremental | UP distilled |

### Ops Practices (5)

| Acronym | Full Name | Description | Tools |
|---------|-----------|-------------|-------|
| **ChatOps** | Chat Operations | Operations in chat | Slack, Teams |
| **AIOps** | AI Operations | ML for ops | Anomaly detection |
| **Observability** | Observability Engineering | Deep system insight | Logs, metrics, traces |
| **OnCall** | On-Call Practices | Incident response | PagerDuty, Opsgenie |
| **Blameless** | Blameless Postmortems | Learn from failures | Culture |

---

## Documentation (15)

| Acronym | Full Name | Description | Artifacts |
|---------|-----------|-------------|-----------|
| **ADRs** | Architecture Decision Records | Document decisions | decisions/*.md |
| **RFC** | Request for Comments | Design discussion | RFCs/*.md |
| **SpecDD** | Specification-Driven Development | Spec as tests | BDD specs |
| **Docs-as-Code** | Documentation as Code | Version-controlled docs | Markdown in repo |
| **Arc42** | Architecture Documentation | Standard arch docs | arc42.org |
| **C4** | Context-Containers-Components-Code | Visual architecture | diagrams/ |
| **UML** | Unified Modeling Language | Visual modeling | class, sequence |
| **OpenAPI** | OpenAPI Specification | REST API docs | openapi.yaml |
| **AsyncAPI** | AsyncAPI Specification | Event API docs | asyncapi.yaml |
| **Storybook** | Component Documentation | UI component docs | component stories |
| **MDD** | Markdown-Driven Documentation | Markdown as source | docs/*.md |
| **LivingDocs** | Living Documentation | Always up-to-date docs | Concordance, Cucumber |
| **Runbooks** | Operations Runbooks | Incident procedures | On-call guides |
| **DesignDocs** | Design Documentation | Feature design specs | RFC style |
| **APIDocs** | API Documentation | Developer reference | Swagger, Redoc |

---

## AI-Augmented (20)

| Acronym | Full Name | Description | Tools |
|---------|-----------|-------------|-------|
| **AI-DD** | AI-Driven Development | AI-assisted coding | Copilot, Claude |
| **PromptDD** | Prompt-Driven Development | Prompt as spec | Prompt engineering |
| **StoryDD** | Story-Driven Development | User stories as tests | Gherkin + AI |
| **TraceDD** | Trace-Driven Development | Observability-first | OpenTelemetry |
| **SynthDD** | Synthetic Data-Driven | AI-generated test data | Faker + AI |
| **LintDD** | Lint-Driven Development | Static analysis as spec | Clippy, ESLint |
| **TypeDD** | Type-Driven Development | Types as spec | TypeScript, Rust |
| **GenDD** | Generative Development | AI code generation | CodeGen, Copilot |
| **RefactorDD** | Refactoring-Driven | Continuous improvement | AI-assisted refactor |
| **AICD** | AI-Collaborative Development | Human-AI pairing | Pair programming with AI |
| **AIAssist** | AI-Assisted Review | Code review by AI | GitHub Copilot |
| **AIBugFind** | AI Bug Detection | ML-based bug finding | CodeQL AI |
| **AISpecGen** | AI Specification Generation | Auto-generate specs | LLM |
| **AITestGen** | AI Test Generation | Auto-generate tests | Copilot, Claude |
| **AICodeReview** | AI Code Review | Automated PR review | Review tools |
| **AIRefactor** | AI-Assisted Refactoring | ML-guided refactors | AI tools |
| **AIChat** | AI Chat-Based Development | Conversational coding | Cursor, Claude |
| **AISummarize** | AI PR Summarization | Auto PR descriptions | AI tools |
| **AIExplain** | AI Code Explanation | Auto documentation | AI tools |
| **AIOnboard** | AI Onboarding | ML-assisted onboarding | AI mentors |

---

## Data & Persistence (15)

| Acronym | Full Name | Description | Use Case |
|---------|-----------|-------------|----------|
| **CRUD** | Create-Read-Update-Delete | Basic data operations | Simple APIs |
| **ACID** | Atomicity-Consistency-Isolation-Durability | Transaction properties | Financial systems |
| **BASE** | Basically Available-Soft state-Eventually consistent | Eventual consistency | Distributed systems |
| **CAP** | Consistency-Availability-Partition tolerance | Distributed trade-offs | System design |
| **2PC** | Two-Phase Commit | Distributed transactions | Distributed DBs |
| **Saga** | Saga Pattern | Choreographed transactions | Microservices |
| **Outbox** | Transactional Outbox | Reliable events | Event sourcing |
| **CDC** | Change Data Capture | Event-driven data sync | Data pipelines |
| **CQRS** | Command Query Responsibility Segregation | Read/write separation | High performance |
| **ETL** | Extract-Transform-Load | Data pipeline | Data warehousing |
| **ELT** | Extract-Load-Transform | Modern data pipeline | Cloud data |
| **DataVault** | Data Vault Modeling | Scalable data warehouse | Enterprise DW |
| **StarSchema** | Star Schema | Dimensional modeling | BI/reporting |
| **Snowflake** | Snowflake Schema | Normalized DW | Complex analytics |
| **DataMesh** | Data Mesh Architecture | Domain-oriented data | Distributed data |

---

## Team & Collaboration (10)

| Acronym | Full Name | Description | Practices |
|---------|-----------|-------------|-----------|
| **MobProg** | Mob Programming | Whole team coding | Ensemble |
| **PairProg** | Pair Programming | Two coders | Driver/navigator |
| **CodeReview** | Code Review | Peer inspection | PR-based |
| **PairReview** | Pair Code Review | Two reviewers | Collaborative |
| **CollectiveOwn** | Collective Code Ownership | Shared ownership | All can edit |
| **TeamTop** | Team Topologies | Org structure | Value streams |
| **UnbrokenWindow** | Broken Window Theory | Maintain quality | No tech debt |
| **Pareto** | Pareto Principle | 80/20 rule | Prioritization |
| **Swarm** | Swarm Testing | Collective QA | Shared testing |
| **Guilds** | Community of Practice | Skill sharing | Cross-team learning |

---

## Security & Compliance (10)

| Acronym | Full Name | Description | Practices |
|---------|-----------|-------------|-----------|
| **SSDLC** | Secure Software Development Lifecycle | Security in SDLC | Security gates |
| **ThreatMod** | Threat Modeling | Security design | STRIDE, PASTA |
| **ZeroTrust** | Zero Trust Architecture | Never trust | Verify always |
| **DevSecOps** | DevSecOps | Security as code | Shift-left security |
| **Compliance-as-Code** | Compliance as Code | Policy as code | Policy engines |
| **SecretsMgmt** | Secrets Management | Secure credentials | Vault, K8s secrets |
| **SBOM** | Software Bill of Materials | Dependency inventory | Security |
| **VEX** | Vulnerability Exploitability eXchange | Vulnerability context | Security |
| **SAST** | Static Application Security Testing | Source code analysis | CI integration |
| **DAST** | Dynamic Application Security Testing | Runtime testing | Black-box |

---

## Performance & Optimization (10)

| Acronym | Full Name | Description | Practices |
|---------|-----------|-------------|-----------|
| **Profile** | Profiling-Driven Optimization | Measure first | perf, py-spy |
| **Bench** | Benchmark-Driven Development | Performance tests | Criterion, GoBench |
| **OptC** | Optimization by Constraints | Constraint-based | Load testing |
| **Caching** | Caching Strategies | Multi-level cache | Redis, CDN |
| **LazyLoad** | Lazy Loading | Defer loading | Images, modules |
| **EagerLoad** | Eager Loading | Pre-fetch | ORM optimization |
| **Memo** | Memoization | Cache function results | Optimization |
| **Pool** | Connection Pooling | Reuse connections | Database pools |
| **Batch** | Batch Processing | Bulk operations | Performance |
| **Async** | Asynchronous Processing | Non-blocking | Performance |

---

## Legacy & Modernization (10)

| Acronym | Full Name | Description | Practices |
|---------|-----------|-------------|-----------|
| **StranglerFig** | Strangler Fig Pattern | Incremental replace | Legacy |
| **BranchByAbstr** | Branch by Abstraction | Abstractions for change | Big rewrites |
| **Decorator** | Decorator Pattern | Extend behavior | Legacy enhancement |
| **Facade** | Facade Pattern | Simplified interface | Legacy wrapper |
| **ParallelRun** | Parallel Run | Run old and new | Migration |
| **FeatureToggle** | Feature Toggles | Gradual rollouts | Modernization |
| **ExpandContract** | Expand-Contract | Phased changes | Breaking changes |
| **BlueGreen** | Blue-Green Deployment | Zero-downtime deploy | Migration |
| **Canary** | Canary Release | Gradual rollout | Risk mitigation |
| **ABTest** | A/B Testing | Compare approaches | Migration validation |

---

## Quick Reference: By Project Phase

### Discovery Phase
- **DDD** - Domain modeling
- **StoryDD** - User story mapping
- **C4** - Context diagrams
- **ADRs** - Capture decisions
- **ODD** - Ontology development

### Design Phase
- **Hex** - Hexagonal Architecture
- **CQRS** - Data architecture
- **SpecDD** - Requirements as tests
- **Arc42** - Documentation
- **ADD** - Architecture design

### Implementation Phase
- **TDD** - Unit tests
- **BDD** - Integration tests
- **Contract** - API contracts
- **PBT** - Property tests
- **CDD** - Contract testing

### Deployment Phase
- **CI/CD** - Automation
- **GitOps** - Infrastructure
- **Smoke** - Sanity checks
- **Chaos** - Resilience
- **Progressive** - Gradual rollout

### Operations Phase
- **SRE** - Reliability
- **TraceDD** - Observability
- **FinOps** - Cost management
- **ShiftR** - Production testing
- **AIOps** - AI-driven ops

---

## Implementation Matrix

| Practice | Complexity | Investment | ROI | Recommended For |
|----------|------------|------------|-----|----------------|
| TDD | Medium | High | High | All production code |
| BDD | Medium | Medium | High | User-facing features |
| DDD | High | Very High | High | Complex domains |
| Hexagonal | Medium | Medium | High | Library code |
| CQRS | Very High | Very High | Medium | High-scale systems |
| Contract Testing | Low | Low | High | Microservices |
| Property Testing | Medium | Medium | High | Data processing |
| Chaos Engineering | High | High | Medium | Critical services |
| AI-DD | Low | Low | High | All development |

---

## Phenotype Ecosystem Compliance

### Mandatory Practices (All Projects)

| Category | Required Practices |
|----------|-------------------|
| **Architecture** | Hexagonal, Clean, SoC, SOLID |
| **Testing** | TDD, BDD, Contract, E2E |
| **Process** | CI/CD, GitOps, Agile |
| **Documentation** | ADRs, RFC, Docs-as-Code |
| **AI** | AI-DD, TypeDD, LintDD |

### Recommended Practices (By Project Type)

| Project Type | Recommended |
|-------------|-------------|
| **Shared Libraries** | SOLID, Hex, Clean |
| **Microservices** | CQRS, EDA, Saga |
| **Web Apps** | BDD, E2E, MicroFE |
| **CLI Tools** | TDD, CLI testing |
| **APIs** | Contract, OpenAPI |

---

## References

- [xDD Patterns - Wikipedia](https://en.wikipedia.org/wiki/List_of_software_development_philosophies)
- [SOLID Principles](https://en.wikipedia.org/wiki/SOLID)
- [Microservices Patterns - Chris Richardson](https://microservices.io/patterns/)
- [Testing Pyramid - Martin Fowler](https://martinfowler.com/articles/practical-test-pyramid.html)
- [Arc42 Documentation](https://arc42.org/)
- [C4 Model](https://c4model.com/)
- [Team Topologies](https://teamtopologies.com/)
- [Living Documentation - Cyrille Martraire](https://livingdocumentation.org/)
- [Domain-Driven Design - Eric Evans](https://domainlanguage.com/ddd/)
- [Clean Architecture - Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Hexagonal Architecture - Alistair Cockburn](https://alistair.cockburn.us/hexagonal-architecture/)
- [Property-Based Testing - John Hughes](https://www.youtube.com/watch?v=zhR2jdxT0p4)
- [Mutation Testing - Pitest](https://pitest.org/)
- [Contract Testing - Pact](https://pact.io/)
- [SAFe Framework](https://www.scaledagileframework.com/)
- [Threat Modeling - OWASP](https://owasp.org/www-community/Threat_Modeling)
- [Zero Trust Architecture - NIST](https://csrc.nist.gov/publications/detail/sp/800-207/final)
