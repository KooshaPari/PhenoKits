# ADR-002: Dynamic Tool Registry with Hot-Reload

**Status:** Accepted  
**Date:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Reviewers:** Core Engineering Team

## Context

The DinoforgeMcp server must manage tool definitions and their execution handlers efficiently. Key requirements:

1. **Dynamic registration:** Tools added/removed without server restart
2. **Category organization:** Logical grouping for tool discovery
3. **Middleware support:** Pre/post execution hooks
4. **Change notifications:** Clients notified of tool updates
5. **Performance:** O(1) tool lookup, minimal registration overhead

The primary decision is between a static tool map populated at startup versus a dynamic registry with lifecycle management.

## Decision

We will implement a **dynamic tool registry** with the following architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                     TOOL REGISTRY                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ┌─────────────┐      ┌─────────────┐      ┌───────────┐  │
│   │  Register   │─────►│  Tool Store │◄─────│  Execute  │  │
│   │   Handler   │      │  (O(1) map) │      │  Handler  │  │
│   └─────────────┘      └──────┬──────┘      └───────────┘  │
│                               │                             │
│                               ▼                             │
│                      ┌─────────────────┐                    │
│                      │  Category Index │                    │
│                      │   (secondary)   │                    │
│                      └────────┬────────┘                    │
│                               │                             │
│                               ▼                             │
│                      ┌─────────────────┐                    │
│                      │  Change Hooks   │──► Notify clients │
│                      └─────────────────┘                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Core Components:**

1. **Tool Store:** Dictionary-based storage for O(1) lookup by name
2. **Category Index:** Inverted index for category-based queries
3. **Middleware Chain:** Pluggable pre/post execution processing
4. **Hook System:** Async notifications on registration changes

## Consequences

### Positive

1. **Runtime flexibility:** Tools added without restart during development
2. **Plugin architecture:** Third-party tools register dynamically
3. **A/B testing:** Tool versions swapped based on context
4. **Operational agility:** Emergency tool disable without redeploy
5. **Clean middleware:** Cross-cutting concerns (logging, auth) separated

### Negative

1. **Thread safety:** Concurrent access requires careful locking
2. **Consistency:** Tool cache on clients may be stale
3. **Complexity:** More moving parts than static configuration
4. **Testing:** Dynamic state harder to reproduce than static

### Mitigations

| Risk | Mitigation |
|------|------------|
| Race conditions | asyncio.Lock for all mutations |
| Stale cache | Change notifications + cache TTL |
| State complexity | Immutable tool definitions, copy-on-write |
| Test flakiness | Fixture-based registry reset between tests |

## Implementation

```python
class ToolRegistry:
    """Dynamic tool registry with hot-reload support."""
    
    def __init__(self):
        self._tools: Dict[str, RegisteredTool] = {}
        self._categories: Dict[str, Set[str]] = defaultdict(set)
        self._hooks: List[Callable[[str, Tool], Awaitable[None]]] = []
        self._lock = asyncio.Lock()
    
    async def register(
        self,
        tool: Tool,
        handler: Callable,
        category: str = "general",
        middleware: Optional[List[Callable]] = None
    ) -> None:
        """Register a tool atomically."""
        async with self._lock:
            # Store immutable copy
            frozen_tool = tool.model_copy(deep=True)
            registered = RegisteredTool(
                tool=frozen_tool,
                handler=handler,
                middleware=tuple(middleware or []),
                registered_at=datetime.utcnow()
            )
            
            self._tools[tool.name] = registered
            self._categories[category].add(tool.name)
        
        # Notify outside lock
        await self._notify("registered", frozen_tool)
    
    async def unregister(self, tool_name: str) -> bool:
        """Remove a tool from registry."""
        async with self._lock:
            if tool_name not in self._tools:
                return False
            
            tool = self._tools[tool_name].tool
            del self._tools[tool_name]
            
            # Cleanup category index
            for category, tools in list(self._categories.items()):
                tools.discard(tool_name)
                if not tools:
                    del self._categories[category]
        
        await self._notify("unregistered", tool)
        return True
    
    async def execute(
        self,
        tool_name: str,
        arguments: dict,
        context: ExecutionContext
    ) -> ToolResult:
        """Execute tool through middleware chain."""
        # Read-only access, no lock needed
        registered = self._tools.get(tool_name)
        if not registered:
            raise ToolNotFoundError(tool_name)
        
        # Build and execute middleware chain
        chain = self._build_chain(registered.middleware, registered.handler)
        return await chain(arguments, context)
    
    def _build_chain(self, middleware: Tuple[Callable, ...], handler: Callable):
        """Compose middleware into execution chain."""
        async def chain(args: dict, ctx: ExecutionContext) -> ToolResult:
            # Pre-processing
            for mw in middleware:
                args, ctx = await mw.pre_process(args, ctx)
            
            # Execution
            result = await handler(args, ctx)
            
            # Post-processing
            for mw in reversed(middleware):
                result = await mw.post_process(result, ctx)
            
            return result
        
        return chain
```

## Middleware Interface

```python
class ToolMiddleware(ABC):
    """Base class for tool middleware."""
    
    async def pre_process(
        self,
        arguments: dict,
        context: ExecutionContext
    ) -> Tuple[dict, ExecutionContext]:
        """Transform arguments before execution."""
        return arguments, context
    
    async def post_process(
        self,
        result: ToolResult,
        context: ExecutionContext
    ) -> ToolResult:
        """Transform result after execution."""
        return result

# Example: Logging Middleware
class LoggingMiddleware(ToolMiddleware):
    async def pre_process(self, arguments, context):
        context.logger.info(
            f"Executing {context.tool_name}",
            extra={"arguments_keys": list(arguments.keys())}
        )
        return arguments, context
    
    async def post_process(self, result, context):
        context.logger.info(
            f"Completed {context.tool_name}",
            extra={"is_error": result.is_error}
        )
        return result
```

## Performance Targets

| Operation | Target Latency | Max Latency |
|-----------|---------------|-------------|
| Register | < 1ms | 5ms |
| Unregister | < 1ms | 5ms |
| Lookup | < 0.1ms | 0.5ms |
| Execute (no middleware) | < 0.5ms | 2ms |
| Execute (3 middleware) | < 1ms | 5ms |

## Alternatives Considered

### Alternative A: Static Tool Map
Tools defined in configuration file, loaded at startup.

**Rejected:** Does not support dynamic plugin loading or hot-reload for development.

### Alternative B: Database-Backed Registry
Tool definitions stored in PostgreSQL/Redis.

**Rejected:** Overkill for single-server deployment; adds operational complexity.

### Alternative C: File Watcher
Watch filesystem for tool definition changes.

**Rejected:** File watching unreliable across platforms; explicit API preferred.

## Related Decisions

- ADR-001: Multi-Transport Architecture
- ADR-003: Error Handling Strategy
- SOTA Research: Tool Registry Pattern Section

## References

1. [MCP Tools Specification](https://modelcontextprotocol.io/specification)
2. [Python asyncio Lock Patterns](https://docs.python.org/3/library/asyncio-sync.html)
3. [Middleware Pattern in Web Frameworks](https://fastapi.tiangolo.com/tutorial/middleware/)

---

*Decision record maintained under src/docs/adr/*
