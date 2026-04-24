# helMo Charter

## 1. Mission Statement

**helMo** (agentkit) is a hexagonal architecture CLI framework designed to enable extensible, testable, and maintainable command-line applications within the Phenotype ecosystem. The mission is to provide a robust structural foundation that isolates domain logic from technical concerns—empowering developers to build CLI tools that are pure in their business logic, adaptable in their interfaces, and reliable in their operation.

The project exists to demonstrate that CLI frameworks need not conflate parsing, business logic, and I/O—through the disciplined application of hexagonal (ports and adapters) architecture, helMo enables command handlers that are testable without process spawning, adapters that are swappable without domain changes, and plugins that extend functionality without core modification.

---

## 2. Tenets (Unless You Know Better Ones)

### Tenet 1: Hexagonal Architecture Discipline

The domain module is pure Rust with no I/O and minimal external dependencies. Application layer orchestrates domain logic. Adapters module contains primary (inbound: CLI parsing) and secondary (outbound: config loading) adapters. Infrastructure contains concrete implementations. Architecture is enforced by structure, not just convention.

### Tenet 2. Domain Purity

Domain logic has no side effects. No file system access. No network calls. No external process spawning. Domain functions are pure: input determines output. Testing requires no setup. Reasoning requires no context.

### Tenet 3. Adapter Swappability

Adapters implement ports defined in domain. Primary adapters can be replaced: CLI parsing, API endpoints, message queue consumers. Secondary adapters can be replaced: file config, database config, environment config. Domain logic unchanged when adapters swap.

### Tenet 4. Plugin Extensibility

Dynamic plugin loading via libloading. Plugins satisfy domain-defined traits at compile time. Runtime loading without host recompilation. Plugin isolation prevents crashes from propagating. Plugin sandboxing for untrusted extensions.

### Tenet 5. Testability First

Domain tests require no I/O setup. Adapter tests mock ports. Integration tests verify wiring. No test requires actual process spawning. Fast tests enable frequent execution. Confidence through coverage.

### Tenet 6. Configuration as Data

Configuration is data, not code. Strongly typed configuration structs. Validation at load time. Layered configuration: defaults, files, environment, arguments. Configuration changes without recompilation.

### Tenet 7. Observable Operations

Structured logging throughout. Metrics collection at boundaries. Health checks for dependencies. Distributed tracing support. Operations visibility without debugging sessions.

---

## 3. Scope & Boundaries

### In Scope

**Core Framework:**
- Hexagonal architecture scaffolding
- Port and trait definitions
- Domain module structure
- Application service orchestration
- Adapter pattern implementations

**CLI Infrastructure:**
- Command-line parsing (clap integration)
- Subcommand routing
- Argument validation
- Help generation
- Shell completion generation

**Plugin System:**
- Dynamic loading infrastructure
- Plugin trait definitions
- Plugin lifecycle management
- Plugin isolation mechanisms
- Plugin configuration

**Configuration Management:**
- Layered configuration loading
- Configuration validation
- Environment variable integration
- Secret management integration
- Configuration hot-reload

**Testing Support:**
- Domain testing utilities
- Adapter mocking
- Integration test harness
- Test fixtures and factories
- Performance benchmarking

**Observability:**
- Structured logging integration
- Metrics collection
- Health check framework
- Tracing integration
- Error reporting

### Out of Scope

- Specific CLI applications (built on helMo)
- Business logic (belongs in domain)
- Concrete storage implementations (adapter concern)
- Network protocols (adapter concern)
- UI frameworks (different domain)

### Boundaries

- Framework provides structure; applications provide content
- Domain is pure; adapters handle impurity
- Core is stable; plugins are extensible
- Framework is agnostic to specific use cases

---

## 4. Target Users & Personas

### Primary Persona: CLI Developer Clara

**Role:** Developer building a new CLI tool
**Goals:** Build testable, maintainable CLI with clean architecture
**Pain Points:** Untestable handlers, tangled dependencies, unclear boundaries
**Needs:** Framework structure, testing utilities, clear patterns
**Tech Comfort:** High, values architecture

### Secondary Persona: Plugin Developer Paulo

**Role:** Developer extending existing helMo-based CLI
**Goals:** Add functionality without modifying core
**Pain Points:** Recompilation requirements, fragile extension points
**Needs:** Plugin API, documentation, examples
**Tech Comfort:** High, extension focus

### Tertiary Persona: Testing Engineer Theo

**Role:** QA engineer testing CLI applications
**Goals:** Comprehensive testing without process spawning
**Pain Points:** Slow tests, flaky integration tests, poor coverage
**Needs:** Testable architecture, mocking utilities, fast feedback
**Tech Comfort:** High, testing specialist

### Quaternary Persona: DevOps Engineer Diana

**Role:** Operations engineer deploying and monitoring CLI tools
**Goals:** Observable, configurable, reliable CLI tools
**Pain Points:** Poor observability, configuration sprawl, debugging difficulty
**Needs:** Logging, metrics, health checks, configuration management
**Tech Comfort:** High, operations focus

### Quinary Persona: Architect Alex

**Role:** Technical lead evaluating CLI frameworks
**Goals:** Understand architectural patterns, assess maintainability
**Pain Points:** Frameworks that compromise on architecture, unclear patterns
**Needs:** Clear architectural documentation, pattern examples, ADRs
**Tech Comfort:** Very high, architecture focused

---

## 5. Success Criteria (Measurable)

### Architecture Metrics

- **Domain Purity:** 100% of domain code has no I/O side effects
- **Test Coverage:** 90%+ code coverage without integration tests
- **Adapter Isolation:** All adapters implement domain-defined ports
- **Plugin Compatibility:** 100% of plugins implement required traits

### Performance Metrics

- **Startup Time:** <100ms for typical CLI with plugins
- **Plugin Load:** <50ms per plugin load
- **Command Routing:** <10ms for command dispatch
- **Memory Overhead:** <10MB base overhead

### Quality Metrics

- **Compilation Success:** 100% of examples compile without warnings
- **Test Pass Rate:** 100% of tests pass on supported platforms
- **Documentation:** 100% of public APIs documented
- **Example Coverage:** Example for each major feature

### Adoption Metrics

- **Project Usage:** 5+ Phenotype projects using helMo
- **Plugin Count:** 10+ community plugins available
- **Developer Satisfaction:** 4.0/5+ satisfaction rating
- **Architecture Adoption:** Hexagonal patterns adopted by users

---

## 6. Governance Model

### Component Organization

```
helMo/
├── domain/              # Pure domain logic
│   ├── models/          # Domain entities
│   ├── ports/           # Port trait definitions
│   ├── services/        # Domain services
│   └── errors/          # Domain errors
├── application/         # Application services
│   ├── commands/        # Command handlers
│   ├── queries/         # Query handlers
│   └── orchestration/   # Workflow orchestration
├── adapters/            # Adapters
│   ├── primary/         # Inbound adapters
│   │   ├── cli/         # CLI parsing
│   │   └── api/         # API endpoints
│   └── secondary/       # Outbound adapters
│       ├── config/      # Configuration loading
│       ├── storage/     # Persistence
│       └── external/    # External services
├── infrastructure/        # Infrastructure concerns
│   ├── logging/         # Logging implementation
│   ├── metrics/         # Metrics collection
│   └── tracing/         # Distributed tracing
├── plugins/             # Plugin system
│   ├── loader/          # Dynamic loading
│   ├── traits/          # Plugin trait definitions
│   └── sandbox/         # Plugin isolation
└── testing/             # Testing utilities
```

### Development Process

**New Domain Features:**
- Port definition review
- Purity verification
- Test coverage requirements
- Documentation requirements

**New Adapters:**
- Port compliance verification
- Test implementation
- Configuration documentation
- Migration guide if replacing existing

**Plugin API Changes:**
- Backward compatibility assessment
- Migration guide
- Version bump policy
- Community notification

---

## 7. Charter Compliance Checklist

### For New Domain Features

- [ ] No I/O in domain code
- [ ] Port definitions updated if needed
- [ ] Tests require no external setup
- [ ] Documentation includes examples
- [ ] Error handling follows domain patterns

### For New Adapters

- [ ] Implements domain port
- [ ] Tests with mocked dependencies
- [ ] Configuration documented
- [ ] Performance benchmarked
- [ ] Migration guide if applicable

### For Plugin Development

- [ ] Implements required traits
- [ ] Error handling doesn't crash host
- [ ] Documentation complete
- [ ] Example provided
- [ ] Version compatibility noted

---

## 8. Decision Authority Levels

### Level 1: Domain Maintainer Authority

**Scope:** Domain bug fixes, pure function additions, documentation
**Process:** Maintainer approval
**Examples:** New domain service, error variant addition

### Level 2: Adapter Authority

**Scope:** New adapters, adapter improvements, configuration changes
**Process:** Technical review, port compliance check
**Examples:** New storage adapter, config loading improvement

### Level 3: Architecture Authority

**Scope:** Port definition changes, plugin API changes, core framework changes
**Process:** Written ADR, backward compatibility assessment, steering approval
**Examples:** New port trait, plugin API v2, architecture pattern change

### Level 4: Strategic Authority

**Scope:** Framework direction, major version changes, ecosystem integration
**Process:** Executive decision with technical input
**Examples:** Major version release, significant ecosystem changes

---

## 9. Security & Compliance Considerations

### Plugin Security

- Plugin code review for official plugins
- Sandboxing for community plugins
- Capability-based restrictions
- No filesystem access without permission
- Network access controlled

### Configuration Security

- Secrets handled through pheno-credentials
- No secrets in configuration files
- Audit logging for configuration changes
- Validation prevents injection

### Supply Chain

- Dependency scanning
- Vulnerability monitoring
- Reproducible builds
- Signed releases

---

## 10. Operational Guidelines

### Release Process

- Semantic versioning
- Changelog maintenance
- Migration guides for breaking changes
- Deprecation warnings
- Long-term support policy

### Support

- Issue triage process
- Security response procedure
- Community support channels
- Commercial support options

### Documentation

- Architecture Decision Records
- API documentation
- Tutorial and guides
- Example projects
- Troubleshooting guides

---

## 11. Integration Points

### Phenotype Ecosystem

- **AgilePlus:** Feature tracking
- **pheno-credentials:** Secret management
- **cliproxy:** CLI proxy integration
- **phenodocs:** Documentation platform
- **helMo.corrupted:** Resilience testing

### External Integrations

- **clap:** CLI parsing
- **libloading:** Dynamic loading
- **tracing:** Observability
- **prometheus:** Metrics
- **config:** Configuration loading

---

*This charter governs helMo, the hexagonal CLI framework. Architecture discipline enables maintainable software.*

*Last Updated: April 2026*
*Next Review: July 2026*

---

## 12. Development Workflows

### Local Development Setup

1. Clone the repository
2. Install required dependencies per project documentation
3. Run initial setup scripts if available
4. Verify setup by running tests
5. Configure IDE/editor with project settings
6. Set up pre-commit hooks if applicable

### Contribution Process

1. Create feature branch from main
2. Implement changes with tests
3. Ensure all quality checks pass
4. Update documentation for API changes
5. Create pull request with description
6. Address review feedback
7. Merge after approval and CI pass

### Testing Requirements

- Unit tests for all new functionality
- Integration tests for feature workflows
- Performance benchmarks for critical paths
- End-to-end tests for user scenarios
- Regression tests for bug fixes

### Release Management

1. Update version according to semver
2. Update CHANGELOG with all changes
3. Run full test suite
4. Create release tag
5. Build and publish artifacts
6. Update documentation references

---

## 13. Quality Standards

### Code Quality

- Follow project style guidelines
- Maintain test coverage thresholds
- No linting errors
- Static analysis passes
- Security scan clean

### Documentation Quality

- All public APIs documented
- README accurate and current
- Architecture decisions in ADRs
- Examples for common use cases
- Troubleshooting guides maintained

### Performance Standards

- Benchmarks meet targets
- No performance regressions
- Resource usage optimized
- Scalability tested

---

*Last Updated: April 2026*
*Next Review: July 2026*
