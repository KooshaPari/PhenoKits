# ADR-002 — stdio Transport as Primary MCP Communication

**Status:** Accepted
**Date:** 2026-04-04
**Decider:** Phenotype Core Team

## Context

MCP servers support multiple transport mechanisms for communication between hosts and servers. The transport choice impacts security, deployment complexity, latency, and integration patterns.

**Transport Options:**
- **stdio** — Standard input/output streams, process-based
- **HTTP/SSE** — Server-sent events over HTTP
- **WebSocket** — Bidirectional persistent connection
- **Unix Domain Sockets** — Local inter-process communication

**Deployment Contexts:**
- Claude Desktop — Local desktop application
- agent-wave — Local/remote agent orchestration
- CI/CD — Ephemeral test runners
- Production — Long-running services

## Decision

browser-agent-mcp adopts **stdio transport** as the primary communication mechanism, with HTTP/SSE as the secondary option for remote deployments.

## Rationale

### Transport Comparison

| Transport | Latency | Security | Deployment | Claude Desktop |
|-----------|---------|----------|------------|----------------|
| stdio | 2-8ms | Excellent (process isolation) | Simple | ✅ Native |
| HTTP/SSE | 15-50ms | Good (TLS, auth) | Moderate | ⚠️ Proxy required |
| WebSocket | 10-30ms | Good (TLS, auth) | Complex | ❌ Not supported |
| Unix Socket | 1-2ms | Excellent | Simple | ❌ Not supported |

### Architecture Fit

```
┌─────────────────────────────────────────────────────────────────┐
│                    stdio TRANSPORT ARCHITECTURE                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌───────────────────────┐     ┌───────────────────────┐      │
│  │     Claude Desktop    │     │     Host Process      │      │
│  │  ┌─────────────────┐  │     │  ┌─────────────────┐   │      │
│  │  │  MCP Client     │  │     │  │  MCP Client     │   │      │
│  │  │  (stdio)        │  │     │  │  (stdio)        │   │      │
│  │  └────────┬────────┘  │     │  └────────┬────────┘   │      │
│  │           │           │     │           │            │      │
│  │           │ stdin     │     │           │ stdin      │      │
│  │           ▼           │     │           ▼            │      │
│  │  ┌─────────────────┐  │     │  ┌─────────────────┐    │      │
│  │  │  MCP Server     │  │     │  │  MCP Server     │    │      │
│  │  │  (browser-agent)│  │     │  │  (browser-agent)│    │      │
│  │  │                 │  │     │  │                 │    │      │
│  │  │  ┌───────────┐  │  │     │  │  ┌───────────┐   │    │      │
│  │  │  │ Playwright│  │  │     │  │  │ Playwright│   │    │      │
│  │  │  │ Browser   │  │  │     │  │  │ Browser   │   │    │      │
│  │  │  └───────────┘  │  │     │  │  └───────────┘   │    │      │
│  │  └─────────────────┘  │     │  └─────────────────┘    │      │
│  │           │           │     │           │            │      │
│  │           │ stdout    │     │           │ stdout     │      │
│  │           │ (JSON-RPC)│     │           │ (JSON-RPC) │      │
│  └───────────┼───────────┘     └───────────┼────────────┘      │
│              │                             │                    │
│              └──────────► ◄─────────────────┘                    │
│                     Bidirectional JSON-RPC                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### stdio Advantages

1. **Security by Design**
   ```
   Process Isolation Model:
   ┌─────────────────────────────────────────┐
   │ Host Process (Claude Desktop)         │
   │ ┌─────────────────────────────────────┐ │
   │ │ MCP Client                          │ │
   │ │ ├─ JSON-RPC encoding               │ │
   │ │ └─ Message routing                 │ │
   │ └──────────┬──────────────────────────┘ │
   │            │ stdin/stdout               │
   │ ┌──────────▼──────────────────────────┐│
   │ │ MCP Server (browser-agent-mcp)      ││
   │ │ ├─ JSON-RPC decoding               ││
   │ │ ├─ Tool routing                    ││
   │ │ └─ Browser control                 ││
   │ └──────────┬──────────────────────────┘│
   │            │                             │
   │ ┌──────────▼──────────────────────────┐│
   │ │ Browser Process (isolated)          ││
   │ │ ├─ Chromium/Firefox/WebKit         ││
   │ │ └─ Web content execution            ││
   │ └─────────────────────────────────────┘│
   └─────────────────────────────────────────┘
   
   Security Properties:
   - No network exposure
   - Process-level isolation
   - Implicit sandbox via process boundaries
   - No credential leakage via network
   ```

2. **Simplicity**
   - No network configuration required
   - No port management
   - No firewall rules
   - No TLS certificates
   - No authentication tokens for local use

3. **Claude Desktop Native**
   ```json
   {
     "mcpServers": {
       "browser-agent": {
         "command": "python",
         "args": ["-m", "browser_agent_mcp"],
         "env": {
           "BROWSER_HEADLESS": "true"
         }
       }
     }
   }
   ```

4. **Debugging Simplicity**
   ```bash
   # Direct invocation for debugging
   echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | \
     python -m browser_agent_mcp
   
   # Capture stderr separately
   python -m browser_agent_mcp 2>debug.log
   ```

### HTTP/SSE as Secondary

**Use Case: Remote Deployments**

```
┌─────────────────────────────────────────────────────────────────┐
│                  HTTP/SSE TRANSPORT ARCHITECTURE                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────┐        ┌──────────────────┐              │
│  │  Remote Client   │◄──────►│  MCP HTTP Server │              │
│  │  (agent-wave)    │  HTTP  │  (browser-agent) │              │
│  └──────────────────┘        └────────┬─────────┘              │
│                                       │                         │
│                                       │ localhost              │
│                                       ▼                         │
│                              ┌──────────────────┐              │
│                              │  Playwright      │              │
│                              │  Browser         │              │
│                              └──────────────────┘              │
│                                                                 │
│  Security:                                                      │
│  - TLS termination at reverse proxy                            │
│  - API key authentication                                       │
│  - Rate limiting                                                │
│  - Request logging/auditing                                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Consequences

### Positive

1. **Zero Network Configuration**
   - Works out-of-the-box on any machine
   - No port conflicts
   - No firewall modifications
   - Works in restricted corporate environments

2. **Maximum Security**
   - No network attack surface
   - Process isolation by default
   - No credential transmission over network
   - Easy to audit all communication

3. **Deployment Simplicity**
   - Single binary/script execution
   - No service management
   - No health checks for transport
   - Automatic cleanup on process exit

4. **Claude Desktop Native**
   - First-class integration
   - No additional configuration
   - Follows MCP specification exactly

### Negative

1. **Single Host Limitation**
   - One server instance per client
   - No multi-tenant capability via stdio
   - Resource duplication across clients

2. **No Remote Access**
   - Cannot expose to remote agents
   - Limits distributed deployment patterns
   - Requires HTTP fallback for remote

3. **Process Lifecycle Coupling**
   - Server terminates with client
   - Browser state lost on disconnect
   - No persistent sessions

4. **Debugging Complexity**
   - stdout/stderr interleaving
   - Binary data requires encoding
   - No wire inspection tools

### Mitigations

| Limitation | Mitigation |
|------------|------------|
| Single host | Implement HTTP mode for remote; connection pooling for efficiency |
| No remote | Clear separation: stdio for local, HTTP for remote |
| Lifecycle coupling | Browser state serialization; quick reconnection |
| Debugging | Structured logging to stderr; debug mode with file logging |

## Implementation Details

### stdio Server Implementation

```python
# browser_agent/server.py
import asyncio
import sys
from mcp.server import Server
from mcp.server.stdio import stdio_server
from mcp.types import TextContent

server = Server("browser-agent")

@server.call_tool()
async def handle_tool(name: str, arguments: dict):
    # Tool implementation
    return [TextContent(type="text", text=result)]

async def main():
    # Log to stderr (stdout reserved for JSON-RPC)
    import structlog
    logger = structlog.get_logger()
    logger.info("server_starting", transport="stdio")
    
    async with stdio_server() as (read_stream, write_stream):
        await server.run(read_stream, write_stream)

if __name__ == "__main__":
    asyncio.run(main())
```

### Transport Selection Logic

```python
# browser_agent/config.py
from enum import Enum
import os

class TransportType(Enum):
    STDIO = "stdio"
    HTTP = "http"
    WEBSOCKET = "websocket"

def get_transport() -> TransportType:
    """Determine transport from environment."""
    transport = os.getenv("MCP_TRANSPORT", "stdio")
    return TransportType(transport)

def create_server():
    transport = get_transport()
    
    if transport == TransportType.STDIO:
        return create_stdio_server()
    elif transport == TransportType.HTTP:
        return create_http_server()
    else:
        raise ValueError(f"Unsupported transport: {transport}")
```

### HTTP Mode for Remote

```python
# browser_agent/server_http.py
from fastapi import FastAPI, HTTPException
from fastapi.responses import StreamingResponse
from mcp.types import CallToolRequest

app = FastAPI()

@app.post("/mcp/call-tool")
async def call_tool(request: CallToolRequest):
    try:
        result = await handle_tool(request.name, request.arguments)
        return {"content": result}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/mcp/sse")
async def sse_endpoint():
    """Server-sent events for notifications."""
    async def event_generator():
        while True:
            event = await notification_queue.get()
            yield f"data: {json.dumps(event)}\n\n"
    
    return StreamingResponse(
        event_generator(),
        media_type="text/event-stream"
    )
```

## Alternatives Considered

### HTTP/SSE Primary

**Pros:**
- Remote access capability
- Standard web debugging tools
- Load balancer friendly

**Cons:**
- More complex deployment
- Security configuration burden
- Not Claude Desktop native

**Decision:** Rejected — stdio simplicity outweighs HTTP flexibility for primary use case

### WebSocket Primary

**Pros:**
- Bidirectional streaming
- Lower latency than HTTP polling
n- Good for real-time updates

**Cons:**
- Complex reconnection logic
- Proxy/firewall issues
- Not supported by Claude Desktop

**Decision:** Rejected — Not needed for current use cases

### Unix Domain Sockets

**Pros:**
- Lowest latency (1-2ms)
- File permission security
- No port management

**Cons:**
- Platform-specific (Unix only)
- No Windows support
- Not MCP specification compliant

**Decision:** Rejected — Platform compatibility critical

## Migration Path

**Phase 1:** stdio only (MVP)
- Implement core functionality
- Claude Desktop integration
- Local testing

**Phase 2:** HTTP mode (Remote)
- Add HTTP server capability
- Authentication/authorization
- Production deployment

**Phase 3:** WebSocket (Future)
- Bidirectional streaming
- Real-time browser events
- Advanced use cases

## References

- [MCP Specification - Transports](https://modelcontextprotocol.io/specification/transport)
- [MCP Python SDK - stdio](https://github.com/modelcontextprotocol/python-sdk)
- [Claude Desktop MCP Configuration](https://modelcontextprotocol.io/quickstart)
- [Server-Sent Events Spec](https://html.spec.whatwg.org/multipage/server-sent-events.html)

---

*Supersedes: N/A*
*Superseded by: N/A*
