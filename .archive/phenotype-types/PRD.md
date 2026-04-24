# Product Requirements Document (PRD) - phenotype-types

## 1. Executive Summary

**phenotype-types** is the shared type definitions library for the Phenotype ecosystem. It provides common type definitions, schemas, and type guards used across all Phenotype projects.

**Vision**: To be the canonical source of truth for all shared types in the Phenotype ecosystem.

**Mission**: Ensure type consistency and compatibility across all Phenotype services and clients.

**Current Status**: Planning phase.

---

## 2. Functional Requirements

### FR-TYPE-001: Core Types
**Priority**: P0 (Critical)
**Description**: Fundamental type definitions
**Acceptance Criteria**:
- Entity IDs
- Timestamps
- Enumerations
- Result types
- Option/Maybe types

### FR-TYPE-002: Domain Types
**Priority**: P1 (High)
**Description**: Domain-specific types
**Acceptance Criteria**:
- User types
- Resource types
- Event types
- Configuration types
- API types

### FR-SCHEMA-001: JSON Schema
**Priority**: P1 (High)
**Description**: Schema definitions
**Acceptance Criteria**:
- JSON Schema export
- Type generation
- Validation functions
- OpenAPI integration

---

## 4. Release Criteria

### Version 1.0
- [ ] Core type definitions
- [ ] JSON Schema export
- [ ] Type guards
- [ ] Documentation

---

*Document Version*: 1.0  
*Last Updated*: 2026-04-05
