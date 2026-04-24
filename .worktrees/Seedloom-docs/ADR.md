# ADR: seed — Database Seeding Framework

## ADR-001: Factory + Fixture Dual Pattern

**Status**: Accepted

**Context**: Test data creation approaches: (a) factory-only (dynamic, random), (b) fixture-only (static, file-based), (c) both.

**Decision**: Support both factories and fixtures as complementary patterns.

**Rationale**: Factories excel for unit/integration tests needing isolation and randomness. Fixtures excel for integration tests needing stable reference data. Neither alone covers all use cases.

**Consequences**: Library has more surface area. Documented with clear guidance on when to use each pattern.

---

## ADR-002: Adapter Pattern for Database Backends

**Status**: Accepted

**Context**: seed must work with multiple ORMs (Prisma, Drizzle) and raw drivers. Tight coupling to one ORM limits adoption.

**Decision**: `SeedAdapter` interface with official adapters for Prisma, Drizzle, and raw SQL. Users can implement `SeedAdapter` for other backends.

**Rationale**: Adapter pattern decouples core seed logic from database specifics. Same factory/fixture code works across backends.

**Consequences**: Official adapters must be maintained separately. New ORM versions may break adapters.

---

## ADR-003: @faker-js/faker over Custom Data Generators

**Status**: Accepted

**Context**: Factory field values need realistic data. Build custom generators or use faker.

**Decision**: Integrate `@faker-js/faker` as a peer dependency.

**Rationale**: faker is the de-facto standard for fake data generation with 100+ data categories. Building equivalent generators is high effort with no benefit.

**Consequences**: `@faker-js/faker` version must be managed as a peer dep. API changes in faker require adapter updates.
