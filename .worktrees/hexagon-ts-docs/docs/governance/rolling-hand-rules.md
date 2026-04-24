# Rolling Hand Rules

**Version:** 1.0.1
**Last Updated:** 2026-03-25
**Purpose:** Cross-project governance that is **rolling** (evolving), **enforceable** (CI where possible), and **transparent**.

---

## 1. AgilePlus Mandate

All work MUST be tracked through AgilePlus:
- **Reference**: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- **CLI**: `cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>`

### Work Requirements

1. **Check for AgilePlus spec before implementing**
2. **Create spec for new work**: `agileplus specify --title "<feature>" --description "<desc>"`
3. **Update work package status**: `agileplus status <feature-id> --wp <wp-id> --state <state>`
4. **No code without corresponding AgilePlus spec**

### Spec-Plan-Tasks Chain

Every feature MUST have:
- `spec.md` - Overview and requirements
- `plan.md` - Timeline and phases
- `tasks/WP*.md` - Work packages with states

---

## 2. xDD Methodology

Choose the appropriate development methodology based on complexity:

| Complexity | Methodology | When to Use |
|------------|-------------|-------------|
| Low | TDD | Core algorithms, utilities |
| Medium | BDD | User-facing features |
| High | DDD | Complex business domains |
| Critical | SDD | Mission-critical systems |

### xDD Methods Available

- **TDD**: Test-Driven Development - Red/Green/Refactor
- **BDD**: Behavior-Driven Development - Gherkin scenarios
- **SDD**: Specification-Driven Development - Formal specs
- **DDD**: Domain-Driven Design - Bounded contexts
- **ADD**: Attribute-Driven Design - Quality attributes
- **CQRS**: Command Query Responsibility Segregation
- **Event Sourcing**: Events as source of truth

See [xDD Methodologies Encyclopedia](./xdd-methodologies-encyclopedia.md) for 100+ methodologies.

---

## 3. Design Principles

### SOLID Principles

| Principle | Description |
|-----------|-------------|
| **S**ingle Responsibility | One reason to change |
| **O**pen/Closed | Open for extension, closed for modification |
| **L**iskov Substitution | Subtypes substitutable |
| **I**nterface Segregation | Small, focused interfaces |
| **D**ependency Inversion | Depend on abstractions |

### Other Core Principles

| Principle | Description |
|-----------|-------------|
| **DRY** | Don't Repeat Yourself |
| **KISS** | Keep It Simple, Stupid |
| **YAGNI** | You Aren't Gonna Need It |
| **GRASP** | General Responsibility Assignment |
| **PoLA** | Principle of Least Astonishment |
| **LoD** | Law of Demeter |

---

## 4. Architecture Patterns

### Required Patterns

All projects MUST follow:

1. **Hexagonal Architecture** (Ports & Adapters)
   - Domain: Core business logic
   - Application: Use cases
   - Adapters: Primary (REST, CLI) and Secondary (DB, Cache)

2. **Clean Architecture** layers
   - Domain Layer
   - Application Layer
   - Interface Adapters Layer
   - Frameworks & Drivers Layer

### Project Structure

```
project/
├── src/
│   ├── domain/           # Entities, value objects, services, ports
│   ├── application/      # Commands, queries, handlers
│   ├── adapters/        # Primary and secondary adapters
│   └── infrastructure/ # Framework code
├── kitty-specs/         # AgilePlus specs
├── tests/               # Integration tests
└── docs/               # Documentation
```

---

## 5. Code Quality

### Anti-Patterns to Avoid

- **God Object**: Class doing too much
- **Shotgun Surgery**: One change requires many modifications
- **Feature Envy**: Class overly using another class
- **Data Clump**: Variables always together
- **Primitive Obsession**: Overuse of primitives

### Quality Metrics

| Metric | Target |
|--------|--------|
| Cyclomatic Complexity | < 10 per function |
| Test Coverage | > 80% |
| Code Duplication | < 5% |
| Documentation | > 20% comments |

---

## 6. Git Workflow

### Branch Discipline

- **Canonical repo**: Tracks `main` only
- **Feature work**: `repos/worktrees/<project>/<category>/<branch>`
- **Return to main**: For merge/integration checkpoints

### Commit Standards

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `refactor:` Code refactoring
- `test:` Adding tests
- `chore:` Maintenance

---

## 7. Documentation

### Required Documentation

Every project MUST have:
- `AGENTS.md` - Agent rules with AgilePlus mandate
- `CLAUDE.md` - Project instructions with AgilePlus mandate
- `kitty-specs/` - AgilePlus specs for all features
- `worklog.md` - Reference to AgilePlus tracking

### UTF-8 Encoding

All markdown files MUST use UTF-8. Validate with:
```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus validate-encoding --all --fix
```

---

## 8. Enforcement

### CI Quality Gates

1. **Encoding Check**: UTF-8 validation
2. **Format Check**: Code formatting
3. **Lint Check**: Static analysis
4. **Test Check**: Unit tests
5. **Coverage Check**: Test coverage > 80%

### Governance Review

- Code audits per rolling hand rules
- Architecture decision records (ADRs)
- Design reviews for major changes

---

## 9. Delegation

Use child agents for:
- Multi-repo scans
- Code audits
- Documentation generation
- Test generation

Parent agent integrates results.

---

## See Also

- [AgilePlus Mandate](./agileplus-mandate.md)
- [xDD Methodologies Encyclopedia](./xdd-methodologies-encyclopedia.md)
- [Release Branch Governance](./release-branch-governance.md)
- [PHENOTYPE Org-Wide Engineering Standard](./PHENOTYPE_ORG_WIDE_ENGINEERING_STANDARD.md)
