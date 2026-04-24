# Architecture Decision Record Template

**Based on nanovms ADR-001 standard**

---

## ADR-[NNN]: [Title]

**Status**: [Proposed | Accepted | Deprecated | Superseded by ADR-XXX]

**Date**: [YYYY-MM-DD]

**Context**: [The problem statement. What is the issue we're seeing that is motivating this decision?]

---

## Decision Drivers

| Driver | Priority | Notes |
|--------|----------|-------|
| [Driver 1] | [High/Med/Low] | [Why it matters] |
| [Driver 2] | [High/Med/Low] | [Why it matters] |

---

## Options Considered

### Option 1: [Name]

**Description**: [What this option is]

**Pros**:
- [Pro 1]
- [Pro 2]

**Cons**:
- [Con 1]
- [Con 2]

**Performance Data**:
| Metric | Value | Source |
|--------|-------|--------|
| [Metric] | [Value] | [Citation/link] |

### Option 2: [Name]

[Same structure as Option 1]

### Option 3: [Name]

[Same structure as Option 1]

---

## Decision

**Chosen Option**: [Name]

**Rationale**: [Why this option was selected over others]

**Evidence**: [Benchmark results, case studies, references that informed this decision]

---

## Performance Benchmarks

```bash
# Reproducible benchmark command
[benchmark command with exact flags]
```

**Results**:

| Benchmark | Value | Comparison to Alternatives |
|-----------|-------|---------------------------|
| [Bench 1] | [Result] | [% better/worse than alternatives] |

---

## Implementation Plan

- [ ] Phase 1: [Description] - Target: [Date]
- [ ] Phase 2: [Description] - Target: [Date]
- [ ] Phase 3: [Description] - Target: [Date]

---

## Consequences

### Positive

- [Consequence 1]
- [Consequence 2]

### Negative

- [Consequence 1]
- [Consequence 2]

### Neutral

- [Consequence 1]

---

## References

- [Reference 1](URL) - [Brief description]
- [Reference 2](URL) - [Brief description]
- [Reference 3](URL) - [Brief description]

---

**Quality Checklist**:
- [ ] Problem statement clearly articulates the issue
- [ ] At least 3 options considered
- [ ] Each option has pros/cons
- [ ] Performance data with source citations
- [ ] Decision rationale explicitly stated
- [ ] Benchmark commands are reproducible
- [ ] Positive AND negative consequences documented
- [ ] References to supporting evidence
