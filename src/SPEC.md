# SPEC.md — src/

**Version:** 1.0  
**Status:** Draft  
**Last Updated:** 2026-04-04  
**Maintainer:** Phenotype Core Team

---

## Table of Contents

1. [Mission](#1-mission)
2. [Tenets](#2-tenets)
3. [Overview](#3-overview)
4. [Architecture](#4-architecture)
5. [Components](#5-components)
6. [Transport Layer](#6-transport-layer)
7. [Protocol Implementation](#7-protocol-implementation)
8. [Tool System](#8-tool-system)
9. [Resource System](#9-resource-system)
10. [Security Model](#10-security-model)
11. [Error Handling](#11-error-handling)
12. [Performance](#12-performance)
13. [Configuration](#13-configuration)
14. [Testing](#14-testing)
15. [Integration Points](#15-integration-points)
16. [Deployment](#16-deployment)
17. [Observability](#17-observability)
18. [Roadmap](#18-roadmap)
19. [References](#19-references)

---

## 1. Mission

The `src/` directory provides foundational infrastructure for AI-native tooling across the Phenotype ecosystem. It implements the Model Context Protocol (MCP) specification, enabling secure, standardized integration between AI systems and external tools, data sources, and services.

**Primary Goals:**

1. **Protocol Compliance:** Full implementation of the MCP specification
2. **Transport Flexibility:** Support for stdio, HTTP, and WebSocket transports
3. **Developer Experience:** Simple API for tool registration and execution
4. **Production Readiness:** Observability, security, and performance at scale
5. **Ecosystem Integration:** Seamless connection with heliosApp, agent-wave, and phenotype-cli

---

## 2. Tenets

Unless you know better ones, these tenets guide development:

### 2.1 Simplicity

Keep the core small and focused. Each component should have a single, well-defined responsibility. Prefer composition over inheritance. Avoid premature abstraction.

**Implications:**
- Transport implementations share common protocol logic
- Tool registry uses plain dictionaries, not complex data structures
- Configuration is explicit, not magical

### 2.2 Performance

Optimize for the common case. Stdio transport should add <1ms overhead. HTTP transport should handle >1000 req/s per core.

**Implications:**
- Zero-copy where possible
- Lazy initialization of heavy dependencies
- Connection pooling for HTTP clients

### 2.3 Security

Never expose internal error details to clients. All inputs validated against schemas. Authentication required for network transports.

**Implications:**
- Three-layer error transformation
- PII automatic redaction
- Capability-based access control

### 2.4 Observability

Every operation must be traceable. Distributed tracing IDs propagate through all layers. Structured logging for all significant events.

**Implications:**
- Trace ID generation at entry points
- Context propagation through middleware
- Metrics export in Prometheus format

### 2.5 Compatibility

Follow the MCP specification exactly. No protocol extensions without specification updates. Version negotiation on initialization.

**Implications:**
- Strict JSON-RPC 2.0 compliance
- Capability negotiation required
- Backward compatibility for protocol versions

---

## 3. Overview

### 3.1 Purpose

The `src/` directory contains:

- **DinoforgeMcp:** Production-ready MCP server implementation
- **Shared Libraries:** Reusable components for the Phenotype ecosystem
- **Transport Layer:** Multi-transport communication infrastructure
- **Tool Registry:** Dynamic tool management system

### 3.2 Scope

**In Scope:**
- MCP protocol implementation (stdio, HTTP, WebSocket)
- Tool registration and execution
- Resource management
- Error handling and logging
- Configuration management

**Out of Scope:**
- LLM inference (handled by agent-wave)
- Persistent storage (handled by external services)
- Authentication provider (delegates to external systems)

### 3.3 ASCII Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              src/ DIRECTORY                                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                           API SURFACE                                        │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │ │
│  │  │   heliosApp  │  │  agent-wave  │  │ phenotype-cli│  │    Other     │    │ │
│  │  │   (HTTP)     │  │   (MCP)      │  │   (stdio)    │  │   Clients    │    │ │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘    │ │
│  │         │                 │                 │                 │             │ │
│  └─────────┼─────────────────┼─────────────────┼─────────────────┼─────────────┘ │
│            │                 │                 │                 │               │
│  ┌─────────┴─────────────────┴─────────────────┴─────────────────┴─────────────┐  │
│  │                         TRANSPORT LAYER                                    │  │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────────┐ │  │
│  │  │     Stdio       │  │      HTTP       │  │         WebSocket           │ │  │
│  │  │   Transport     │  │   Transport     │  │        Transport            │ │  │
│  │  │                 │  │                 │  │                             │ │  │
│  │  │ • Subprocess    │  │ • REST API      │  │ • Full-duplex               │ │  │
│  │  │ • Local CLI     │  │ • Load balance  │  │ • Real-time                 │ │  │
│  │  │ • Simple        │  │ • Scalable      │  │ • Streaming                 │ │  │
│  │  └────────┬────────┘  └────────┬────────┘  └─────────────┬───────────────┘ │ │  │
│  │           │                    │                         │                 │ │  │
│  └───────────┴────────────────────┴─────────────────────────┴─────────────────┘ │  │
│                            │                                                    │
│  ┌─────────────────────────▼─────────────────────────────────────────────────┐  │
│  │                      PROTOCOL LAYER                                        │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐  │  │
│  │  │                    JSON-RPC 2.0 Handler                              │  │  │
│  │  │                                                                     │  │  │
│  │  │  • Message parsing/serialization                                     │  │  │
│  │  │  • Request/Response correlation                                       │  │  │
│  │  │  • Batch request support                                            │  │  │
│  │  │  • Error code standardization                                       │  │  │
│  │  └─────────────────────────────────────────────────────────────────────┘  │  │
│  └─────────────────────────┬─────────────────────────────────────────────────┘  │
│                            │                                                    │
│  ┌─────────────────────────▼─────────────────────────────────────────────────┐  │
│  │                     APPLICATION LAYER                                      │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐  │  │
│  │  │                      MCP Server Core                                 │  │  │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐ │  │  │
│  │  │  │   Tools     │  │  Resources  │  │   Prompts   │  │  Sampling  │ │  │  │
│  │  │  │  Registry   │  │   Manager   │  │   Handler   │  │   Handler  │ │  │  │
│  │  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └─────┬──────┘ │  │  │
│  │  │         │                │                │                │        │  │  │
│  │  │         └────────────────┴────────────────┴────────────────┘        │  │  │
│  │  │                              │                                      │  │  │
│  │  │                    ┌─────────▼──────────┐                          │  │  │
│  │  │                    │  Tool Execution    │                          │  │  │
│  │  │                    │  Engine            │                          │  │  │
│  │  │                    │                    │                          │  │  │
│  │  │                    │ • Middleware chain │                          │  │  │
│  │  │                    │ • Error handling   │                          │  │  │
│  │  │                    │ • Context passing  │                          │  │  │
│  │  │                    └────────────────────┘                          │  │  │
│  │  └─────────────────────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                     SHARED LIBRARIES                                     │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │   │
│  │  │  Config  │  │  Logging │  │  Errors  │  │  Schema  │  │  Utils   │  │   │
│  │  │  Manager │  │  Context │  │  Types   │  │  Validator│  │  Helpers │  │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                   │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 3.4 Directory Structure

```
src/
├── README.md                      # High-level overview
├── SPEC.md                        # This specification
├── PLAN.md                        # Implementation roadmap
├── CHARTER.md                     # Project charter (tenets, mission)
├── ROADMAP.md                     # Feature roadmap
│
├── Tools/                         # MCP server implementations
│   └── DinoforgeMcp/              # Primary MCP server
│       ├── dinoforge_mcp/         # Package source
│       │   ├── __init__.py        # Package exports
│       │   ├── server.py          # MCP server core
│       │   ├── transport/         # Transport implementations
│       │   │   ├── __init__.py
│       │   │   ├── stdio.py       # Stdio transport
│       │   │   ├── http.py        # HTTP transport
│       │   │   └── websocket.py   # WebSocket transport
│       │   ├── protocol/          # Protocol layer
│       │   │   ├── __init__.py
│       │   │   ├── handler.py     # JSON-RPC handler
│       │   │   ├── messages.py    # Message types
│       │   │   └── errors.py      # Error definitions
│       │   ├── tools/             # Tool system
│       │   │   ├── __init__.py
│       │   │   ├── registry.py    # Tool registry
│       │   │   ├── executor.py    # Tool executor
│       │   │   └── middleware.py  # Middleware system
│       │   ├── resources/         # Resource system
│       │   │   ├── __init__.py
│       │   │   ├── manager.py     # Resource manager
│       │   │   └── providers.py   # Resource providers
│       │   ├── config/            # Configuration
│       │   │   ├── __init__.py
│       │   │   ├── manager.py     # Config manager
│       │   │   └── schema.py      # Config schema
│       │   ├── logging/           # Logging utilities
│       │   │   ├── __init__.py
│       │   │   ├── context.py     # Log context
│       │   │   └── formatters.py  # Log formatters
│       │   └── types/             # Shared types
│       │       ├── __init__.py
│       │       ├── common.py      # Common types
│       │       └── errors.py      # Error types
│       ├── tests/                 # Test suite
│       │   ├── unit/              # Unit tests
│       │   ├── integration/       # Integration tests
│       │   └── protocol/            # Protocol compliance tests
│       ├── pyproject.toml         # Package config
│       └── setup.py               # Legacy setup
│
├── SharedLibs/                    # Shared libraries (planned)
│   ├── Config/                    # Unified configuration
│   ├── Logging/                   # Structured logging
│   ├── Errors/                    # Error handling
│   └── Utils/                     # Common utilities
│
└── docs/                          # Documentation
    ├── adr/                       # Architecture Decision Records
    │   ├── ADR-001-multi-transport-architecture.md
    │   ├── ADR-002-dynamic-tool-registry.md
    │   └── ADR-003-structured-error-handling.md
    ├── research/                  # Research documents
    │   └── SOTA-MCP-Transport-Systems.md
    └── examples/                  # Usage examples
```

---

## 4. Architecture

### 4.1 Layered Architecture

The implementation follows a strict layered architecture:

| Layer | Responsibility | Components |
|-------|---------------|------------|
| **Application** | MCP features | Tools, Resources, Prompts, Sampling |
| **Protocol** | Message handling | JSON-RPC parser, request router |
| **Transport** | Communication | Stdio, HTTP, WebSocket adapters |
| **Infrastructure** | Cross-cutting | Config, Logging, Security |

**Layer Rules:**
1. Dependencies only flow downward (Application → Protocol → Transport)
2. No layer skipping (Application cannot call Transport directly)
3. Interface contracts define layer boundaries

### 4.2 Component Diagram

```
┌────────────────────────────────────────────────────────────────┐
│                     APPLICATION LAYER                           │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────────┐  │
│  │   Server    │───►│   Router    │◄───│  Capability Manager │  │
│  │   Core      │    │             │    │                     │  │
│  │             │    │ Routes to   │    │ • Tool registry     │  │
│  │ • Init      │    │ appropriate │    │ • Resource index    │  │
│  │ • Shutdown  │    │ handler     │    │ • Prompt store      │  │
│  │ • Health    │    │             │    │ • Sampling API      │  │
│  └─────────────┘    └──────┬──────┘    └─────────────────────┘  │
│                            │                                   │
│              ┌─────────────┼─────────────┐                   │
│              ▼             ▼             ▼                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Tool        │  │ Resource    │  │  Prompt             │  │
│  │ Handler     │  │ Handler     │  │  Handler            │  │
│  │             │  │             │  │                     │  │
│  │ • Register  │  │ • Read      │  │ • Get template      │  │
│  │ • List      │  │ • List      │  │ • List              │  │
│  │ • Call      │  │ • Subscribe │  │                     │  │
│  └──────┬──────┘  └─────────────┘  └─────────────────────┘  │
│         │                                                      │
└─────────┼──────────────────────────────────────────────────────┘
          │
┌─────────▼──────────────────────────────────────────────────────┐
│                     PROTOCOL LAYER                              │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │              JSON-RPC 2.0 Processor                      │  │
│  │                                                         │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │  │
│  │  │  Parser     │  │  Router     │  │  Serializer │     │  │
│  │  │             │  │             │  │             │     │  │
│  │  │ JSON → Obj  │  │ Route to    │  │ Obj → JSON  │     │  │
│  │  │ Validate    │  │ handler     │  │ Format      │     │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘     │  │
│  │                                                         │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
          │
┌─────────▼──────────────────────────────────────────────────────┐
│                     TRANSPORT LAYER                               │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │                   Transport Abstraction                    │ │
│  │                                                          │ │
│  │   Interface:                                             │ │
│  │   • read_message() → JSONRPCMessage                       │ │
│  │   • write_message(message)                              │ │
│  │   • close()                                             │ │
│  │                                                          │ │
│  └──────────────────────────┬────────────────────────────────┘ │
│                             │                                │
│       ┌─────────────────────┼─────────────────────┐           │
│       ▼                     ▼                     ▼           │
│  ┌─────────┐           ┌─────────┐           ┌─────────┐     │
│  │  Stdio  │           │   HTTP  │           │   WS    │     │
│  │Adapter  │           │Adapter  │           │Adapter  │     │
│  │         │           │         │           │         │     │
│  │stdin/   │           │Request/ │           │Frame    │     │
│  │stdout   │           │Response │           │handling │     │
│  └─────────┘           └─────────┘           └─────────┘     │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

### 4.3 Data Flow

**Request Lifecycle:**
```
1. Client sends request
   ↓
2. Transport layer receives raw bytes
   ↓
3. Protocol layer parses JSON-RPC message
   ↓
4. Router identifies handler (tool/resource/prompt)
   ↓
5. Application layer validates and executes
   ↓
6. Response generated
   ↓
7. Protocol layer serializes to JSON
   ↓
8. Transport layer sends to client
```

---

## 5. Components

### 5.1 Component Table

| Component | Path | Purpose | Language | Status | Priority |
|-----------|------|---------|----------|--------|----------|
| DinoforgeMcp | `Tools/DinoforgeMcp/` | MCP server package | Python | Beta | P0 |
| MCP Server Core | `dinoforge_mcp/server.py` | Main server orchestration | Python | Beta | P0 |
| Stdio Transport | `transport/stdio.py` | Local CLI transport | Python | Beta | P0 |
| HTTP Transport | `transport/http.py` | REST API transport | Python | Planned | P1 |
| WebSocket Transport | `transport/websocket.py` | Real-time transport | Python | Planned | P1 |
| Protocol Handler | `protocol/handler.py` | JSON-RPC processing | Python | Beta | P0 |
| Tool Registry | `tools/registry.py` | Dynamic tool management | Python | Beta | P0 |
| Tool Executor | `tools/executor.py` | Tool execution engine | Python | Beta | P0 |
| Resource Manager | `resources/manager.py` | Resource lifecycle | Python | Planned | P2 |
| Config Manager | `config/manager.py` | Configuration management | Python | Planned | P2 |
| Logging Context | `logging/context.py` | Structured logging | Python | Beta | P1 |

### 5.2 Component Details

#### 5.2.1 MCP Server Core

**Responsibilities:**
- Server lifecycle management (init, run, shutdown)
- Capability negotiation
- Health checking
- Graceful degradation

**Interface:**
```python
class MCPServer:
    """Core MCP server implementation."""
    
    def __init__(
        self,
        name: str,
        version: str,
        capabilities: ServerCapabilities
    ):
        self.name = name
        self.version = version
        self.capabilities = capabilities
        self.initialized = False
    
    async def initialize(
        self,
        client_capabilities: ClientCapabilities
    ) -> InitializeResult:
        """Perform protocol initialization."""
        pass
    
    async def run(self, transport: Transport) -> None:
        """Run server with specified transport."""
        pass
    
    async def shutdown(self) -> None:
        """Graceful shutdown."""
        pass
```

#### 5.2.2 Transport Layer

**Abstract Transport Interface:**
```python
class Transport(ABC):
    """Abstract base for all transports."""
    
    @abstractmethod
    async def read_message(self) -> JSONRPCMessage:
        """Read a single JSON-RPC message.
        
        Returns:
            Parsed JSON-RPC message object
            
        Raises:
            TransportError: If transport fails
            EOFError: If connection closed
        """
        pass
    
    @abstractmethod
    async def write_message(self, message: JSONRPCMessage) -> None:
        """Write a single JSON-RPC message.
        
        Args:
            message: JSON-RPC message to send
            
        Raises:
            TransportError: If write fails
        """
        pass
    
    @abstractmethod
    async def close(self) -> None:
        """Close transport connection gracefully."""
        pass
```

#### 5.2.3 Tool Registry

**Responsibilities:**
- Tool registration and unregistration
- Tool discovery and listing
- Category-based organization
- Change notifications

**Implementation:**
```python
class ToolRegistry:
    """Dynamic tool registry with hot-reload support."""
    
    def __init__(self):
        self._tools: Dict[str, RegisteredTool] = {}
        self._categories: Dict[str, Set[str]] = defaultdict(set)
        self._hooks: List[Callable] = []
        self._lock = asyncio.Lock()
    
    async def register(
        self,
        tool: Tool,
        handler: Callable,
        category: str = "general",
        middleware: Optional[List[Callable]] = None
    ) -> None:
        """Register a tool atomically."""
        pass
    
    async def unregister(self, tool_name: str) -> bool:
        """Remove a tool from registry."""
        pass
    
    async def list_tools(self, category: Optional[str] = None) -> List[Tool]:
        """List all tools or tools in category."""
        pass
    
    def on_change(self, hook: Callable[[str, Tool], Awaitable[None]]) -> None:
        """Register change notification hook."""
        pass
```

---

## 6. Transport Layer

### 6.1 Transport Comparison

| Transport | Latency | Throughput | Complexity | Stateful | Best For |
|-----------|---------|------------|------------|----------|----------|
| **Stdio** | <1ms | 1K req/s | Low | No | Local CLI integration |
| **HTTP/1.1** | 5-20ms | 100 req/s | Low | No | Simple REST APIs |
| **HTTP/2** | 5-15ms | 10K req/s | Medium | No | Microservices |
| **WebSocket** | 3-10ms | 50K req/s | Medium | Yes | Real-time applications |

### 6.2 Stdio Transport

**Design:**
- Reads from `stdin`, writes to `stdout`
- Line-delimited JSON messages
- Process lifetime equals connection lifetime
- No authentication (relies on OS process isolation)

**Implementation:**
```python
class StdioTransport(Transport):
    """MCP transport over standard input/output."""
    
    def __init__(
        self,
        reader: Optional[asyncio.StreamReader] = None,
        writer: Optional[asyncio.StreamWriter] = None
    ):
        self.reader = reader or asyncio.StreamReader()
        self.writer = writer or sys.stdout
        self._closed = False
    
    async def read_message(self) -> JSONRPCMessage:
        """Read line from stdin and parse as JSON."""
        if self._closed:
            raise TransportError("Transport closed")
        
        try:
            line = await self.reader.readline()
            if not line:
                raise EOFError("Connection closed by peer")
            
            return JSONRPCMessage.parse(line.decode('utf-8'))
        except json.JSONDecodeError as e:
            raise ProtocolError(f"Invalid JSON: {e}")
    
    async def write_message(self, message: JSONRPCMessage) -> None:
        """Serialize message to JSON and write to stdout."""
        if self._closed:
            raise TransportError("Transport closed")
        
        json_line = json.dumps(message.to_dict(), separators=(',', ':'))
        self.writer.write(json_line.encode('utf-8') + b'\n')
        await self.writer.drain()
    
    async def close(self) -> None:
        """Close transport (no-op for stdio)."""
        self._closed = True
```

### 6.3 HTTP Transport

**Endpoint Design:**

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/mcp/v1/initialize` | POST | Protocol initialization |
| `/mcp/v1/message` | POST | Send JSON-RPC message |
| `/mcp/v1/tools` | GET | List available tools |
| `/mcp/v1/tools/{name}` | GET | Get tool details |
| `/mcp/v1/resources` | GET | List resources |
| `/mcp/v1/stream` | GET (SSE) | Subscribe to events |

**Request Format:**
```http
POST /mcp/v1/message HTTP/1.1
Host: api.example.com
Content-Type: application/json
Authorization: Bearer <token>
X-MCP-Version: 2024-11-05

{
  "jsonrpc": "2.0",
  "id": "req-123",
  "method": "tools/call",
  "params": {
    "name": "search",
    "arguments": {"query": "python"}
  }
}
```

### 6.4 WebSocket Transport

**Frame Types:**
- Text frames: JSON-RPC messages
- Binary frames: Large resource content
- Ping/Pong: Keep-alive

**Connection Lifecycle:**
```python
class WebSocketTransport(Transport):
    """MCP transport over WebSocket."""
    
    def __init__(self, websocket: WebSocket, max_queue: int = 100):
        self.ws = websocket
        self.send_queue = asyncio.Queue(maxsize=max_queue)
        self._closed = False
        self._tasks: Set[asyncio.Task] = set()
    
    async def run(self) -> None:
        """Start bidirectional communication."""
        # Spawn sender and receiver tasks
        self._tasks.add(asyncio.create_task(self._sender_loop()))
        self._tasks.add(asyncio.create_task(self._receiver_loop()))
        
        # Wait for completion
        await asyncio.gather(*self._tasks, return_exceptions=True)
    
    async def _sender_loop(self) -> None:
        """Dedicated sender coroutine with backpressure."""
        while not self._closed:
            try:
                message = await asyncio.wait_for(
                    self.send_queue.get(),
                    timeout=1.0
                )
                await self.ws.send(json.dumps(message.to_dict()))
            except asyncio.TimeoutError:
                # Send ping to keep connection alive
                await self.ws.ping()
    
    async def _receiver_loop(self) -> None:
        """Dedicated receiver coroutine."""
        async for message in self.ws:
            if message.type == WSMsgType.TEXT:
                yield JSONRPCMessage.parse(message.data)
            elif message.type == WSMsgType.ERROR:
                raise TransportError(f"WebSocket error: {message.data}")
```

### 6.5 Transport Selection Decision Tree

```
                        Deployment Context
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
    Local Only            Networked              Cloud
        │                     │                     │
    ┌───┴───┐           ┌─────┴─────┐         ┌─────┴─────┐
    │       │           │           │         │           │
 Simple  Complex     Internal    External   Serverless  Dedicated
    │       │           │           │         │           │
    ▼       ▼           ▼           ▼         ▼           ▼
  Stdio   Unix        HTTP/1.1    HTTP/2    HTTP/2      WebSocket
          Socket      + SSE       + mTLS    (stateless) (stateful)
                                           
        Latency Req?              Real-time?
            │                        │
    ┌───────┴───────┐          ┌─────┴─────┐
    │               │          │           │
   <10ms          >10ms     Required    Optional
    │               │          │           │
    ▼               ▼          ▼           ▼
 Unix/Stdio      HTTP      WebSocket    HTTP/2
 (local)       (any)       + gRPC       (efficient)
```

---

## 7. Protocol Implementation

### 7.1 JSON-RPC 2.0 Compliance

**Required Features:**
- Request/Response correlation by ID
- Notification support (no ID required)
- Batch request processing
- Error object standardization

**Message Types:**

```python
class JSONRPCMessage(BaseModel):
    """Base JSON-RPC message."""
    jsonrpc: Literal["2.0"] = "2.0"
    id: Optional[Union[str, int]] = None

class JSONRPCRequest(JSONRPCMessage):
    """JSON-RPC request message."""
    method: str
    params: Optional[Union[dict, list]] = None

class JSONRPCResponse(JSONRPCMessage):
    """JSON-RPC response message."""
    result: Optional[Any] = None
    error: Optional[JSONRPCError] = None

class JSONRPCError(BaseModel):
    """JSON-RPC error object."""
    code: int
    message: str
    data: Optional[Any] = None
```

### 7.2 MCP Protocol Messages

**Initialize:**
```python
class InitializeRequest(JSONRPCRequest):
    """Client initialization request."""
    method: Literal["initialize"] = "initialize"
    params: InitializeParams

class InitializeParams(BaseModel):
    protocolVersion: str
    capabilities: ClientCapabilities
    clientInfo: Implementation

class InitializeResult(BaseModel):
    protocolVersion: str
    capabilities: ServerCapabilities
    serverInfo: Implementation
```

**Tools:**
```python
class ListToolsRequest(JSONRPCRequest):
    """List available tools."""
    method: Literal["tools/list"] = "tools/list"

class CallToolRequest(JSONRPCRequest):
    """Execute a tool."""
    method: Literal["tools/call"] = "tools/call"
    params: CallToolParams

class CallToolParams(BaseModel):
    name: str
    arguments: Optional[dict] = None

class CallToolResult(BaseModel):
    content: List[Content]
    isError: bool = False
```

### 7.3 Protocol Handler

```python
class ProtocolHandler:
    """Process JSON-RPC messages and route to handlers."""
    
    def __init__(self, server: MCPServer):
        self.server = server
        self._handlers: Dict[str, Callable] = {
            "initialize": self._handle_initialize,
            "tools/list": self._handle_list_tools,
            "tools/call": self._handle_call_tool,
            "resources/list": self._handle_list_resources,
            "resources/read": self._handle_read_resource,
            "prompts/list": self._handle_list_prompts,
            "prompts/get": self._handle_get_prompt,
        }
    
    async def handle(self, message: JSONRPCMessage) -> Optional[JSONRPCMessage]:
        """Route message to appropriate handler."""
        if isinstance(message, JSONRPCRequest):
            handler = self._handlers.get(message.method)
            if not handler:
                return self._error(
                    message.id,
                    -32601,
                    f"Method not found: {message.method}"
                )
            
            try:
                result = await handler(message.params)
                return JSONRPCResponse(id=message.id, result=result)
            except MCPError as e:
                return JSONRPCResponse(
                    id=message.id,
                    error=JSONRPCError(
                        code=e.code,
                        message=e.message,
                        data=e.data
                    )
                )
        
        return None  # Notifications don't require response
```

---

## 8. Tool System

### 8.1 Tool Definition

Tools are defined using JSON Schema for automatic validation:

```python
class Tool(BaseModel):
    """MCP tool definition."""
    name: str
    description: str
    inputSchema: dict  # JSON Schema
    
    class Config:
        extra = "allow"  # Allow custom fields

# Example tool definition
calculate_mortgage = Tool(
    name="calculate_mortgage",
    description="Calculate monthly mortgage payments",
    inputSchema={
        "type": "object",
        "properties": {
            "principal": {
                "type": "number",
                "description": "Loan principal amount"
            },
            "rate": {
                "type": "number",
                "description": "Annual interest rate (decimal)"
            },
            "years": {
                "type": "integer",
                "description": "Loan term in years"
            }
        },
        "required": ["principal", "rate", "years"]
    }
)
```

### 8.2 Tool Registration

**Decorator Pattern:**
```python
from dinoforge_mcp import FastMCP

mcp = FastMCP("Dinoforge")

@mcp.tool()
def calculate_mortgage(principal: float, rate: float, years: int) -> str:
    """Calculate monthly mortgage payment.
    
    Args:
        principal: Loan amount in dollars
        rate: Annual interest rate (e.g., 0.05 for 5%)
        years: Loan term in years
    
    Returns:
        Formatted monthly payment amount
    """
    monthly_rate = rate / 12
    num_payments = years * 12
    payment = principal * (
        monthly_rate * (1 + monthly_rate) ** num_payments
    ) / ((1 + monthly_rate) ** num_payments - 1)
    return f"${payment:,.2f} per month"
```

**Programmatic Registration:**
```python
async def register_math_tools(registry: ToolRegistry):
    """Register all math-related tools."""
    
    await registry.register(
        tool=Tool(
            name="add",
            description="Add two numbers",
            inputSchema={
                "type": "object",
                "properties": {
                    "a": {"type": "number"},
                    "b": {"type": "number"}
                },
                "required": ["a", "b"]
            }
        ),
        handler=lambda args: args["a"] + args["b"],
        category="math"
    )
```

### 8.3 Tool Execution Flow

```
┌─────────────┐
│  Client     │
│  Request    │
└──────┬──────┘
       ▼
┌─────────────────┐
│  Parse Request  │
│  • Validate     │
│    JSON-RPC     │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Find Tool      │
│  • Registry     │
│    lookup       │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Validate Args  │
│  • JSON Schema  │
│    validation   │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Pre-process    │
│  • Middleware   │
│    chain        │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Execute Tool   │
│  • Handler      │
│    function     │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Post-process   │
│  • Middleware   │
│    chain        │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Format Result  │
│  • MCP content  │
│    objects      │
└────────┬────────┘
         ▼
┌─────────────────┐
│  Send Response  │
│  • JSON-RPC     │
└────────┬────────┘
         ▼
┌─────────────┐
│   Client    │
│  Response   │
└─────────────┘
```

### 8.4 Content Types

Tool results support multiple content types:

| Type | MIME Type | Use Case |
|------|-----------|----------|
| Text | `text/plain` | Simple text output |
| Markdown | `text/markdown` | Formatted documentation |
| JSON | `application/json` | Structured data |
| Image | `image/*` | Charts, diagrams |
| Binary | `application/octet-stream` | Files, downloads |

```python
class TextContent(BaseModel):
    type: Literal["text"] = "text"
    text: str

class ImageContent(BaseModel):
    type: Literal["image"] = "image"
    data: str  # Base64 encoded
    mimeType: str

Content = Union[TextContent, ImageContent, ...]
```

---

## 9. Resource System

### 9.1 Resource URIs

Resources are identified by URIs following the pattern:

```
{scheme}://{authority}/{path}?{query}

Examples:
- file:///home/user/project/src/main.py
- git://github.com/org/repo/blob/main/README.md
- db://postgres/tables/users
- api://stripe.com/v1/customers/cus_123
```

### 9.2 Resource Providers

```python
class ResourceProvider(ABC):
    """Base class for resource providers."""
    
    @property
    @abstractmethod
    def scheme(self) -> str:
        """URI scheme this provider handles."""
        pass
    
    @abstractmethod
    async def read(self, uri: str) -> Resource:
        """Read resource content."""
        pass
    
    @abstractmethod
    async def list(self, uri_pattern: str) -> List[Resource]:
        """List resources matching pattern."""
        pass

class FileProvider(ResourceProvider):
    """File system resource provider."""
    
    scheme = "file"
    
    async def read(self, uri: str) -> Resource:
        path = urlparse(uri).path
        async with aiofiles.open(path, 'r') as f:
            content = await f.read()
        return Resource(
            uri=uri,
            mimeType=mimetypes.guess_type(path)[0] or "text/plain",
            text=content
        )
```

### 9.3 Resource Templates

Templates allow dynamic resource discovery:

```python
ResourceTemplate(
    uriTemplate="file:///{path}",
    name="Project Files",
    mimeType="text/plain",
    description="Access files in the project directory"
)
```

---

## 10. Security Model

### 10.1 Threat Model

| Threat | Severity | Mitigation |
|--------|----------|------------|
| Tool injection | Critical | Input validation, schema enforcement |
| Data exfiltration | Critical | Resource access controls |
| Privilege escalation | High | Capability-based access |
| DoS via tool abuse | High | Rate limiting, quotas |
| Prompt injection | High | Output sanitization |
| Man-in-the-middle | Medium | TLS, mTLS |

### 10.2 Authentication

**Token-Based (HTTP/WebSocket):**
```python
async def authenticate_request(request: HTTPRequest) -> Identity:
    """Validate bearer token."""
    auth_header = request.headers.get("Authorization", "")
    if not auth_header.startswith("Bearer "):
        raise AuthenticationError("Missing or invalid authorization")
    
    token = auth_header[7:]
    try:
        payload = jwt.decode(
            token,
            key=public_key,
            algorithms=["RS256"],
            audience="dinoforge-mcp"
        )
        return Identity(
            subject=payload["sub"],
            scopes=payload.get("scope", "").split()
        )
    except jwt.ExpiredSignatureError:
        raise AuthenticationError("Token expired")
```

**mTLS (Service-to-Service):**
```python
ssl_context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
ssl_context.verify_mode = ssl.CERT_REQUIRED
ssl_context.load_cert_chain("/certs/server.crt", "/certs/server.key")
ssl_context.load_verify_locations("/certs/ca.crt")
```

### 10.3 Authorization

**Capability-Based Access Control:**
```python
class CBACAuthorizer:
    """Fine-grained authorization."""
    
    async def authorize(
        self,
        identity: Identity,
        resource: str,
        action: str
    ) -> AuthorizationDecision:
        required_capability = f"{action}:{resource}"
        
        for capability in identity.capabilities:
            if self._matches(capability, required_capability):
                return AuthorizationDecision(allowed=True)
        
        return AuthorizationDecision(
            allowed=False,
            reason=f"Missing capability: {required_capability}"
        )
    
    def _matches(self, capability: str, required: str) -> bool:
        """Check if capability matches required pattern.
        
        Supports wildcards:
        - tools:* matches any tool
        - resources:file:* matches any file resource
        """
        pattern = capability.replace("*", ".*")
        return re.match(f"^{pattern}$", required) is not None
```

### 10.4 Input Sanitization

```python
class SecureToolExecutor:
    """Execute tools with security controls."""
    
    async def execute(
        self,
        tool: Tool,
        arguments: dict,
        context: ExecutionContext
    ) -> ToolResult:
        # 1. Schema validation
        try:
            validate(instance=arguments, schema=tool.input_schema)
        except ValidationError as e:
            raise ToolValidationError(f"Invalid arguments: {e.message}")
        
        # 2. Dangerous pattern detection
        args_str = json.dumps(arguments)
        if contains_dangerous_patterns(args_str):
            raise SecurityError("Dangerous pattern detected in arguments")
        
        # 3. Resource access check
        for resource in tool.required_resources:
            if not await self.authorizer.check_access(context.identity, resource):
                raise UnauthorizedError(f"Access denied: {resource}")
        
        # 4. Rate limit check
        if not await self.rate_limiter.allow(context.identity.subject, tool.name):
            raise RateLimitExceeded(f"Rate limit exceeded for {tool.name}")
        
        # 5. Execute
        return await self._execute_tool(tool, arguments, context)
```

---

## 11. Error Handling

### 11.1 Error Hierarchy

```
Exception
├── MCPError
│   ├── ProtocolError
│   │   ├── ParseError
│   │   ├── InvalidRequestError
│   │   ├── MethodNotFoundError
│   │   ├── InvalidParamsError
│   │   └── InternalError
│   ├── ToolError
│   │   ├── ToolNotFoundError
│   │   ├── ToolExecutionError
│   │   └── ToolValidationError
│   ├── ResourceError
│   │   ├── ResourceNotFoundError
│   │   └── ResourceAccessError
│   ├── SecurityError
│   │   ├── AuthenticationError
│   │   ├── AuthorizationError
│   │   └── RateLimitExceeded
│   └── TransportError
│       ├── ConnectionError
│       └── TimeoutError
```

### 11.2 Error Code System

**Standard JSON-RPC Codes:**

| Code | Name | Description |
|------|------|-------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid Request | Not a valid Request object |
| -32601 | Method not found | Method doesn't exist |
| -32602 | Invalid params | Invalid method parameters |
| -32603 | Internal error | Internal JSON-RPC error |

**Server-Specific Codes:**

| Code | Name | Description |
|------|------|-------------|
| -32000 | Tool not found | Requested tool unavailable |
| -32001 | Tool execution failed | Tool raised exception |
| -32002 | Validation error | Input validation failed |
| -32003 | Rate limit exceeded | Too many requests |
| -32004 | Unauthorized | Insufficient permissions |
| -32005 | Resource not found | Resource unavailable |
| -32006 | Timeout | Operation timed out |

### 11.3 Error Transformation

```python
class ErrorHandler:
    """Three-layer error transformation."""
    
    async def handle(
        self,
        error: Exception,
        context: ExecutionContext
    ) -> dict:
        """Transform exception to client-safe error."""
        
        # Layer 1: Log full details internally
        self._log_error(error, context)
        
        # Layer 2: Convert to MCPError
        if not isinstance(error, MCPError):
            mcp_error = self._wrap_exception(error)
        else:
            mcp_error = error
        
        # Layer 3: Sanitize for client
        return self._sanitize_for_client(mcp_error, context)
    
    def _sanitize_for_client(
        self,
        error: MCPError,
        context: ExecutionContext
    ) -> dict:
        """Remove sensitive information before sending to client."""
        return {
            "code": error.code,
            "message": error.message,
            "data": {
                "trace_id": context.trace_id,
                # Internal details omitted
            }
        }
```

---

## 12. Performance

### 12.1 Performance Targets

| Metric | Target | Max | Measurement |
|--------|--------|-----|-------------|
| Request Latency (p50) | < 10ms | 50ms | Round-trip |
| Request Latency (p99) | < 50ms | 200ms | Round-trip |
| Throughput | > 1000 | 500 | req/s |
| Memory Footprint | < 128MB | 256MB | Per instance |
| Startup Time | < 500ms | 2s | Import to ready |
| Tool Registration | < 100ms | 500ms | Per tool |
| Concurrent Connections | 10,000 | 50,000 | Max connections |

### 12.2 Optimization Strategies

**Lazy Loading:**
```python
class LazyToolRegistry:
    """Load tools only when first accessed."""
    
    def __init__(self):
        self._tool_definitions: Dict[str, Tool] = {}
        self._tool_handlers: Dict[str, Callable] = {}
        self._loaded: Set[str] = set()
    
    async def get_tool(self, name: str) -> Optional[Tool]:
        if name not in self._loaded:
            await self._load_tool(name)
        return self._tool_definitions.get(name)
```

**Connection Pooling:**
```python
class HTTPTransport:
    """HTTP transport with connection pooling."""
    
    def __init__(self):
        self.session = aiohttp.ClientSession(
            connector=aiohttp.TCPConnector(
                limit=100,              # Max connections
                limit_per_host=20,      # Max per host
                enable_cleanup_closed=True,
                force_close=False,
            )
        )
```

**Caching:**
```python
class CachedToolRegistry:
    """Tool registry with caching."""
    
    def __init__(self, cache_ttl: float = 60.0):
        self._cache: Dict[str, Tuple[Tool, float]] = {}
        self._ttl = cache_ttl
    
    async def get_tool(self, name: str) -> Optional[Tool]:
        if name in self._cache:
            tool, timestamp = self._cache[name]
            if time.time() - timestamp < self._ttl:
                return tool
        
        tool = await self._load_tool(name)
        self._cache[name] = (tool, time.time())
        return tool
```

### 12.3 Benchmarking

```python
@pytest.mark.benchmark
async def test_tool_call_latency():
    """Measure tool call latency."""
    server = create_test_server()
    
    latencies = []
    for _ in range(1000):
        start = time.monotonic()
        await server.call_tool("echo", {"message": "test"})
        latencies.append((time.monotonic() - start) * 1000)
    
    p50 = statistics.median(latencies)
    p99 = np.percentile(latencies, 99)
    
    assert p50 < 10, f"p50 latency {p50}ms exceeds 10ms target"
    assert p99 < 50, f"p99 latency {p99}ms exceeds 50ms target"
```

---

## 13. Configuration

### 13.1 Configuration Hierarchy

Configuration is loaded from multiple sources in priority order:

1. **Environment variables** (highest priority)
2. **Configuration file** (`dinoforge.yaml`)
3. **Default values** (lowest priority)

```yaml
# dinoforge.yaml
server:
  name: "dinoforge-mcp"
  version: "1.0.0"
  
transport:
  type: "stdio"  # stdio, http, websocket
  http:
    host: "0.0.0.0"
    port: 8080
    tls:
      enabled: true
      cert_file: "/certs/server.crt"
      key_file: "/certs/server.key"
  websocket:
    ping_interval: 30
    max_message_size: 10485760

logging:
  level: "INFO"
  format: "json"
  output: "stdout"
  
security:
  auth:
    type: "token"  # token, mtls, none
    token:
      issuer: "auth.phenotype.io"
      audience: "dinoforge-mcp"
  rate_limit:
    enabled: true
    requests_per_second: 100
    burst_size: 20

tools:
  max_concurrent: 100
  timeout_seconds: 30
  enable_hot_reload: true
```

### 13.2 Configuration Schema

```python
class ServerConfig(BaseModel):
    """Server configuration."""
    name: str = "dinoforge-mcp"
    version: str = "1.0.0"

class TransportConfig(BaseModel):
    """Transport configuration."""
    type: Literal["stdio", "http", "websocket"] = "stdio"
    http: Optional[HTTPConfig] = None
    websocket: Optional[WebSocketConfig] = None

class LoggingConfig(BaseModel):
    """Logging configuration."""
    level: Literal["DEBUG", "INFO", "WARNING", "ERROR"] = "INFO"
    format: Literal["json", "text"] = "json"
    output: Literal["stdout", "stderr", "file"] = "stdout"

class Config(BaseModel):
    """Top-level configuration."""
    server: ServerConfig = Field(default_factory=ServerConfig)
    transport: TransportConfig = Field(default_factory=TransportConfig)
    logging: LoggingConfig = Field(default_factory=LoggingConfig)
    security: SecurityConfig = Field(default_factory=SecurityConfig)
    tools: ToolsConfig = Field(default_factory=ToolsConfig)
```

---

## 14. Testing

### 14.1 Test Pyramid

```
                    ┌─────────┐
                    │  E2E    │  <- 5% (slow, critical paths)
                    │  Tests  │
                    ├─────────┤
                    │  Integ  │  <- 15% (external services)
                    │  Tests  │
                    ├─────────┤
                    │  Proto  │  <- 20% (protocol compliance)
                    │  Tests  │
                    ├─────────┤
                    │  Unit   │  <- 60% (fast, isolated)
                    │  Tests  │
                    └─────────┘
```

### 14.2 Protocol Compliance Tests

```python
class TestMCPCompliance:
    """Verify MCP specification compliance."""
    
    async def test_initialize(self, server: MCPServer):
        """Test protocol initialization."""
        result = await server.initialize({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        })
        
        assert result["protocolVersion"] == "2024-11-05"
        assert "capabilities" in result
        assert "serverInfo" in result
    
    async def test_tool_lifecycle(self, server: MCPServer):
        """Test tool listing and execution."""
        # List tools
        tools = await server.list_tools()
        assert isinstance(tools, list)
        
        if tools:
            # Call tool
            tool = tools[0]
            result = await server.call_tool(
                tool.name,
                self._generate_valid_args(tool)
            )
            
            assert "content" in result
            assert isinstance(result["isError"], bool)
    
    async def test_error_format(self, server: MCPServer):
        """Test error response format."""
        result = await server.call_tool("nonexistent_tool", {})
        
        assert result["isError"] is True
        assert "content" in result
```

### 14.3 Transport Tests

```python
@pytest.mark.parametrize("transport_type", ["stdio", "http", "websocket"])
async def test_transport_reliability(transport_type):
    """Verify transport reliability."""
    server = create_server(transport_type)
    client = create_client(transport_type)
    
    # Test 100 sequential requests
    for i in range(100):
        result = await client.call_tool("echo", {"message": f"test{i}"})
        assert result.content[0].text == f"test{i}"
    
    # Test concurrent requests
    tasks = [
        client.call_tool("echo", {"message": f"concurrent{i}"})
        for i in range(50)
    ]
    results = await asyncio.gather(*tasks)
    assert len(results) == 50
```

### 14.4 Coverage Requirements

| Component | Unit | Integration | Protocol |
|-----------|------|-------------|----------|
| Server Core | 90% | 80% | 100% |
| Transports | 85% | 75% | 100% |
| Tool System | 90% | 70% | 100% |
| Protocol Handler | 95% | 80% | 100% |

---

## 15. Integration Points

### 15.1 Consumer Matrix

| Consumer | Integration Type | Transport | Authentication |
|----------|------------------|-----------|----------------|
| **heliosApp** | MCP Client | HTTP/WebSocket | Bearer token |
| **agent-wave** | MCP Client | stdio | None (process) |
| **phenotype-cli** | CLI stdio | stdio | None (process) |
| **template-commons** | Template ref | N/A | N/A |

### 15.2 Integration Patterns

**heliosApp Integration:**
```python
# heliosApp connects via HTTP
mcp_client = MCPClient(
    transport=HTTPTransport(
        base_url="https://dinoforge.phenotype.io",
        auth_token=os.environ["MCP_TOKEN"]
    )
)

# Discover and call tools
tools = await mcp_client.list_tools()
result = await mcp_client.call_tool("search_docs", {"query": "API"})
```

**agent-wave Integration:**
```python
# agent-wave spawns subprocess with stdio
process = await asyncio.create_subprocess_exec(
    "dinoforge-mcp",
    stdin=asyncio.subprocess.PIPE,
    stdout=asyncio.subprocess.PIPE,
)

transport = StdioTransport(process.stdout, process.stdin)
server = MCPServer(transport)
```

**phenotype-cli Integration:**
```python
# CLI invokes tools directly
@cli.command()
@click.argument("tool_name")
@click.argument("arguments", nargs=-1)
def tool(tool_name, arguments):
    """Execute an MCP tool."""
    server = create_stdio_server()
    result = asyncio.run(server.call_tool(tool_name, parse_args(arguments)))
    click.echo(result.content[0].text)
```

---

## 16. Deployment

### 16.1 Deployment Patterns

**Local Development:**
```bash
# Install in development mode
pip install -e Tools/DinoforgeMcp

# Run with stdio transport
dinoforge-mcp --transport stdio
```

**Container Deployment:**
```dockerfile
# Dockerfile
FROM python:3.11-slim

WORKDIR /app
COPY Tools/DinoforgeMcp/ .
RUN pip install -e .

EXPOSE 8080
CMD ["dinoforge-mcp", "--transport", "http", "--port", "8080"]
```

**Kubernetes Deployment:**
```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: dinoforge-mcp
spec:
  replicas: 3
  selector:
    matchLabels:
      app: dinoforge-mcp
  template:
    metadata:
      labels:
        app: dinoforge-mcp
    spec:
      containers:
      - name: mcp
        image: phenotype/dinoforge-mcp:latest
        ports:
        - containerPort: 8080
        env:
        - name: TRANSPORT_TYPE
          value: "http"
        - name: AUTH_TYPE
          value: "token"
        resources:
          limits:
            memory: "256Mi"
            cpu: "500m"
```

### 16.2 Environment Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `MCP_TRANSPORT` | Transport type | `stdio` |
| `MCP_HTTP_PORT` | HTTP server port | `8080` |
| `MCP_LOG_LEVEL` | Logging level | `INFO` |
| `MCP_AUTH_TYPE` | Authentication type | `none` |
| `MCP_RATE_LIMIT` | Requests per second | `100` |
| `MCP_MAX_TOOLS` | Maximum concurrent tools | `100` |

---

## 17. Observability

### 17.1 Logging

**Structured JSON Logging:**
```json
{
  "timestamp": "2026-04-04T12:34:56.789Z",
  "level": "INFO",
  "logger": "dinoforge_mcp.server",
  "message": "Tool executed successfully",
  "trace_id": "abc123",
  "span_id": "def456",
  "fields": {
    "tool_name": "calculate_mortgage",
    "duration_ms": 5.2,
    "result_size": 128
  }
}
```

**Log Levels:**

| Level | Usage |
|-------|-------|
| DEBUG | Detailed debugging, request/response bodies |
| INFO | Significant events, tool executions |
| WARNING | Recoverable issues, rate limiting |
| ERROR | Failed operations, exceptions |

### 17.2 Metrics

**Prometheus Metrics:**

```python
# Counter
TOOL_CALLS_TOTAL = Counter(
    "dinoforge_tool_calls_total",
    "Total tool calls",
    ["tool_name", "status"]
)

# Histogram
REQUEST_DURATION = Histogram(
    "dinoforge_request_duration_seconds",
    "Request duration",
    buckets=[0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
)

# Gauge
ACTIVE_CONNECTIONS = Gauge(
    "dinoforge_active_connections",
    "Number of active connections"
)
```

### 17.3 Tracing

**OpenTelemetry Integration:**
```python
from opentelemetry import trace

tracer = trace.get_tracer("dinoforge-mcp")

async def handle_request(self, request: JSONRPCMessage):
    with tracer.start_as_current_span("handle_request") as span:
        span.set_attribute("rpc.method", request.method)
        span.set_attribute("rpc.id", request.id)
        
        result = await self._process(request)
        
        span.set_attribute("rpc.status", "success")
        return result
```

### 17.4 Health Checks

**Liveness Probe:**
```python
async def liveness_check() -> HealthStatus:
    """Check if server is running."""
    return HealthStatus(status="healthy")
```

**Readiness Probe:**
```python
async def readiness_check() -> HealthStatus:
    """Check if server is ready to accept traffic."""
    checks = {
        "registry": await check_registry(),
        "transports": await check_transports(),
    }
    
    if all(checks.values()):
        return HealthStatus(status="ready")
    else:
        return HealthStatus(
            status="not_ready",
            details={k: v for k, v in checks.items() if not v}
        )
```

---

## 18. Roadmap

### 18.1 Current Status (Q2 2026)

| Feature | Status | Notes |
|---------|--------|-------|
| Stdio transport | Beta | CLI integration working |
| Tool registry | Beta | Dynamic registration |
| Protocol compliance | Beta | Core spec implemented |
| HTTP transport | In Progress | REST API design |
| WebSocket transport | Planned | Real-time support |
| Resource system | Planned | URI-based access |
| Prompt templates | Planned | LLM prompt management |
| Sampling API | Planned | LLM completion requests |

### 18.2 Future Milestones

**Q3 2026:**
- HTTP transport GA
- WebSocket transport beta
- Resource system beta
- Multi-tool batch operations

**Q4 2026:**
- Prompt templates GA
- Sampling API beta
- Distributed caching
- Advanced observability

**2027:**
- Tool marketplace integration
- Federated tool discovery
- Streaming responses
- GraphQL transport (experimental)

### 18.3 Feature Request Process

1. File issue in `phenotype/phenotype` with `enhancement` label
2. Include use case and proposed API
3. Discussion period (minimum 7 days)
4. Decision by core maintainers
5. Implementation tracking in AgilePlus

---

## 19. References

### 19.1 External Documentation

1. [MCP Specification](https://modelcontextprotocol.io/specification)
2. [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
3. [Python asyncio Guide](https://docs.python.org/3/library/asyncio.html)
4. [Pydantic Documentation](https://docs.pydantic.dev/)

### 19.2 Internal Documentation

1. [ADR-001: Multi-Transport Architecture](docs/adr/ADR-001-multi-transport-architecture.md)
2. [ADR-002: Dynamic Tool Registry](docs/adr/ADR-002-dynamic-tool-registry.md)
3. [ADR-003: Structured Error Handling](docs/adr/ADR-003-structured-error-handling.md)
4. [SOTA Research: MCP & Transport Systems](docs/research/SOTA-MCP-Transport-Systems.md)

### 19.3 Related Projects

| Project | Repository | Relationship |
|---------|------------|--------------|
| heliosApp | `repos/worktrees/heliosApp` | HTTP transport consumer |
| agent-wave | `repos/worktrees/agent-wave` | Stdio transport consumer |
| phenotype-cli | `repos/worktrees/phenotype-cli` | CLI integration |

---

## Appendix A: Glossary

| Term | Definition |
|------|------------|
| **MCP** | Model Context Protocol - standard for AI tool integration |
| **JSON-RPC** | Remote procedure call protocol using JSON |
| **Transport** | Communication channel (stdio, HTTP, WebSocket) |
| **Capability** | Feature advertisement in MCP initialization |
| **Tool** | Executable function exposed via MCP |
| **Resource** | Addressable content accessible via URI |
| **Sampling** | LLM completion request from server to client |
| **CBAC** | Capability-Based Access Control |
| **Middleware** | Pre/post processing hooks for tool execution |
| **Hot-reload** | Dynamic tool registration without restart |

## Appendix B: Code Examples

### Complete Server Example
```python
#!/usr/bin/env python3
"""Complete DinoforgeMCP server example."""

import asyncio
from dinoforge_mcp import FastMCP, TextContent

mcp = FastMCP("Dinoforge")

@mcp.tool()
async def search_documents(
    query: str,
    limit: int = 10
) -> str:
    """Search documentation.
    
    Args:
        query: Search query string
        limit: Maximum results to return
    
    Returns:
        Formatted search results
    """
    # Implementation
    results = await docs.search(query, limit)
    return format_results(results)

@mcp.tool()
def calculate(expression: str) -> str:
    """Evaluate mathematical expression."""
    try:
        result = eval(expression, {"__builtins__": {}}, {})
        return str(result)
    except Exception as e:
        return f"Error: {e}"

@mcp.resource("docs://{topic}")
async def get_documentation(topic: str) -> str:
    """Get documentation for a topic."""
    return await docs.load(topic)

if __name__ == "__main__":
    mcp.run(transport='stdio')
```

### Client Example
```python
#!/usr/bin/env python3
"""MCP client example."""

import asyncio
from dinoforge_mcp import MCPClient, StdioTransport

async def main():
    # Connect to server
    transport = StdioTransport.from_command(["dinoforge-mcp"])
    client = MCPClient(transport)
    
    # Initialize
    await client.initialize()
    
    # List tools
    tools = await client.list_tools()
    print(f"Available tools: {[t.name for t in tools]}")
    
    # Call tool
    result = await client.call_tool("calculate", {"expression": "2 + 2"})
    print(f"Result: {result.content[0].text}")
    
    # Cleanup
    await client.close()

if __name__ == "__main__":
    asyncio.run(main())
```

---

*End of Specification*

*For questions or issues, contact the Phenotype Core Team.*

## Appendix C: Transport Implementation Details

### C.1 Stdio Transport Deep Dive

**Process Lifecycle Management:**

When using stdio transport, the MCP server runs as a subprocess with the following lifecycle:

```
Parent Process                          Child Process (MCP Server)
     │                                        │
     │────── spawn subprocess ──────────────>│
     │                                        │
     │◄═════ bidirectional pipes ════════════│
     │  • stdin → child input               │
     │  • stdout → child output             │
     │                                        │
     │◄───── process messages ─────────────>│
     │                                        │
     │────── terminate signal ──────────────>│
     │                                        │
     │◄═════ process exit ════════════════════│
```

**Platform-Specific Considerations:**

| Platform | Stdio Behavior | Considerations |
|----------|----------------|----------------|
| Linux | Full duplex pipes | Default 64KB pipe buffer |
| macOS | Full duplex pipes | Smaller default buffer (16KB) |
| Windows | Named pipes | Requires async io support |
| WSL | Linux-compatible | May have edge cases |

**Buffer Management:**

```python
class StdioTransport(Transport):
    """Optimized stdio transport with buffering."""
    
    def __init__(
        self,
        reader: asyncio.StreamReader,
        writer: asyncio.StreamWriter,
        buffer_size: int = 65536
    ):
        self.reader = reader
        self.writer = writer
        self.buffer_size = buffer_size
        self._write_buffer = []
        self._flush_task = None
    
    async def write_message(self, message: JSONRPCMessage) -> None:
        """Buffer and flush messages."""
        json_line = json.dumps(message.to_dict(), separators=(',', ':'))
        self._write_buffer.append(json_line.encode('utf-8') + b'\n')
        
        # Schedule flush if not pending
        if self._flush_task is None or self._flush_task.done():
            self._flush_task = asyncio.create_task(self._flush())
    
    async def _flush(self) -> None:
        """Flush buffered writes."""
        await asyncio.sleep(0)  # Yield to event loop
        
        while self._write_buffer:
            chunk = b''.join(self._write_buffer)
            self._write_buffer.clear()
            self.writer.write(chunk)
            await self.writer.drain()
```

### C.2 HTTP Transport Implementation

**Request Routing:**

```python
class HTTPTransport:
    """HTTP transport with REST endpoints."""
    
    def __init__(self, host: str, port: int):
        self.app = web.Application()
        self._setup_routes()
        self.runner = None
    
    def _setup_routes(self):
        """Configure HTTP endpoints."""
        self.app.router.add_post(
            '/mcp/v1/message',
            self._handle_message
        )
        self.app.router.add_get(
            '/mcp/v1/tools',
            self._handle_list_tools
        )
        self.app.router.add_get(
            '/mcp/v1/tools/{name}',
            self._handle_get_tool
        )
        self.app.router.add_get(
            '/mcp/v1/stream',
            self._handle_stream
        )
        self.app.router.add_get(
            '/health',
            self._handle_health
        )
    
    async def _handle_message(self, request: web.Request) -> web.Response:
        """Handle JSON-RPC message."""
        try:
            body = await request.json()
            message = JSONRPCMessage.parse(body)
            
            # Process through protocol handler
            result = await self.protocol_handler.handle(message)
            
            return web.json_response(
                result.to_dict() if result else {"status": "ok"}
            )
        except json.JSONDecodeError as e:
            return web.json_response(
                {
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32700,
                        "message": f"Parse error: {e}"
                    }
                },
                status=400
            )
```

**Server-Sent Events (SSE) Implementation:**

```python
async def _handle_stream(self, request: web.Request) -> web.Response:
    """Handle SSE stream for server events."""
    response = web.StreamResponse(
        status=200,
        headers={
            'Content-Type': 'text/event-stream',
            'Cache-Control': 'no-cache',
            'Connection': 'keep-alive',
        }
    )
    await response.prepare(request)
    
    # Subscribe to server events
    queue = asyncio.Queue()
    self.event_subscribers.add(queue)
    
    try:
        while True:
            event = await asyncio.wait_for(
                queue.get(),
                timeout=30.0
            )
            
            # Format SSE event
            sse_data = f"event: {event.type}\ndata: {json.dumps(event.data)}\n\n"
            await response.write(sse_data.encode('utf-8'))
            
    except asyncio.TimeoutError:
        # Send keep-alive comment
        await response.write(b': keep-alive\n\n')
    except ConnectionResetError:
        pass
    finally:
        self.event_subscribers.discard(queue)
    
    return response
```

### C.3 WebSocket Transport Implementation

**Frame Handling:**

```python
class WebSocketTransport(Transport):
    """WebSocket transport with frame management."""
    
    def __init__(self, websocket: web.WebSocketResponse):
        self.ws = websocket
        self._message_queue = asyncio.Queue(maxsize=1000)
        self._closed = False
        self._tasks = set()
    
    async def run(self) -> None:
        """Start bidirectional communication."""
        # Start sender and receiver tasks
        receiver = asyncio.create_task(self._receiver_loop())
        sender = asyncio.create_task(self._sender_loop())
        pinger = asyncio.create_task(self._pinger_loop())
        
        self._tasks.update([receiver, sender, pinger])
        
        # Wait for any task to complete (usually on close)
        done, pending = await asyncio.wait(
            self._tasks,
            return_when=asyncio.FIRST_COMPLETED
        )
        
        # Cancel remaining tasks
        for task in pending:
            task.cancel()
        
        # Wait for cancellations
        await asyncio.gather(*pending, return_exceptions=True)
    
    async def _receiver_loop(self) -> None:
        """Receive and queue incoming messages."""
        async for msg in self.ws:
            if msg.type == WSMsgType.TEXT:
                try:
                    message = JSONRPCMessage.parse(msg.data)
                    await self._message_queue.put(message)
                except json.JSONDecodeError:
                    await self._send_error(-32700, "Parse error")
                    
            elif msg.type == WSMsgType.BINARY:
                # Handle binary content (resources)
                await self._handle_binary(msg.data)
                
            elif msg.type == WSMsgType.ERROR:
                logging.error(f"WebSocket error: {self.ws.exception()}")
                break
            elif msg.type == WSMsgType.CLOSE:
                break
    
    async def _sender_loop(self) -> None:
        """Send queued messages."""
        while not self._closed:
            message = await self._message_queue.get()
            if message is None:  # Shutdown signal
                break
                
            await self.ws.send_json(message.to_dict())
    
    async def _pinger_loop(self) -> None:
        """Send periodic ping frames."""
        while not self._closed:
            await asyncio.sleep(30.0)
            if self.ws.closed:
                break
            await self.ws.ping()
```

## Appendix D: Testing Strategy Details

### D.1 Test Fixtures

```python
@pytest.fixture
async def mcp_server():
    """Create test MCP server with sample tools."""
    server = MCPServer(
        name="test-server",
        version="1.0.0"
    )
    
    # Register test tools
    await server.register_tool(
        Tool(
            name="echo",
            description="Echo input",
            inputSchema={
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                },
                "required": ["message"]
            }
        ),
        lambda args: args["message"]
    )
    
    yield server
    
    await server.shutdown()

@pytest.fixture
async def stdio_transport():
    """Create stdio transport pair."""
    reader = asyncio.StreamReader()
    writer = MockStreamWriter()
    transport = StdioTransport(reader, writer)
    yield transport

@pytest.fixture
async def http_client(aiohttp_client, mcp_server):
    """Create HTTP test client."""
    app = web.Application()
    transport = HTTPTransport(mcp_server)
    app.router.add_post('/mcp/v1/message', transport.handle)
    return await aiohttp_client(app)
```

### D.2 Property-Based Testing

```python
from hypothesis import given, strategies as st

@given(
    tool_name=st.text(min_size=1, max_size=50).filter(lambda x: x.isalnum()),
    arguments=st.dictionaries(
        keys=st.text(min_size=1, max_size=20),
        values=st.one_of(st.text(), st.integers(), st.booleans()),
        min_size=0,
        max_size=10
    )
)
async def test_tool_input_validation(tool_name, arguments):
    """Property-based test for tool validation."""
    server = create_test_server()
    
    # Should not crash on any input
    try:
        result = await server.call_tool(tool_name, arguments)
        assert isinstance(result.is_error, bool)
    except MCPError as e:
        # Errors should be properly formed
        assert e.code in range(-32700, -32000)
        assert isinstance(e.message, str)
```

### D.3 Load Testing

```python
async def test_concurrent_tool_calls():
    """Test concurrent tool execution."""
    server = create_test_server()
    
    # Launch 1000 concurrent requests
    tasks = [
        server.call_tool("echo", {"message": f"concurrent-{i}"})
        for i in range(1000)
    ]
    
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    # All should succeed
    errors = [r for r in results if isinstance(r, Exception)]
    assert len(errors) == 0, f"Got {len(errors)} errors"
    
    # Verify results
    for i, result in enumerate(results):
        assert result.content[0].text == f"concurrent-{i}"

async def test_memory_stability():
    """Test memory usage under load."""
    import psutil
    import gc
    
    process = psutil.Process()
    
    # Baseline
    gc.collect()
    baseline = process.memory_info().rss
    
    # Run many requests
    server = create_test_server()
    for _ in range(10000):
        await server.call_tool("echo", {"message": "x" * 1000})
    
    # Force GC and check memory
    gc.collect()
    final = process.memory_info().rss
    
    # Memory should not grow significantly
    growth = (final - baseline) / baseline
    assert growth < 0.5, f"Memory grew by {growth*100:.1f}%"
```

## Appendix E: Deployment Checklists

### E.1 Production Release Checklist

**Pre-Deployment:**
- [ ] All unit tests passing
- [ ] Integration tests passing
- [ ] Protocol compliance tests passing
- [ ] Performance benchmarks within targets
- [ ] Security scan completed
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in pyproject.toml

**Deployment:**
- [ ] Staging deployment successful
- [ ] Smoke tests passed
- [ ] Gradual rollout (5% → 25% → 100%)
- [ ] Rollback plan tested
- [ ] Monitoring dashboards reviewed
- [ ] Alerts configured

**Post-Deployment:**
- [ ] Error rates within normal range
- [ ] Latency within SLA
- [ ] Customer impact metrics reviewed
- [ ] On-call handoff completed

### E.2 Configuration Validation

```python
class ConfigValidator:
    """Validate production configuration."""
    
    REQUIRED_SETTINGS = [
        'server.name',
        'transport.type',
        'logging.level',
    ]
    
    SECURITY_SETTINGS = [
        'security.auth.type',
        'security.rate_limit.enabled',
    ]
    
    def validate(self, config: dict) -> ValidationResult:
        """Validate configuration for production."""
        errors = []
        warnings = []
        
        # Check required settings
        for setting in self.REQUIRED_SETTINGS:
            if not self._get_nested(config, setting):
                errors.append(f"Missing required setting: {setting}")
        
        # Security checks
        if config.get('transport', {}).get('type') != 'stdio':
            for setting in self.SECURITY_SETTINGS:
                if not self._get_nested(config, setting):
                    warnings.append(f"Missing security setting: {setting}")
        
        # Performance checks
        if config.get('tools', {}).get('max_concurrent', 0) > 1000:
            warnings.append("max_concurrent > 1000 may cause resource exhaustion")
        
        return ValidationResult(errors=errors, warnings=warnings)
```

## Appendix F: Troubleshooting Guide

### F.1 Common Issues

**Issue: High latency on stdio transport**

Symptoms: Requests taking >100ms locally

Diagnosis:
```bash
# Check for blocking I/O
strace -f -e trace=write,read -p <pid>

# Check process priority
nice -n 0 python -m dinoforge_mcp
```

Solutions:
1. Ensure async/await used throughout
2. Check for blocking operations in tool handlers
3. Use `asyncio.to_thread()` for CPU-bound work

**Issue: HTTP 502 errors**

Symptoms: Intermittent 502 Bad Gateway

Diagnosis:
```bash
# Check upstream health
curl -v http://localhost:8080/health

# Check connection limits
ss -tan | grep :8080 | wc -l
```

Solutions:
1. Increase `keepalive_timeout`
2. Tune `worker_connections` in nginx
3. Enable HTTP keep-alive

**Issue: Memory leak**

Symptoms: Memory usage grows continuously

Diagnosis:
```python
import tracemalloc
tracemalloc.start()

# After some operations
snapshot = tracemalloc.take_snapshot()
top_stats = snapshot.statistics('lineno')
for stat in top_stats[:10]:
    print(stat)
```

Solutions:
1. Check for unclosed connections
2. Verify task cancellation on timeout
3. Review circular references

### F.2 Debug Mode

```python
# Enable detailed debugging
import logging
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

# Enable asyncio debug
asyncio.run(main(), debug=True)

# Enable transport tracing
transport = StdioTransport(
    reader, writer,
    trace=True  # Log all messages
)
```

### F.3 Log Analysis Commands

```bash
# Find slow requests
grep "duration_ms" app.log | \
  jq -r 'select(.duration_ms > 100) | "\(.timestamp) \(.method) \(.duration_ms)ms"'

# Count errors by type
grep "level.*ERROR" app.log | \
  jq -r '.error_code' | sort | uniq -c | sort -rn

# Find rate-limited clients
grep "rate_limit" app.log | \
  jq -r '.client_id' | sort | uniq -c | sort -rn | head -10
```

---

*End of Specification*

*For questions or issues, contact the Phenotype Core Team.*
