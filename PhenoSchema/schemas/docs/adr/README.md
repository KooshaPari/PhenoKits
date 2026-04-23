# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records (ADRs) for the schemas system.

## What is an ADR?

An Architecture Decision Record (ADR) captures an important architectural decision made along with its context and consequences. ADRs help teams:

- Document why decisions were made
- Understand the trade-offs considered
- Onboard new team members faster
- Avoid revisiting decisions unnecessarily
- Provide evidence for audits and reviews

## Index

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [ADR-001](./ADR-001-schema-system-architecture.md) | Schema System Architecture — Polyglot Schema Management | ACCEPTED | 2026-04-05 |
| [ADR-002](./ADR-002-schema-registry-versioning.md) | Schema Registry and Versioning Strategy | ACCEPTED | 2026-04-05 |
| [ADR-003](./ADR-003-code-generation-validation.md) | Code Generation and Runtime Validation Strategy | ACCEPTED | 2026-04-05 |

## Status Definitions

- **PROPOSED** - Under discussion, not yet accepted
- **ACCEPTED** - Decision accepted, being implemented
- **DEPRECATED** - Decision no longer relevant, superseded
- **SUPERSEDED** - Replaced by a newer ADR (link provided)
- **REJECTED** - Decision was considered but not accepted

## Creating a New ADR

1. Use the next sequential number (ADR-004, ADR-005, etc.)
2. Copy the template below
3. Fill in all sections
4. Submit for review
5. Update this index

## ADR Template

```markdown
# ADR-NNN: Title

**Date:** YYYY-MM-DD

**Status:** PROPOSED

**Author:** Your Name

---

## Context

What is the issue that we're seeing that is motivating this decision or change?

## Decision

What is the change that we're proposing or have agreed to implement?

## Consequences

What becomes easier or more difficult to do because of this change?

### Positive

- Benefit 1
- Benefit 2

### Negative

- Drawback 1
- Drawback 2

## Alternatives Considered

### Alternative 1

**Pros:** ...
**Cons:** ...
**Rejected:** Why?

### Alternative 2

**Pros:** ...
**Cons:** ...
**Rejected:** Why?

## References

- Links to related ADRs
- External documentation
- Research papers
- Meeting notes

---

**Decision Delta:**
- Summary of changes

**Review Date:** YYYY-MM-DD
```

## References

- [ADR GitHub Organization](https://adr.github.io/)
- [Documenting Architecture Decisions](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions) - Michael Nygard
- [Architecture Decision Records](https://cognitect.com/blog/2016/1/28/architecture-decision-records) - Cognitect
