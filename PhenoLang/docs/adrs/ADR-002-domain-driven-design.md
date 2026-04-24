# ADR-002: Domain-Driven Design with Pure Functional Cores

## Status
**ACCEPTED**

## Context

PhenoLang manages complex infrastructure orchestration with 33+ modules handling resources, deployments, services, and configurations. The current codebase shows signs of:

- **Anemic Domain Models**: Data classes with no behavior, logic scattered in services
- **Transaction Script Pattern**: Complex workflows as procedural code
- **Inconsistent State Management**: No clear boundaries for consistency
- **Implicit Business Rules**: Constraints embedded in multiple places

We need a cohesive approach to domain modeling that scales across 681 Python files.

## Decision

We will implement **Domain-Driven Design (DDD)** with **Functional Core, Imperative Shell** architecture.

### Decision Details

#### Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  UI Layer (pheno-tui, pheno-ui, CLI)                        │
│  ├─ View models, formatters                                 │
│  └─ Input validation (syntactic only)                       │
├─────────────────────────────────────────────────────────────┤
│  Application Layer (pheno-orchestration)                       │
│  ├─ Use case orchestration                                  │
│  ├─ Transaction boundaries                                   │
│  ├─ Cross-aggregate coordination                             │
│  └─ Event publishing                                         │
├─────────────────────────────────────────────────────────────┤
│  Domain Layer (pheno-domain, pheno-core) ★ HEART            │
│  ├─ Entities: Rich models with behavior                      │
│  ├─ Value Objects: Immutable, validated                    │
│  ├─ Domain Services: Cross-aggregate operations              │
│  ├─ Domain Events: Immutable facts                           │
│  └─ Aggregates: Consistency boundaries                       │
├─────────────────────────────────────────────────────────────┤
│  Infrastructure Layer (pheno-db, pheno-adapters)              │
│  ├─ Repository implementations                               │
│  ├─ External API clients                                     │
│  └─ Persistence mechanisms                                   │
└─────────────────────────────────────────────────────────────┘
```

#### Functional Core, Imperative Shell

Domain logic remains pure and testable; side effects isolated to the shell:

```python
# Domain Layer: Pure, no side effects, no I/O
class Deployment(AggregateRoot):
    """Pure domain logic. No database calls, no HTTP requests."""
    
    def __init__(self, id: DeploymentId, spec: DeploymentSpec):
        self._id = id
        self._spec = spec
        self._resources: Dict[ResourceId, Resource] = {}
        self._status = DeploymentStatus.DRAFT
        self._events: List[DomainEvent] = []
    
    def add_resource(self, resource: Resource) -> 'Deployment':
        """Return new Deployment instance (immutable)."""
        if self._status != DeploymentStatus.DRAFT:
            raise InvalidStateTransition("Cannot modify deployed deployment")
        
        new_resources = {**self._resources, resource.id: resource}
        new_events = [*self._events, ResourceAdded(self._id, resource.id)]
        
        # Return new instance with updated state
        new_deployment = Deployment(self._id, self._spec)
        new_deployment._resources = new_resources
        new_deployment._events = new_events
        return new_deployment
    
    def deploy(self, provider: Provider) -> 'Deployment':
        """Pure deployment logic."""
        if self._status != DeploymentStatus.DRAFT:
            raise InvalidStateTransition()
        
        # Validation: Pure function on domain state
        self._validate_dependencies()
        
        # Return deployment with updated status and events
        new_deployment = Deployment(self._id, self._spec)
        new_deployment._resources = self._resources
        new_deployment._status = DeploymentStatus.DEPLOYING
        new_deployment._events = [*self._events, DeploymentStarted(self._id)]
        return new_deployment

# Application Layer: Imperative shell, orchestrates side effects
class DeploymentService:
    """Coordinates domain operations with infrastructure."""
    
    def __init__(self, 
                 repository: DeploymentRepository,
                 event_bus: EventBus,
                 provider_factory: ProviderFactory):
        self._repository = repository
        self._event_bus = event_bus
        self._provider_factory = provider_factory
    
    async def deploy(self, deployment_id: DeploymentId) -> None:
        """Orchestrate deployment with side effects."""
        # Load (side effect: database)
        deployment = await self._repository.get(deployment_id)
        if not deployment:
            raise DeploymentNotFound(deployment_id)
        
        # Create provider (side effect: API client initialization)
        provider = self._provider_factory.create(deployment.spec.provider_config)
        
        # Pure domain operation
        updated_deployment = deployment.deploy(provider)
        
        # Persist (side effect: database)
        await self._repository.save(updated_deployment)
        
        # Publish events (side effect: message queue)
        for event in updated_deployment.uncommitted_events:
            await self._event_bus.publish(event)
        
        # External API calls (side effect: cloud provider)
        for resource in deployment.resources:
            await provider.provision(resource)
```

### Entity Design Standards

#### Rich Entity Pattern

```python
@dataclass
class Resource(Entity):
    """Resource entity with full lifecycle management."""
    
    # Identity
    _id: ResourceId = field(init=False)
    
    # Attributes
    _name: str
    _type: ResourceType
    _config: ResourceConfig
    _tags: Dict[str, str] = field(default_factory=dict)
    
    # State
    _status: ResourceStatus = field(default=ResourceStatus.PENDING)
    _health: HealthStatus = field(default=HealthStatus.UNKNOWN)
    
    # Metadata
    _created_at: datetime = field(default_factory=datetime.utcnow)
    _updated_at: Optional[datetime] = None
    _events: List[DomainEvent] = field(default_factory=list, repr=False)
    
    # Domain invariants
    def __post_init__(self):
        if not self._name or len(self._name) > 64:
            raise ValueError("Name must be 1-64 characters")
        
        self._validate_config_for_type()
    
    @property
    def id(self) -> ResourceId:
        return self._id
    
    @property
    def status(self) -> ResourceStatus:
        return self._status
    
    def provision(self, context: ProvisionContext) -> 'Resource':
        """Initiate provisioning. Returns new instance (immutable)."""
        if self._status != ResourceStatus.PENDING:
            raise InvalidStateTransition(
                f"Cannot provision from {self._status}"
            )
        
        # Validate with context
        context.validate_provision(self._config)
        
        # Return new instance with updated state
        new_resource = replace(self)
        new_resource._status = ResourceStatus.PROVISIONING
        new_resource._events = [*self._events, ResourceProvisioning(self._id)]
        return new_resource
    
    def mark_provisioned(self, endpoint: Endpoint) -> 'Resource':
        """Mark as successfully provisioned."""
        if self._status != ResourceStatus.PROVISIONING:
            raise InvalidStateTransition()
        
        new_resource = replace(self)
        new_resource._status = ResourceStatus.RUNNING
        new_resource._endpoint = endpoint
        new_resource._health = HealthStatus.HEALTHY
        new_resource._updated_at = datetime.utcnow()
        new_resource._events = [*self._events, ResourceProvisioned(self._id, endpoint)]
        return new_resource
    
    def _validate_config_for_type(self) -> None:
        """Validate configuration against resource type constraints."""
        validators = {
            ResourceType.DATABASE: self._validate_database_config,
            ResourceType.SERVICE: self._validate_service_config,
            ResourceType.BUCKET: self._validate_bucket_config,
        }
        
        validator = validators.get(self._type)
        if validator:
            validator()
```

#### Value Object Standards

```python
@dataclass(frozen=True)
class PortNumber:
    """Validated port number value object."""
    
    value: int
    
    def __post_init__(self):
        if not isinstance(self.value, int):
            raise TypeError("Port must be an integer")
        if not 1 <= self.value <= 65535:
            raise ValueError(f"Port must be 1-65535, got {self.value}")
    
    def is_privileged(self) -> bool:
        return self.value < 1024
    
    def is_ephemeral(self) -> bool:
        return self.value >= 49152
    
    def next_available(self) -> 'PortNumber':
        """Return next available port number."""
        if self.value >= 65535:
            raise ValueError("No ports available")
        return PortNumber(self.value + 1)
    
    def __str__(self) -> str:
        return str(self.value)
    
    def __int__(self) -> int:
        return self.value

@dataclass(frozen=True)
class ResourceId:
    """Strongly-typed resource identifier."""
    
    value: UUID
    prefix: str = "res"
    
    def __post_init__(self):
        if not isinstance(self.value, UUID):
            object.__setattr__(self, 'value', UUID(self.value))
    
    @classmethod
    def generate(cls) -> 'ResourceId':
        return cls(UUID.uuid4())
    
    @classmethod
    def parse(cls, s: str) -> 'ResourceId':
        """Parse from string like 'res-550e8400-e29b-41d4-a716-446655440000'."""
        if not s.startswith(f"{cls.prefix}-"):
            raise ValueError(f"Invalid ResourceId format: {s}")
        return cls(UUID(s[len(cls.prefix)+1:]))
    
    def __str__(self) -> str:
        return f"{self.prefix}-{self.value}"
```

### Aggregate Boundaries

```python
class Deployment(AggregateRoot):
    """
    Aggregate root for deployment operations.
    
    Invariants:
    - All resources must be in the same region OR explicitly allowed multi-region
    - Services must reference resources within this deployment
    - Configuration is immutable after deployment starts
    - At least one resource required for deployment
    """
    
    def __init__(self, id: DeploymentId, spec: DeploymentSpec):
        self._id = id
        self._spec = spec
        self._resources: Dict[ResourceId, Resource] = {}
        self._services: Dict[ServiceId, Service] = {}
        self._config: DeploymentConfig
        self._status = DeploymentStatus.DRAFT
        self._version = 0
        self._events: List[DomainEvent] = []
    
    def add_resource(self, resource: Resource) -> 'Deployment':
        """Add resource, maintaining aggregate invariants."""
        # Invariant: Region compatibility
        if self._resources and not self._spec.allow_multi_region:
            existing_region = next(iter(self._resources.values())).region
            if resource.region != existing_region:
                raise RegionMismatch(existing_region, resource.region)
        
        # Return new instance with resource added
        new_deployment = replace(self)
        new_deployment._resources = {**self._resources, resource.id: resource}
        new_deployment._events = [*self._events, ResourceAdded(self._id, resource.id)]
        return new_deployment
    
    def deploy(self) -> 'Deployment':
        """Execute deployment, enforcing all aggregate invariants."""
        # Invariant: At least one resource
        if not self._resources:
            raise EmptyDeploymentError()
        
        # Invariant: DRAFT status only
        if self._status != DeploymentStatus.DRAFT:
            raise InvalidDeploymentStatus(self._status)
        
        # Invariant: All dependencies resolved
        self._validate_dependencies()
        
        new_deployment = replace(self)
        new_deployment._status = DeploymentStatus.DEPLOYING
        new_deployment._version = self._version + 1
        new_deployment._events = [*self._events, DeploymentStarted(self._id)]
        return new_deployment
    
    def _validate_dependencies(self) -> None:
        """Validate resource dependency graph has no cycles."""
        graph = {rid: r.dependencies for rid, r in self._resources.items()}
        
        # Topological sort to detect cycles
        visited = set()
        temp_mark = set()
        
        def visit(node: ResourceId):
            if node in temp_mark:
                raise CyclicDependency(node)
            if node in visited:
                return
            
            temp_mark.add(node)
            for dep in graph.get(node, []):
                if dep not in self._resources:
                    raise MissingDependency(node, dep)
                visit(dep)
            temp_mark.remove(node)
            visited.add(node)
        
        for rid in graph:
            if rid not in visited:
                visit(rid)
```

## Consequences

### Positive
- **Testability**: Pure domain logic tested without mocks or infrastructure
- **Clarity**: Business rules co-located with domain concepts
- **Consistency**: Aggregate boundaries enforce transactional consistency
- **Evolution**: Changes to business rules localized to domain layer
- **Debugging**: Domain state changes explicit and traceable

### Negative
- **Boilerplate**: Immutable updates require explicit `replace()` calls
- **Learning Curve**: Team must understand DDD patterns and functional concepts
- **Performance**: Object allocation for every state change (usually acceptable)
- **ORM Complexity**: Immutable entities may require custom ORM mappings

### Mitigations

1. **Code Generation**: Use dataclass `replace()` and custom templates
2. **Documentation**: Comprehensive DDD pattern guide in `docs/patterns/ddd.md`
3. **Performance**: Profile before optimizing; consider mutable for hot paths
4. **ORM**: Custom SQLAlchemy hybrid properties or separate DTOs

## Implementation Plan

### Phase 1: Foundation (Current Sprint)
- [ ] Define base Entity, ValueObject, AggregateRoot classes
- [ ] Create exception hierarchy for domain errors
- [ ] Implement first rich entity (Resource)
- [ ] Add unit tests for pure domain logic

### Phase 2: Migration (Next 2 Sprints)
- [ ] Migrate pheno-domain models to rich entities
- [ ] Implement repository pattern for persistence
- [ ] Add domain event infrastructure
- [ ] Migrate workflows to application services

### Phase 3: Rollout (Following Month)
- [ ] Module-by-module entity enrichment
- [ ] Documentation and training
- [ ] Performance profiling and optimization

## Alternatives Considered

### Alternative 1: Anemic Domain Model (Current State)
- **Pros**: Simple ORM mapping, familiar to many developers
- **Cons**: Logic scattered, inconsistent state, hard to test
- **Verdict**: Rejected; explicit goal to move away from this

### Alternative 2: Event Sourcing Everywhere
- **Pros**: Complete audit trail, temporal queries
- **Cons**: Complexity overhead, not all aggregates need it
- **Verdict**: Partial; use selectively where audit is critical

### Alternative 3: CQRS (Command Query Responsibility Segregation)
- **Pros**: Optimized read models, scalable
- **Cons**: Eventual consistency complexity
- **Verdict**: Future consideration for query-heavy features

## Related Decisions
- ADR-001: Parser Combinator Architecture
- ADR-003: Structured Concurrency for Resource Management
- ADR-005: Code Generation from Specifications

## References
- Evans, E. (2003). *Domain-Driven Design*. Addison Wesley.
- Vernon, V. (2016). *Domain-Driven Design Distilled*. Addison Wesley.
- Millett, S. (2015). *Patterns, Principles, and Practices of DDD*. Wrox.
- Clean Architecture (Robert C. Martin)
- Functional Core, Imperative Shell (Gary Bernhardt)
