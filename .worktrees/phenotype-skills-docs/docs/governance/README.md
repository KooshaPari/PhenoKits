# Governance Documentation Index

**Version:** 1.0.0
**Updated:** 2026-03-25

---

## Overview

This directory contains architectural governance, methodology references, and process documentation for the Phenotype ecosystem.

---

## Core Governance Documents

| Document | Description |
|---------|-------------|
| [agileplus-mandate.md](./agileplus-mandate.md) | AgilePlus workflow mandate |
| [release-branch-governance.md](./release-branch-governance.md) | Release branch strategy |

---

## xDD Methodologies

Comprehensive reference for all development methodologies, patterns, and best practices.

### Primary Reference

| Document | Description | Size |
|---------|-------------|------|
| [xdd-methodologies-encyclopedia.md](./xdd-methodologies-encyclopedia.md) | Complete encyclopedia of 100+ methodologies | 32KB |
| [xdd-quick-reference.md](./xdd-quick-reference.md) | Printable quick reference card | 29KB |

### Decision Guides

| Document | Description |
|---------|-------------|
| [architecture-decision-tree.md](./architecture-decision-tree.md) | Interactive pattern selection decision tree |

---

## Quick Start Guide

### For New Projects

1. **Assess Complexity**: Use [architecture-decision-tree.md](./architecture-decision-tree.md)
2. **Select Patterns**: Follow the decision tree for your use case
3. **Reference xDD**: Consult [xdd-methodologies-encyclopedia.md](./xdd-methodologies-encyclopedia.md) for specific methodologies

### Pattern Selection Flow

```
Simple CRUD → Layered Architecture
         ↓
Medium Domain → Clean Architecture
         ↓
Complex Domain → Hexagonal + DDD
         ↓
Distributed → Microservices + Saga + EDA
```

---

## xDD Methodologies Summary

### Development Methodologies (xDD)

| Category | Methodologies |
|----------|---------------|
| **TDD Family** | TDD, BDD, ATDD, SDD, FDD, CDD, MDD, RDD |
| **Domain** | DDD, ADD, IDD |
| **Specification** | SpecDD, RDD (Readme-Driven) |
| **AI-Driven** | AI-DD, PDD, StoryDD, TraceDD |

### Design Principles

| Category | Principles |
|----------|------------|
| **SOLID** | S, O, D, L, I, P (6 of 7) |
| **Core** | DRY, KISS, YAGNI |
| **Architecture** | GRASP, SoC, CoC, LoD, PoLA |

### Architecture Patterns

| Category | Patterns |
|----------|----------|
| **Core** | Clean, Hexagonal, Onion, Layered |
| **Distributed** | Microservices, EDA, Event Sourcing |
| **Data** | CQRS, Saga, Outbox |

### Quality Practices

| Category | Practices |
|----------|----------|
| **Testing** | TDD, BDD, Property-Based, Mutation, Contract |
| **Process** | CI/CD, DevOps, GitOps, Shift-Left |

---

## File Structure Conventions

### Recommended Top-Level Structure

```
project/
├── src/                    # Source code (language-specific)
├── lib/                    # Libraries
├── bin/                    # Executables
├── cmd/                    # Command-line apps
├── internal/               # Private app code
├── pkg/                    # Public packages
├── api/                    # API definitions
├── configs/                # Configuration
├── scripts/                # Build/deployment scripts
├── tests/                  # Integration tests
├── docs/                   # Documentation
├── examples/               # Examples
└── tooling/                # Build tools
```

### Hexagonal/Clean Architecture Layout

```
src/
├── domain/                 # Core business logic
│   ├── entities/
│   ├── value_objects/
│   ├── services/
│   ├── events/
│   └── ports/
├── application/           # Use cases
│   ├── commands/
│   ├── queries/
│   └── handlers/
├── infrastructure/        # External adapters
│   ├── persistence/
│   ├── messaging/
│   └── external/
└── interface/             # Inbound adapters
    ├── controllers/
    └── gateways/
```

---

## Anti-Patterns to Avoid

| Anti-Pattern | Description | Solution |
|--------------|-------------|----------|
| God Object | Single class doing too much | SRP, Split |
| Shotgun Surgery | One change requires many | Refactor to Cohesion |
| Spaghetti Code | Tangled dependencies | Layered Architecture |
| Premature Optimization | Optimize before profiling | YAGNI, Profile First |
| Over-Engineering | Unnecessary complexity | KISS, YAGNI |

---

## Links to External Resources

- [Martin Fowler's Patterns](https://martinfowler.com/patterns/)
- [Domain-Driven Design](https://domainlanguage.com/ddd/)
- [The Twelve-Factor App](https://12factor.net/)
- [Architecture Decision Records](https://adr.github.io/)
- [Clean Architecture Blog](https://blog.cleancoder.com/)

---

## Contributing to This Documentation

1. Follow UTF-8 encoding standards
2. Use consistent heading hierarchy
3. Include code examples where applicable
4. Add to the index when creating new documents
