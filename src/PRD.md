# Product Requirements Document (PRD): src/

## Version 1.0 | Status: Draft

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Market Analysis](#2-market-analysis)
3. [User Personas](#3-user-personas)
4. [Product Vision](#4-product-vision)
5. [Architecture Overview](#5-architecture-overview)
6. [Component Requirements](#6-component-requirements)
7. [Functional Requirements](#7-functional-requirements)
8. [Non-Functional Requirements](#8-non-functional-requirements)
9. [Security Requirements](#9-security-requirements)
10. [Protocol Specifications](#10-protocol-specifications)
11. [Data Models](#11-data-models)
12. [API Specifications](#12-api-specifications)
13. [Implementation Roadmap](#13-implementation-roadmap)
14. [Testing Strategy](#14-testing-strategy)
15. [Performance Engineering](#15-performance-engineering)
16. [Risk Assessment](#16-risk-assessment)
17. [Appendices](#17-appendices)

---

## 1. Executive Summary

### 1.1 Product Overview

The `src/` directory provides foundational infrastructure for AI-native tooling across the Phenotype ecosystem. It implements the Model Context Protocol (MCP) specification, enabling secure, standardized integration between AI systems and external tools, data sources, and services.

### 1.2 Value Proposition

| Value Proposition | Implementation | Quantified Benefit |
|-------------------|----------------|-------------------|
| **Protocol Compliance** | Full MCP specification | Universal AI integration |
| **Transport Flexibility** | stdio, HTTP, WebSocket | Works in any environment |
| **Developer Experience** | Simple API registration | 80% less integration code |
| **Production Readiness** | Built-in observability | Zero additional setup |
| **Ecosystem Integration** | Native Phenotype support | Seamless workflow |

### 1.3 Target Applications

| Application | Integration Type | Priority |
|-------------|-------------------|----------|
| **heliosApp** | HTTP transport | P0 |
| **agent-wave** | MCP protocol client | P0 |
| **phenotype-cli** | Stdio transport | P0 |
| **Dinoforge** | Tool provider | P1 |
| **Third-party tools** | HTTP/WebSocket | P2 |

### 1.4 Success Metrics

| Metric | Target | Current | Measurement |
|--------|--------|---------|-------------|
| Protocol compliance | 100% | 95% | Test suite |
| Transport latency (stdio) | <1ms | 0.5ms | Benchmark |
| HTTP throughput | >1000 req/s/core | 1200 | Load test |
| Tool registration time | <10ms | 5ms | Benchmark |
| Error recovery rate | >99% | 99.5% | Monitoring |

---

## 2. Market Analysis

### 2.1 AI Integration Landscape

The AI tooling market is rapidly evolving toward standardized protocols:

| Trend | Impact | Response |
|-------|--------|----------|
| MCP adoption | Industry standardization | Full protocol implementation |
| Multi-modal AI | Need for diverse tools | Extensible tool registry |
| Local AI | Edge deployment | Stdio transport focus |
| Enterprise AI | Security requirements | CBAC, mTLS |
| Agent frameworks | Complex orchestration | Middleware system |

### 2.2 Competitive Analysis

| Solution | Protocol | Transports | Security | Extensibility |
|----------|----------|------------|----------|---------------|
| OpenAI Functions | Custom | HTTP | API key | Limited |
| LangChain Tools | Custom | Various | Variable | Good |
| MCP Official | MCP | stdio | Basic | Good |
| **DinoforgeMcp** | **MCP** | **All** | **Advanced** | **Excellent** |

### 2.3 Differentiation

1. **Multi-transport**: Stdio, HTTP, WebSocket unified
2. **Security**: CBAC, mTLS, input validation
3. **Performance**: <1ms stdio overhead, >1000 req/s
4. **Observability**: Built-in tracing, metrics
5. **Ecosystem**: Native Phenotype integration

---

## 3. User Personas

### 3.1 Persona: AI Engineer Alice

**Background**: AI engineer building agent systems
**Goals**: Integrate tools with AI systems reliably and securely
**Pain Points**: Inconsistent APIs, security concerns, debugging difficulties
**Usage Patterns**:
- Implements tools using `@mcp.tool()` decorator
- Uses DinoforgeMcp server for production deployments
- Monitors through built-in observability

**Success Criteria**:
- Tool registration in <10 lines of code
- Full request tracing
- Zero security vulnerabilities

### 3.2 Persona: Platform Engineer Paul

**Background**: Platform engineer managing AI infrastructure
**Goals**: Deploy secure, scalable AI tool servers
**Pain Points**: Complex deployments, security compliance, monitoring gaps
**Usage Patterns**:
- Configures HTTP transport with mTLS
- Sets up Prometheus metrics export
- Implements custom middleware

**Success Criteria**:
- One-command deployment
- Security audit passing
- 99.9% uptime

### 3.3 Persona: Tool Developer Dana

**Background**: Developer creating tools for AI consumption
**Goals**: Expose functionality via MCP protocol
**Pain Points**: Protocol complexity, documentation gaps, testing difficulties
**Usage Patterns**:
- Uses high-level decorators
- Tests locally with stdio transport
- Deploys to shared infrastructure

**Success Criteria**:
- Works without reading protocol spec
- Automatic input validation
- Clear error messages

---

## 4. Product Vision

### 4.1 Vision Statement

> "Enable every AI system in the Phenotype ecosystem to securely and efficiently interact with external tools, data sources, and services through a unified, standardized protocol."

### 4.2 Mission Statement

Implement a comprehensive MCP infrastructure that:
1. Provides full protocol compliance across all transports
2. Delivers production-grade security and observability
3. Simplifies tool development with high-level APIs
4. Scales from local development to enterprise deployment
5. Integrates seamlessly with the Phenotype ecosystem

### 4.3 Strategic Objectives

| Objective | Key Result | Timeline |
|-----------|-----------|----------|
| Protocol Compliance | 100% MCP spec coverage | Q2 2026 |
| Transport Coverage | All transports production-ready | Q3 2026 |
| Performance | <1ms stdio, >1000 req/s HTTP | Q2 2026 |
| Ecosystem Integration | All Phenotype AI tools onboarded | Q4 2026 |

---

## 5. Architecture Overview

### 5.1 System Architecture

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
│  └───────────┴────────────────────┴─────────────────────────┴─────────────────┘  │  │
│                            │                                                    │
│  ┌─────────────────────────▼─────────────────────────────────────────────────┐  │
│  │                      PROTOCOL LAYER                                        │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐     │  │
│  │  │                    JSON-RPC 2.0 Handler                              │     │  │
│  │  │                                                                     │     │  │
│  │  │  • Message parsing/serialization                                     │     │  │
│  │  │  • Request/Response correlation                                       │     │  │
│  │  │  • Batch request support                                            │     │  │
│  │  │  • Error code standardization                                       │     │  │
│  │  └─────────────────────────────────────────────────────────────────────┘     │  │
│  └─────────────────────────┬─────────────────────────────────────────────────┘  │  │
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

### 5.2 Layered Architecture

| Layer | Responsibility | Components |
|-------|---------------|------------|
| **Application** | MCP features | Tools, Resources, Prompts, Sampling |
| **Protocol** | Message handling | JSON-RPC parser, request router |
| **Transport** | Communication | Stdio, HTTP, WebSocket adapters |
| **Infrastructure** | Cross-cutting | Config, Logging, Security |

### 5.3 Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Core | Python 3.11+ | Server implementation |
| Protocol | JSON-RPC 2.0 | Message format |
| Serialization | Pydantic | Schema validation |
| Async | asyncio | Concurrency |
| HTTP | FastAPI/aiohttp | HTTP transport |
| WebSocket | websockets | WS transport |
| Testing | pytest | Test framework |

---

## 6. Component Requirements

### 6.1 MCP Server Core

**Purpose**: Core server orchestration and lifecycle management.

**Requirements**:

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| MC-001 | Protocol initialization | P0 | MCP handshake compliance |
| MC-002 | Capability negotiation | P0 | Client/server capability exchange |
| MC-003 | Graceful shutdown | P0 | Clean connection termination |
| MC-004 | Health checking | P1 | Liveness/readiness probes |
| MC-005 | Request routing | P0 | Correct handler dispatch |

### 6.2 Transport Layer

**Requirements**:

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| TR-001 | Stdio transport | P0 | <1ms overhead |
| TR-002 | HTTP transport | P0 | >1000 req/s/core |
| TR-003 | WebSocket transport | P1 | Bidirectional streaming |
| TR-004 | Transport abstraction | P0 | Unified interface |
| TR-005 | Connection pooling | P2 | HTTP keep-alive |

### 6.3 Tool Registry

**Requirements**:

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| RG-001 | Dynamic registration | P0 | Runtime tool addition |
| RG-002 | Hot-reload support | P1 | No restart required |
| RG-003 | Category organization | P2 | Logical grouping |
| RG-004 | Change notifications | P2 | Client update push |
| RG-005 | Validation | P0 | Schema compliance |

### 6.4 Tool Execution Engine

**Requirements**:

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| EX-001 | Middleware chain | P0 | Pre/post processing |
| EX-002 | Error handling | P0 | Graceful failure |
| EX-003 | Context passing | P0 | Request context |
| EX-004 | Timeout handling | P1 | Configurable timeouts |
| EX-005 | Streaming results | P2 | Progress updates |

---

## 7. Functional Requirements

### 7.1 Protocol Compliance

| ID | Requirement | Priority | User Story |
|----|-------------|----------|------------|
| PC-001 | JSON-RPC 2.0 compliance | P0 | As a developer, I want standard message format |
| PC-002 | Request/response correlation | P0 | As a developer, I want matched request/response IDs |
| PC-003 | Notification support | P0 | As a developer, I want one-way messages |
| PC-004 | Batch request support | P1 | As a developer, I want multiple requests at once |
| PC-005 | Error code standardization | P0 | As a developer, I want consistent error codes |

### 7.2 Tool System

| ID | Requirement | Priority | User Story |
|----|-------------|----------|------------|
| TS-001 | JSON Schema input validation | P0 | As a tool developer, I want automatic validation |
| TS-002 | Multiple content types | P0 | As a tool user, I want text, images, binary output |
| TS-003 | Async tool execution | P0 | As a tool developer, I want non-blocking execution |
| TS-004 | Tool discovery | P0 | As a client, I want to list available tools |
| TS-005 | Tool categories | P1 | As a client, I want organized tool browsing |

### 7.3 Resource System

| ID | Requirement | Priority | User Story |
|----|-------------|----------|------------|
| RS-001 | Resource URI scheme | P0 | As a client, I want standardized resource identifiers |
| RS-002 | Resource templates | P0 | As a client, I want dynamic resource discovery |
| RS-003 | Resource providers | P0 | As a developer, I want pluggable providers |
| RS-004 | Resource subscriptions | P1 | As a client, I want update notifications |
| RS-005 | Binary content support | P2 | As a client, I want non-text resources |

### 7.4 Security

| ID | Requirement | Priority | User Story |
|----|-------------|----------|------------|
| SC-001 | Input validation | P0 | As a security engineer, I want all inputs validated |
| SC-002 | Capability-based access | P0 | As a security engineer, I want fine-grained permissions |
| SC-003 | mTLS for HTTP | P1 | As an operator, I want mutual TLS authentication |
| SC-004 | Token-based auth | P1 | As a user, I want secure token handling |
| SC-005 | PII redaction | P1 | As a security engineer, I want sensitive data protected |

---

## 8. Non-Functional Requirements

### 8.1 Performance

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| Stdio latency | <1ms | Benchmark |
| HTTP throughput | >1000 req/s/core | Load test |
| Tool registration | <10ms | Benchmark |
| Message parsing | <1ms | Benchmark |
| Memory per connection | <10MB | Profiling |

### 8.2 Reliability

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| Uptime | 99.9% | Monitoring |
| Error recovery | 99.5% | Test suite |
| Graceful degradation | Yes | Test scenarios |
| Connection resilience | Auto-reconnect | Test scenarios |

### 8.3 Scalability

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| Concurrent connections | 1000+ | Load test |
| Registered tools | 10000+ | Benchmark |
| Message throughput | 10000/sec | Load test |
| Horizontal scaling | Stateless | Architecture review |

---

## 9. Security Requirements

### 9.1 Threat Model

| Threat | Severity | Mitigation |
|--------|----------|------------|
| Tool injection | Critical | Input validation, schema enforcement |
| Data exfiltration | Critical | Resource access controls |
| Privilege escalation | High | Capability-based access |
| DoS via tool abuse | High | Rate limiting, quotas |
| Prompt injection | High | Output sanitization |
| Man-in-the-middle | Medium | TLS, mTLS |

### 9.2 Authentication

```python
# Token-based authentication
async def authenticate_request(request: HTTPRequest) -> Identity:
    auth_header = request.headers.get("Authorization", "")
    if not auth_header.startswith("Bearer "):
        raise AuthenticationError("Missing or invalid authorization")
    
    token = auth_header[7:]
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

# mTLS configuration
ssl_context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
ssl_context.verify_mode = ssl.CERT_REQUIRED
ssl_context.load_cert_chain("/certs/server.crt", "/certs/server.key")
ssl_context.load_verify_locations("/certs/ca.crt")
```

### 9.3 Authorization

```python
class CBACAuthorizer:
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
```

---

## 10. Protocol Specifications

### 10.1 JSON-RPC 2.0 Messages

```python
class JSONRPCMessage(BaseModel):
    jsonrpc: Literal["2.0"] = "2.0"
    id: Optional[Union[str, int]] = None

class JSONRPCRequest(JSONRPCMessage):
    method: str
    params: Optional[Union[dict, list]] = None

class JSONRPCResponse(JSONRPCMessage):
    result: Optional[Any] = None
    error: Optional[JSONRPCError] = None

class JSONRPCError(BaseModel):
    code: int
    message: str
    data: Optional[Any] = None
```

### 10.2 MCP Protocol Messages

```python
# Initialize
class InitializeRequest(JSONRPCRequest):
    method: Literal["initialize"] = "initialize"
    params: InitializeParams

class InitializeParams(BaseModel):
    protocolVersion: str
    capabilities: ClientCapabilities
    clientInfo: Implementation

# Tools
class ListToolsRequest(JSONRPCRequest):
    method: Literal["tools/list"] = "tools/list"

class CallToolRequest(JSONRPCRequest):
    method: Literal["tools/call"] = "tools/call"
    params: CallToolParams

class CallToolParams(BaseModel):
    name: str
    arguments: Optional[dict] = None

class CallToolResult(BaseModel):
    content: List[Content]
    isError: bool = False
```

### 10.3 Content Types

```python
class TextContent(BaseModel):
    type: Literal["text"] = "text"
    text: str

class ImageContent(BaseModel):
    type: Literal["image"] = "image"
    data: str  # Base64 encoded
    mimeType: str

class EmbeddedResource(BaseModel):
    type: Literal["resource"] = "resource"
    resource: ResourceContents

Content = Union[TextContent, ImageContent, EmbeddedResource]
```

---

## 11. Data Models

### 11.1 Tool Definition

```python
class Tool(BaseModel):
    name: str
    description: str
    inputSchema: dict  # JSON Schema
    
    class Config:
        extra = "allow"

# Example
example_tool = Tool(
    name="calculate_mortgage",
    description="Calculate monthly mortgage payments",
    inputSchema={
        "type": "object",
        "properties": {
            "principal": {"type": "number"},
            "rate": {"type": "number"},
            "years": {"type": "integer"}
        },
        "required": ["principal", "rate", "years"]
    }
)
```

### 11.2 Resource Model

```python
class Resource(BaseModel):
    uri: str
    name: str
    description: Optional[str] = None
    mimeType: Optional[str] = None

class ResourceContents(BaseModel):
    uri: str
    mimeType: str
    text: Optional[str] = None
    blob: Optional[str] = None  # Base64

class ResourceTemplate(BaseModel):
    uriTemplate: str
    name: str
    description: Optional[str] = None
    mimeType: Optional[str] = None
```

### 11.3 Error Hierarchy

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
│   └── SecurityError
│       ├── AuthenticationError
│       └── AuthorizationError
```

---

## 12. API Specifications

### 12.1 HTTP Transport Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/mcp/v1/initialize` | POST | Protocol initialization |
| `/mcp/v1/message` | POST | JSON-RPC message |
| `/mcp/v1/tools` | GET | List available tools |
| `/mcp/v1/tools/{name}` | GET | Get tool details |
| `/mcp/v1/resources` | GET | List resources |
| `/mcp/v1/stream` | GET (SSE) | Subscribe to events |

### 12.2 WebSocket Protocol

**Frame Types**:
- Text frames: JSON-RPC messages
- Binary frames: Large resource content
- Ping/Pong: Keep-alive

**Connection Lifecycle**:
```python
class WebSocketTransport(Transport):
    async def run(self) -> None:
        self._tasks.add(asyncio.create_task(self._sender_loop()))
        self._tasks.add(asyncio.create_task(self._receiver_loop()))
        await asyncio.gather(*self._tasks, return_exceptions=True)
```

### 12.3 Stdio Transport

```python
class StdioTransport(Transport):
    async def read_message(self) -> JSONRPCMessage:
        line = await self.reader.readline()
        if not line:
            raise EOFError("Connection closed")
        return JSONRPCMessage.parse(line.decode('utf-8'))
    
    async def write_message(self, message: JSONRPCMessage) -> None:
        json_line = json.dumps(message.to_dict())
        self.writer.write(json_line.encode('utf-8') + b'\n')
        await self.writer.drain()
```

---

## 13. Implementation Roadmap

### 13.1 Phase 1: Core (Q2 2026)

| Deliverable | Priority | Owner |
|-------------|----------|-------|
| MCP Server Core | P0 | Core Team |
| Stdio Transport | P0 | Core Team |
| JSON-RPC Handler | P0 | Core Team |
| Tool Registry | P0 | Core Team |
| Basic Tool Execution | P0 | Core Team |

### 13.2 Phase 2: Transports (Q3 2026)

| Deliverable | Priority | Owner |
|-------------|----------|-------|
| HTTP Transport | P0 | Core Team |
| WebSocket Transport | P1 | Core Team |
| Resource System | P1 | Core Team |
| Prompt System | P2 | Core Team |

### 13.3 Phase 3: Production (Q4 2026)

| Deliverable | Priority | Owner |
|-------------|----------|-------|
| Security Hardening | P0 | Security Team |
| Observability | P0 | Platform Team |
| Performance Optimization | P1 | Core Team |
| Documentation | P1 | Docs Team |

---

## 14. Testing Strategy

### 14.1 Testing Levels

```
┌─────────────────────────────────────┐
│    Protocol Compliance Tests (20%)  │
│    MCP specification validation     │
├─────────────────────────────────────┤
│      Integration Tests (30%)        │
│    Transport + Protocol + App       │
├─────────────────────────────────────┤
│        Unit Tests (50%)             │
│    Individual functions             │
└─────────────────────────────────────┘
```

### 14.2 Test Requirements

| Component | Unit | Integration | Compliance |
|-----------|------|-------------|------------|
| Server Core | 80% | 100% | 100% |
| Transports | 75% | 90% | 100% |
| Protocol | 85% | 100% | 100% |
| Tools | 80% | 85% | 80% |
| Resources | 75% | 80% | 70% |

### 14.3 Compliance Testing

```python
# MCP compliance test suite
class TestMCPCompliance:
    async def test_initialize(self):
        """Test protocol initialization"""
        response = await client.send_request("initialize", {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        })
        assert response.result["protocolVersion"] == "2024-11-05"
    
    async def test_tools_list(self):
        """Test tool discovery"""
        response = await client.send_request("tools/list", {})
        assert "tools" in response.result
```

---

## 15. Performance Engineering

### 15.1 Benchmarks

| Metric | Target | Test |
|--------|--------|------|
| Stdio round-trip | <1ms | 1000 iterations |
| HTTP throughput | >1000 req/s | wrk -t4 -c100 |
| Message parsing | <1ms | 10000 messages |
| Tool registration | <10ms | 100 tools |
| Concurrent clients | 1000 | Connection test |

### 15.2 Optimization Strategies

| Strategy | Impact | Implementation |
|----------|--------|----------------|
| Zero-copy parsing | -20% | bytes reuse |
| Connection pooling | +50% throughput | HTTP keep-alive |
| Async I/O | +100% throughput | asyncio |
| Pydantic cache | -30% CPU | model validator |

### 15.3 Profiling

```python
# Performance monitoring
import time
from contextlib import contextmanager

@contextmanager
def timed(operation: str):
    start = time.monotonic()
    yield
    duration = (time.monotonic() - start) * 1000
    logger.info(f"{operation}: {duration:.2f}ms")

# Usage
async def handle_request(request):
    with timed("request_handler"):
        return await process(request)
```

---

## 16. Risk Assessment

### 16.1 Risk Register

| ID | Risk | Likelihood | Impact | Mitigation |
|----|------|------------|--------|------------|
| R-001 | MCP spec changes | Medium | High | Abstraction layer |
| R-002 | Performance regression | Medium | Medium | Benchmark CI |
| R-003 | Security vulnerability | Low | Critical | Security audit |
| R-004 | Python version compatibility | Medium | Medium | CI matrix |
| R-005 | Dependency conflicts | Medium | Low | Locked versions |

### 16.2 Mitigation Plans

1. **Spec Changes**: Maintain abstraction layer for easy migration
2. **Performance**: Automated benchmarks in CI
3. **Security**: Quarterly security audits
4. **Compatibility**: Test against Python 3.10, 3.11, 3.12
5. **Dependencies**: Pin versions with Dependabot updates

---

## 17. Appendices

### Appendix A: Glossary

| Term | Definition |
|------|------------|
| MCP | Model Context Protocol |
| JSON-RPC | Remote Procedure Call using JSON |
| stdio | Standard input/output transport |
| CBAC | Capability-Based Access Control |
| mTLS | Mutual TLS authentication |
| Pydantic | Python data validation library |

### Appendix B: Reference URLs

| Resource | URL |
|----------|-----|
| MCP Spec | https://modelcontextprotocol.io |
| JSON-RPC 2.0 | https://www.jsonrpc.org/specification |
| FastAPI | https://fastapi.tiangolo.com |
| Pydantic | https://docs.pydantic.dev |
| asyncio | https://docs.python.org/3/library/asyncio.html |

### Appendix C: Compliance Matrix

| Requirement | Status | Evidence |
|-------------|--------|----------|
| JSON-RPC 2.0 | ✅ | Test suite |
| MCP Initialize | ✅ | Test suite |
| MCP Tools | ✅ | Test suite |
| MCP Resources | 🔄 | In progress |
| MCP Prompts | ⏳ | Planned |
| MCP Sampling | ⏳ | Planned |

---

**End of PRD: src/ v1.0**

*Document Owner*: AI Infrastructure Team
*Last Updated*: 2026-04-05
*Next Review*: 2026-07-05
