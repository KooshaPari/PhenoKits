# Product Requirements Document — hexagon-ts

## Overview

`hexagon-ts` is the canonical TypeScript hexagonal architecture template for the Phenotype ecosystem. It provides a production-ready scaffold for TypeScript/Node.js services that enforces ports-and-adapters architecture using TypeScript interfaces, clean dependency injection, and idiomatic TypeScript patterns.

## Problem Statement

TypeScript services in the Phenotype ecosystem lack a standardised hexagonal starting point. Without a template, each service mixes domain logic with infrastructure and framework concerns, making testing and governance enforcement difficult.

## Goals

- Provide a TypeScript package scaffold with enforced hexagonal layer separation.
- Demonstrate interface-based port definitions with constructor injection (no IoC container required).
- Target TypeScript 5.x with `strict: true` and `noUncheckedIndexedAccess` enabled.
- Include `vitest` for testing and `oxlint` for linting.

## Non-Goals

- Does not implement production business logic.
- Does not prescribe a specific HTTP framework (Hono, Fastify, Express are all supported as adapters).
- Not a frontend template.

## Epics & User Stories

### E1 — Scaffold Structure
- E1.1: `pnpm install && pnpm build` succeeds immediately after cloning.
- E1.2: Source structure: `src/domain/`, `src/application/`, `src/ports/`, `src/adapters/`.
- E1.3: `tsconfig.json` with `strict: true` and path aliases for each layer.

### E2 — Domain Layer
- E2.1: Domain entities are plain TypeScript types or classes with no external imports.
- E2.2: Domain errors are typed discriminated union `Result<T, E>` or typed Error subclasses.
- E2.3: `domain/` has no imports from other layers.

### E3 — Port Interfaces
- E3.1: Inbound ports are TypeScript interfaces in `src/ports/inbound/`.
- E3.2: Outbound ports are TypeScript interfaces in `src/ports/outbound/`.
- E3.3: Async port methods return `Promise<T>`.

### E4 — Application Layer
- E4.1: Use case classes accept port interfaces via constructor.
- E4.2: Application layer imports only from `domain` and `ports`.

### E5 — Adapters
- E5.1: An in-memory outbound adapter implements the outbound port interface.
- E5.2: An inbound adapter stub exists.

### E6 — Testing
- E6.1: `pnpm test` (vitest) passes with zero failures.
- E6.2: `pnpm lint` (oxlint) passes with zero errors.
- E6.3: `pnpm typecheck` (tsc --noEmit) passes with zero errors.

## Acceptance Criteria

- `pnpm build`, `pnpm test`, `pnpm lint`, `pnpm typecheck` all succeed.
- `eslint-plugin-boundaries` or path alias enforcement prevents cross-layer imports.
- `strict: true` in tsconfig, zero type errors.
