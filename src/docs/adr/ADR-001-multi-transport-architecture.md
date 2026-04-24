# ADR-001: Multi-Transport Architecture for MCP Servers

**Status:** Accepted  
**Date:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Reviewers:** Core Engineering Team

## Context

The Phenotype ecosystem requires a robust Model Context Protocol (MCP) server implementation that can operate across diverse deployment scenarios. The `src/Tools/DinoforgeMcp` package must support:

1. Local CLI integration (stdio)
2. Microservice deployment (HTTP)
3. Real-time applications (WebSocket)
4. Future extensibility for additional transports

The key question is whether to implement separate transports as standalone servers or design a unified transport abstraction layer.

## Decision

We will implement a **layered transport architecture** with three distinct layers:

```
┌─────────────────────────────────────────────────────────┐
│                  MCP APPLICATION                        │
│         (Tools, Resources, Prompts)                     │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                 PROTOCOL LAYER                          │
│        (JSON-RPC 2.0 Message Handling)                  │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                TRANSPORT ABSTRACTION                    │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  │
│  │  Stdio  │  │  HTTP   │  │   WS    │  │ Future  │  │
│  │Transport│  │Transport│  │Transport│  │Transport│  │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘  │
└─────────────────────────────────────────────────────────┘
```

**Key characteristics:**
1. **Common protocol handler:** JSON-RPC processing is transport-agnostic
2. **Pluggable transports:** Each transport implements a minimal interface
3. **No transport mixing:** Single transport per server instance
4. **Unified configuration:** Consistent initialization pattern across transports

## Consequences

### Positive

1. **Code reuse:** ~80% of server logic shared across transports
2. **Consistent behavior:** Same tool execution regardless of transport
3. **Simplified testing:** Protocol tests run once, transport tests isolated
4. **Type safety:** Shared Pydantic models enforce consistency
5. **Operational simplicity:** Clear transport-specific deployment patterns

### Negative

1. **HTTP/2 complexity:** No native HTTP/2 support in initial implementation
2. **WebSocket state:** Stateful connections require connection management
3. **Feature parity:** Some features (streaming) may vary by transport
4. **Documentation burden:** Multiple transport configurations to maintain

### Neutral

1. **Performance:** Stdio remains fastest; HTTP adds serialization overhead
2. **Scalability:** HTTP enables horizontal scaling; stdio is process-bound
3. **Security:** Each transport has distinct authentication requirements

## Implementation

```python
# Transport interface contract
class Transport(ABC):
    @abstractmethod
    async def read_message(self) -> JSONRPCMessage:
        """Read a single JSON-RPC message."""
        pass
    
    @abstractmethod
    async def write_message(self, message: JSONRPCMessage) -> None:
        """Write a single JSON-RPC message."""
        pass
    
    @abstractmethod
    async def close(self) -> None:
        """Clean shutdown of transport."""
        pass

# Protocol layer (transport-agnostic)
class ProtocolHandler:
    def __init__(self, transport: Transport, server: MCPServer):
        self.transport = transport
        self.server = server
    
    async def run(self):
        while True:
            message = await self.transport.read_message()
            response = await self.server.handle(message)
            await self.transport.write_message(response)
```

## Alternatives Considered

### Alternative A: Single-Transport Design
Implement only stdio transport as primary use case.

**Rejected:** HTTP and WebSocket required for heliosApp integration and remote tool access.

### Alternative B: Separate Server Implementations
Create `DinoforgeMcpStdio`, `DinoforgeMcpHttp`, `DinoforgeMcpWs` as separate packages.

**Rejected:** Code duplication violates DRY principle; protocol changes require N updates.

### Alternative C: gRPC-First Architecture
Use gRPC as primary protocol with JSON-RPC gateway.

**Rejected:** MCP specification mandates JSON-RPC 2.0; gRPC adds complexity without benefit.

## Related Decisions

- ADR-002: Tool Registry Pattern
- ADR-003: Error Handling Strategy
- SPEC.md: Transport Architecture Section

## References

1. [MCP Transport Specification](https://modelcontextprotocol.io/specification)
2. [nanovms/nanos Transport Design](https://github.com/nanovms/nanos/wiki/Architecture)
3. [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)

---

*Decision record maintained under src/docs/adr/*
