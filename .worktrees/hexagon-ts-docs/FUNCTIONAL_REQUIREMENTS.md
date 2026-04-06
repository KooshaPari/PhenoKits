# Functional Requirements — hexagon-ts

## FR-STRUCT-001
The repository SHALL contain `package.json`, `tsconfig.json`, and a `src/` directory.

## FR-STRUCT-002
Source SHALL be organised as: `src/domain/`, `src/application/`, `src/ports/`, `src/adapters/`.

## FR-STRUCT-003
`src/domain/` SHALL NOT import from `application/`, `ports/`, or `adapters/`.

## FR-STRUCT-004
`src/application/` SHALL import only from `domain/` and `ports/`.

## FR-STRUCT-005
`src/adapters/` SHALL implement interfaces defined in `ports/`.

## FR-DOMAIN-001
Domain entities SHALL be TypeScript `type`, `interface`, or `class` with zero external package imports.

## FR-DOMAIN-002
Domain errors SHALL be typed: either `Result<T, DomainError>` discriminated union or typed `Error` subclasses.

## FR-PORTS-001
Inbound port interfaces SHALL be defined in `src/ports/inbound/` with `I` prefix or `Port` suffix naming.

## FR-PORTS-002
Outbound port interfaces SHALL be defined in `src/ports/outbound/`.

## FR-PORTS-003
All port methods performing I/O SHALL return `Promise<T>`.

## FR-APP-001
Use case classes SHALL accept port interface instances via `constructor(private readonly port: IPort)`.

## FR-APP-002
Use cases SHALL return typed `Promise<Result<T, E>>` — never `Promise<any>`.

## FR-ADAPTER-001
At least one in-memory outbound adapter SHALL implement the outbound port interface.

## FR-ADAPTER-002
At least one inbound adapter stub SHALL exist in `src/adapters/inbound/`.

## FR-TEST-001
`pnpm test` (vitest) SHALL exit 0.

## FR-TEST-002
Each use case SHALL have a unit test with a mock port implementation.

## FR-LINT-001
`pnpm lint` (oxlint or eslint) SHALL exit 0 with zero errors.

## FR-TYPE-001
`pnpm typecheck` (tsc --noEmit) SHALL exit 0 with zero errors.

## FR-TYPE-002
`tsconfig.json` SHALL have `"strict": true`.
