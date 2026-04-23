# State of the Art: Language SDKs, Parsers & Domain Modeling

**Project:** PhenoLang - Language/Parser SDK  
**Scope:** 681 Python files across 33 domain-specific modules  
**Research Date:** 2026-04-05  
**Classification:** Architecture Deep Dive / Foundational Research

---

## Executive Summary

PhenoLang represents a comprehensive domain-specific language (DSL) ecosystem for infrastructure orchestration, comprising 33+ Python modules spanning core language features, domain modeling, TUI components, vector operations, and async orchestration. This research establishes the theoretical foundations, surveys contemporary solutions, and derives architectural patterns for building production-grade language SDKs at scale.

Key findings indicate that modern language SDK design requires:
- **Parser combinators** for maintainable grammar definitions (vs. monolithic lexer/parser generators)
- **Domain-driven design (DDD)** with pure functional cores and imperative shells
- **Structured concurrency** for resource-safe async operations
- **Vector-native architectures** for AI-augmented tooling
- **Immediate-mode TUI** frameworks for responsive developer interfaces

---

## Table of Contents

1. [Language Design & Parser Architecture](#1-language-design--parser-architecture)
2. [Domain-Driven Design at Scale](#2-domain-driven-design-at-scale)
3. [Async & Structured Concurrency](#3-async--structured-concurrency)
4. [Vector Databases & Embeddings](#4-vector-databases--embeddings)
5. [TUI Architecture & Terminal Interfaces](#5-tui-architecture--terminal-interfaces)
6. [Error Handling & Exception Design](#6-error-handling--exception-design)
7. [Pattern Languages & Code Generation](#7-pattern-languages--code-generation)
8. [Security & Credential Management](#8-security--credential-management)
9. [Synthesis & Recommendations](#9-synthesis--recommendations)
10. [References](#10-references)

---

## 1. Language Design & Parser Architecture

### 1.1 The Parser Landscape

Modern language implementation offers three primary approaches:

| Approach | Tooling | Maintainability | Performance | Learning Curve |
|----------|---------|-------------------|-------------|----------------|
| **Parser Combinators** | funcparserlib, pyparsing, parsy | High | Moderate | Low |
| **Parser Generators** | ANTLR, PLY, Lark | Medium | High | High |
| **Hand-written Recursive Descent** | Manual | Low | Very High | Medium |
| **PEG/Packrat Parsing** | Arpeggio, pegen | High | Moderate | Medium |

### 1.2 Parser Combinator Theory

Parser combinators build parsers from atomic parser functions using higher-order composition operators. The theoretical foundation rests on monadic composition:

```
-- Parser type as state monad
type Parser a = String -> Maybe (a, String)

-- Sequential composition (bind)
(>>=) :: Parser a -> (a -> Parser b) -> Parser b
p >>= f = \input -> case p input of
    Just (result, rest) -> f result rest
    Nothing -> Nothing

-- Alternative composition
(<|>) :: Parser a -> Parser a -> Parser a
p1 <|> p2 = \input -> case p1 input of
    Just result -> Just result
    Nothing -> p2 input
```

**Key insight:** Combinator parsers enable **testable grammar units** and **incremental language evolution**. Each production rule becomes independently testable.

### 1.3 Modern Parser Implementations

#### 1.3.1 Lark (Python)
Lark combines the ease of EBNF grammar specification with multiple parsing algorithms:
- **Earley**: Handles ambiguous grammars, suitable for natural language
- **LALR(1)**: Deterministic, fast, limited lookahead
- **CYK**: Polynomial time for context-free grammars

```python
# Lark example: DSL for infrastructure definition
g = """
    ?start: resource+
    
    resource: "resource" IDENT "{" property* "}"
    
    property: IDENT ":" value
    
    value: STRING | NUMBER | "true" | "false" | list | object
    
    list: "[" [value ("," value)*] "]"
    object: "{" [property ("," property)*] "}"
    
    IDENT: /[a-zA-Z_][a-zA-Z0-9_]*/
    STRING: /"[^"]*"/ | /'[^']*'/
    NUMBER: /-?\d+(\.\d+)?/
    
    %ignore /\s+/
    %ignore /#.*\n/
"""
```

#### 1.3.2 tree-sitter (Multi-language)
Tree-sitter represents the state-of-the-art for **incremental parsing**:
- O(1) reparsing after edits via **LR(1) with GLR fallback**
- Concrete syntax tree preserves all source details
- Query language for pattern matching
- Native bindings for 20+ languages

```python
# Tree-sitter for syntax-aware tooling
from tree_sitter import Language, Parser

parser = Parser()
parser.set_language(PHENO_LANG)

tree = parser.parse(bytes(source_code, "utf8"))

# Query for resource declarations
query = PHENO_LANG.query("""
    (resource_declaration
        name: (identifier) @resource_name
        type: (resource_type) @resource_type)
""")
captures = query.captures(tree.root_node)
```

### 1.4 Grammar Design Patterns

#### 1.4.1 Expression Handling: Pratt Parsing
Pratt parsing (top-down operator precedence) elegantly handles operator precedence:

```python
# Pratt parser for expression parsing
class PrattParser:
    def parse_expression(self, precedence=0):
        prefix = self.parse_nud()  # Null denotation
        
        while precedence < self.current_token.precedence:
            prefix = self.parse_led(prefix)  # Left denotation
        
        return prefix
    
    def parse_nud(self):
        token = self.consume()
        if token.type == "NUMBER":
            return NumberNode(token.value)
        if token.type == "IDENT":
            return VariableNode(token.value)
        if token.type == "LPAREN":
            expr = self.parse_expression()
            self.expect("RPAREN")
            return expr
        raise ParseError(f"Unexpected token: {token}")
```

#### 1.4.2 Whitespace Sensitivity
Modern languages (Python, YAML, Haskell) use off-side rule for block structure:

```python
# Indentation-aware parsing
class IndentationTracker:
    def __init__(self):
        self.indent_stack = [0]
        self.pending_dedents = 0
    
    def process_token(self, token):
        if token.type == "NEWLINE":
            return self.handle_newline(token)
        if token.column > self.indent_stack[-1]:
            self.indent_stack.append(token.column)
            return [IndentToken("INDENT")]
        while token.column < self.indent_stack[-1]:
            self.indent_stack.pop()
            self.pending_dedents += 1
        return []
```

### 1.5 PhenoLang Parser Architecture Requirements

Given the 681-file codebase spanning multiple domains, PhenoLang's parser strategy should prioritize:

| Criterion | Priority | Rationale |
|-----------|----------|-----------|
| **Modularity** | Critical | 33 modules need independent grammar evolution |
| **Error Recovery** | High | Developer experience for DSL authoring |
| **Performance** | Medium | Not real-time, but must handle large configs |
| **Tree Preservation** | High | Source-to-source transformation needs |
| **Query Capability** | High | IDE support, linting, refactoring |

**Recommendation:** Hybrid approach using:
1. **Lark** for configuration DSLs (declarative grammar)
2. **Parser combinators** (parsy) for embedded expression languages
3. **tree-sitter** for IDE/language server integration

---

## 2. Domain-Driven Design at Scale

### 2.1 DDD Core Concepts

Domain-Driven Design (Evans, 2003; Vernon, 2016) provides patterns for modeling complex domains:

```
┌─────────────────────────────────────────────────────────────┐
│                    DOMAIN MODEL LAYERS                      │
├─────────────────────────────────────────────────────────────┤
│  User Interface Layer                                        │
│  ├─ TUI components (textual, rich)                          │
│  ├─ CLI arguments, flags                                      │
│  └─ Output formatters                                          │
├─────────────────────────────────────────────────────────────┤
│  Application Layer                                           │
│  ├─ Orchestration services                                     │
│  ├─ Transaction boundaries                                     │
│  └─ DTOs for cross-layer communication                       │
├─────────────────────────────────────────────────────────────┤
│  Domain Layer (The Heart)                                    │
│  ├─ Entities (identity-bearing objects)                      │
│  ├─ Value Objects (immutable, compared by value)               │
│  ├─ Domain Services (cross-aggregate operations)               │
│  ├─ Domain Events (immutable notifications)                    │
│  └─ Aggregates (consistency boundaries)                       │
├─────────────────────────────────────────────────────────────┤
│  Infrastructure Layer                                        │
│  ├─ Database adapters                                          │
│  ├─ External API clients                                       │
│  ├─ Message queues                                             │
│  └─ File system, network                                       │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 PhenoLang Domain Modules Analysis

The 33 Python modules suggest a highly decomposed domain model:

| Module Category | Modules | Pattern |
|-----------------|---------|---------|
| **Core Language** | pheno-core, pheno-domain | Entities, VOs |
| **Infrastructure** | pheno-db, pheno-storage, pheno-caching, pheno-ports | Repository, Adapter |
| **Integration** | pheno-adapters, pheno-integration, pheno-web, pheno-plugins | Anti-corruption Layer |
| **Security** | pheno-auth, pheno-security, pheno-credentials | ACL, Claims |
| **Observability** | pheno-logging, pheno-events, pheno-errors | Event Sourcing |
| **Developer Experience** | pheno-dev, pheno-tools, pheno-tui, pheno-ui | Facade, Builder |
| **Resilience** | pheno-async, pheno-resilience, pheno-exceptions | Circuit Breaker, Retry |
| **AI/ML** | pheno-vector | Vector Store, Embedding |

### 2.3 Entity Design Patterns

#### 2.3.1 Rich Domain Model vs. Anemic Domain Model

```python
# Anti-pattern: Anemic Domain Model
class Resource:
    def __init__(self):
        self.id = None
        self.status = None
        self.config = {}
    
# Rich Domain Model
class Resource(Entity):
    def __init__(self, id: ResourceId, config: ResourceConfig):
        self._id = id
        self._config = config
        self._status = ResourceStatus.PROVISIONING
        self._events: List[DomainEvent] = []
    
    @property
    def id(self) -> ResourceId:
        return self._id
    
    def provision(self, provider: Provider) -> None:
        if self._status != ResourceStatus.PROVISIONING:
            raise InvalidStateTransition()
        
        self._status = ResourceStatus.PROVISIONING
        self._events.append(ResourceProvisioningStarted(self._id))
        
        try:
            provider.allocate(self._config)
            self._status = ResourceStatus.RUNNING
            self._events.append(ResourceProvisioned(self._id))
        except ProviderError as e:
            self._status = ResourceStatus.FAILED
            self._events.append(ResourceProvisioningFailed(self._id, str(e)))
            raise
    
    def decommission(self) -> None:
        if self._status not in (ResourceStatus.RUNNING, ResourceStatus.STOPPED):
            raise InvalidStateTransition()
        
        self._status = ResourceStatus.TERMINATING
        self._events.append(ResourceDecommissioning(self._id))
```

#### 2.3.2 Value Objects

Value Objects are immutable, compared by value, and validated at construction:

```python
from dataclasses import dataclass
from typing import NewType

# Primitive obsession vs. domain typing
# Bad: raw strings/ints everywhere
def create_port(port: int) -> None: ...

# Good: PortNumber validates at construction
@dataclass(frozen=True)
class PortNumber:
    value: int
    
    def __post_init__(self):
        if not 1 <= self.value <= 65535:
            raise ValueError(f"Port must be 1-65535, got {self.value}")
        if self.value < 1024:
            # Warn about privileged ports
            import warnings
            warnings.warn(f"Port {self.value} requires elevated privileges")
    
    def __str__(self) -> str:
        return str(self.value)
    
    def is_privileged(self) -> bool:
        return self.value < 1024
    
    def is_ephemeral(self) -> bool:
        return self.value >= 49152

# Usage
http_port = PortNumber(8080)  # Valid
https_port = PortNumber(443)   # Valid, warns about privileges
# bad_port = PortNumber(70000)  # Raises ValueError
```

### 2.4 Aggregate Design

Aggregates enforce consistency boundaries. The rule: **one transaction = one aggregate**.

```python
# Aggregate Root: Deployment
class Deployment(AggregateRoot):
    """A deployment aggregates resources, services, and configuration.
    
    Invariants:
    - All resources must be in compatible regions
    - Services must reference valid resources
    - Configuration is versioned and immutable
    """
    
    def __init__(self, id: DeploymentId, spec: DeploymentSpec):
        self._id = id
        self._spec = spec
        self._resources: Dict[ResourceId, Resource] = {}
        self._services: Dict[ServiceId, Service] = {}
        self._version = 1
        self._status = DeploymentStatus.DRAFT
    
    def add_resource(self, resource: Resource) -> None:
        """Add resource to deployment."""
        self._enforce_region_compatibility(resource)
        self._resources[resource.id] = resource
        self._events.append(ResourceAddedToDeployment(self._id, resource.id))
    
    def bind_service(self, service: Service, resource: Resource) -> None:
        """Bind service to resource."""
        if resource.id not in self._resources:
            raise ResourceNotInDeployment()
        
        service.bind_to(resource)
        self._services[service.id] = service
        self._events.append(ServiceBound(self._id, service.id, resource.id))
    
    def deploy(self) -> None:
        """Execute deployment."""
        if self._status != DeploymentStatus.DRAFT:
            raise InvalidDeploymentStatus()
        
        self._status = DeploymentStatus.DEPLOYING
        
        # Provision resources in dependency order
        for resource in self._dependency_order(self._resources.values()):
            resource.provision(self._spec.provider)
        
        # Start services
        for service in self._services.values():
            service.start()
        
        self._status = DeploymentStatus.RUNNING
        self._version += 1
```

### 2.5 Domain Events & Event Sourcing

Domain events capture state changes for audit, integration, and replay:

```python
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
    
    def _payload(self) -> dict:
        raise NotImplementedError

# Concrete events
@dataclass(frozen=True)
class ResourceProvisioned(DomainEvent):
    resource_id: ResourceId
    provider: str
    region: str
    
    @property
    def event_type(self) -> str:
        return "resource.provisioned"
    
    def _payload(self) -> dict:
        return {
            "resource_id": str(self.resource_id),
            "provider": self.provider,
            "region": self.region,
        }

# Event Store
class EventStore:
    """Append-only event storage with versioning."""
    
    async def append(self, aggregate_id: str, events: List[DomainEvent], expected_version: int) -> None:
        """Append events with optimistic concurrency control."""
        async with self._db.transaction() as txn:
            current = await txn.fetch(
                "SELECT version FROM aggregates WHERE id = %s",
                aggregate_id
            )
            
            if current and current["version"] != expected_version:
                raise ConcurrencyException(expected_version, current["version"])
            
            for event in events:
                await txn.execute(
                    """INSERT INTO events (aggregate_id, event_type, payload, version, occurred_on)
                       VALUES (%s, %s, %s, %s, %s)""",
                    aggregate_id, event.event_type, event.to_dict(),
                    expected_version + 1, event.occurred_on
                )
            
            await txn.execute(
                "UPDATE aggregates SET version = %s WHERE id = %s",
                expected_version + len(events), aggregate_id
            )
```

---

## 3. Async & Structured Concurrency

### 3.1 Structured Concurrency Theory

Structured concurrency (Martin Sústrik, 2018; Nathaniel Smith, 2018) mandates that:
1. **Child tasks complete before parent** - no orphaned goroutines/tasks
2. **Cancellation propagates downward** - parent cancellation cancels children
3. **Errors bubble upward** - first failure cancels siblings

```
Traditional (Unstructured)          Structured
┌─────────────────────┐            ┌─────────────────────┐
│     Parent Task     │            │     Parent Task     │
│  ┌───────────────┐  │            │  ┌───────────────┐  │
│  │  Fire Child A  │──┼──►         │  │  Spawn Child A│  │
│  └───────────────┘  │            │  │  Spawn Child B│  │
│  ┌───────────────┐  │            │  │  Spawn Child C│  │
│  │  Fire Child B  │──┼──►         │  └───────┬───────┘  │
│  └───────────────┘  │            │          │          │
│         ...         │            │     ┌────┴────┐     │
│  ┌───────────────┐  │            │     ▼         ▼     │
│  │  ??? Where    │  │            │  [Children    ]     │
│  │  are children?│  │            │  [must finish]◄─────┤
│  └───────────────┘  │            │  [before exit]      │
└─────────────────────┘            └─────────────────────┘
```

### 3.2 Python Structured Concurrency: asyncio.TaskGroup

Python 3.11+ introduces `TaskGroup` for structured concurrency:

```python
import asyncio

async def orchestrate_deployment(resources: List[Resource]) -> None:
    """Provision all resources concurrently with structured concurrency."""
    
    # TaskGroup ensures all tasks complete or fail together
    async with asyncio.TaskGroup() as tg:
        tasks = []
        for resource in resources:
            # Any exception cancels all other tasks
            task = tg.create_task(resource.provision())
            tasks.append((resource, task))
    
    # At this point, all provisioned or exception raised
    results = [(r, t.result()) for r, t in tasks]
    return results

# With timeout and cancellation
async def provision_with_timeout(resources: List[Resource], timeout: float) -> None:
    try:
        async with asyncio.timeout(timeout):
            async with asyncio.TaskGroup() as tg:
                for resource in resources:
                    tg.create_task(resource.provision())
    except asyncio.TimeoutError:
        # TaskGroup automatically cancelled all pending tasks
        logger.error(f"Provisioning timed out after {timeout}s")
        raise ProvisioningTimeout()
```

### 3.3 Async Architecture Patterns

#### 3.3.1 Supervisor Pattern

```python
from typing import Callable, Awaitable
import asyncio

class Supervisor:
    """Supervises async workers with restart strategies."""
    
    def __init__(self, 
                 restart_strategy: RestartStrategy = RestartStrategy.ONE_FOR_ONE,
                 max_restarts: int = 5,
                 restart_window: float = 60.0):
        self._strategy = restart_strategy
        self._max_restarts = max_restarts
        self._restart_window = restart_window
        self._workers: Dict[str, Worker] = {}
        self._restart_history: Dict[str, List[float]] = {}
    
    def add_worker(self, 
                   name: str, 
                   factory: Callable[[], Awaitable[None]],
                   dependencies: Set[str] = None) -> None:
        self._workers[name] = Worker(name, factory, dependencies or set())
    
    async def start(self) -> None:
        """Start all workers respecting dependencies."""
        # Topological sort by dependencies
        ordered = self._topological_sort(self._workers)
        
        for name in ordered:
            await self._start_worker(name)
    
    async def _start_worker(self, name: str) -> None:
        worker = self._workers[name]
        
        try:
            task = asyncio.create_task(worker.run())
            worker.set_task(task)
            
            # Monitor for failure
            asyncio.create_task(self._monitor_worker(name, task))
        except Exception as e:
            await self._handle_failure(name, e)
    
    async def _monitor_worker(self, name: str, task: asyncio.Task) -> None:
        try:
            await task
        except asyncio.CancelledError:
            pass
        except Exception as e:
            await self._handle_failure(name, e)
    
    async def _handle_failure(self, name: str, error: Exception) -> None:
        """Handle worker failure according to restart strategy."""
        history = self._restart_history.get(name, [])
        now = asyncio.get_event_loop().time()
        
        # Remove old history outside window
        history = [t for t in history if now - t < self._restart_window]
        
        if len(history) >= self._max_restarts:
            logger.error(f"Worker {name} exceeded max restarts, shutting down supervisor")
            await self.shutdown()
            raise SupervisorShutdown(f"Worker {name} failed too many times")
        
        history.append(now)
        self._restart_history[name] = history
        
        if self._strategy == RestartStrategy.ONE_FOR_ONE:
            await self._restart_worker(name)
        elif self._strategy == RestartStrategy.ONE_FOR_ALL:
            await self.shutdown()
            await self.start()
        elif self._strategy == RestartStrategy.REST_FOR_ONE:
            await self._restart_worker_and_dependents(name)

class RestartStrategy(Enum):
    ONE_FOR_ONE = auto()   # Restart only failed worker
    ONE_FOR_ALL = auto()   # Restart all workers
    REST_FOR_ONE = auto()  # Restart failed and dependent workers
```

#### 3.3.2 Circuit Breaker Pattern

```python
import time
from enum import Enum, auto
from dataclasses import dataclass
from typing import Callable, TypeVar

T = TypeVar('T')

class CircuitState(Enum):
    CLOSED = auto()      # Normal operation
    OPEN = auto()        # Failing, reject calls
    HALF_OPEN = auto()   # Testing if recovered

@dataclass
class CircuitBreakerConfig:
    failure_threshold: int = 5
    recovery_timeout: float = 30.0
    half_open_max_calls: int = 3
    success_threshold: int = 2

class CircuitBreaker:
    """Circuit breaker for external service calls."""
    
    def __init__(self, name: str, config: CircuitBreakerConfig = None):
        self.name = name
        self._config = config or CircuitBreakerConfig()
        self._state = CircuitState.CLOSED
        self._failures = 0
        self._successes = 0
        self._last_failure_time: Optional[float] = None
        self._half_open_calls = 0
    
    async def call(self, operation: Callable[[], Awaitable[T]]) -> T:
        if self._state == CircuitState.OPEN:
            if time.time() - self._last_failure_time > self._config.recovery_timeout:
                self._state = CircuitState.HALF_OPEN
                self._half_open_calls = 0
                self._successes = 0
            else:
                raise CircuitBreakerOpen(self.name)
        
        if self._state == CircuitState.HALF_OPEN:
            if self._half_open_calls >= self._config.half_open_max_calls:
                raise CircuitBreakerOpen(self.name)
            self._half_open_calls += 1
        
        try:
            result = await operation()
            self._on_success()
            return result
        except Exception as e:
            self._on_failure()
            raise
    
    def _on_success(self) -> None:
        if self._state == CircuitState.HALF_OPEN:
            self._successes += 1
            if self._successes >= self._config.success_threshold:
                self._state = CircuitState.CLOSED
                self._failures = 0
        else:
            self._failures = 0
    
    def _on_failure(self) -> None:
        self._failures += 1
        self._last_failure_time = time.time()
        
        if self._failures >= self._config.failure_threshold:
            self._state = CircuitState.OPEN
```

### 3.4 Async Database Patterns

```python
from contextlib import asynccontextmanager
from typing import AsyncIterator

class AsyncRepository(Generic[T]):
    """Generic async repository with connection pooling."""
    
    def __init__(self, pool: asyncpg.Pool, 
                 table: str,
                 entity_factory: Callable[[dict], T]):
        self._pool = pool
        self._table = table
        self._entity_factory = entity_factory
    
    @asynccontextmanager
    async def _transaction(self) -> AsyncIterator[asyncpg.Connection]:
        async with self._pool.acquire() as conn:
            async with conn.transaction():
                yield conn
    
    async def get_by_id(self, id: UUID) -> Optional[T]:
        async with self._pool.acquire() as conn:
            row = await conn.fetchrow(
                f"SELECT * FROM {self._table} WHERE id = $1", id
            )
            return self._entity_factory(dict(row)) if row else None
    
    async def save(self, entity: T) -> None:
        async with self._transaction() as conn:
            data = self._to_dict(entity)
            await conn.execute(f"""
                INSERT INTO {self._table} (id, data, updated_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT (id) DO UPDATE SET
                    data = EXCLUDED.data,
                    updated_at = EXCLUDED.updated_at
            """, entity.id, json.dumps(data))
```

---

## 4. Vector Databases & Embeddings

### 4.1 Vector Search Fundamentals

Vector similarity search enables semantic retrieval:

```
┌─────────────────────────────────────────────────────────────┐
│                   VECTOR SEARCH PIPELINE                    │
├─────────────────────────────────────────────────────────────┤
│  1. Embedding                                               │
│     Text ──► BERT/OpenAI ──► [0.1, -0.3, 0.8, ...] (1536d) │
│                                                             │
│  2. Indexing                                                │
│     ┌──────────┐  ┌──────────┐  ┌──────────┐                │
│     │  HNSW    │  │  IVF    │  │  PQ     │                │
│     │  Graph   │  │  Flat   │  │  Scalar │                │
│     └──────────┘  └──────────┘  └──────────┘                │
│                                                             │
│  3. Query                                                   │
│     Query ──► Embed ──► Search Index ──► Top-K Results      │
│                                                             │
│  4. Reranking                                               │
│     Cross-encoder reranking for precision                  │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Vector Database Landscape

| Database | Embedding | Indexing | Scaling | Multi-modal | Open Source |
|----------|-----------|----------|---------|-------------|-------------|
| **pgvector** | External | ivfflat, hnsw | Postgres replica | No | Yes |
| **Pinecone** | External | Metadata + vector | Managed | No | No |
| **Weaviate** | Built-in | HNSW | Kubernetes | Yes | Yes |
| **Milvus/Zilliz** | External | GPU-accelerated | Distributed | Yes | Yes |
| **Chroma** | Built-in | HNSW | Single-node | Images | Yes |
| **Qdrant** | External | HNSW | Distributed | No | Yes |
| **Redis** | External | FLAT, HNSW | Redis Cluster | No | Yes |

### 4.3 pgvector Implementation

```python
# pgvector for Postgres vector operations
import asyncpg
from pgvector.asyncpg import register_vector

class VectorRepository:
    """Vector storage and similarity search with pgvector."""
    
    def __init__(self, pool: asyncpg.Pool, embedding_dim: int = 1536):
        self._pool = pool
        self._dim = embedding_dim
    
    async def setup(self) -> None:
        """Initialize vector extension and tables."""
        async with self._pool.acquire() as conn:
            await register_vector(conn)
            
            await conn.execute("CREATE EXTENSION IF NOT EXISTS vector")
            
            await conn.execute(f"""
                CREATE TABLE IF NOT EXISTS embeddings (
                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                    content TEXT NOT NULL,
                    source_type VARCHAR(50),
                    source_id VARCHAR(255),
                    embedding vector({self._dim}),
                    metadata JSONB,
                    created_at TIMESTAMPTZ DEFAULT NOW()
                )
            """)
            
            # HNSW index for fast approximate search
            await conn.execute(f"""
                CREATE INDEX IF NOT EXISTS idx_embedding_hnsw 
                ON embeddings USING hnsw (embedding vector_cosine_ops)
                WITH (m = 16, ef_construction = 64)
            """)
            
            # IVFFlat for exact search fallback
            await conn.execute(f"""
                CREATE INDEX IF NOT EXISTS idx_embedding_ivf
                ON embeddings USING ivfflat (embedding vector_cosine_ops)
                WITH (lists = 100)
            """)
    
    async def store(self, 
                    content: str,
                    embedding: List[float],
                    source_type: str = None,
                    source_id: str = None,
                    metadata: dict = None) -> UUID:
        """Store content with its embedding."""
        async with self._pool.acquire() as conn:
            row = await conn.fetchrow(
                """INSERT INTO embeddings (content, source_type, source_id, embedding, metadata)
                   VALUES ($1, $2, $3, $4, $5)
                   RETURNING id""",
                content, source_type, source_id, embedding, 
                json.dumps(metadata) if metadata else None
            )
            return row['id']
    
    async def search(self,
                     query_embedding: List[float],
                     top_k: int = 10,
                     filter_metadata: dict = None) -> List[SearchResult]:
        """Similarity search with optional metadata filtering."""
        async with self._pool.acquire() as conn:
            if filter_metadata:
                # Combined vector + metadata search
                rows = await conn.fetch(
                    """SELECT id, content, source_type, source_id, metadata,
                              1 - (embedding <=> $1) AS similarity
                       FROM embeddings
                       WHERE metadata @> $2
                       ORDER BY embedding <=> $1
                       LIMIT $3""",
                    query_embedding, json.dumps(filter_metadata), top_k
                )
            else:
                # Pure vector search
                rows = await conn.fetch(
                    """SELECT id, content, source_type, source_id, metadata,
                              1 - (embedding <=> $1) AS similarity
                       FROM embeddings
                       ORDER BY embedding <=> $1
                       LIMIT $2""",
                    query_embedding, top_k
                )
            
            return [SearchResult(**dict(row)) for row in rows]
    
    async def hybrid_search(self,
                            query_text: str,
                            query_embedding: List[float],
                            top_k: int = 10,
                            vector_weight: float = 0.7) -> List[SearchResult]:
        """Combine BM25 text search with vector similarity."""
        async with self._pool.acquire() as conn:
            rows = await conn.fetch(
                """WITH text_scores AS (
                       SELECT id, ts_rank(to_tsvector('english', content), 
                                          plainto_tsquery('english', $1)) AS text_score
                       FROM embeddings
                       WHERE to_tsvector('english', content) @@ plainto_tsquery('english', $1)
                   ),
                   vector_scores AS (
                       SELECT id, 1 - (embedding <=> $2) AS vector_score
                       FROM embeddings
                       ORDER BY embedding <=> $2
                       LIMIT $3 * 2
                   )
                   SELECT e.id, e.content, e.source_type, e.source_id, e.metadata,
                          ($4 * COALESCE(v.vector_score, 0) + 
                           (1 - $4) * COALESCE(t.text_score, 0)) AS score
                   FROM embeddings e
                   LEFT JOIN text_scores t ON e.id = t.id
                   LEFT JOIN vector_scores v ON e.id = v.id
                   WHERE t.id IS NOT NULL OR v.id IS NOT NULL
                   ORDER BY score DESC
                   LIMIT $3""",
                query_text, query_embedding, top_k, vector_weight
            )
            return [SearchResult(**dict(row)) for row in rows]
```

### 4.4 Embedding Strategies

```python
from typing import Protocol
import openai
import numpy as np

class Embedder(Protocol):
    """Protocol for text embedding providers."""
    
    async def embed(self, texts: List[str]) -> List[List[float]]:
        ...
    
    @property
    def dimension(self) -> int:
        ...

class OpenAIEmbedder:
    """OpenAI text-embedding-3-* models."""
    
    MODELS = {
        "text-embedding-3-small": 1536,
        "text-embedding-3-large": 3072,
        "text-embedding-ada-002": 1536,
    }
    
    def __init__(self, model: str = "text-embedding-3-small"):
        self._client = openai.AsyncOpenAI()
        self._model = model
    
    async def embed(self, texts: List[str]) -> List[List[float]]:
        response = await self._client.embeddings.create(
            model=self._model,
            input=texts
        )
        return [e.embedding for e in response.data]
    
    @property
    def dimension(self) -> int:
        return self.MODELS[self._model]

class LocalEmbedder:
    """Local embedding with sentence-transformers."""
    
    def __init__(self, model_name: str = "all-MiniLM-L6-v2"):
        from sentence_transformers import SentenceTransformer
        self._model = SentenceTransformer(model_name)
    
    async def embed(self, texts: List[str]) -> List[List[float]]:
        # CPU-bound, run in thread pool
        loop = asyncio.get_event_loop()
        embeddings = await loop.run_in_executor(
            None, self._model.encode, texts
        )
        return embeddings.tolist()
    
    @property
    def dimension(self) -> int:
        return self._model.get_sentence_embedding_dimension()

# Chunking strategies for long documents
class Chunker:
    """Chunk documents for embedding with overlap."""
    
    def __init__(self, 
                 chunk_size: int = 512,
                 chunk_overlap: int = 50,
                 separator: str = "\n"):
        self.chunk_size = chunk_size
        self.chunk_overlap = chunk_overlap
        self.separator = separator
    
    def chunk(self, text: str) -> List[str]:
        """Split text into overlapping chunks."""
        words = text.split()
        chunks = []
        
        for i in range(0, len(words), self.chunk_size - self.chunk_overlap):
            chunk = " ".join(words[i:i + self.chunk_size])
            chunks.append(chunk)
            
            if i + self.chunk_size >= len(words):
                break
        
        return chunks
```

---

## 5. TUI Architecture & Terminal Interfaces

### 5.1 Terminal UI Landscape

Modern Python TUI frameworks offer different architectural approaches:

| Framework | Paradigm | Widgets | Async | Markdown | Performance |
|-----------|----------|---------|-------|----------|-------------|
| **Textual** | Reactive | Rich | Yes | Yes | Good |
| **Rich** | Imperative | N/A | No | Yes | Excellent |
| **prompt-toolkit** | Event-driven | Extensive | Yes | No | Good |
| **blessed** | Low-level | Basic | No | No | Excellent |
| **urwid** | Event-driven | Many | Limited | No | Moderate |

### 5.2 Textual Reactive Architecture

Textual implements a React-like component model for terminals:

```python
from textual.app import App, ComposeResult
from textual.containers import Container, Grid
from textual.widgets import Header, Footer, Static, Button, Input, DataTable
from textual.reactive import reactive
from textual.binding import Binding

class ResourceWidget(Static):
    """Widget displaying a single resource status."""
    
    status = reactive("pending")
    
    def compose(self) -> ComposeResult:
        with Container():
            yield Static(self.status, classes="status-indicator")
            yield Static(id="resource-details")
    
    def watch_status(self, status: str) -> None:
        """React to status changes."""
        self.update(self._render_status(status))
        self.add_class(f"status-{status}")
    
    def _render_status(self, status: str) -> str:
        icons = {
            "pending": "⏳",
            "provisioning": "🔧",
            "running": "✅",
            "failed": "❌",
            "stopped": "⏹️",
        }
        return f"{icons.get(status, '?')} {status.upper()}"

class DeploymentApp(App):
    """Main TUI application for deployment orchestration."""
    
    CSS = """
    Screen { align: center middle; }
    .status-running { color: green; }
    .status-failed { color: red; }
    .status-pending { color: yellow; }
    
    ResourceWidget {
        width: 100%;
        height: 3;
        border: solid green;
        padding: 1;
    }
    """
    
    BINDINGS = [
        Binding("q", "quit", "Quit"),
        Binding("r", "refresh", "Refresh"),
        Binding("d", "deploy", "Deploy"),
        Binding("s", "stop", "Stop All"),
    ]
    
    resources: reactive[list] = reactive([])
    
    def compose(self) -> ComposeResult:
        yield Header()
        
        with Container(id="main"):
            yield Input(placeholder="Search resources...", id="search")
            
            with Grid(id="resource-grid"):
                for resource in self.resources:
                    yield ResourceWidget(resource)
        
        yield Footer()
    
    async def action_deploy(self) -> None:
        """Deploy all pending resources."""
        pending = [r for r in self.resources if r.status == "pending"]
        
        # Update UI immediately
        for resource in pending:
            resource.status = "provisioning"
        
        # Async deployment with progress
        async with asyncio.TaskGroup() as tg:
            for resource in pending:
                tg.create_task(self._provision_resource(resource))
    
    async def _provision_resource(self, resource: Resource) -> None:
        """Provision a single resource with progress updates."""
        try:
            await resource.provision()
            resource.status = "running"
        except Exception as e:
            resource.status = "failed"
            self.notify(f"Failed to provision {resource.name}: {e}", severity="error")

# Data table for structured data display
class ResourceTable(DataTable):
    """Sortable, filterable resource table."""
    
    def on_mount(self) -> None:
        self.add_columns("Name", "Type", "Status", "Region", "Created")
        self.cursor_type = "row"
        self.zebra_stripes = True
    
    def add_resource(self, resource: Resource) -> None:
        self.add_row(
            resource.name,
            resource.type,
            resource.status,
            resource.region,
            resource.created_at.strftime("%Y-%m-%d %H:%M"),
        )
    
    def on_data_table_row_selected(self, event: DataTable.RowSelected) -> None:
        """Handle row selection for detail view."""
        row = self.get_row(event.row_key)
        self.app.push_screen(ResourceDetailScreen(row[0]))
```

### 5.3 Layout & Composition Patterns

```python
from textual.containers import Horizontal, Vertical, ScrollableContainer
from textual.widgets import Tree, Log, ProgressBar

class DashboardLayout(App):
    """Complex dashboard with multiple panes."""
    
    def compose(self) -> ComposeResult:
        yield Header()
        
        with Horizontal():
            # Sidebar with navigation
            with Vertical(classes="sidebar"):
                yield Tree("Resources", id="nav-tree")
                yield Log(id="system-log")
            
            # Main content area
            with Vertical(classes="main"):
                # Top: Status overview
                with Horizontal(classes="status-bar"):
                    yield Static("Total: 0", id="total-count")
                    yield Static("Running: 0", id="running-count")
                    yield Static("Failed: 0", id="failed-count")
                
                # Middle: Resource list
                with ScrollableContainer():
                    yield ResourceTable(id="resource-table")
                
                # Bottom: Progress for active operations
                with Horizontal(classes="progress-area"):
                    yield ProgressBar(total=100, id="main-progress")
        
        yield Footer()
```

---

## 6. Error Handling & Exception Design

### 6.1 Exception Hierarchy Design

```python
# Base exception hierarchy for domain-specific errors

class PhenoLangError(Exception):
    """Base for all PhenoLang errors."""
    
    def __init__(self, message: str, *,
                 code: str = None,
                 details: dict = None,
                 cause: Exception = None):
        super().__init__(message)
        self.code = code or self._default_code()
        self.details = details or {}
        self.cause = cause
    
    def _default_code(self) -> str:
        return self.__class__.__name__.upper()
    
    def to_dict(self) -> dict:
        return {
            "error": self.code,
            "message": str(self),
            "details": self.details,
            "cause": str(self.cause) if self.cause else None,
        }

# Domain-specific exceptions
class DomainError(PhenoLangError):
    """Errors in domain logic."""
    pass

class ValidationError(DomainError):
    """Invalid input data."""
    
    def __init__(self, message: str, *,
                 field: str = None,
                 value = None,
                 constraints: dict = None):
        super().__init__(message, code="VALIDATION_ERROR")
        self.field = field
        self.value = value
        self.constraints = constraints or {}

class ResourceError(DomainError):
    """Resource lifecycle errors."""
    pass

class ResourceNotFound(ResourceError):
    """Resource does not exist."""
    
    def __init__(self, resource_id: str, resource_type: str = None):
        msg = f"Resource {resource_id} not found"
        if resource_type:
            msg += f" (type: {resource_type})"
        super().__init__(msg, code="RESOURCE_NOT_FOUND",
                         details={"resource_id": resource_id, "resource_type": resource_type})

class InvalidStateTransition(ResourceError):
    """Illegal state change attempted."""
    
    def __init__(self, resource_id: str, 
                 from_state: str, 
                 attempted_action: str):
        super().__init__(
            f"Cannot {attempted_action} from state {from_state}",
            code="INVALID_STATE_TRANSITION",
            details={
                "resource_id": resource_id,
                "from_state": from_state,
                "attempted_action": attempted_action,
            }
        )

# Infrastructure exceptions
class InfrastructureError(PhenoLangError):
    """External system failures."""
    pass

class DatabaseError(InfrastructureError):
    """Database operation failed."""
    pass

class ProviderError(InfrastructureError):
    """Cloud provider API error."""
    
    def __init__(self, provider: str, 
                 operation: str,
                 status_code: int = None,
                 response: dict = None):
        super().__init__(
            f"{provider} error during {operation}",
            code="PROVIDER_ERROR",
            details={
                "provider": provider,
                "operation": operation,
                "status_code": status_code,
                "response": response,
            }
        )
```

### 6.2 Error Handling Patterns

#### 6.2.1 Railway-Oriented Programming

```python
from typing import TypeVar, Generic, Callable, Union
from dataclasses import dataclass

T = TypeVar('T')
E = TypeVar('E')

@dataclass
class Ok(Generic[T]):
    value: T
    
    def is_ok(self) -> bool:
        return True
    
    def is_err(self) -> bool:
        return False
    
    def map(self, f: Callable[[T], U]) -> 'Ok[U]':
        return Ok(f(self.value))
    
    def bind(self, f: Callable[[T], 'Result[U, E]']) -> 'Result[U, E]':
        return f(self.value)

@dataclass
class Err(Generic[E]):
    error: E
    
    def is_ok(self) -> bool:
        return False
    
    def is_err(self) -> bool:
        return True
    
    def map(self, f: Callable[[T], U]) -> 'Err[E]':
        return self
    
    def bind(self, f: Callable[[T], 'Result[U, E]']) -> 'Err[E]':
        return self

Result = Union[Ok[T], Err[E]]

# Usage
from typing import Any

def parse_port(value: Any) -> Result[PortNumber, ValidationError]:
    """Parse and validate port number."""
    if not isinstance(value, int):
        return Err(ValidationError("Port must be an integer", value=value))
    
    try:
        return Ok(PortNumber(value))
    except ValueError as e:
        return Err(ValidationError(str(e), value=value))

def find_resource(id: str) -> Result[Resource, ResourceNotFound]:
    """Find resource by ID."""
    resource = repository.get(id)
    if resource is None:
        return Err(ResourceNotFound(id))
    return Ok(resource)

# Composition
result = (Ok("8080")
          .bind(lambda s: parse_port(int(s)) if s.isdigit() else Err(ValidationError("Not a number")))
          .bind(lambda port: find_resource(f"resource-{port.value}")))

if result.is_ok():
    print(f"Found: {result.value}")
else:
    print(f"Error: {result.error}")
```

#### 6.2.2 Exception Translation

```python
from contextlib import contextmanager
from functools import wraps
import logging

logger = logging.getLogger(__name__)

@contextmanager
def translate_exceptions(mapping: dict[type, type] = None):
    """Translate low-level exceptions to domain exceptions."""
    mapping = mapping or {}
    
    try:
        yield
    except tuple(mapping.keys()) as e:
        target = mapping[type(e)]
        raise target(f"Operation failed: {e}") from e
    except Exception as e:
        logger.exception("Unexpected error")
        raise InfrastructureError(f"Unexpected error: {e}") from e

# Usage
def get_resource_with_translation(resource_id: str) -> Resource:
    with translate_exceptions({
        asyncpg.PostgresError: DatabaseError,
        aiohttp.ClientError: ProviderError,
    }):
        return await repository.get(resource_id)

# Decorator version
def with_error_handling(**mappings):
    """Decorator for automatic exception translation."""
    def decorator(func):
        @wraps(func)
        async def wrapper(*args, **kwargs):
            try:
                return await func(*args, **kwargs)
            except tuple(mappings.keys()) as e:
                target = mappings[type(e)]
                raise target(f"{func.__name__} failed: {e}") from e
        return wrapper
    return decorator

@with_error_handling(
    asyncpg.PostgresError=DatabaseError,
    ValueError=ValidationError,
)
async def create_resource(config: dict) -> Resource:
    ...
```

---

## 7. Pattern Languages & Code Generation

### 7.1 Code Generation Strategies

| Strategy | Use Case | Tools | Maintainability |
|----------|----------|-------|-----------------|
| **Template-based** | Configurable boilerplate | Jinja2, Mako | High |
| **AST-based** | Source transformation | libcst, ast | High |
| **Annotation-driven** | Decorators, metaclasses | Python introspection | Medium |
| **Codegen from spec** | API clients, schemas | OpenAPI, protobuf | High |
| **Macro-based** | Compile-time expansion | Rust macros, C++ | N/A in Python |

### 7.2 Jinja2 Template Architecture

```python
from jinja2 import Environment, PackageLoader, select_autoescape
from dataclasses import dataclass
from typing import List

@dataclass
class EntitySpec:
    name: str
    fields: List[FieldSpec]
    events: List[str]
    aggregate: bool = False

@dataclass
class FieldSpec:
    name: str
    type: str
    nullable: bool = False
    default = None

class CodeGenerator:
    """Generate domain code from specifications."""
    
    def __init__(self):
        self.env = Environment(
            loader=PackageLoader('pheno_lang', 'templates'),
            autoescape=select_autoescape(['html', 'xml']),
            trim_blocks=True,
            lstrip_blocks=True,
        )
    
    def generate_entity(self, spec: EntitySpec) -> str:
        template = self.env.get_template('entity.py.j2')
        return template.render(spec=spec)
    
    def generate_repository(self, spec: EntitySpec) -> str:
        template = self.env.get_template('repository.py.j2')
        return template.render(spec=spec)
    
    def generate_tests(self, spec: EntitySpec) -> str:
        template = self.env.get_template('test_entity.py.j2')
        return template.render(spec=spec)

# Template: entity.py.j2
"""
# {{ spec.name }} - Auto-generated entity
from dataclasses import dataclass
from typing import Optional, List
from uuid import UUID
from datetime import datetime
from pheno_domain.base import Entity, DomainEvent

@dataclass
class {{ spec.name }}(Entity):
    \"\"\"{{ spec.name }} entity.\"\"\"
    
    {% for field in spec.fields %}
    {{ field.name }}: {% if field.nullable %}Optional[{% endif %}{{ field.type }}{% if field.nullable %}]{% endif %}
    {%- if field.default is not none %} = {{ field.default }}{% endif %}
    {% endfor %}
    
    {% if spec.aggregate %}
    _events: List[DomainEvent] = field(default_factory=list)
    
    def apply(self, event: DomainEvent) -> None:
        \"\"\"Apply event to aggregate state.\"\"\"
        self._events.append(event)
        # Event handlers would dispatch here
    {% endif %}
    
    {% for event in spec.events %}
    def on_{{ event | snake_case }}(self, event: {{ event }}) -> None:
        \"\"\"Handle {{ event }} event.\"\"\"
        pass
    {% endfor %}
"""
```

### 7.3 AST-Based Transformation

```python
import libcst as cst
from libcst import matchers as m

class AddTypeHints(cst.Codemod):
    """Add type hints based on docstring analysis."""
    
    def leave_FunctionDef(
        self, 
        original_node: cst.FunctionDef,
        updated_node: cst.FunctionDef
    ) -> cst.FunctionDef:
        
        # Extract types from docstring
        docstring = self._get_docstring(updated_node)
        param_types, return_type = self._parse_docstring_types(docstring)
        
        # Add type annotations
        new_params = []
        for param in updated_node.params.params:
            if param.name.value in param_types:
                annotation = cst.Annotation(
                    annotation=cst.parse_expression(param_types[param.name.value])
                )
                new_params.append(param.with_changes(annotation=annotation))
            else:
                new_params.append(param)
        
        # Add return annotation
        new_returns = None
        if return_type:
            new_returns = cst.Annotation(
                annotation=cst.parse_expression(return_type)
            )
        
        return updated_node.with_changes(
            params=updated_node.params.with_changes(params=new_params),
            returns=new_returns,
        )

# Usage
source = """
def greet(name):
    \"\"\"Greet someone.
    
    :param name: The name to greet (str)
    :returns: Greeting message (str)
    \"\"\"
    return f"Hello, {name}!"
"""

tree = cst.parse_module(source)
transformed = tree.visit(AddTypeHints())
print(transformed.code)
```

---

## 8. Security & Credential Management

### 8.1 Secret Management Patterns

```python
from cryptography.fernet import Fernet
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
import base64
import os
from typing import Protocol
from contextlib import contextmanager

class SecretStore(Protocol):
    """Protocol for secret storage backends."""
    
    async def get(self, key: str) -> str:
        ...
    
    async def set(self, key: str, value: str) -> None:
        ...
    
    async def delete(self, key: str) -> None:
        ...

class KeychainStore(SecretStore):
    """macOS Keychain integration."""
    
    async def get(self, key: str) -> str:
        import keyring
        value = keyring.get_password("pheno_lang", key)
        if value is None:
            raise SecretNotFound(key)
        return value
    
    async def set(self, key: str, value: str) -> None:
        import keyring
        keyring.set_password("pheno_lang", key, value)
    
    async def delete(self, key: str) -> None:
        import keyring
        keyring.delete_password("pheno_lang", key)

class FileStore(SecretStore):
    """Encrypted file-based store for CI/testing."""
    
    def __init__(self, path: str, master_key: bytes):
        self._path = path
        self._fernet = Fernet(master_key)
    
    async def get(self, key: str) -> str:
        if not os.path.exists(self._path):
            raise SecretNotFound(key)
        
        with open(self._path, 'rb') as f:
            encrypted = f.read()
        
        decrypted = self._fernet.decrypt(encrypted)
        data = json.loads(decrypted)
        
        if key not in data:
            raise SecretNotFound(key)
        
        return data[key]
    
    async def set(self, key: str, value: str) -> None:
        data = {}
        if os.path.exists(self._path):
            with open(self._path, 'rb') as f:
                encrypted = f.read()
            decrypted = self._fernet.decrypt(encrypted)
            data = json.loads(decrypted)
        
        data[key] = value
        
        encrypted = self._fernet.encrypt(json.dumps(data).encode())
        with open(self._path, 'wb') as f:
            f.write(encrypted)
        
        os.chmod(self._path, 0o600)

# Credential rotation
class RotatingCredential:
    """Credential with automatic rotation."""
    
    def __init__(self, 
                 store: SecretStore,
                 key: str,
                 rotation_days: int = 90):
        self._store = store
        self._key = key
        self._rotation_days = rotation_days
        self._cache: Optional[str] = None
        self._cached_at: Optional[datetime] = None
    
    async def get(self) -> str:
        # Check cache
        if self._cache and self._cached_at:
            if datetime.now() - self._cached_at < timedelta(minutes=5):
                return self._cache
        
        # Fetch from store
        value = await self._store.get(self._key)
        metadata = self._extract_metadata(value)
        
        # Check if rotation needed
        if self._needs_rotation(metadata):
            new_value = await self._rotate()
            await self._store.set(self._key, new_value)
            value = new_value
        
        # Cache and return
        self._cache = self._extract_value(value)
        self._cached_at = datetime.now()
        return self._cache
    
    def _needs_rotation(self, metadata: dict) -> bool:
        created = datetime.fromisoformat(metadata['created_at'])
        return datetime.now() - created > timedelta(days=self._rotation_days)
    
    async def _rotate(self) -> str:
        # Generate new credential
        new_credential = secrets.token_urlsafe(32)
        metadata = {
            'value': new_credential,
            'created_at': datetime.now().isoformat(),
            'version': self._extract_metadata(await self._store.get(self._key)).get('version', 0) + 1,
        }
        return json.dumps(metadata)
```

### 8.2 OAuth Flow Implementation

```python
from dataclasses import dataclass
from typing import Optional, Callable
import httpx
import time

@dataclass
class TokenSet:
    access_token: str
    refresh_token: Optional[str]
    expires_at: float
    token_type: str = "Bearer"
    scope: Optional[str] = None
    
    @property
    def is_expired(self) -> bool:
        return time.time() >= self.expires_at - 60  # 60s buffer

class OAuthClient:
    """OAuth 2.0 client with automatic token refresh."""
    
    def __init__(self,
                 client_id: str,
                 client_secret: str,
                 authorization_url: str,
                 token_url: str,
                 redirect_uri: str,
                 store: SecretStore = None):
        self.client_id = client_id
        self.client_secret = client_secret
        self.authorization_url = authorization_url
        self.token_url = token_url
        self.redirect_uri = redirect_uri
        self._store = store
        self._token: Optional[TokenSet] = None
    
    def get_authorization_url(self, 
                              state: str,
                              scope: str = None,
                              pkce: bool = True) -> tuple[str, Optional[str]]:
        """Generate authorization URL with optional PKCE."""
        params = {
            'client_id': self.client_id,
            'redirect_uri': self.redirect_uri,
            'response_type': 'code',
            'state': state,
        }
        
        if scope:
            params['scope'] = scope
        
        code_verifier = None
        if pkce:
            code_verifier = self._generate_code_verifier()
            params['code_challenge'] = self._generate_code_challenge(code_verifier)
            params['code_challenge_method'] = 'S256'
        
        url = f"{self.authorization_url}?{urlencode(params)}"
        return url, code_verifier
    
    async def exchange_code(self, 
                            code: str,
                            code_verifier: str = None) -> TokenSet:
        """Exchange authorization code for tokens."""
        data = {
            'grant_type': 'authorization_code',
            'client_id': self.client_id,
            'client_secret': self.client_secret,
            'code': code,
            'redirect_uri': self.redirect_uri,
        }
        
        if code_verifier:
            data['code_verifier'] = code_verifier
        
        async with httpx.AsyncClient() as client:
            response = await client.post(self.token_url, data=data)
            response.raise_for_status()
            token_data = response.json()
        
        self._token = TokenSet(
            access_token=token_data['access_token'],
            refresh_token=token_data.get('refresh_token'),
            expires_at=time.time() + token_data['expires_in'],
            token_type=token_data.get('token_type', 'Bearer'),
            scope=token_data.get('scope'),
        )
        
        if self._store:
            await self._store.set(
                f"oauth:{self.client_id}",
                json.dumps({
                    'access_token': self._token.access_token,
                    'refresh_token': self._token.refresh_token,
                    'expires_at': self._token.expires_at,
                })
            )
        
        return self._token
    
    async def get_access_token(self) -> str:
        """Get valid access token, refreshing if necessary."""
        if self._token is None and self._store:
            # Load from store
            stored = await self._store.get(f"oauth:{self.client_id}")
            data = json.loads(stored)
            self._token = TokenSet(**data)
        
        if self._token is None:
            raise NotAuthenticated("No token available")
        
        if self._token.is_expired and self._token.refresh_token:
            await self._refresh_token()
        
        return self._token.access_token
    
    async def _refresh_token(self) -> None:
        """Refresh access token."""
        data = {
            'grant_type': 'refresh_token',
            'client_id': self.client_id,
            'client_secret': self.client_secret,
            'refresh_token': self._token.refresh_token,
        }
        
        async with httpx.AsyncClient() as client:
            response = await client.post(self.token_url, data=data)
            response.raise_for_status()
            token_data = response.json()
        
        self._token = TokenSet(
            access_token=token_data['access_token'],
            refresh_token=token_data.get('refresh_token', self._token.refresh_token),
            expires_at=time.time() + token_data['expires_in'],
            token_type=token_data.get('token_type', 'Bearer'),
            scope=token_data.get('scope', self._token.scope),
        )
        
        if self._store:
            await self._store.set(
                f"oauth:{self.client_id}",
                json.dumps({
                    'access_token': self._token.access_token,
                    'refresh_token': self._token.refresh_token,
                    'expires_at': self._token.expires_at,
                })
            )
```

---

## 9. Synthesis & Recommendations

### 9.1 Architectural Principles for PhenoLang

Based on this research, PhenoLang should adopt the following architectural stance:

| Principle | Rationale | Implementation |
|-----------|-----------|----------------|
| **Parser Combinators for Expression Languages** | Testable, composable grammar | `parsy` for inline parsers |
| **Lark for Configuration DSLs** | Declarative, readable | EBNF grammar files |
| **tree-sitter for Tooling** | IDE integration, incremental | External grammar, Python bindings |
| **Rich Domain Models** | Business logic encapsulation | Entities with behavior, not data bags |
| **Structured Concurrency** | Resource safety, cancellation | `asyncio.TaskGroup`, `asyncio.timeout` |
| **Vector-native** | AI-augmented features | pgvector for semantic search |
| **Immediate-mode TUI** | Responsive interfaces | Textual reactive components |
| **Railway-oriented error handling** | Composable error propagation | `Result[T, E]` types |
| **Code generation from specs** | Maintainability at scale | Jinja2 templates, OpenAPI |

### 9.2 Module Organization Strategy

```
PhenoLang/python/
├── pheno-core/              # Domain primitives, base classes
├── pheno-domain/            # Aggregates, entities, value objects
├── pheno-patterns/          # DDD patterns, repositories
├── pheno-async/             # Structured concurrency primitives
├── pheno-events/            # Event sourcing infrastructure
├── pheno-vector/            # Vector database, embeddings
├── pheno-parser/            # Combinator library, DSL parsers
├── pheno-ui/                # TUI components (Textual)
├── pheno-tui/               # Terminal applications
├── pheno-auth/              # OAuth, credential management
├── pheno-security/          # Encryption, secrets
├── pheno-credentials/       # Secret stores
├── pheno-exceptions/        # Error hierarchy
├── pheno-errors/            # Error handling utilities
├── pheno-logging/           # Structured logging
├── pheno-config/            # Configuration parsing
├── pheno-db/                # Database adapters
├── pheno-storage/           # File/object storage
├── pheno-caching/           # Cache implementations
├── pheno-ports/             # Port management
├── pheno-resources/         # Resource lifecycle
├── pheno-adapters/          # External API adapters
├── pheno-integration/       # Integration patterns
├── pheno-web/               # Web framework adapters
├── pheno-plugins/           # Plugin system
├── pheno-tools/             # CLI tools
├── pheno-dev/               # Development utilities
├── pheno-utilities/         # General utilities
└── pheno-application/       # Application layer orchestration
```

### 9.3 Technology Selection Matrix

| Concern | Primary Choice | Alternative | Rationale |
|---------|---------------|-------------|-----------|
| Parser (DSL) | Lark | pyparsing | EBNF clarity |
| Parser (inline) | parsy | funcparserlib | Simple, typed |
| IDE/parser | tree-sitter | - | Incremental, multi-lang |
| TUI | Textual | prompt-toolkit | Modern reactive |
| Async | asyncio + TaskGroup | trio | Standard library |
| Vector DB | pgvector | Pinecone | Postgres ecosystem |
| Embeddings | OpenAI | Local (sentence-transformers) | Quality vs. privacy |
| Repository | asyncpg | psycopg3 | Mature async |
| HTTP | httpx | aiohttp | HTTP/2, sync/async |
| Secrets | keyring | pass | OS integration |

---

## 10. References

### Language Design & Parsing
1. Aho, A.V., et al. (2006). *Compilers: Principles, Techniques, and Tools* (2nd ed.). Addison Wesley.
2. Ford, B. (2004). Parsing Expression Grammars: A Recognition-Based Syntactic Foundation. *POPL*.
3. Hutton, G., & Meijer, E. (1998). Monadic Parser Combinators. *JFP*.
4. Parr, T. (2013). *The Definitive ANTLR 4 Reference*. Pragmatic Bookshelf.

### Domain-Driven Design
5. Evans, E. (2003). *Domain-Driven Design*. Addison Wesley.
6. Vernon, V. (2016). *Domain-Driven Design Distilled*. Addison Wesley.
7. Millett, S., & Tune, N. (2015). *Patterns, Principles, and Practices of Domain-Driven Design*. Wrox.

### Concurrency & Async
8. Smith, N. (2018). Notes on Structured Concurrency. *njs blog*.
9. Sústrik, M. (2018). Structured Concurrency. *250bpm blog*.
10. Knuppel, B., et al. (2023). Structured Concurrency in Practice. *ASPLOS*.

### Vector Search & AI
11. Johnson, J., et al. (2019). Billion-Scale Similarity Search with GPUs. *IEEE TPAMI*.
12. Malkov, Y.A., & Yashunin, D.A. (2020). Efficient and Robust Approximate Nearest Neighbor Search Using Hierarchical Navigable Small World Graphs. *IEEE TPAMI*.
13. Reimers, N., & Gurevych, I. (2019). Sentence-BERT: Sentence Embeddings using Siamese BERT-Networks. *EMNLP*.

### Terminal Interfaces
14. Will McGugan. (2021). *Rich: Python library for rich text and beautiful formatting*.
15. Textualize.io. (2023). *Textual: Rapid Application Development for Python*.

### Error Handling
16. Railway Oriented Programming. *F# for Fun and Profit*.
17. Marceau, F., et al. (2011). Death, Taxes, and Incomplete Error Handling. *ICSE*.

### Security
18. OWASP. (2023). *OWASP Authentication Cheat Sheet*.
19. IETF. (2016). *RFC 7636: PKCE*.
20. IETF. (2012). *RFC 6749: OAuth 2.0*.

---

## Document Metadata

- **Classification:** Architecture Research
- **Scope:** 681 Python files, 33 modules
- **Audience:** Core architecture team, senior developers
- **Review Cycle:** Quarterly
- **Last Updated:** 2026-04-05
- **Version:** 1.0

### 10.10 Future Research Directions

- Quantum-resistant encryption for credential storage
- ML-based anomaly detection for deployment patterns
- Federated learning for cross-organization optimization
- WebAssembly System Interface (WASI) adoption
- eBPF-based observability integration

