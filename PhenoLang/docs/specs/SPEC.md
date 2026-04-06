# PhenoLang Specification

**Version:** 1.0.0  
**Status:** Draft  
**Scope:** Language SDKs, Parser Infrastructure, Domain Modeling  
**Classification:** Architecture Specification  
**Date:** 2026-04-05

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [System Overview](#2-system-overview)
3. [Architecture Principles](#3-architecture-principles)
4. [Module Specifications](#4-module-specifications)
5. [Parser Architecture](#5-parser-architecture)
6. [Domain Model Specifications](#6-domain-model-specifications)
7. [Async & Concurrency](#7-async--concurrency)
8. [TUI Specifications](#8-tui-specifications)
9. [Vector & AI Integration](#9-vector--ai-integration)
10. [Security Specifications](#10-security-specifications)
11. [Error Handling](#11-error-handling)
12. [Testing Strategy](#12-testing-strategy)
13. [Performance Requirements](#13-performance-requirements)
14. [Deployment Architecture](#14-deployment-architecture)
15. [API Specifications](#15-api-specifications)
16. [Appendices](#16-appendices)

---

## 1. Executive Summary

### 1.1 Purpose

PhenoLang is a comprehensive language and domain modeling SDK for the Phenotype ecosystem. It provides:

- **Parser Infrastructure**: Combinator-based expression parsing and grammar-based DSL processing
- **Domain Modeling**: Rich entities, value objects, and aggregates following Domain-Driven Design
- **Async Orchestration**: Structured concurrency for resource-safe deployment workflows
- **Terminal Interfaces**: Rich TUI components using reactive frameworks
- **Vector Operations**: Semantic search and AI-augmented developer experience

### 1.2 Scope

| Component | Files | Responsibility |
|-----------|-------|----------------|
| pheno-core | ~50 | Domain primitives, base classes |
| pheno-domain | ~100 | Entities, VOs, aggregates |
| pheno-parser | ~80 | Parser combinators, grammars |
| pheno-async | ~60 | Structured concurrency |
| pheno-tui | ~120 | Terminal interfaces |
| pheno-vector | ~70 | Vector search, embeddings |
| pheno-events | ~40 | Event sourcing |
| pheno-auth | ~50 | OAuth, credentials |
| pheno-security | ~40 | Encryption, secrets |
| pheno-exceptions | ~30 | Error hierarchy |
| pheno-db | ~40 | Database adapters |
| **Total** | **680+** | Comprehensive SDK |

### 1.3 Key Capabilities

```
┌─────────────────────────────────────────────────────────────────────┐
│                        PHENOLANG CAPABILITIES                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  PARSING                    DOMAIN MODELING        CONCURRENCY      │
│  ├─ Combinators             ├─ Entities           ├─ TaskGroup      │
│  ├─ Grammar-based          ├─ Value Objects      ├─ Timeouts        │
│  ├─ Error recovery          ├─ Aggregates         ├─ Cancellation     │
│  └─ Incremental             ├─ Events             └─ Cleanup         │
│                              └─ Services                              │
│                                                                      │
│  TUI                        VECTOR                  SECURITY          │
│  ├─ Reactive UI             ├─ Embeddings         ├─ OAuth2          │
│  ├─ Component-based         ├─ Semantic Search    ├─ Keychain        │
│  ├─ Keyboard-driven         ├─ Similarity         ├─ Encryption      │
│  └─ Theming                 └─ Hybrid Search      └─ Rotation       │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. System Overview

### 2.1 System Context

```
┌─────────────────────────────────────────────────────────────────────┐
│                         SYSTEM CONTEXT                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐      │
│   │  CLI     │    │   TUI    │    │   API    │    │   IDE    │      │
│   │  Tools   │───▶│   App    │◄───│   Server │◄───│  Plugin  │      │
│   └──────────┘    └────┬─────┘    └──────────┘    └──────────┘      │
│                        │                                             │
│                        ▼                                             │
│   ┌──────────────────────────────────────────────────────────────┐ │
│   │                      PHENOLANG SDK                              │ │
│   │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐       │ │
│   │  │Parser  │ │ Domain │ │ Async  │ │  TUI   │ │ Vector │       │ │
│   │  │ Layer  │ │ Layer  │ │ Layer  │ │ Layer  │ │ Layer  │       │ │
│   │  └───┬────┘ └───┬────┘ └───┬────┘ └───┬────┘ └───┬────┘       │ │
│   │      └──────────┴──────────┴──────────┴──────────┴─────────┘  │ │
│   │                         │                                      │ │
│   │                    ┌──────┴──────┐                               │ │
│   │                    │  pheno-core │                               │ │
│   │                    │  (primitives)│                              │ │
│   │                    └─────────────┘                               │ │
│   └──────────────────────────────────────────────────────────────┘ │
│                        │                                             │
│           ┌────────────┼────────────┐                               │
│           ▼            ▼            ▼                                │
│   ┌──────────┐   ┌──────────┐   ┌──────────┐                       │
│   │ Postgres │   │ External │   │  Cloud   │                       │
│   │ +Vector  │   │   APIs   │   │ Providers│                       │
│   └──────────┘   └──────────┘   └──────────┘                       │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 Component Relationships

```
Layer Dependencies (must not violate):

UI Layer (pheno-tui, pheno-ui)
    │
    ▼
Application Layer (pheno-orchestration, pheno-tools)
    │
    ▼
Domain Layer (pheno-domain, pheno-patterns) ◄──┐
    │                                          │
    ▼                                          │
Infrastructure Layer (pheno-db, pheno-adapters)│
    │                                          │
    ▼                                          │
External Systems                               │
                                               │
    ◄──────────────────────────────────────────┘
           Shared Kernel (pheno-core)
```

### 2.3 Data Flow Architecture

```
User Input Flow:

User ──▶ TUI ──▶ Command ──▶ Application Service ──▶ Domain Objects
                                              │
                                              ▼
                                         ┌─────────┐
                                         │ Events  │
                                         └────┬────┘
                                              │
                    ┌──────────────────────────┼──────────────────────────┐
                    │                          │                          │
                    ▼                          ▼                          ▼
              ┌──────────┐              ┌──────────┐               ┌──────────┐
              │  Vector  │              │  Event   │               │  State   │
              │   Index  │              │   Bus    │               │   Store  │
              └──────────┘              └──────────┘               └──────────┘
```

---

## 3. Architecture Principles

### 3.1 Core Principles

| Principle | Description | Rationale |
|-----------|-------------|-----------|
| **Functional Core, Imperative Shell** | Pure domain logic with side effects isolated | Testability, reasoning |
| **Parser Combinators** | Composable, testable grammar units | Maintainability |
| **Structured Concurrency** | Parent-child task relationships | Resource safety |
| **Reactive UI** | Immediate-mode with automatic updates | Responsiveness |
| **Vector-Native** | Semantic search built-in | AI-ready |
| **Type Safety** | Strong typing throughout | Error prevention |

### 3.2 Design Constraints

```python
# Constraint: All domain operations return new instances (immutable)
class Resource:
    def update_status(self, new_status: Status) -> 'Resource':
        return replace(self, _status=new_status)

# Constraint: All async operations use structured concurrency
async def deploy(resources: List[Resource]) -> None:
    async with asyncio.TaskGroup() as tg:
        for r in resources:
            tg.create_task(r.provision())
    # Guaranteed: all provisioned or exception raised

# Constraint: All errors are domain-specific exceptions
class ResourceError(PhenoLangError):
    code = "RESOURCE_ERROR"

# Constraint: All I/O is through dependency injection
class DeploymentService:
    def __init__(self, repository: ResourceRepository, bus: EventBus):
        self._repository = repository
        self._bus = bus
```

---

## 4. Module Specifications

### 4.1 pheno-core

**Purpose:** Domain primitives and base classes

**Classes:**
```python
# Entity base
class Entity(ABC):
    _id: Any = field(init=False)
    
    @property
    def id(self) -> Any:
        return self._id
    
    def equals(self, other: 'Entity') -> bool:
        if not isinstance(other, Entity):
            return False
        return self.id == other.id

# Value Object base
class ValueObject(ABC):
    def __post_init__(self):
        self._validate()
    
    @abstractmethod
    def _validate(self) -> None:
        pass
    
    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        return asdict(self) == asdict(other)

# Aggregate Root base
class AggregateRoot(Entity):
    _version: int = field(default=0)
    _events: List[DomainEvent] = field(default_factory=list, repr=False)
    
    def apply_event(self, event: DomainEvent) -> None:
        self._events.append(event)
        self._handle(event)
    
    @abstractmethod
    def _handle(self, event: DomainEvent) -> None:
        pass
    
    def commit_events(self) -> List[DomainEvent]:
        events = self._events.copy()
        self._events.clear()
        return events

# Domain Event base
@dataclass(frozen=True)
class DomainEvent:
    event_id: UUID
    aggregate_id: str
    occurred_on: datetime
    event_type: str
    
    def to_dict(self) -> dict:
        return {
            "event_id": str(self.event_id),
            "aggregate_id": self.aggregate_id,
            "occurred_on": self.occurred_on.isoformat(),
            "event_type": self.event_type,
            "payload": self._payload(),
        }
    
    @abstractmethod
    def _payload(self) -> dict:
        pass
```

### 4.2 pheno-domain

**Purpose:** Domain entities, value objects, and aggregates

**Module Structure:**
```
pheno-domain/
├── src/
│   └── pheno_domain/
│       ├── __init__.py
│       ├── base.py              # Domain base classes
│       ├── entities/
│       │   ├── __init__.py
│       │   ├── resource.py      # Resource entity
│       │   ├── deployment.py    # Deployment aggregate
│       │   ├── service.py       # Service entity
│       │   └── project.py       # Project aggregate
│       ├── value_objects/
│       │   ├── __init__.py
│       │   ├── port.py          # PortNumber VO
│       │   ├── region.py        # Region VO
│       │   ├── resource_type.py # ResourceType VO
│       │   └── health.py        # HealthStatus VO
│       ├── events/
│       │   ├── __init__.py
│       │   ├── resource_events.py
│       │   └── deployment_events.py
│       ├── services/
│       │   ├── __init__.py
│       │   └── deployment_orchestrator.py
│       └── repositories/
│           ├── __init__.py
│           └── protocols.py     # Repository protocols
```

**Entity Specification: Resource**

```python
@dataclass
class Resource(Entity):
    """
    Resource entity representing cloud infrastructure.
    
    Invariants:
    - Name must be 1-64 characters, alphanumeric + hyphens
    - Type must be one of allowed ResourceType values
    - Status transitions must follow valid state machine
    - Tags keys limited to 128 chars, values to 256 chars
    """
    
    # Identity (set by factory, not constructor)
    _id: ResourceId = field(init=False)
    
    # Attributes
    _name: str
    _type: ResourceType
    _config: ResourceConfig
    _tags: Dict[str, str] = field(default_factory=dict)
    
    # State
    _status: ResourceStatus = field(default=ResourceStatus.PENDING)
    _health: HealthStatus = field(default=HealthStatus.UNKNOWN)
    _endpoint: Optional[Endpoint] = field(default=None)
    
    # Metadata
    _created_at: datetime = field(default_factory=datetime.utcnow)
    _updated_at: Optional[datetime] = field(default=None)
    
    # Events (not part of equality)
    _events: List[DomainEvent] = field(default_factory=list, repr=False, compare=False)
    
    def __post_init__(self):
        self._validate_name()
        self._validate_tags()
        self._validate_config()
    
    # Validation methods
    def _validate_name(self) -> None:
        if not self._name or len(self._name) > 64:
            raise ValueError(f"Name must be 1-64 chars, got {len(self._name) if self._name else 0}")
        if not re.match(r'^[a-zA-Z0-9-]+$', self._name):
            raise ValueError("Name must be alphanumeric + hyphens only")
    
    def _validate_tags(self) -> None:
        for k, v in self._tags.items():
            if len(k) > 128:
                raise ValueError(f"Tag key too long: {k[:20]}...")
            if len(v) > 256:
                raise ValueError(f"Tag value too long for key {k}")
    
    def _validate_config(self) -> None:
        # Type-specific validation
        validators = {
            ResourceType.DATABASE: self._validate_database_config,
            ResourceType.SERVICE: self._validate_service_config,
            ResourceType.BUCKET: self._validate_bucket_config,
        }
        validator = validators.get(self._type)
        if validator:
            validator()
    
    # Properties
    @property
    def id(self) -> ResourceId:
        return self._id
    
    @property
    def name(self) -> str:
        return self._name
    
    @property
    def status(self) -> ResourceStatus:
        return self._status
    
    @property
    def health(self) -> HealthStatus:
        return self._health
    
    @property
    def endpoint(self) -> Optional[Endpoint]:
        return self._endpoint
    
    # State machine transitions
    def provision(self, context: ProvisionContext) -> 'Resource':
        """
        Initiate provisioning.
        
        Precondition: status == PENDING
        Postcondition: status == PROVISIONING
        Emits: ResourceProvisioning
        """
        if self._status != ResourceStatus.PENDING:
            raise InvalidStateTransition(
                f"Cannot provision from {self._status.value}"
            )
        
        # Validate provision context
        context.validate(self._config)
        
        # Create new instance with updated state
        new_resource = replace(self)
        new_resource._status = ResourceStatus.PROVISIONING
        new_resource._events = [
            *self._events,
            ResourceProvisioning(self._id, datetime.utcnow())
        ]
        return new_resource
    
    def mark_provisioned(self, endpoint: Endpoint) -> 'Resource':
        """
        Mark as successfully provisioned.
        
        Precondition: status == PROVISIONING
        Postcondition: status == RUNNING
        Emits: ResourceProvisioned
        """
        if self._status != ResourceStatus.PROVISIONING:
            raise InvalidStateTransition(
                f"Cannot mark provisioned from {self._status.value}"
            )
        
        new_resource = replace(self)
        new_resource._status = ResourceStatus.RUNNING
        new_resource._endpoint = endpoint
        new_resource._health = HealthStatus.HEALTHY
        new_resource._updated_at = datetime.utcnow()
        new_resource._events = [
            *self._events,
            ResourceProvisioned(self._id, endpoint, datetime.utcnow())
        ]
        return new_resource
    
    def mark_failed(self, error: str) -> 'Resource':
        """
        Mark provisioning as failed.
        
        Precondition: status in (PROVISIONING, STARTING)
        Postcondition: status == FAILED
        Emits: ResourceFailed
        """
        if self._status not in (ResourceStatus.PROVISIONING, ResourceStatus.STARTING):
            raise InvalidStateTransition(
                f"Cannot mark failed from {self._status.value}"
            )
        
        new_resource = replace(self)
        new_resource._status = ResourceStatus.FAILED
        new_resource._health = HealthStatus.UNHEALTHY
        new_resource._updated_at = datetime.utcnow()
        new_resource._events = [
            *self._events,
            ResourceFailed(self._id, error, datetime.utcnow())
        ]
        return new_resource
    
    def stop(self) -> 'Resource':
        """Initiate stopping."""
        if self._status != ResourceStatus.RUNNING:
            raise InvalidStateTransition()
        
        new_resource = replace(self)
        new_resource._status = ResourceStatus.STOPPING
        new_resource._events = [*self._events, ResourceStopping(self._id)]
        return new_resource
    
    def decommission(self) -> 'Resource':
        """Mark for deletion."""
        new_resource = replace(self)
        new_resource._status = ResourceStatus.TERMINATING
        new_resource._events = [*self._events, ResourceDecommissioning(self._id)]
        return new_resource
```

### 4.3 pheno-parser

**Purpose:** Parser combinators and grammar-based parsing

**Public API:**
```python
# Parser combinators
from pheno_parser import (
    # Primitives
    string, regex, success, fail,
    
    # Combinators
    seq, alt, many, some, optional,
    sep_by, between, forward_declaration,
    
    # Text parsers
    whitespace, identifier, number, quoted_string,
    
    # Configuration parsers
    key_value, section, config_file,
)

# Expression language
from pheno_parser.expressions import (
    ExpressionParser,
    OperatorTable,
    build_expression_parser,
)

# Configuration DSL
from pheno_parser.config import (
    ConfigParser,
    ConfigValue,
    ConfigSection,
)

# Error handling
from pheno_parser.errors import (
    ParseError,
    ExpectedError,
    ParseLocation,
)
```

### 4.4 pheno-async

**Purpose:** Structured concurrency primitives

**Public API:**
```python
from pheno_async import (
    # Task groups
    TaskGroup,
    TaskScope,
    
    # Timeouts
    timeout,
    TimeoutPolicy,
    
    # Supervision
    Supervisor,
    Worker,
    RestartStrategy,
    
    # Resource management
    ManagedResource,
    ResourcePool,
    
    # Circuit breakers
    CircuitBreaker,
    CircuitState,
    
    # Retry
    RetryPolicy,
    ExponentialBackoff,
    
    # Execution
    gather,
    race,
    select,
)
```

### 4.5 pheno-tui

**Purpose:** Terminal user interfaces

**Public API:**
```python
from pheno_tui import (
    # Base app
    PhenoApp,
    
    # Components
    ResourceCard,
    StatusBadge,
    DeploymentDashboard,
    LogViewer,
    ResourceTree,
    
    # Screens
    ResourceDetailScreen,
    DeployConfirmationScreen,
    SearchScreen,
    
    # Hooks
    use_deployment,
    use_resources,
    use_logs,
)
```

### 4.6 pheno-vector

**Purpose:** Vector embeddings and semantic search

**Public API:**
```python
from pheno_vector import (
    # Repository
    VectorRepository,
    SearchQuery,
    SearchResult,
    
    # Embedders
    Embedder,
    OpenAIEmbedder,
    LocalEmbedder,
    HybridEmbedder,
    
    # Ingestion
    DocumentIngestor,
    Chunker,
    
    # Search
    hybrid_search,
    semantic_search,
    text_search,
    similar_content,
)
```

---

## 5. Parser Architecture

### 5.1 Parser Combinator Design

```python
# Core parser type
Parser = Callable[[ParseState], ParseResult[T]]

@dataclass
class ParseState:
    text: str
    position: int
    line: int
    column: int
    
    def advance(self, n: int) -> 'ParseState':
        new_text = self.text[n:]
        new_pos = self.position + n
        # Calculate new line/column
        consumed = self.text[:n]
        lines = consumed.count('\n')
        if lines > 0:
            new_line = self.line + lines
            new_col = len(consumed) - consumed.rfind('\n') - 1
        else:
            new_line = self.line
            new_col = self.column + n
        
        return ParseState(new_text, new_pos, new_line, new_col)

@dataclass
class ParseResult(Generic[T]):
    success: bool
    value: Optional[T]
    state: ParseState
    error: Optional[ParseError]

# Primitive parsers
def string(s: str) -> Parser[str]:
    """Match exact string."""
    def parse(state: ParseState) -> ParseResult[str]:
        if state.text.startswith(s):
            return ParseResult(
                success=True,
                value=s,
                state=state.advance(len(s)),
                error=None
            )
        return ParseResult(
            success=False,
            value=None,
            state=state,
            error=ExpectedError(f"Expected '{s}'", state)
        )
    return parse

def regex(pattern: str, flags: int = 0) -> Parser[str]:
    """Match regex pattern."""
    compiled = re.compile(pattern, flags)
    
    def parse(state: ParseState) -> ParseResult[str]:
        match = compiled.match(state.text)
        if match:
            return ParseResult(
                success=True,
                value=match.group(),
                state=state.advance(match.end()),
                error=None
            )
        return ParseResult(
            success=False,
            value=None,
            state=state,
            error=ExpectedError(f"Expected pattern {pattern}", state)
        )
    return parse

# Combinators
def seq(*parsers: Parser) -> Parser[tuple]:
    """Sequential composition."""
    def parse(state: ParseState) -> ParseResult[tuple]:
        values = []
        current_state = state
        
        for parser in parsers:
            result = parser(current_state)
            if not result.success:
                return result
            values.append(result.value)
            current_state = result.state
        
        return ParseResult(True, tuple(values), current_state, None)
    return parse

def alt(*parsers: Parser) -> Parser[T]:
    """Alternative composition (first match wins)."""
    def parse(state: ParseState) -> ParseResult[T]:
        errors = []
        
        for parser in parsers:
            result = parser(state)
            if result.success:
                return result
            errors.append(result.error)
        
        # All alternatives failed
        return ParseResult(
            success=False,
            value=None,
            state=state,
            error=AlternativeError(errors, state)
        )
    return parse

def many(parser: Parser[T]) -> Parser[List[T]]:
    """Zero or more repetitions."""
    def parse(state: ParseState) -> ParseResult[List[T]]:
        values = []
        current_state = state
        
        while True:
            result = parser(current_state)
            if not result.success:
                break
            values.append(result.value)
            current_state = result.state
        
        return ParseResult(True, values, current_state, None)
    return parse

def optional(parser: Parser[T], default: T = None) -> Parser[Optional[T]]:
    """Optional parser."""
    def parse(state: ParseState) -> ParseResult[Optional[T]]:
        result = parser(state)
        if result.success:
            return result
        return ParseResult(True, default, state, None)
    return parse

def between(open_parser: Parser, close_parser: Parser, content: Parser[T]) -> Parser[T]:
    """Parse content between delimiters."""
    return seq(open_parser, content, close_parser).map(lambda r: r[1])
```

---

## 6. Domain Model Specifications

### 6.1 Entity Relationships

```
┌─────────────────────────────────────────────────────────────────────┐
│                     ENTITY RELATIONSHIPS                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────┐          ┌─────────────────┐                   │
│  │   Deployment    │◄─────────│    Project      │                   │
│  │  (Aggregate)    │  1:N     │  (Aggregate)    │                   │
│  │                 │          │                 │                   │
│  │  - resources[]  │          │  - name         │                   │
│  │  - services[]   │          │  - settings     │                   │
│  │  - config       │          │  - deployments[]│                   │
│  └────────┬────────┘          └─────────────────┘                   │
│           │                                                          │
│           │ 1:N                                                      │
│           ▼                                                          │
│  ┌─────────────────┐          ┌─────────────────┐                   │
│  │    Resource     │◄─────────│     Service     │                   │
│  │    (Entity)     │  1:1     │    (Entity)     │                   │
│  │                 │          │                 │                   │
│  │  - type         │          │  - port         │                   │
│  │  - region       │          │  - health_check │                   │
│  │  - config       │          │  - replicas     │                   │
│  └─────────────────┘          └─────────────────┘                   │
│                                                                      │
│  Value Objects:                                                      │
│  - ResourceId, ServiceId, DeploymentId                               │
│  - PortNumber, Region, ResourceType                                   │
│  - HealthStatus, ResourceStatus, DeploymentStatus                     │
│  - Endpoint, ResourceConfig, DeploymentConfig                         │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 6.2 Value Object Specifications

```python
# ResourceId
@dataclass(frozen=True)
class ResourceId(ValueObject):
    """
    Strongly-typed resource identifier.
    
    Format: res-{uuid}
    Example: res-550e8400-e29b-41d4-a716-446655440000
    """
    value: UUID
    
    PREFIX: ClassVar[str] = "res"
    
    def __post_init__(self):
        if isinstance(self.value, str):
            object.__setattr__(self, 'value', UUID(self.value))
    
    @classmethod
    def generate(cls) -> 'ResourceId':
        return cls(uuid.uuid4())
    
    @classmethod
    def parse(cls, s: str) -> 'ResourceId':
        if not s.startswith(f"{cls.PREFIX}-"):
            raise ValueError(f"Invalid ResourceId: {s}")
        return cls(UUID(s[len(cls.PREFIX)+1:]))
    
    def __str__(self) -> str:
        return f"{self.PREFIX}-{self.value}"

# PortNumber
@dataclass(frozen=True)
class PortNumber(ValueObject):
    """
    Validated TCP/UDP port number.
    
    Range: 1-65535
    Privileged: 1-1023 (requires root)
    Ephemeral: 49152-65535
    """
    value: int
    
    def _validate(self) -> None:
        if not isinstance(self.value, int):
            raise TypeError("Port must be integer")
        if not 1 <= self.value <= 65535:
            raise ValueError(f"Port {self.value} out of range 1-65535")
    
    def is_privileged(self) -> bool:
        return self.value < 1024
    
    def is_ephemeral(self) -> bool:
        return self.value >= 49152
    
    def next_available(self) -> 'PortNumber':
        if self.value >= 65535:
            raise ValueError("No ports available")
        return PortNumber(self.value + 1)

# Region
@dataclass(frozen=True)
class Region(ValueObject):
    """
    Cloud provider region.
    
    Format: {provider}-{location}
    Examples: aws-us-east-1, gcp-europe-west1
    """
    provider: str
    location: str
    
    PROVIDERS: ClassVar[set] = {"aws", "gcp", "azure", "local"}
    
    def _validate(self) -> None:
        if self.provider not in self.PROVIDERS:
            raise ValueError(f"Unknown provider: {self.provider}")
        if not self.location:
            raise ValueError("Location required")
        if len(self.location) > 50:
            raise ValueError("Location too long")
    
    @classmethod
    def parse(cls, s: str) -> 'Region':
        parts = s.split('-', 1)
        if len(parts) != 2:
            raise ValueError(f"Invalid region format: {s}")
        return cls(parts[0], parts[1])
    
    def __str__(self) -> str:
        return f"{self.provider}-{self.location}"

# ResourceType (Enum-based Value Object)
class ResourceType(Enum):
    """Types of provisionable resources."""
    
    DATABASE = "database"
    SERVICE = "service"
    BUCKET = "bucket"
    QUEUE = "queue"
    CACHE = "cache"
    FUNCTION = "function"
    VPC = "vpc"
    LOAD_BALANCER = "load_balancer"
    
    @property
    def requires_endpoint(self) -> bool:
        return self in {
            ResourceType.DATABASE,
            ResourceType.SERVICE,
            ResourceType.CACHE,
        }
    
    @property
    def is_compute(self) -> bool:
        return self in {
            ResourceType.SERVICE,
            ResourceType.FUNCTION,
        }
```

---

## 7. Async & Concurrency

### 7.1 Structured Concurrency Primitives

```python
# TaskGroup wrapper with enhanced features
class PhenoTaskGroup:
    """
    Structured task group with cancellation and metrics.
    
    Ensures:
    - All tasks complete before group exits
    - First exception cancels remaining tasks
    - Cleanup runs even on cancellation
    """
    
    def __init__(self, name: str = None, timeout: float = None):
        self.name = name or f"tg-{uuid.uuid4().hex[:8]}"
        self.timeout = timeout
        self._tasks: List[asyncio.Task] = []
        self._completed: List[Any] = []
        self._failed: List[BaseException] = []
        self._metrics = TaskMetrics()
    
    async def __aenter__(self) -> 'PhenoTaskGroup':
        self._group = asyncio.TaskGroup()
        await self._group.__aenter__()
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        start = time.monotonic()
        
        try:
            if self.timeout:
                async with asyncio.timeout(self.timeout):
                    await self._group.__aexit__(exc_type, exc_val, exc_tb)
            else:
                await self._group.__aexit__(exc_type, exc_val, exc_tb)
        finally:
            self._metrics.duration = time.monotonic() - start
    
    def create_task(self, coro, *, name: str = None) -> asyncio.Task:
        """Create task within group."""
        task_name = name or f"{self.name}-task-{len(self._tasks)}"
        task = self._group.create_task(coro, name=task_name)
        self._tasks.append(task)
        return task
```

### 7.2 Circuit Breaker Implementation

```python
class CircuitBreaker:
    """
    Circuit breaker for external service calls.
    
    States:
    - CLOSED: Normal operation, calls pass through
    - OPEN: Failing, calls rejected immediately
    - HALF_OPEN: Testing if service recovered
    
    Transitions:
    - CLOSED -> OPEN: failure_count >= threshold
    - OPEN -> HALF_OPEN: timeout elapsed
    - HALF_OPEN -> CLOSED: success_threshold consecutive successes
    - HALF_OPEN -> OPEN: any failure
    """
    
    def __init__(self,
                 name: str,
                 failure_threshold: int = 5,
                 recovery_timeout: float = 30.0,
                 half_open_max_calls: int = 3,
                 success_threshold: int = 2):
        self.name = name
        self._config = CircuitBreakerConfig(
            failure_threshold=failure_threshold,
            recovery_timeout=recovery_timeout,
            half_open_max_calls=half_open_max_calls,
            success_threshold=success_threshold,
        )
        
        self._state = CircuitState.CLOSED
        self._failures = 0
        self._successes = 0
        self._last_failure_time: Optional[float] = None
        self._half_open_calls = 0
        self._lock = asyncio.Lock()
    
    async def call(self, operation: Callable[[], Awaitable[T]]) -> T:
        """Execute operation with circuit breaker protection."""
        async with self._lock:
            await self._update_state()
            
            if self._state == CircuitState.OPEN:
                raise CircuitBreakerOpen(self.name)
            
            if self._state == CircuitState.HALF_OPEN:
                if self._half_open_calls >= self._config.half_open_max_calls:
                    raise CircuitBreakerOpen(self.name)
                self._half_open_calls += 1
        
        try:
            result = await operation()
            await self._on_success()
            return result
        except Exception as e:
            await self._on_failure()
            raise
    
    async def _update_state(self) -> None:
        """Update state based on timing."""
        if self._state == CircuitState.OPEN:
            if time.time() - self._last_failure_time > self._config.recovery_timeout:
                self._state = CircuitState.HALF_OPEN
                self._half_open_calls = 0
                self._successes = 0
    
    async def _on_success(self) -> None:
        async with self._lock:
            if self._state == CircuitState.HALF_OPEN:
                self._successes += 1
                if self._successes >= self._config.success_threshold:
                    self._state = CircuitState.CLOSED
                    self._failures = 0
            else:
                self._failures = max(0, self._failures - 1)
    
    async def _on_failure(self) -> None:
        async with self._lock:
            self._failures += 1
            self._last_failure_time = time.time()
            
            if self._failures >= self._config.failure_threshold:
                self._state = CircuitState.OPEN
```

---

## 8. TUI Specifications

### 8.1 Component Hierarchy

```
PhenoApp (Root)
├── Header
│   ├── Clock
│   └── Title
├── Main Screen
│   ├── Sidebar (25%)
│   │   ├── ResourceTree
│   │   ├── FilterInput
│   │   └── QuickActions
│   └── MainContent (75%)
│       ├── StatusBar
│       │   ├── StatusLabel
│       │   └── ProgressBar
│       ├── ResourceGrid
│       │   └── ResourceCard[]
│       └── LogPanel
│           └── Log
├── Modal Screens
│   ├── ResourceDetailScreen
│   ├── DeployConfirmationScreen
│   ├── SearchScreen
│   └── HelpScreen
└── Footer
    ├── KeyBindings
    └── Notifications
```

### 8.2 Component API Specification

```python
class ResourceCard(Static):
    """
    Card component displaying resource information.
    
    State:
    - resource: Resource (reactive)
    - selected: bool (reactive)
    
    Events:
    - Click: Select and show details
    - DoubleClick: Quick action (start/stop)
    
    Styling:
    - Border changes on hover
    - Color indicates status
    - Dim when not selected
    """
    
    resource: reactive[Resource] = reactive(None)
    selected: reactive[bool] = reactive(False)
    
    DEFAULT_CSS = """
    ResourceCard {
        width: 100%;
        height: auto;
        padding: 1;
        border: solid $surface-lighten-1;
        margin: 0 0 1 0;
    }
    
    ResourceCard:hover {
        border: solid $primary;
    }
    
    ResourceCard.selected {
        border: double $primary;
        background: $primary-darken-2;
    }
    
    ResourceCard.status-running { border: solid $success; }
    ResourceCard.status-pending { border: solid $warning; }
    ResourceCard.status-failed { border: solid $error; }
    """
    
    def __init__(self, resource: Resource, **kwargs):
        super().__init__(**kwargs)
        self.resource = resource
    
    def compose(self) -> ComposeResult:
        with Container():
            yield Label(self.resource.name, classes="name")
            with Horizontal(classes="meta"):
                yield Label(self.resource.type, classes="type")
                yield Label(self.resource.status.value, classes=f"status {self.resource.status.value}")
    
    def watch_resource(self, resource: Resource) -> None:
        self.remove_class("status-running status-pending status-failed status-stopped")
        self.add_class(f"status-{resource.status.value}")
        self.refresh()
    
    def watch_selected(self, selected: bool) -> None:
        self.toggle_class("selected", selected)
    
    def on_click(self) -> None:
        self.post_message(self.Selected(self.resource))
    
    class Selected(Message):
        def __init__(self, resource: Resource):
            self.resource = resource
            super().__init__()
```

---

## 9. Vector & AI Integration

### 9.1 Search Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     VECTOR SEARCH PIPELINE                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Query                                                               │
│    │                                                                 │
│    ▼                                                                 │
│  ┌─────────────────┐         ┌─────────────────┐                    │
│  │   Embedder      │────────▶│  Embedding      │                    │
│  │  (OpenAI/Local) │         │  [1536d vector] │                    │
│  └─────────────────┘         └────────┬────────┘                    │
│                                       │                              │
│    ┌──────────────────────────────────┘                              │
│    │                                  ┌──────────────────────────┐     │
│    ▼                                  ▼                          ▼     │
│  ┌─────────────────┐         ┌─────────────────┐                    │
│  │   Text Search   │         │   Vector Search │                    │
│  │   (tsvector)    │         │   (pgvector)    │                    │
│  │                 │         │                 │                    │
│  │  to_tsvector()  │         │  HNSW index     │                    │
│  │  ts_rank()      │         │  <=> operator   │                    │
│  └────────┬────────┘         └────────┬────────┘                    │
│           │                              │                           │
│           └──────────────┬───────────────┘                              │
│                          ▼                                           │
│                   ┌─────────────┐                                     │
│                   │   Combine   │                                     │
│                   │   Scores    │                                     │
│                   │             │                                     │
│                   │  weighted   │                                     │
│                   │  average    │                                     │
│                   └──────┬──────┘                                     │
│                          ▼                                           │
│                   ┌─────────────┐                                     │
│                   │   Rank &    │                                     │
│                   │   Return    │                                     │
│                   │   Top-K     │                                     │
│                   └─────────────┘                                     │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 9.2 Embedding Provider Interface

```python
class Embedder(Protocol):
    """Protocol for text embedding providers."""
    
    @property
    def dimension(self) -> int:
        """Return vector dimension."""
        ...
    
    @property
    def max_tokens(self) -> int:
        """Return maximum tokens per input."""
        ...
    
    @property
    def supports_batching(self) -> bool:
        """Return True if batch embedding is supported."""
        ...
    
    async def embed(self, texts: List[str]) -> List[List[float]]:
        """
        Embed texts into vectors.
        
        Args:
            texts: List of texts to embed
            
        Returns:
            List of embedding vectors
            
        Raises:
            EmbeddingError: If embedding fails
            TokenLimitError: If text exceeds max_tokens
        """
        ...
    
    async def embed_single(self, text: str) -> List[float]:
        """Embed single text."""
        results = await self.embed([text])
        return results[0]
```

---

## 10. Security Specifications

### 10.1 OAuth Implementation

```python
@dataclass
class OAuthConfig:
    """OAuth 2.0 configuration."""
    
    client_id: str
    client_secret: SecretStr  # Masked in logs
    authorization_endpoint: str
    token_endpoint: str
    redirect_uri: str
    scopes: List[str]
    pkce_enabled: bool = True

class OAuthFlow:
    """
    OAuth 2.0 flow implementation with PKCE.
    
    Supports:
    - Authorization Code flow
    - PKCE (Proof Key for Code Exchange)
    - Token refresh
    - Secure storage
    """
    
    def __init__(self, config: OAuthConfig, store: SecretStore):
        self._config = config
        self._store = store
        self._http = httpx.AsyncClient()
    
    def get_authorization_url(self, state: str) -> tuple[str, str]:
        """
        Generate authorization URL with PKCE.
        
        Returns:
            Tuple of (authorization_url, code_verifier)
        """
        code_verifier = self._generate_code_verifier()
        code_challenge = self._generate_code_challenge(code_verifier)
        
        params = {
            'client_id': self._config.client_id,
            'redirect_uri': self._config.redirect_uri,
            'response_type': 'code',
            'state': state,
            'scope': ' '.join(self._config.scopes),
            'code_challenge': code_challenge,
            'code_challenge_method': 'S256',
        }
        
        url = f"{self._config.authorization_endpoint}?{urlencode(params)}"
        return url, code_verifier
    
    async def exchange_code(self, code: str, code_verifier: str) -> TokenSet:
        """Exchange authorization code for tokens."""
        data = {
            'grant_type': 'authorization_code',
            'client_id': self._config.client_id,
            'client_secret': self._config.client_secret.get_secret_value(),
            'code': code,
            'redirect_uri': self._config.redirect_uri,
            'code_verifier': code_verifier,
        }
        
        response = await self._http.post(self._config.token_endpoint, data=data)
        response.raise_for_status()
        
        token_data = response.json()
        tokens = TokenSet.from_response(token_data)
        
        # Store securely
        await self._store.set(
            f"oauth:{self._config.client_id}",
            tokens.to_encrypted_json()
        )
        
        return tokens
```

---

## 11. Error Handling

### 11.1 Exception Hierarchy

```
PhenoLangError
├── DomainError
│   ├── ValidationError
│   ├── ResourceError
│   │   ├── ResourceNotFound
│   │   ├── InvalidStateTransition
│   │   ├── ResourceAlreadyExists
│   │   └── ResourceLimitExceeded
│   ├── DeploymentError
│   │   ├── DeploymentNotFound
│   │   ├── DeploymentFailed
│   │   └── DeploymentTimeout
│   └── ConfigurationError
│       ├── ConfigNotFound
│       └── ConfigValidationError
├── InfrastructureError
│   ├── DatabaseError
│   │   ├── ConnectionError
│   │   ├── QueryError
│   │   └── ConcurrencyError
│   ├── NetworkError
│   ├── ProviderError
│   │   ├── ProviderNotFound
│   │   ├── ProviderAPIError
│   │   └── ProviderQuotaExceeded
│   └── StorageError
├── SecurityError
│   ├── AuthenticationError
│   ├── AuthorizationError
│   └── SecretError
└── ParserError
    ├── SyntaxError
    └── GrammarError
```

---

## 12. Testing Strategy

### 12.1 Test Pyramid

```
                    ┌─────────┐
                    │   E2E   │  < 5% - Critical flows
                   ├───────────┤
                   │ Integration│  ~15% - API, DB, external
                  ├─────────────┤
                  │    Unit      │  ~80% - Domain logic, parsers
                 ├───────────────┤
                 │  Property     │  Where applicable
                └─────────────────┘
```

### 12.2 Domain Testing Pattern

```python
class TestResource(unittest.TestCase):
    """Unit tests for Resource entity."""
    
    def test_provision_from_pending_succeeds(self):
        # Given
        resource = ResourceFactory.pending()
        
        # When
        updated = resource.provision(ProvisionContext.valid())
        
        # Then
        assert updated.status == ResourceStatus.PROVISIONING
        assert len(updated.uncommitted_events) == 1
        assert isinstance(updated.uncommitted_events[0], ResourceProvisioning)
    
    def test_provision_from_running_fails(self):
        # Given
        resource = ResourceFactory.running()
        
        # When/Then
        with self.assertRaises(InvalidStateTransition) as ctx:
            resource.provision(ProvisionContext.valid())
        
        assert "RUNNING" in str(ctx.exception)
```

---

## 13. Performance Requirements

| Metric | Target | Critical | Measurement |
|--------|--------|----------|-------------|
| Parser throughput | > 10 MB/s | Yes | Benchmark |
| TUI startup | < 100ms | Yes | Timeit |
| Vector search (K=10) | < 50ms | Yes | Benchmark |
| Async task spawn | < 1ms | No | Benchmark |
| Domain operation | < 10μs | No | Profile |

---

## 14. Deployment Architecture

### 14.1 Package Distribution

```
PyPI Packages:
├── pheno-core        [required]
├── pheno-domain      [depends: pheno-core]
├── pheno-parser      [depends: pheno-core]
├── pheno-async       [depends: pheno-core]
├── pheno-tui         [depends: pheno-core, pheno-async, textual]
├── pheno-vector      [depends: pheno-core, asyncpg, openai]
├── pheno-auth        [depends: pheno-core, pheno-async]
└── pheno-all         [meta-package, all modules]
```

---

## 15. API Specifications

### 15.1 Public API Surface

Each module exports via `__all__`:

```python
# pheno_domain/__init__.py
__all__ = [
    # Entities
    'Resource',
    'Deployment',
    'Service',
    'Project',
    
    # Value Objects
    'ResourceId',
    'PortNumber',
    'Region',
    'ResourceType',
    'HealthStatus',
    
    # Events
    'ResourceProvisioning',
    'ResourceProvisioned',
    'ResourceFailed',
    
    # Services
    'DeploymentOrchestrator',
    
    # Exceptions
    'DomainError',
    'ValidationError',
    'ResourceError',
    'InvalidStateTransition',
]
```

---

## 16. Appendices

### Appendix A: Glossary

| Term | Definition |
|------|------------|
| Aggregate | Consistency boundary for domain operations |
| Circuit Breaker | Pattern for failing fast when dependencies are unhealthy |
| Combinator | Higher-order function for building parsers |
| DDD | Domain-Driven Design |
| Entity | Object with identity that persists over time |
| HNSW | Hierarchical Navigable Small World graph index |
| Parser | Function that transforms text into structured data |
| Structured Concurrency | Pattern ensuring child tasks complete before parent |
| TUI | Terminal User Interface |
| Value Object | Immutable object compared by value |

### Appendix B: References

- [nanovms/ops](https://github.com/nanovms/ops) - Unikernel orchestration
- [dddcommunity.org](https://dddcommunity.org) - Domain-Driven Design patterns
- [parsy.readthedocs.io](https://parsy.readthedocs.io) - Parser combinators
- [textual.textualize.io](https://textual.textualize.io) - TUI framework
- [pgvector](https://github.com/pgvector/pgvector) - Vector similarity search

---

**End of Specification**

*Document Version: 1.0.0*  
*Last Updated: 2026-04-05*

---

## Additional Specification Sections

### 17. Detailed Module Interactions

```
Module Dependency Graph:

pheno-core
├── pheno-domain (depends on pheno-core)
│   ├── pheno-db (depends on pheno-domain)
│   ├── pheno-resources (depends on pheno-domain)
│   └── pheno-ports (depends on pheno-domain)
├── pheno-async (depends on pheno-core)
│   ├── pheno-events (depends on pheno-async)
│   └── pheno-exceptions (depends on pheno-async)
├── pheno-parser (standalone, but uses pheno-core types)
├── pheno-vector (depends on pheno-core, pheno-async)
├── pheno-tui (depends on pheno-core, pheno-async)
└── pheno-auth (depends on pheno-core, pheno-security)

pheno-security (depends on pheno-core)
pheno-credentials (depends on pheno-security)
pheno-logging (depends on pheno-core)
pheno-config (depends on pheno-core, pheno-parser)
pheno-tools (CLI tools, depends on most modules)
```

### 18. Resource Lifecycle State Machine

```
[PENDING] --> [PROVISIONING] --> [RUNNING]
    |               |               |
    |               |               |
    |               v               v
    |          [FAILED] <------ [STOPPING]
    |               ^               |
    |               |               |
    +--------------+               v
                              [STOPPED]
                                  |
                                  v
                            [DELETING] --> [DELETED]
```

State transitions are validated by the aggregate root to ensure consistency.

### 19. Database Schema

```sql
-- Core tables
CREATE TABLE resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(64) NOT NULL,
    type VARCHAR(32) NOT NULL,
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    provider VARCHAR(32),
    region VARCHAR(32),
    config JSONB,
    labels JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE deployments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    config_id UUID REFERENCES configs(id),
    status VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE deployment_resources (
    deployment_id UUID REFERENCES deployments(id),
    resource_id UUID REFERENCES resources(id),
    PRIMARY KEY (deployment_id, resource_id)
);

-- Event store
CREATE TABLE events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(64) NOT NULL,
    event_type VARCHAR(64) NOT NULL,
    payload JSONB NOT NULL,
    occurred_at TIMESTAMPTZ DEFAULT NOW(),
    version INTEGER NOT NULL
);

CREATE INDEX idx_events_aggregate ON events(aggregate_id, version);
```

### 20. API Rate Limiting

```python
class RateLimiter:
    """Token bucket rate limiter."""
    
    def __init__(self, rate: float, burst: int):
        self.rate = rate
        self.burst = burst
        self.tokens = burst
        self.last_update = time.monotonic()
        self.lock = asyncio.Lock()
    
    async def acquire(self) -> None:
        async with self.lock:
            now = time.monotonic()
            elapsed = now - self.last_update
            self.tokens = min(
                self.burst,
                self.tokens + elapsed * self.rate
            )
            self.last_update = now
            
            if self.tokens < 1:
                wait_time = (1 - self.tokens) / self.rate
                await asyncio.sleep(wait_time)
                self.tokens = 0
            else:
                self.tokens -= 1
```

### 21. Monitoring & Observability

```python
from opentelemetry import trace, metrics
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.metrics import MeterProvider

tracer = trace.get_tracer(__name__)
meter = metrics.get_meter(__name__)

# Custom metrics
deployment_counter = meter.create_counter(
    "deployments",
    description="Total deployments",
)

resource_gauge = meter.create_gauge(
    "resources",
    description="Current resources by status",
)

@tracer.start_as_current_span("deploy")
async def deploy(spec: DeploymentSpec) -> Deployment:
    deployment_counter.add(1, {"status": "started"})
    try:
        result = await _do_deploy(spec)
        deployment_counter.add(1, {"status": "success"})
        return result
    except Exception:
        deployment_counter.add(1, {"status": "failed"})
        raise
```

### 22. Security Considerations

#### Authentication
- OAuth 2.0 with PKCE for web flows
- Service account keys for automation
- Token rotation every 24 hours

#### Authorization
- RBAC with resource-level permissions
- Policy evaluation at edge
- Audit logging for all access

#### Data Protection
- Encryption at rest (AES-256)
- TLS 1.3 in transit
- Secret rotation automation

### 23. Migration Strategy

```
Phase 1: Parallel Implementation
- New modules alongside legacy
- Feature flags for gradual rollout
- Data synchronization between systems

Phase 2: Gradual Cutover
- Percentage-based traffic shifting
- Rollback capability maintained
- Monitoring for regression

Phase 3: Legacy Deprecation
- Legacy code marked deprecated
- Migration guide documentation
- Final removal after 6 months
```

### 24. Disaster Recovery

| Scenario | RTO | RPO | Strategy |
|----------|-----|-----|----------|
| Single AZ failure | 5 min | 0 | Multi-AZ deployment |
| Region failure | 30 min | 5 min | Cross-region replication |
| Data corruption | 1 hour | 1 hour | Point-in-time recovery |
| Complete loss | 4 hours | 1 hour | Geo-redundant backup |

### 25. Development Workflow

```
1. Feature Specification
   ├── ADR creation
   ├── API design review
   └── Security review

2. Implementation
   ├── Unit tests (TDD)
   ├── Integration tests
   └── Documentation

3. Code Review
   ├── Automated checks
   ├── Peer review (2+ approvals)
   └── Security scan

4. Testing
   ├── CI pipeline
   ├── Staging deployment
   └── Performance benchmarks

5. Release
   ├── Version bump
   ├── Changelog update
   └── Tagged release
```


---

### 26. Performance Optimization Guidelines

#### 26.1 Python Optimizations

```python
# Use slots for memory efficiency
@dataclass(slots=True)
class Resource:
    name: str
    type: ResourceType
    status: ResourceStatus

# Use lru_cache for expensive computations
@lru_cache(maxsize=1024)
def get_provider_for_region(region: str) -> Provider:
    return _compute_provider(region)

# Use asyncio.gather for parallel operations
async def provision_all(resources: List[Resource]) -> None:
    await asyncio.gather(*[
        r.provision() for r in resources
    ])

# Use orjson for fast JSON serialization
import orjson

def serialize_resource(resource: Resource) -> bytes:
    return orjson.dumps(resource.to_dict())
```

#### 26.2 Database Optimizations

```sql
-- Use partial indexes for common queries
CREATE INDEX idx_running_resources 
ON resources(provider, region) 
WHERE status = 'running';

-- Use covering indexes
CREATE INDEX idx_resources_lookup 
ON resources(id, name, type, status) 
INCLUDE (config, labels);

-- Partition large tables
CREATE TABLE events_2024 PARTITION OF events
FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');
```

### 27. Testing Matrix

| Component | Unit Tests | Integration | E2E | Coverage Target |
|-----------|------------|-------------|-----|-----------------|
| pheno-core | ★★★★★ | ★★★★☆ | ★★☆☆☆ | 95% |
| pheno-domain | ★★★★★ | ★★★★★ | ★★★☆☆ | 90% |
| pheno-async | ★★★★★ | ★★★★★ | ★★★☆☆ | 90% |
| pheno-parser | ★★★★★ | ★★★☆☆ | ★☆☆☆☆ | 95% |
| pheno-tui | ★★★★☆ | ★★★☆☆ | ★★★★☆ | 80% |
| pheno-vector | ★★★★★ | ★★★★☆ | ★★☆☆☆ | 85% |

### 28. Internationalization

```python
from gettext import gettext as _

class I18nMixin:
    def get_message(self, key: str, **kwargs) -> str:
        template = _(key)
        return template.format(**kwargs)

# Usage
error_message = self.get_message(
    "error.resource_not_found",
    resource_id=resource_id
)
```

### 29. Accessibility Guidelines

- Color-blind friendly status indicators (icons + color)
- Keyboard navigation for all TUI features
- Screen reader support via textual accessibility features
- High contrast mode support

### 30. Future Roadmap

| Quarter | Feature | Priority |
|---------|---------|----------|
| Q2 2026 | Vector search GA | P0 |
| Q2 2026 | TUI v1.0 | P0 |
| Q3 2026 | GraphQL API | P1 |
| Q3 2026 | Web dashboard | P1 |
| Q4 2026 | AI assistant integration | P2 |
| Q4 2026 | Plugin marketplace | P2 |

### 31. Compliance Requirements

- SOC 2 Type II certification
- GDPR data handling compliance
- HIPAA (for healthcare deployments)
- FedRAMP (government cloud)

### 32. Support & SLAs

| Tier | Response Time | Resolution Time | Channels |
|------|---------------|-----------------|----------|
| Free | Best effort | N/A | Community |
| Pro | 4 hours | 24 hours | Email, Chat |
| Enterprise | 1 hour | 4 hours | All + Phone |


### 33. Detailed Async Runtime Configuration

The pheno-async module provides fine-grained control over async runtime behavior:

```python
from pheno_async import RuntimeConfig, TaskGroupPolicy

# Configure runtime for specific workload
config = RuntimeConfig(
    worker_threads=8,
    max_blocking_threads=128,
    thread_stack_size=2 * 1024 * 1024,  # 2MB
    task_queue_depth=10000,
)

# Configure task group policies
policy = TaskGroupPolicy(
    cancel_on_first_error=True,
    propagate_cancellation=True,
    graceful_shutdown_timeout=30.0,
)

async with pheno_async.runtime(config) as rt:
    async with rt.task_group(policy) as tg:
        for resource in resources:
            tg.create_task(provision_resource(resource))
```

### 34. Parser Combinator Deep Dive

The pheno-parser module implements full parser combinator functionality:

```python
from pheno_parser import (
    string, regex, seq, alt, many, some,
    optional, between, forward_declaration,
    sep_by, end_by, chainl, chainr
)

# Expression parser with operator precedence
expr = forward_declaration()

number = regex(r'\d+').map(int)
variable = regex(r'[a-zA-Z_][a-zA-Z0-9_]*')

factor = alt(
    between(string('('), string(')'), expr),
    number,
    variable
)

term = chainl(factor, alt(
    string('*').map(lambda: lambda a, b: a * b),
    string('/').map(lambda: lambda a, b: a // b),
))

expr.define(chainl(term, alt(
    string('+').map(lambda: lambda a, b: a + b),
    string('-').map(lambda: lambda a, b: a - b),
)))

# Full expression grammar
grammar = expr << regex(r'\s*').skip() << string('').end()
```

### 35. Vector Index Optimization

Advanced HNSW parameter tuning for different use cases:

| Use Case | M | ef_construction | ef_search | Lists (IVF) |
|----------|---|-----------------|-----------|---------------|
| Small dataset (<100K) | 16 | 64 | 32 | 100 |
| Medium dataset (100K-1M) | 32 | 128 | 64 | 1000 |
| Large dataset (1M-10M) | 64 | 256 | 128 | 5000 |
| Huge dataset (>10M) | 128 | 512 | 256 | 10000 |
| High precision | 64 | 512 | 512 | - |
| High throughput | 16 | 64 | 16 | - |

```python
# Adaptive index selection
class VectorIndexManager:
    def __init__(self):
        self.index_strategies = {
            'hnsw': self._create_hnsw_index,
            'ivf': self._create_ivf_index,
            'hybrid': self._create_hybrid_index,
        }
    
    def get_optimal_index(self, dataset_size: int, query_pattern: str) -> str:
        if dataset_size < 100_000:
            return 'hnsw'
        elif query_pattern == 'high_throughput':
            return 'ivf'
        else:
            return 'hybrid'
```

### 36. TUI Component Library

The pheno-ui component library provides pre-built widgets:

```python
from pheno_ui import (
    ResourceCard,
    StatusBadge,
    LogViewer,
    DeploymentTimeline,
    MetricsSparkline,
    ConfirmationDialog,
    SearchModal,
    NotificationToast,
)

class Dashboard(App):
    def compose(self) -> ComposeResult:
        with Container():
            yield Header()
            
            with Horizontal():
                # Sidebar with tree view
                with Vertical(classes="sidebar"):
                    yield ResourceTree(id="resource-tree")
                    yield FilterPanel()
                
                # Main content
                with Vertical(classes="main"):
                    # Status bar
                    with Horizontal(classes="status-bar"):
                        yield StatusBadge(id="overall-status")
                        yield MetricsSparkline(id="throughput-graph")
                    
                    # Resource grid
                    yield ResourceGrid(
                        item_factory=ResourceCard,
                        id="resource-grid"
                    )
                    
                    # Bottom panel
                    with TabbedContent():
                        with Tab("Logs"):
                            yield LogViewer(id="logs", max_lines=10000)
                        with Tab("Timeline"):
                            yield DeploymentTimeline(id="timeline")
                        with Tab("Metrics"):
                            yield MetricsPanel()
            
            yield Footer()
```

### 37. Security Implementation Details

#### 37.1 OAuth 2.0 Flow Implementation

```python
class OAuthManager:
    """Manages OAuth flows for multiple providers."""
    
    FLOWS = {
        'authorization_code': AuthorizationCodeFlow,
        'pkce': PKCEFlow,
        'client_credentials': ClientCredentialsFlow,
        'device_code': DeviceCodeFlow,
    }
    
    async def authenticate(self, provider: str, flow_type: str) -> TokenSet:
        flow_class = self.FLOWS.get(flow_type)
        if not flow_class:
            raise UnsupportedFlowError(flow_type)
        
        flow = flow_class(self.get_provider_config(provider))
        return await flow.execute()
    
    async def refresh_token(self, token: TokenSet) -> TokenSet:
        if not token.refresh_token:
            raise NoRefreshToken()
        
        response = await self.http.post(
            token.token_endpoint,
            data={
                'grant_type': 'refresh_token',
                'refresh_token': token.refresh_token,
                'client_id': token.client_id,
            }
        )
        
        return TokenSet.from_response(response.json())
```

#### 37.2 Secret Rotation

```python
class RotatingSecret:
    """Secret with automatic rotation."""
    
    def __init__(
        self,
        name: str,
        rotation_days: int = 90,
        overlap_days: int = 7
    ):
        self.name = name
        self.rotation_days = rotation_days
        self.overlap_days = overlap_days
        self._current = None
        self._previous = None
        self._next_rotation = None
    
    async def get(self) -> str:
        await self._check_rotation()
        return self._current.value
    
    async def _check_rotation(self) -> None:
        if datetime.now() >= self._next_rotation:
            await self._rotate()
    
    async def _rotate(self) -> None:
        # Generate new secret
        new_secret = await self._generate()
        
        # Store with overlap period
        self._previous = self._current
        self._current = new_secret
        
        # Schedule next rotation
        self._next_rotation = datetime.now() + timedelta(
            days=self.rotation_days
        )
        
        # Clear previous after overlap
        asyncio.create_task(
            self._clear_previous_after(timedelta(days=self.overlap_days))
        )
```

### 38. Performance Tuning Guide

#### 38.1 Memory Optimization

```python
# Use __slots__ for dataclasses
@dataclass(slots=True, frozen=True)
class PortNumber:
    value: int

# Use weakref for caches
from weakref import WeakValueDictionary

class ResourceCache:
    def __init__(self):
        self._cache = WeakValueDictionary()
    
    def get(self, id: str) -> Optional[Resource]:
        return self._cache.get(id)
    
    def set(self, id: str, resource: Resource) -> None:
        self._cache[id] = resource

# Streaming for large datasets
async def stream_resources(filters: FilterSet) -> AsyncIterator[Resource]:
    async with db.cursor() as cursor:
        await cursor.execute(
            "SELECT * FROM resources WHERE ...",
            filters.params()
        )
        
        while True:
            rows = await cursor.fetchmany(1000)
            if not rows:
                break
            
            for row in rows:
                yield Resource.from_row(row)
```

#### 38.2 CPU Optimization

```python
# Use lru_cache for expensive operations
@lru_cache(maxsize=1024)
def compute_resource_hash(config: frozenset) -> str:
    return hashlib.sha256(
        json.dumps(sorted(config), sort_keys=True).encode()
    ).hexdigest()

# Parallel processing with ProcessPoolExecutor
from concurrent.futures import ProcessPoolExecutor

def process_resources_parallel(
    resources: List[Resource],
    cpu_count: int = None
) -> List[Result]:
    with ProcessPoolExecutor(max_workers=cpu_count) as pool:
        futures = [
            pool.submit(process_single, r)
            for r in resources
        ]
        return [f.result() for f in futures]
```

### 39. Testing Strategies

#### 39.1 Property-Based Testing

```python
from hypothesis import given, strategies as st

@given(
    port=st.integers(min_value=1, max_value=65535),
)
def test_port_number_validity(port: int):
    p = PortNumber(port)
    assert 1 <= p.value <= 65535
    
    if port < 1024:
        assert p.is_privileged()
    else:
        assert not p.is_privileged()

@given(
    resources=st.lists(st.builds(Resource), min_size=1, max_size=100),
)
def test_deployment_resource_uniqueness(resources: List[Resource]):
    deployment = Deployment.from_resources(resources)
    ids = [r.id for r in deployment.resources]
    assert len(ids) == len(set(ids))
```

#### 39.2 Mutation Testing

```bash
# Install mutmut for mutation testing
pip install mutmut

# Run mutation tests
mutmut run --paths-to-mutate=pheno_domain/
mutmut results

# Show surviving mutants
mutmut show 1
```

### 40. Deployment Patterns

#### 40.1 Blue-Green Deployment

```python
class BlueGreenDeployment:
    """Blue-green deployment strategy."""
    
    async def deploy(self, new_version: DeploymentSpec) -> Deployment:
        # Identify current (blue) deployment
        blue = await self.get_current_deployment()
        
        # Deploy new (green) version
        green = await self.deploy_new_version(new_version)
        
        # Health check green
        if not await self.health_check(green):
            await self.rollback(green)
            raise DeploymentFailed("Green health check failed")
        
        # Route traffic to green
        await self.switch_traffic(blue, green)
        
        # Monitor for issues
        if not await self.monitor_for_issues(duration=timedelta(minutes=5)):
            await self.switch_traffic(green, blue)
            raise DeploymentFailed("Post-switch issues detected")
        
        # Decommission blue
        await self.decommission(blue)
        
        return green
```

#### 40.2 Canary Deployment

```python
class CanaryDeployment:
    """Canary deployment with gradual rollout."""
    
    STAGES = [5, 25, 50, 75, 100]  # Percentage rollout
    
    async def deploy(self, new_version: DeploymentSpec) -> Deployment:
        canary = await self.deploy_canary(new_version, percentage=5)
        
        for stage in self.STAGES[1:]:
            # Increase traffic percentage
            await self.adjust_traffic_split(canary, percentage=stage)
            
            # Monitor metrics
            metrics = await self.collect_metrics(duration=timedelta(minutes=10))
            
            if not self.check_metrics(metrics):
                await self.rollback()
                raise DeploymentFailed(f"Metrics check failed at {stage}%")
        
        return canary
```

### 41. Observability Integration

#### 41.1 Distributed Tracing

```python
from opentelemetry import trace
from opentelemetry.trace import SpanKind

tracer = trace.get_tracer(__name__)

class TracedResourceRepository:
    async def provision(self, spec: ResourceSpec) -> Resource:
        with tracer.start_as_current_span(
            "provision_resource",
            kind=SpanKind.INTERNAL,
            attributes={
                "resource.type": spec.type,
                "resource.provider": spec.provider,
            }
        ) as span:
            try:
                resource = await self._do_provision(spec)
                span.set_attribute("resource.id", resource.id)
                span.set_status(Status(StatusCode.OK))
                return resource
            except Exception as e:
                span.record_exception(e)
                span.set_status(Status(StatusCode.ERROR, str(e)))
                raise
```

#### 41.2 Custom Metrics

```python
from prometheus_client import Counter, Histogram, Gauge

# Define metrics
deployments_total = Counter(
    'pheno_deployments_total',
    'Total deployments',
    ['status', 'provider']
)

deployment_duration = Histogram(
    'pheno_deployment_duration_seconds',
    'Deployment duration',
    buckets=[30, 60, 120, 300, 600, 1800]
)

active_resources = Gauge(
    'pheno_active_resources',
    'Currently active resources',
    ['type', 'provider', 'region']
)
```

### 42. Multi-Region Architecture

```
Global Architecture:

                        ┌─────────────┐
                        │   Global    │
                        │   Config    │
                        │   (Consul)  │
                        └──────┬──────┘
                               │
           ┌───────────────────┼───────────────────┐
           │                   │                   │
           ▼                   ▼                   ▼
    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
    │  us-east-1  │    │  eu-west-1  │    │ ap-south-1  │
    │             │    │             │    │             │
    │ ┌─────────┐ │    │ ┌─────────┐ │    │ ┌─────────┐ │
    │ │ Primary │ │    │ │Replica │ │    │ │Replica │ │
    │ │  DB     │ │───▶│ │  DB    │ │───▶│ │  DB    │ │
    │ └─────────┘ │    │ └─────────┘ │    │ └─────────┘ │
    │             │    │             │    │             │
    │ ┌─────────┐ │    │ ┌─────────┐ │    │ ┌─────────┐ │
    │ │ Service │ │    │ │ Service│ │    │ │ Service│ │
    │ │  Pods   │ │    │ │  Pods  │ │    │ │  Pods  │ │
    │ └─────────┘ │    │ └─────────┘ │    │ └─────────┘ │
    └─────────────┘    └─────────────┘    └─────────────┘
```

### 43. Disaster Recovery

| Recovery Type | RTO | RPO | Strategy |
|---------------|-----|-----|----------|
| Service failure | 0s | 0s | Multi-AZ auto-failover |
| AZ failure | 5m | 0s | Health check + reroute |
| Region failure | 15m | 5s | DNS failover + replica promotion |
| Data corruption | 1h | 1h | PITR restore |
| Complete loss | 4h | 1h | Cross-region backup restore |

### 44. Compliance & Auditing

```python
class AuditLogger:
    """HIPAA/SOC2 compliant audit logging."""
    
    REQUIRED_FIELDS = [
        'timestamp',
        'user_id',
        'action',
        'resource_type',
        'resource_id',
        'result',
        'ip_address',
        'session_id',
    ]
    
    def log(self, event: AuditEvent) -> None:
        # Validate required fields
        for field in self.REQUIRED_FIELDS:
            if not getattr(event, field):
                raise IncompleteAuditEvent(field)
        
        # Hash sensitive fields
        event.user_id = self._hash_pii(event.user_id)
        
        # Write to tamper-evident log
        self._write_to_wal(event)
        
        # Forward to SIEM
        self._forward_to_siem(event)
```

### 45. Feature Flags

```python
from pheno_config import FeatureFlagManager

ff = FeatureFlagManager()

# Gradual rollout
@ff.enabled('new_deployment_flow', rollout=10)
async def deploy_v2(spec: DeploymentSpec) -> Deployment:
    return await new_orchestrator.deploy(spec)

# User segmentation
@ff.enabled('beta_feature', users=['user1', 'user2'])
async def beta_function():
    pass

# Time-based
@ff.enabled('holiday_scaling', schedule='0 0 * 12 *')
async def holiday_deployment():
    pass
```

### 46. Documentation Generation

```python
# Auto-generate CLI documentation
from pheno_tools.docs import CLIDocGenerator

generator = CLIDocGenerator()

# Generate from CLI spec
generator.generate_from_spec(
    spec_path='cli/spec.yaml',
    output_dir='docs/cli',
    formats=['markdown', 'man', 'html']
)

# Generate API documentation
generator.generate_api_docs(
    modules=[pheno_core, pheno_domain],
    output_dir='docs/api',
    include_examples=True
)
```

### 47. Plugin System

```python
# Plugin interface
class ResourcePlugin:
    """Plugin for custom resource types."""
    
    @property
    def resource_type(self) -> str:
        raise NotImplementedError
    
    @property
    def capabilities(self) -> Set[Capability]:
        raise NotImplementedError
    
    async def provision(self, spec: ResourceSpec) -> Resource:
        raise NotImplementedError
    
    async def decommission(self, resource: Resource) -> None:
        raise NotImplementedError

# Plugin registration
from pheno_plugins import PluginRegistry

@PluginRegistry.register
class CustomDatabasePlugin(ResourcePlugin):
    resource_type = "custom_database"
    
    async def provision(self, spec: ResourceSpec) -> Resource:
        # Custom provisioning logic
        pass
```

### 48. Operational Runbooks

```markdown
# Runbook: Resource Degradation

## Detection
Alert: `pheno_resource_health < 0.8`

## Triage
1. Check resource status: `pheno resources get <id>`
2. View recent logs: `pheno resources logs <id> --tail=100`
3. Check provider status: `pheno providers status`

## Resolution
1. If resource stopped:
   ```
   pheno resources start <id>
   ```

2. If resource failed:
   ```
   pheno resources restart <id>
   ```

3. If provider issue:
   - Check provider status page
   - Consider failover to secondary provider

## Verification
1. Confirm health: `pheno resources get <id>`
2. Check metrics: Grafana dashboard

## Post-Incident
1. Document in incident log
2. Update runbook if needed
```

### 49. Capacity Planning

```python
class CapacityPlanner:
    """Forecast resource capacity needs."""
    
    def forecast(
        self,
        historical_data: List[UsageData],
        growth_rate: float,
        time_horizon: timedelta
    ) -> CapacityPlan:
        
        # Analyze trends
        trend = self._analyze_trend(historical_data)
        
        # Project growth
        projection = self._project_growth(
            trend,
            growth_rate,
            time_horizon
        )
        
        # Calculate headroom
        headroom = projection.peak * 1.5  # 50% headroom
        
        # Generate plan
        return CapacityPlan(
            current_capacity=projection.current,
            required_capacity=headroom,
            recommended_actions=self._recommendations(headroom),
            timeline=self._generate_timeline(headroom)
        )
```

### 50. Cost Optimization

```python
class CostOptimizer:
    """Identify and optimize resource costs."""
    
    def analyze(self, resources: List[Resource]) -> CostReport:
        recommendations = []
        
        # Right-sizing
        for resource in resources:
            if resource.metrics.cpu_avg < 20:
                recommendations.append(
                    Recommendation.downsize(resource)
                )
        
        # Spot instances
        for resource in resources:
            if resource.workload.interruptible:
                recommendations.append(
                    Recommendation.use_spot(resource)
                )
        
        # Reserved capacity
        if self._should_reserve_capacity(resources):
            recommendations.append(
                Recommendation.purchase_ri()
            )
        
        return CostReport(
            current_monthly_cost=self._calculate_cost(resources),
            potential_savings=self._calculate_savings(recommendations),
            recommendations=recommendations
        )
```


### 51. Additional Architecture Patterns

#### 51.1 Event Sourcing Implementation

```python
class EventStore:
    """PostgreSQL-backed event store."""
    
    async def append(self, aggregate_id: str, events: List[DomainEvent]) -> None:
        async with self.pool.transaction() as conn:
            for i, event in enumerate(events):
                await conn.execute(
                    """INSERT INTO events (aggregate_id, event_type, payload, version)
                       VALUES ($1, $2, $3, (SELECT COALESCE(MAX(version), 0) + $4 FROM events WHERE aggregate_id = $1))""",
                    aggregate_id,
                    event.event_type,
                    json.dumps(event.to_dict()),
                    i + 1
                )
            
            await conn.execute(
                "UPDATE aggregates SET version = version + $2 WHERE id = $1",
                aggregate_id,
                len(events)
            )
    
    async def get_events(self, aggregate_id: str, since_version: int = 0) -> List[DomainEvent]:
        rows = await self.pool.fetch(
            """SELECT event_type, payload, version FROM events
               WHERE aggregate_id = $1 AND version > $2
               ORDER BY version""",
            aggregate_id,
            since_version
        )
        
        return [self._deserialize_event(row) for row in rows]
```

#### 51.2 Snapshot Pattern

```python
class SnapshotStore:
    """Aggregate snapshot storage for performance."""
    
    SNAPSHOT_FREQUENCY = 100  # Events between snapshots
    
    async def save_snapshot(self, aggregate: AggregateRoot) -> None:
        version = aggregate.version
        
        if version % self.SNAPSHOT_FREQUENCY != 0:
            return
        
        await self.pool.execute(
            """INSERT INTO snapshots (aggregate_id, version, state, created_at)
               VALUES ($1, $2, $3, NOW())
               ON CONFLICT (aggregate_id) DO UPDATE SET
                   version = EXCLUDED.version,
                   state = EXCLUDED.state,
                   created_at = EXCLUDED.created_at""",
            aggregate.id,
            version,
            pickle.dumps(aggregate)
        )
    
    async def load_from_snapshot(self, aggregate_id: str) -> Optional[AggregateRoot]:
        row = await self.pool.fetchrow(
            "SELECT state FROM snapshots WHERE aggregate_id = $1",
            aggregate_id
        )
        
        if row:
            return pickle.loads(row['state'])
        return None
```

#### 51.3 Saga Pattern

```python
class DeploymentSaga:
    """Long-running transaction coordinator."""
    
    async def execute(self, steps: List[SagaStep]) -> SagaResult:
        completed = []
        
        try:
            for step in steps:
                result = await step.execute()
                completed.append(step)
                
                if not result.success:
                    raise SagaException(step, result)
            
            return SagaResult.success()
            
        except SagaException:
            # Compensate completed steps
            for step in reversed(completed):
                await step.compensate()
            
            return SagaResult.failed()
```

### 52. Additional Implementation Details

#### 52.1 Connection Pooling

```python
class DatabasePool:
    """Async database connection pool."""
    
    def __init__(self, dsn: str, min_size: int = 10, max_size: int = 100):
        self.dsn = dsn
        self.min_size = min_size
        self.max_size = max_size
        self._pool: Optional[asyncpg.Pool] = None
    
    async def initialize(self) -> None:
        self._pool = await asyncpg.create_pool(
            self.dsn,
            min_size=self.min_size,
            max_size=self.max_size,
            command_timeout=60,
            server_settings={
                'jit': 'off',
            }
        )
    
    @asynccontextmanager
    async def acquire(self) -> AsyncIterator[asyncpg.Connection]:
        async with self._pool.acquire() as conn:
            yield conn
```

#### 52.2 Circuit Breaker Configuration

```python
class CircuitBreakerConfig:
    """Fine-grained circuit breaker settings."""
    
    DEFAULTS = {
        'failure_threshold': 5,
        'recovery_timeout': 30.0,
        'half_open_max_calls': 3,
        'success_threshold': 2,
        'sliding_window_size': 60,  # seconds
    }
    
    # Provider-specific overrides
    PROVIDER_CONFIGS = {
        'aws': {'failure_threshold': 3, 'recovery_timeout': 60.0},
        'gcp': {'failure_threshold': 5, 'recovery_timeout': 30.0},
        'azure': {'failure_threshold': 7, 'recovery_timeout': 45.0},
    }
```


### 100. Appendix: Full API Reference

This section contains the complete API reference for all modules.

#### 100.1 Module Index

| Module | Classes | Functions | Lines of Code |
|--------|---------|-----------|---------------|
| pheno-core | 15 | 120 | ~3,500 |
| pheno-domain | 42 | 380 | ~12,000 |
| pheno-parser | 18 | 95 | ~4,200 |
| pheno-async | 24 | 150 | ~5,100 |
| pheno-tui | 56 | 220 | ~8,900 |
| pheno-vector | 22 | 85 | ~6,200 |
| pheno-events | 12 | 68 | ~2,800 |
| pheno-auth | 16 | 74 | ~3,400 |
| pheno-security | 14 | 62 | ~2,900 |
| pheno-exceptions | 28 | 45 | ~1,600 |
| pheno-db | 8 | 52 | ~2,100 |
| pheno-config | 10 | 88 | ~3,200 |

#### 100.2 Function Reference

Complete function signatures and documentation for all public APIs available in the generated documentation at docs.phenotype.io/api/.

