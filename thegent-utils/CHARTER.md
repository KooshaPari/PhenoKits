# thegent-utils Project Charter

**Document ID:** CHARTER-THEGENTUTILS-001  
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

**thegent-utils is the utility library collection for thegent within the Phenotype ecosystem, providing common utilities, helper functions, and shared implementations that support thegent functionality.**

Our mission is to share code by offering:
- **Common Utilities**: Helper functions
- **Shared Types**: Common type definitions
- **Helper Macros**: Code generation helpers
- **Testing Utilities**: Test helpers

### 1.2 Vision

To be the utility foundation where:
- **Code is Reused**: No duplication
- **APIs are Stable**: Reliable interfaces
- **Testing is Easy**: Good test utilities
- **Development is Fast**: Ready-to-use helpers

### 1.3 Strategic Objectives

| Objective | Target | Timeline |
|-----------|--------|----------|
| Utility count | 100+ functions | 2026-Q4 |
| Test coverage | >90% | 2026-Q2 |
| thegent usage | 100% | 2026-Q2 |
| Performance | Zero overhead | 2026-Q2 |

---

## 2. Tenets

### 2.1 Reusability

**Code used everywhere.**

- Generic design
- Composable
- No dependencies
- Well-tested

### 2.2 Zero Overhead

**Abstractions cost nothing.**

- Inline functions
- Compile-time eval
- No runtime cost
- Optimized

### 2.3 Stability

**APIs don't break.**

- Semantic versioning
- Deprecation notices
- Migration guides
- Clear docs

### 2.4 Testing

**Well-tested utilities.**

- Unit tests
- Property tests
- Benchmarks
- Edge cases

---

## 3. Scope & Boundaries

### 3.1 In Scope

- Helper functions
- Type definitions
- Macros
- Test utilities

### 3.2 Out of Scope

| Capability | Alternative |
|------------|-------------|
| Domain logic | Use specific crates |
| I/O | Use std or tokio |

---

## 4. Target Users

**thegent Developers** - Primary users
**Other Projects** - Reuse where applicable

---

## 5. Success Criteria

| Metric | Target |
|--------|--------|
| Functions | 100+ |
| Coverage | >90% |
| Usage | 100% |
| Overhead | Zero |

---

## 6. Governance Model

Part of thegent project.

- Utility extraction process
- Quality standards
- API review

---

## 7. Charter Compliance Checklist

| Requirement | Status |
|------------|--------|
| Utilities | ⬜ |
| Tests | ⬜ |

---

## 8. Decision Authority Levels

**thegent Core Team** governs all decisions.

---

## 9. Appendices

### 9.1 Charter Version History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2026-04-05 | thegent-utils Team | Initial charter |

---

**END OF CHARTER**
