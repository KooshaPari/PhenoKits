# Product Requirements Document — dbkit

## Overview

`dbkit` is a Rust database toolkit library for the Phenotype ecosystem. It provides ergonomic, async-first abstractions for database access, connection pooling, migrations, and query building. It wraps established Rust database libraries (`sqlx`) to provide a consistent interface across all Phenotype services.

## Problem Statement

Phenotype Rust services each implement database connection pooling, migration running, and query error mapping independently. `dbkit` centralises these concerns so all services share a single, well-tested implementation.

## Goals

- Provide async database access helpers wrapping `sqlx` (PostgreSQL, SQLite support).
- Connection pool management with health checking.
- Migration runner using embedded SQL files.
- Typed query result mapping to Rust structs.
- Consistent error types across all database operations.

## Non-Goals

- Not an ORM — raw SQL is first-class.
- Does not support synchronous (blocking) database access.
- Does not replace `sqlx` — it wraps it.

## Epics & User Stories

### E1 — Connection Management
- E1.1: As a developer, I create a `DbPool` from a connection URL string.
- E1.2: Pool creation validates the connection URL format before attempting connection.
- E1.3: `DbPool` provides a `health_check()` method that returns `Ok(())` if a connection can be acquired.

### E2 — Query Execution
- E2.1: `DbPool::fetch_one<T>`, `fetch_all<T>`, `execute` methods wrap `sqlx` query execution.
- E2.2: Results are mapped to Rust structs implementing `sqlx::FromRow`.
- E2.3: Query errors are wrapped in `DbError` with the original SQL and error cause.

### E3 — Migrations
- E3.1: `Migrator::run(&pool)` runs all pending SQL migration files from an embedded directory.
- E3.2: Migration files are numbered `NNNN_description.sql` and run in numeric order.
- E3.3: Already-applied migrations are skipped (idempotent).

### E4 — Error Handling
- E4.1: `DbError` variants cover: `ConnectionFailed`, `QueryFailed`, `MigrationFailed`, `NotFound`.
- E4.2: All errors implement `std::error::Error` and `thiserror::Error`.

### E5 — Testing
- E5.1: `cargo test` passes with zero failures using SQLite in-memory for unit tests.
- E5.2: Integration tests are gated behind a `#[cfg(feature = "integration")]` feature flag.

## Acceptance Criteria

- `cargo build` and `cargo test` succeed.
- `cargo clippy -- -D warnings` exits 0.
- All public types have doc comments.
