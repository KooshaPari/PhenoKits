# Hexagonal Architecture for phenotype-skills-clone

This module implements Hexagonal (Ports and Adapters) Architecture following Clean Architecture principles.

## Structure

```
hexagonal/
├── domain/           # Core business logic (no external dependencies)
│   ├── entities/     # Skill, SkillId, etc.
│   ├── value_objects/# Identifier, Email, Url, etc.
│   ├── events/       # DomainEvent base and events
│   └── services/     # Domain services (Specification, Validator)
├── ports/            # Interface definitions
│   ├── inbound/      # Driving ports (primary)
│   │   ├── commands.py   # CreateSkill, UpdateSkill, etc.
│   │   └── queries.py     # GetSkill, ListSkills, etc.
│   └── outbound/     # Driven ports (secondary)
│       ├── repository.py  # SkillRepository
│       ├── cache.py       # SkillCache
│       └── event_bus.py   # SkillEventBus
├── application/      # Use cases and orchestration
│   ├── commands/     # Command handlers
│   ├── queries/      # Query handlers
│   └── handlers/     # Combined handlers
└── adapters/         # Infrastructure implementations
    ├── primary/      # Driving adapters (API, CLI, etc.)
    └── secondary/    # Driven adapters (DB, Cache, etc.)
```

## Key Design Principles

| Principle | Application |
|-----------|-------------|
| **SOLID** | Interface segregation, dependency inversion |
| **DRY** | Shared port interfaces |
| **KISS** | Simple, focused modules |
| **DDD** | Entity, ValueObject, Aggregate patterns |
| **CQRS** | Separate Command/Query handling |
| **Hexagonal** | Ports/Adapters separation |
| **Clean** | Domain, Application, Infrastructure layers |

## Usage

```python
from hexagonal.domain.entities import Skill, SkillId
from hexagonal.ports.inbound import CreateSkillCommand
from hexagonal.ports.outbound import Repository

# Create a skill entity
skill = Skill.create(
    name="Code Review",
    category=SkillCategory.DEVELOPMENT,
    description="Perform code review"
)

# Use application layer
from hexagonal.application.handlers import SkillHandler
handler = SkillHandler(repository=repository)
result = handler.handle_create_skill(command)
```

## Principles

### Domain Layer
- Pure business logic with no external dependencies
- Entities represent core business objects
- Value Objects are immutable and represent concepts
- Domain Events capture state changes

### Ports Layer
- Inbound ports define how the outside world interacts with the domain
- Outbound ports define how the domain interacts with infrastructure
- All ports are abstract interfaces

### Application Layer
- Orchestrates domain objects and ports
- Implements use cases (CQRS pattern)
- Contains commands (writes) and queries (reads)

### Adapters Layer
- Implement the outbound ports
- Handle infrastructure concerns (DB, cache, external APIs)
- Are injected into the application layer
