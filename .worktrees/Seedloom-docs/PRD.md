# PRD: seed — Database Seeding Framework for TypeScript

## Overview
`seed` is a TypeScript database seeding framework providing factories, fixtures, and generators. It supports safe, transactional test data creation with Faker integration and works with any SQL or NoSQL database.

## Problem Statement
Test environments need realistic, reproducible data. Hand-crafted SQL fixtures are brittle, hard to maintain, and do not produce realistic data. `seed` provides a structured approach to test data creation with factories (dynamic) and fixtures (static, reproducible).

## Goals
1. Factory pattern for dynamic test data with faker-generated values
2. Fixture pattern for static, version-controlled data sets
3. Transaction-safe seeding (rollback on failure)
4. Relationship-aware seeding (create related records in correct order)
5. Environment-specific seed profiles (test, staging, demo)

## Epics

### E1: Factory System
- E1.1: Factory definition DSL
- E1.2: Faker integration (name, email, address, lorem, etc.)
- E1.3: create() and createMany() methods
- E1.4: Traits (role-specific overrides, e.g. admin user)
- E1.5: After-create hooks for relationship setup

### E2: Fixture System
- E2.1: YAML/JSON fixture files
- E2.2: Fixture loading with upsert semantics
- E2.3: Fixture dependencies (load in correct order)
- E2.4: Fixture namespacing for multi-tenant setups

### E3: Database Adapters
- E3.1: Prisma adapter
- E3.2: Drizzle adapter
- E3.3: Raw SQL adapter (postgres.js, mysql2)
- E3.4: MongoDB adapter

### E4: Transaction Safety
- E4.1: Seed within a transaction with rollback on error
- E4.2: Savepoint support for nested seeds
- E4.3: Seed isolation for parallel test runs

### E5: Seed Profiles
- E5.1: Environment-tagged seed sets (test, staging, demo)
- E5.2: Seed orchestration: run multiple factories/fixtures in order
- E5.3: CLI: `seed run --profile test`
