# Functional Requirements — dbkit

## FR-POOL-001
The library SHALL expose a `DbPool` struct constructible from a database URL string.

## FR-POOL-002
`DbPool::new(url: &str) -> Result<DbPool, DbError>` SHALL return `Err(DbError::ConnectionFailed)` if the URL is invalid or unreachable.

## FR-POOL-003
`DbPool::health_check() -> Result<(), DbError>` SHALL acquire a connection and release it, returning `Ok(())` on success.

## FR-QUERY-001
`DbPool::fetch_one<T: FromRow>(query, args) -> Result<T, DbError>` SHALL return the first matching row.

## FR-QUERY-002
`DbPool::fetch_all<T: FromRow>(query, args) -> Result<Vec<T>, DbError>` SHALL return all matching rows.

## FR-QUERY-003
`DbPool::execute(query, args) -> Result<u64, DbError>` SHALL return the number of affected rows.

## FR-QUERY-004
When no row is found in `fetch_one`, the error SHALL be `DbError::NotFound`.

## FR-MIG-001
The library SHALL expose a `Migrator` that applies SQL migration files in ascending numeric order.

## FR-MIG-002
Migration files SHALL follow the naming convention `NNNN_description.sql`.

## FR-MIG-003
Migrations SHALL be idempotent — already-applied migrations SHALL be skipped without error.

## FR-ERR-001
`DbError` SHALL have variants: `ConnectionFailed`, `QueryFailed`, `MigrationFailed`, `NotFound`, `InvalidUrl`.

## FR-ERR-002
All `DbError` variants SHALL implement `thiserror::Error`.

## FR-BUILD-001
`cargo build` SHALL succeed with `edition = "2021"` or later.

## FR-TEST-001
`cargo test` SHALL pass using SQLite in-memory for all non-integration tests.
