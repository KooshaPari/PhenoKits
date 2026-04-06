# crates Project Charter

**Document ID:** CHARTER-CRATES-001  
**Version:** 1.0.0  
**Status:** Active  
**Effective Date:** 2026-04-05  
**Last Updated:** 2026-04-05  

---

## Table of Contents

1. [Mission Statement](#1-mission-statement)
2. [Tenets](#2-tenets)
3. [Scope & Boundaries](#3-scope--boundaries)
4. [Target Users](#4-target-users)
5. [Success Criteria](#5-success-criteria)
6. [Governance Model](#6-governance-model)
7. [Charter Compliance Checklist](#7-charter-compliance-checklist)
8. [Decision Authority Levels](#8-decision-authority-levels)
9. [Appendices](#9-appendices)

---

## 1. Mission Statement

### 1.1 Primary Mission

**crates is the shared Rust crate workspace for the Phenotype ecosystem, providing a centralized location for reusable Rust libraries that serve as building blocks across multiple Phenotype projects.**

Our mission is to maximize code reuse and maintain consistency by offering:
- **Shared Libraries**: Common functionality extracted into crates
- **Workspace Management**: Unified build and dependency management
- **Version Consistency**: Aligned versioning across the ecosystem
- **Quality Standards**: Enforced standards for all crates

### 1.2 Vision

To become the crate repository where:
- **Duplication is Eliminated**: Common code lives in one place
- **Dependencies are Managed**: Workspace-level coordination
- **Quality is Uniform**: Same standards for all crates
- **Discovery is Easy**: Find and reuse existing crates

### 1.3 Strategic Objectives

| Objective | Target | Timeline |
|-----------|--------|----------|
| Crate count | 20+ reusable crates | 2026-Q4 |
| Reuse rate | >80% of eligible code | 2026-Q4 |
| Build time | <5 minutes full workspace | 2026-Q3 |
| Documentation | 100% public APIs | 2026-Q3 |

---

## 2. Tenets

### 2.1 DRY (Don't Repeat Yourself)

**Extract common code into shared crates.**

- Identify duplication across projects
- Extract into well-designed crates
- Document and publish for reuse
- Maintain and evolve centrally

### 2.2 Independence

**Crates have no inter-dependencies.**

- Each crate is self-contained
- Consumers pick what they need
- Clear dependency boundaries
- Minimal feature creep

### 2.3 Quality First

**All crates meet high standards.**

- Full test coverage
- Comprehensive documentation
- Examples and tutorials
- Regular security audits

### 2.4 Semantic Versioning

**Clear versioning for compatibility.**

- SemVer compliance
- Changelog maintenance
- Migration guides
- Breaking change notification

### 2.5 Workspace Coordination

**Unified management across crates.**

- Shared dependencies
- Coordinated releases
- Unified CI/CD
- Cross-crate testing

---

## 3. Scope & Boundaries

### 3.1 In Scope

| Domain | Components | Priority |
|--------|------------|----------|
| **Shared Crates** | Reusable library crates | P0 |
| **Workspace Config** | Cargo.toml coordination | P0 |
| **Build System** | Unified build pipeline | P1 |
| **Documentation** | docs.rs integration | P1 |
| **Testing** | Cross-crate test suite | P1 |

### 3.2 Out of Scope

| Capability | Reason | Alternative |
|------------|--------|-------------|
| **Application code** | Not libraries | Use project repos |
| **Binary crates** | Tools go elsewhere | Use tooling/ |
| **External crates** | Third-party | Use crates.io |

### 3.3 Crate Inventory

```
crates/
├── phenotype-event-sourcing/     # Append-only event store
├── phenotype-cache-adapter/      # LRU + DashMap cache
├── phenotype-policy-engine/      # Rule-based policy evaluation
└── phenotype-state-machine/      # Generic FSM with guards
```

---

## 4. Target Users

### 4.1 Primary Users

**Rust Developers in Phenotype Ecosystem**
- Use shared crates instead of duplicating code
- Contribute improvements back to crates
- Request new crates for common needs

### 4.2 Secondary Users

**Crate Maintainers**
- Maintain crate quality and documentation
- Coordinate with dependent projects
- Plan deprecation and migration

---

## 5. Success Criteria

| Metric | Target | Timeline |
|--------|--------|----------|
| Crate count | 20+ | 2026-Q4 |
| Reuse rate | >80% | 2026-Q4 |
| Test coverage | >80% | 2026-Q2 |
| Documentation | 100% | 2026-Q3 |
| Build time | <5 min | 2026-Q3 |

---

## 6. Governance Model

### 6.1 Crate Lifecycle

**New Crates:**
- RFC for justification
- Code review
- Documentation requirement
- Test coverage threshold

**Crate Updates:**
- SemVer compliance
- Changelog updates
- Dependency updates coordinated

---

## 7. Charter Compliance Checklist

| Requirement | Evidence | Status |
|------------|----------|--------|
| Workspace builds | CI passing | ⬜ |
| Documentation | docs.rs | ⬜ |
| Tests | Coverage report | ⬜ |

---

## 8. Decision Authority Levels

**Level 1: Crate Maintainer**
- Bug fixes, documentation updates

**Level 2: Workspace Lead**
- New crates, major updates

**Level 3: Architecture Board**
- Breaking changes, removals

---

## 9. Appendices

### 9.1 Glossary

| Term | Definition |
|------|------------|
| **Crate** | Rust library package |
| **Workspace** | Multi-crate project |
| **SemVer** | Semantic versioning |

### 9.2 Charter Version History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2026-04-05 | crates Team | Initial charter |

---

**END OF CHARTER**
