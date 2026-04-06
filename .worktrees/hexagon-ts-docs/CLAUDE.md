# hexagon-ts

Hexagonal Architecture (Ports & Adapters) template for TypeScript/Node.js. Clean, SOLID, DDD-ready scaffold.

## Stack
- Language: TypeScript (Node.js)
- Key deps: pnpm, tsconfig-strict, vitest
- Docs: `docs/`, `adr/`

## Structure
- `src/domain/`: Pure TypeScript domain layer (no framework imports)
- `src/application/`: Use cases and port interfaces
- `src/adapters/`: Primary (HTTP/CLI) and secondary (DB/external) adapters
- `adr/`: Architecture Decision Records

## Key Patterns
- Strict hexagonal layers: domain imports nothing outside the domain
- Ports as TypeScript interfaces; adapters implement via constructor injection
- tsconfig strict mode always enabled
- DDD naming: entities, aggregates, value objects, domain events

## Adding New Functionality
- Domain logic: `src/domain/`
- Use cases: `src/application/`
- Adapters: `src/adapters/primary/` or `src/adapters/secondary/`
- Use `pnpm test` for tests, `pnpm lint` for linting
