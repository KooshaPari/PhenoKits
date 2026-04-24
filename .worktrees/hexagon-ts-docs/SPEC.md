# hexagon-ts — Specification

Hexagonal Architecture (Ports & Adapters) template for TypeScript/Node.js. Clean, SOLID, DDD-ready scaffold.

## Architecture

```
┌───────────────────────────────────────────────┐
│            Primary Adapters (Driving)          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ Express  │  │   CLI    │  │  gRPC    │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
└───────┼─────────────┼─────────────┼──────────┘
        │             │             │
        ▼             ▼             ▼
┌───────────────────────────────────────────────┐
│         Application (Use Cases)               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ Inbound  │  │ Outbound │  │  Use     │   │
│  │ Interfaces│ │ Interfaces│ │  Cases   │   │
│  └──────────┘  └──────────┘  └──────────┘   │
└───────────────────────────────────────────────┘
        │                        │
        ▼                        ▼
┌──────────────────┐  ┌────────────────────────┐
│   Domain Layer   │  │ Secondary Adapters     │
│  Pure TS         │  │  ┌────┐ ┌────┐ ┌────┐ │
│  0 external deps │  │  │ DB │ │ MQ │ │HTTP│ │
│  Entities, VOs,  │  │  └────┘ └────┘ └────┘ │
│  Events, Rules   │  └────────────────────────┘
└──────────────────┘
```

## Components

| Layer | Directory | Responsibility |
|-------|-----------|---------------|
| Domain | `src/domain/` | Entities, value objects, domain events, business rules |
| Application | `src/application/` | Use cases, port interfaces (inbound/outbound) |
| Primary Adapters | `src/adapters/primary/` | HTTP routes, CLI commands |
| Secondary Adapters | `src/adapters/secondary/` | Repository impls, external clients |
| ADR | `adr/` | Architecture decision records |

## Data Models

```typescript
interface EntityId {
  readonly value: string;
}

class Uuid implements EntityId {
  constructor(public readonly value: string) {}
  static create(): Uuid {
    return new Uuid(crypto.randomUUID());
  }
}

interface Entity<T extends EntityId> {
  readonly id: T;
}

interface AggregateRoot<T extends EntityId> extends Entity<T> {
  collectEvents(): DomainEvent[];
}

interface DomainEvent {
  readonly eventName: string;
  readonly occurredAt: Date;
  readonly payload: Record<string, unknown>;
}

class DomainError extends Error {
  constructor(public readonly code: string, message: string) {
    super(message);
  }
}
```

## API Design

```typescript
// Domain
interface Order extends AggregateRoot<OrderId> {
  readonly customerId: string;
  readonly status: 'pending' | 'confirmed';
  confirm(): void;
}

// Outbound port
interface OrderRepository {
  save(order: Order): Promise<void>;
  findById(id: OrderId): Promise<Order | null>;
}

// Inbound port
interface PlaceOrderUseCase {
  execute(cmd: PlaceOrderCommand): Promise<OrderId>;
}

// Use case
class PlaceOrder implements PlaceOrderUseCase {
  constructor(private repo: OrderRepository) {}

  async execute(cmd: PlaceOrderCommand): Promise<OrderId> {
    const order = createOrder(cmd.customerId, cmd.items);
    await this.repo.save(order);
    return order.id;
  }
}
```

## Directory Layout

```
hexagon-ts/
├── src/
│   ├── domain/
│   │   ├── index.ts
│   │   ├── entity.ts
│   │   ├── value-object.ts
│   │   ├── aggregate.ts
│   │   └── event.ts
│   ├── application/
│   │   ├── index.ts
│   │   ├── ports/
│   │   │   ├── inbound.ts
│   │   │   └── outbound.ts
│   │   └── use-cases/
│   │       └── place-order.ts
│   ├── adapters/
│   │   ├── primary/
│   │   │   ├── express/
│   │   │   └── cli/
│   │   └── secondary/
│   │       ├── postgres.ts
│   │       └── http-client.ts
│   └── index.ts
├── adr/
├── package.json
├── tsconfig.json
└── tests/
```

## Performance Targets

| Metric | Target |
|--------|--------|
| TypeScript | 5.0+ strict mode |
| Package manager | pnpm |
| Build time (tsc) | < 5s |
| Test suite (vitest) | < 10s |
| ESLint | 0 errors |
| Test coverage | > 90% |

## Quality Gates

- `pnpm test` — all tests pass
- `pnpm build` — clean tsc compilation
- `pnpm lint` — 0 errors
- tsconfig strict mode always enabled
- Domain module imports zero external packages
- Ports defined as TypeScript interfaces
- Constructor injection at composition root
