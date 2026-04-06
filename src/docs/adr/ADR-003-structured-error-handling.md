# ADR-003: Structured Error Handling with Context Propagation

**Status:** Accepted  
**Date:** 2026-04-04  
**Author:** Phenotype Architecture Team  
**Reviewers:** Core Engineering Team

## Context

The DinoforgeMcp server requires a comprehensive error handling strategy that:

1. **Complies with MCP spec:** JSON-RPC 2.0 error objects
2. **Provides debugging context:** Stack traces, execution context
3. **Supports distributed tracing:** Request IDs, correlation IDs
4. **Enables programmatic handling:** Error codes, typed exceptions
5. **Prevents information leakage:** Safe error exposure to clients

The key question is how to balance detailed internal error information with safe external exposure.

## Decision

We will implement a **layered error handling architecture** with three distinct error representations:

```
┌─────────────────────────────────────────────────────────────────────┐
│                      ERROR HIERARCHY                                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Layer 1: INTERNAL                                                  │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  Exception with full context                                   │ │
│  │  • Stack trace                                                 │ │
│  │  • Local variables                                             │ │
│  │  • Execution context                                           │ │
│  │  • Internal state dump                                         │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                          │                                          │
│                          ▼ Transform                                 │
│  Layer 2: PROTOCOL                                                    │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  JSON-RPC 2.0 error object                                     │ │
│  │  • code: -32602 (Invalid params)                               │ │
│  │  • message: Human-readable description                         │ │
│  │  • data: Structured error details (sanitized)                  │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                          │                                          │
│                          ▼ Sanitize                                  │
│  Layer 3: CLIENT                                                    │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  Safe for external transmission                                │ │
│  │  • Generic error message                                       │ │
│  │  • Request ID for log correlation                              │ │
│  │  • No internal details                                         │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Error Code System

**MCP Standard Codes:**

| Code | Name | Description |
|------|------|-------------|
| -32700 | Parse error | Invalid JSON received |
| -32600 | Invalid Request | JSON is not a valid Request object |
| -32601 | Method not found | Method does not exist |
| -32602 | Invalid params | Invalid method parameters |
| -32603 | Internal error | Internal JSON-RPC error |

**Server-Specific Codes:**

| Code | Name | Description |
|------|------|-------------|
| -32000 | Tool not found | Requested tool does not exist |
| -32001 | Tool execution failed | Tool raised exception |
| -32002 | Validation error | Input schema validation failed |
| -32003 | Rate limit exceeded | Too many requests |
| -32004 | Unauthorized | Insufficient permissions |
| -32005 | Resource not found | Requested resource unavailable |
| -32006 | Timeout | Operation exceeded time limit |
| -32007 | Transport error | Low-level communication failure |

## Consequences

### Positive

1. **Spec compliance:** Strict adherence to JSON-RPC 2.0 error format
2. **Debugging power:** Full context for developers via logging
3. **Security:** Internal details never exposed to untrusted clients
4. **Observability:** Trace IDs link errors across distributed systems
5. **Client experience:** Actionable error messages for end users

### Negative

1. **Complexity:** Three-layer transformation adds code overhead
2. **Performance:** Context capture has minor runtime cost
3. **Storage:** Detailed logs require retention planning

### Mitigations

| Concern | Mitigation |
|---------|------------|
| Overhead | Lazy context capture; only on error path |
| Log volume | Sampling for high-frequency errors |
| PII leakage | Automatic PII redaction in error data |

## Implementation

```python
# Internal exception hierarchy
class MCPError(Exception):
    """Base for all MCP-related errors."""
    
    def __init__(
        self,
        message: str,
        code: int = -32603,
        data: Optional[dict] = None,
        cause: Optional[Exception] = None
    ):
        super().__init__(message)
        self.code = code
        self.data = data or {}
        self.cause = cause
        self.trace_id = context.get_trace_id()
        self.timestamp = datetime.utcnow()
    
    def to_protocol_error(self) -> dict:
        """Convert to JSON-RPC error object."""
        return {
            "code": self.code,
            "message": self.message,
            "data": self._sanitize_data(self.data)
        }
    
    def _sanitize_data(self, data: dict) -> dict:
        """Remove sensitive information before transmission."""
        # Redact PII patterns
        redacted = {}
        for key, value in data.items():
            if self._is_sensitive_key(key):
                redacted[key] = "[REDACTED]"
            else:
                redacted[key] = value
        return redacted
    
    def _is_sensitive_key(self, key: str) -> bool:
        """Check if key may contain sensitive data."""
        sensitive_patterns = [
            'password', 'secret', 'token', 'key', 'credential',
            'auth', 'private', 'ssn', 'credit_card'
        ]
        return any(pattern in key.lower() for pattern in sensitive_patterns)

# Specific error types
class ToolNotFoundError(MCPError):
    """Tool does not exist in registry."""
    
    def __init__(self, tool_name: str):
        super().__init__(
            message=f"Tool not found: {tool_name}",
            code=-32000,
            data={"tool_name": tool_name}
        )

class ValidationError(MCPError):
    """Input validation failed."""
    
    def __init__(self, errors: List[dict]):
        super().__init__(
            message="Input validation failed",
            code=-32002,
            data={"validation_errors": errors}
        )

class ToolExecutionError(MCPError):
    """Tool execution raised exception."""
    
    def __init__(self, tool_name: str, cause: Exception):
        super().__init__(
            message=f"Tool '{tool_name}' execution failed",
            code=-32001,
            data={
                "tool_name": tool_name,
                "error_type": type(cause).__name__,
                "error_message": str(cause)
            },
            cause=cause
        )

# Error handler with context propagation
class ErrorHandler:
    """Centralized error handling with logging."""
    
    def __init__(self, logger: logging.Logger, include_trace: bool = False):
        self.logger = logger
        self.include_trace = include_trace
    
    async def handle(
        self,
        error: Exception,
        context: ExecutionContext
    ) -> dict:
        """Transform exception to client-safe error response."""
        
        # Log full details internally
        self._log_error(error, context)
        
        # Convert to MCPError if needed
        if not isinstance(error, MCPError):
            mcp_error = self._wrap_exception(error, context)
        else:
            mcp_error = error
        
        # Return protocol-safe error
        protocol_error = mcp_error.to_protocol_error()
        
        # Add trace ID for log correlation
        protocol_error["data"]["trace_id"] = context.trace_id
        
        return protocol_error
    
    def _log_error(self, error: Exception, context: ExecutionContext):
        """Log full error details for debugging."""
        self.logger.error(
            "MCP error occurred",
            extra={
                "trace_id": context.trace_id,
                "tool_name": context.tool_name,
                "error_type": type(error).__name__,
                "error_message": str(error),
                "stack_trace": traceback.format_exc(),
                "context": context.to_dict()
            }
        )
    
    def _wrap_exception(self, error: Exception, context: ExecutionContext) -> MCPError:
        """Wrap unknown exceptions in MCPError."""
        return MCPError(
            message="An unexpected error occurred",
            code=-32603,
            data={
                "trace_id": context.trace_id,
                "request_id": context.request_id
            },
            cause=error
        )
```

## Usage Patterns

**In Tool Implementation:**
```python
async def database_query(arguments: dict, context: ExecutionContext) -> ToolResult:
    """Execute database query with proper error handling."""
    
    # Validation
    if "query" not in arguments:
        raise ValidationError([{
            "field": "query",
            "message": "Query parameter is required"
        }])
    
    # Check permissions
    if not context.identity.can_access("database"):
        raise MCPError(
            message="Access denied to database",
            code=-32004,
            data={"required_permission": "database"}
        )
    
    # Execute with timeout
    try:
        result = await asyncio.wait_for(
            db.execute(arguments["query"]),
            timeout=30.0
        )
        return ToolResult(content=[TextContent(text=str(result))])
    except asyncio.TimeoutError:
        raise MCPError(
            message="Query execution timed out",
            code=-32006,
            data={"timeout_seconds": 30}
        )
    except DatabaseError as e:
        # Convert to safe error - don't expose SQL details
        raise ToolExecutionError("database_query", e)
```

**In Protocol Handler:**
```python
async def handle_request(self, request: dict) -> dict:
    """Handle JSON-RPC request with error wrapping."""
    try:
        result = await self._execute_request(request)
        return {"jsonrpc": "2.0", "id": request["id"], "result": result}
    except MCPError as e:
        # Known error - convert to protocol format
        return {
            "jsonrpc": "2.0",
            "id": request["id"],
            "error": e.to_protocol_error()
        }
    except Exception as e:
        # Unknown error - wrap and convert
        wrapped = await self.error_handler.handle(e, self.context)
        return {
            "jsonrpc": "2.0",
            "id": request["id"],
            "error": wrapped
        }
```

## Alternatives Considered

### Alternative A: Generic Errors Only
Return only generic error messages to all clients.

**Rejected:** Impossible to debug production issues without detailed context.

### Alternative B: Full Stack Traces to Clients
Include complete error details in all responses.

**Rejected:** Security risk; exposes internal implementation details.

### Alternative C: HTTP Status Code Mapping
Map JSON-RPC errors to HTTP status codes for HTTP transport.

**Rejected:** Violates MCP spec; spec mandates JSON-RPC error objects.

## Security Considerations

**Automatic Redaction:**
```python
REDACTED_PATTERNS = [
    r'[\w.-]+@[\w.-]+\.\w+',           # Email addresses
    r'\b\d{3}-\d{2}-\d{4}\b',          # SSN
    r'\b\d{4}[ -]?\d{4}[ -]?\d{4}[ -]?\d{4}\b',  # Credit cards
    r'password["\']?\s*[:=]\s*["\']?[^"\']+',  # Password assignments
    r'eyJ[a-zA-Z0-9_-]*\.eyJ[a-zA-Z0-9_-]*',  # JWT tokens
]

def redact_sensitive_data(text: str) -> str:
    """Remove sensitive patterns from error messages."""
    for pattern in REDACTED_PATTERNS:
        text = re.sub(pattern, '[REDACTED]', text)
    return text
```

## Related Decisions

- ADR-001: Multi-Transport Architecture
- ADR-002: Dynamic Tool Registry
- SPEC.md: Error Handling Section

## References

1. [JSON-RPC 2.0 Error Objects](https://www.jsonrpc.org/specification#error_object)
2. [OWASP Error Handling](https://cheatsheetseries.owasp.org/cheatsheets/Error_Handling_Cheat_Sheet.html)
3. [nanovms Security Guidelines](https://github.com/nanovms/nanos/blob/master/SECURITY.md)

---

*Decision record maintained under src/docs/adr/*
