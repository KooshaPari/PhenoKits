# dbkit

Database abstraction library with ORM and query builder support. Provides a unified interface across SQL databases for Phenotype services.

## Stack
- Language: Rust (inferred from kit naming pattern)
- Key deps: Cargo, sqlx or diesel, tokio

## Structure
- `src/`: Database abstraction source
  - ORM entity mappings
  - Query builder API
  - Connection pool management

## Key Patterns
- Trait-based database backend abstraction
- Async-first (tokio); all queries are async
- Type-safe query building — SQL injection prevention at compile time
- Connection pooling built in

## Adding New Functionality
- New database backend: implement the `DatabaseBackend` trait
- New query types: extend the query builder in `src/`
- Run `cargo test` to verify
