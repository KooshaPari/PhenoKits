# Functional Requirements: seed

## FR-SED-001: Factory Definition
FR-SED-001a: `factory(name, definition)` SHALL accept a definition object where each field is a value or a function returning a value.
FR-SED-001b: Fields defined as functions SHALL be called fresh for each created record.
FR-SED-001c: Factories SHALL support a `traits` map for named override sets.

## FR-SED-002: Factory Methods
FR-SED-002a: `factory.create(overrides?)` SHALL insert one record into the database and return the created entity.
FR-SED-002b: `factory.createMany(n, overrides?)` SHALL insert n records and return an array.
FR-SED-002c: `factory.build(overrides?)` SHALL construct the object without database insertion.
FR-SED-002d: `factory.buildMany(n)` SHALL construct n objects without insertion.

## FR-SED-003: Faker Integration
FR-SED-003a: Factories SHALL have access to `faker` from `@faker-js/faker` in field definitions.
FR-SED-003b: Faker locale SHALL be configurable globally and per-factory.
FR-SED-003c: Faker seed SHALL be settable for reproducible test data in CI.

## FR-SED-004: Fixture Loading
FR-SED-004a: Fixtures SHALL be defined as YAML or JSON files with a list of records per entity.
FR-SED-004b: `loadFixture(file, adapter)` SHALL upsert fixture records (insert if not exists, update if exists).
FR-SED-004c: Fixtures SHALL support a `depends_on` key to enforce loading order.

## FR-SED-005: Database Adapters
FR-SED-005a: Adapters SHALL implement the `SeedAdapter` interface: `insert(table, record) -> Promise<T>`, `insertMany(table, records) -> Promise<T[]>`, `upsert(table, record, key) -> Promise<T>`.
FR-SED-005b: The Prisma adapter SHALL use the Prisma client instance passed at initialization.
FR-SED-005c: The raw SQL adapter SHALL accept a `sql` tagged template function from postgres.js or mysql2.

## FR-SED-006: Transaction Safety
FR-SED-006a: `withTransaction(adapter, fn)` SHALL execute `fn` inside a database transaction.
FR-SED-006b: If `fn` throws, the transaction SHALL be rolled back.
FR-SED-006c: Seed operations inside `withTransaction` SHALL be visible only within the transaction until commit.

## FR-SED-007: Seed Profiles
FR-SED-007a: A seed profile SHALL be a configuration file listing factories and fixtures to run in order.
FR-SED-007b: `seed run --profile <name>` SHALL execute the named profile.
FR-SED-007c: Profiles SHALL support environment targeting: `env: test | staging | demo`.
