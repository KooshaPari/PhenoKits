# ADR-003 — Structured Logging with structlog for Observability

**Status:** Accepted
**Date:** 2026-04-04
**Decider:** Phenotype Core Team

## Context

browser-agent-mcp requires comprehensive observability for debugging, monitoring, and auditing. The browser automation domain presents unique challenges:

- **Complex async flows:** Multiple concurrent browser contexts
- **Security requirements:** Audit trail for all browser actions
- **Debugging needs:** Reproduce issues from production logs
- **Integration complexity:** MCP protocol + Playwright + Browser

**Logging Options:**
- Python standard `logging` — Built-in, widely used
- `structlog` — Structured logging with JSON output
- `loguru` — Modern logging with better ergonomics
- OpenTelemetry — Distributed tracing standard

**Log Format Options:**
- Text (human-readable) — Easy to read
- JSON (machine-parseable) — Easy to query
- Both — Flexibility with overhead

## Decision

browser-agent-mcp adopts **structlog** with **JSON output** as the primary logging solution, with optional text output for development.

## Rationale

### Logging Library Comparison

| Feature | stdlib logging | structlog | loguru | OpenTelemetry |
|---------|---------------|-----------|--------|---------------|
| Structured output | ⚠️ Manual | ✅ Native | ⚠️ Manual | ✅ Native |
| JSON rendering | ⚠️ Custom | ✅ Built-in | ⚠️ Custom | ✅ Built-in |
| Context binding | ❌ | ✅ | ❌ | ✅ |
| Async support | ⚠️ | ✅ | ✅ | ✅ |
| Performance | Good | Good | Good | Moderate |
| Ecosystem | Excellent | Good | Moderate | Growing |
| Type hints | Poor | Good | Moderate | Good |

### Structured Logging Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│              STRUCTURED LOGGING ARCHITECTURE                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Application Code                                               │
│  ├─ logger.info("browser_navigate", url=url, user_id=user_id)   │
│  ├─ logger.bind(request_id=uuid).info("action_complete")       │
│  └─ logger.error("navigation_failed", exception=e)              │
│                                                                 │
│  structlog Pipeline                                             │
│  ├─ Processor 1: Add timestamp (ISO 8601)                     │
│  ├─ Processor 2: Add log level                                 │
│  ├─ Processor 3: Add logger name                               │
│  ├─ Processor 4: Render exceptions                               │
│  └─ Processor 5: Add stack trace (on error)                     │
│                                                                 │
│  Output Renderers                                               │
│  ├─ JSON Renderer ──────────► Production                     │
│  │   {"timestamp": "2026-04-04T10:30:00Z",                      │
│  │    "level": "info",                                          │
│  │    "event": "browser_navigate",                               │
│  │    "url": "https://example.com",                             │
│  │    "user_id": "usr_123",                                     │
│  │    "request_id": "req_456"}                                 │
│  │                                                             │
│  └─ Console Renderer ─────────► Development                     │
│      2026-04-04 10:30:00 [info] browser_navigate              │
│      url=https://example.com user_id=usr_123                   │
│                                                                 │
│  Destinations                                                   │
│  ├─ stderr (local development)                                  │
│  ├─ File (production persistence)                               │
│  ├─ syslog (centralized aggregation)                           │
│  └─ Log aggregator (Datadog, Splunk, etc.)                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Why structlog over stdlib logging

1. **Native Structured Output**
   ```python
   # stdlib logging (manual JSON)
   import json
   import logging
   logger = logging.getLogger(__name__)
   logger.info(json.dumps({
       "event": "browser_navigate",
       "url": url,
       "timestamp": datetime.utcnow().isoformat()
   }))
   
   # structlog (native)
   import structlog
   logger = structlog.get_logger()
   logger.info("browser_navigate", url=url)
   ```

2. **Context Binding**
   ```python
   # Bind context once, use throughout
   request_logger = logger.bind(
       request_id=str(uuid.uuid4()),
       user_id=current_user.id,
       session_id=browser_session.id
   )
   
   # All subsequent logs include context
   request_logger.info("browser_launching")
   request_logger.info("page_navigating", url=url)
   request_logger.info("action_complete", duration_ms=150)
   
   # Output:
   # {"event": "browser_launching", "request_id": "...", "user_id": "...", "session_id": "..."}
   # {"event": "page_navigating", "request_id": "...", "url": "..."}
   # {"event": "action_complete", "request_id": "...", "duration_ms": 150}
   ```

3. **Type Safety**
   ```python
   from structlog import get_logger
   from typing import TYPE_CHECKING
   
   if TYPE_CHECKING:
       from structlog.stdlib import BoundLogger
   
   logger: "BoundLogger" = get_logger()
   # Full type hints in IDE
   ```

### Why JSON over Text

**Production Requirements:**

| Requirement | Text | JSON |
|-------------|------|------|
| Machine parsing | ❌ Regex fragile | ✅ Native |
| Log aggregation | ⚠️ Custom parsers | ✅ Standard |
| Query/filtering | ❌ Limited | ✅ Full structure |
| Correlation IDs | ⚠️ Regex extraction | ✅ Field match |
| Alerting | ⚠️ Pattern matching | ✅ Field conditions |
| Compliance audit | ❌ Hard to prove | ✅ Immutable |

**Example Queries:**

```bash
# Text log - fragile parsing
grep "browser_navigate" app.log | \
  sed 's/.*url=\([^ ]*\).*/\1/' | \
  sort | uniq -c

# JSON log - precise querying
jq 'select(.event == "browser_navigate") | .url' app.json | \
  sort | uniq -c

# Datadog query
@event:browser_navigate @duration_ms:>1000
```

## Consequences

### Positive

1. **Queryability**
   - Precise filtering: `event:browser_navigate AND url:*github.com`
   - Aggregation: `stats count by event`
   - Correlation: `request_id:abc123` traces entire session

2. **Observability Integration**
   ```python
   # Automatic correlation with tracing
   from opentelemetry import trace
   import structlog
   
   def add_trace_info(logger, method_name, event_dict):
       span = trace.get_current_span()
       if span:
           event_dict["trace_id"] = format(span.get_span_context().trace_id, "032x")
           event_dict["span_id"] = format(span.get_span_context().span_id, "016x")
       return event_dict
   
   structlog.configure(
       processors=[
           add_trace_info,
           structlog.processors.TimeStamper(fmt="iso"),
           structlog.processors.JSONRenderer()
       ]
   )
   ```

3. **Security Auditing**
   ```python
   # Immutable audit trail
   logger.info(
       "security_audit",
       action="browser_navigate",
       user_id=user.id,
       ip_address=request.ip,
       url=url,
       allowed_domains=allowed_list,
       timestamp=datetime.utcnow().isoformat()
   )
   ```

4. **Debugging Efficiency**
   - Structured stack traces
   - Context propagation across async boundaries
   - Request correlation across services

### Negative

1. **Human Readability**
   - Raw JSON harder to scan than text
   - Requires tools (jq, log viewer) for analysis

2. **Storage Overhead**
   - JSON structure adds ~20% size vs text
   - Field names repeated in every log

3. **Learning Curve**
   - Team must learn structlog API
   - Context binding discipline required

### Mitigations

| Issue | Mitigation |
|-------|------------|
| Readability | Console renderer for development; JSON for production |
| Storage | Log rotation; sampling for high-volume logs |
| Learning | Documentation; code review enforcement |

## Implementation

### Configuration

```python
# browser_agent/logging_config.py
import structlog
import logging
import sys
from pathlib import Path

def configure_logging(json_output: bool = True, log_level: str = "INFO"):
    """Configure structured logging for browser-agent-mcp."""
    
    shared_processors = [
        # Add timestamp
        structlog.processors.TimeStamper(fmt="iso", utc=True),
        # Add log level
        structlog.processors.add_log_level,
        # Format exceptions
        structlog.processors.format_exc_info,
        # Add caller info
        structlog.processors.CallsiteParameterAdder(
            {
                structlog.processors.CallsiteParameter.FILENAME,
                structlog.processors.CallsiteParameter.FUNC_NAME,
                structlog.processors.CallsiteParameter.LINENO,
            }
        ),
    ]
    
    if json_output:
        # Production: JSON output
        renderer = structlog.processors.JSONRenderer()
    else:
        # Development: Pretty console output
        renderer = structlog.dev.ConsoleRenderer(
            colors=True,
            sort_keys=False
        )
    
    structlog.configure(
        processors=shared_processors + [renderer],
        wrapper_class=structlog.make_filtering_bound_logger(
            getattr(logging, log_level)
        ),
        context_class=dict,
        logger_factory=structlog.PrintLoggerFactory(),
        cache_logger_on_first_use=True,
    )
    
    # Configure stdlib logging to use structlog
    logging.basicConfig(
        format="%(message)s",
        stream=sys.stderr,
        level=getattr(logging, log_level),
    )
```

### Usage Patterns

```python
# browser_agent/server.py
import structlog
from contextlib import asynccontextmanager

logger = structlog.get_logger()

class BrowserManager:
    def __init__(self):
        self._logger = logger.bind(component="browser_manager")
    
    async def launch(self):
        self._logger.info("browser_launching")
        try:
            # ... launch logic
            self._logger.info("browser_launched", duration_ms=elapsed)
        except Exception as e:
            self._logger.error("browser_launch_failed", error=str(e))
            raise
    
    @asynccontextmanager
    async def page(self):
        page_logger = self._logger.bind(page_id=str(uuid.uuid4()))
        page_logger.info("page_creating")
        page = await self._browser.new_page()
        try:
            page_logger.info("page_created")
            yield page
        finally:
            page_logger.info("page_closing")
            await page.close()

# Tool handlers with per-request context
@server.call_tool()
async def handle_tool(name: str, arguments: dict):
    request_id = str(uuid.uuid4())
    request_logger = logger.bind(
        request_id=request_id,
        tool_name=name,
        arguments=arguments  # Be careful with PII
    )
    
    request_logger.info("tool_call_started")
    
    try:
        result = await execute_tool(name, arguments, request_logger)
        request_logger.info("tool_call_completed", duration_ms=elapsed)
        return result
    except Exception as e:
        request_logger.error(
            "tool_call_failed",
            error=str(e),
            error_type=type(e).__name__
        )
        raise
```

### Security Audit Logging

```python
# browser_agent/security.py
import structlog
from datetime import datetime

audit_logger = structlog.get_logger("audit")

def log_security_event(
    action: str,
    user_id: str,
    resource: str,
    outcome: str,
    details: dict | None = None
):
    """Log security-relevant events for compliance."""
    audit_logger.info(
        "security_event",
        timestamp=datetime.utcnow().isoformat(),
        action=action,
        user_id=user_id,
        resource=resource,
        outcome=outcome,
        details=details or {}
    )

# Usage
log_security_event(
    action="browser_navigate",
    user_id=current_user.id,
    resource=url,
    outcome="allowed" if is_allowed else "denied",
    details={
        "domain": parsed.netloc,
        "allowed_domains": ALLOWED_DOMAINS,
        "headers_redacted": True
    }
)
```

### Performance Logging

```python
# browser_agent/performance.py
import time
import structlog
from functools import wraps

perf_logger = structlog.get_logger("performance")

def timed(operation_name: str):
    """Decorator to log operation timing."""
    def decorator(func):
        @wraps(func)
        async def wrapper(*args, **kwargs):
            start = time.monotonic()
            try:
                result = await func(*args, **kwargs)
                elapsed = (time.monotonic() - start) * 1000
                perf_logger.info(
                    "operation_completed",
                    operation=operation_name,
                    duration_ms=round(elapsed, 2),
                    success=True
                )
                return result
            except Exception as e:
                elapsed = (time.monotonic() - start) * 1000
                perf_logger.warning(
                    "operation_failed",
                    operation=operation_name,
                    duration_ms=round(elapsed, 2),
                    success=False,
                    error_type=type(e).__name__
                )
                raise
        return wrapper
    return decorator

# Usage
@timed("browser_navigate")
async def browser_navigate(page, url: str):
    await page.goto(url)
```

## Log Schema

### Standard Fields

Every log entry includes:

```json
{
  "timestamp": "2026-04-04T10:30:00.123456Z",
  "level": "info",
  "event": "browser_navigate",
  "logger": "browser_agent.server"
}
```

### Event-Specific Fields

| Event | Additional Fields |
|-------|-------------------|
| `server_starting` | `transport`, `version`, `config` |
| `server_stopping` | `reason`, `uptime_seconds` |
| `tool_call_started` | `request_id`, `tool_name` |
| `tool_call_completed` | `request_id`, `duration_ms` |
| `tool_call_failed` | `request_id`, `error`, `error_type` |
| `browser_launching` | `browser_type`, `headless` |
| `browser_launched` | `duration_ms`, `version` |
| `page_navigating` | `url`, `wait_until` |
| `page_navigated` | `url`, `status_code`, `duration_ms` |
| `security_event` | `action`, `user_id`, `resource`, `outcome` |

## Alternatives Considered

### Standard Library Logging

**Pros:**
- No additional dependency
- Well-known API
- Extensive documentation

**Cons:**
- Manual JSON formatting
- No native context binding
- Verbose configuration

**Decision:** Rejected — structlog's ergonomics worth the dependency

### Loguru

**Pros:**
- Modern, ergonomic API
- Better than stdlib
- Nice color output

**Cons:**
- No native structured output
- Smaller ecosystem than structlog
- Similar capabilities, less adoption

**Decision:** Rejected — structlog has better structured logging support

### OpenTelemetry Direct

**Pros:**
- Industry standard for distributed tracing
- Vendor-neutral
- Automatic instrumentation

**Cons:**
- Higher complexity
- Overkill for single-server MCP
- Additional infrastructure (collector)

**Decision:** Rejected — Can add OTel integration to structlog later if needed

## Integration with MCP Protocol

MCP protocol errors are logged with appropriate severity:

```python
# Error mapping
ERROR_SEVERITY = {
    "ParseError": "error",
    "InvalidRequest": "warning",
    "MethodNotFound": "warning",
    "InvalidParams": "warning",
    "InternalError": "error",
    "ServerError": "error",
}

@server.error_handler()
async def handle_error(error: mcp.Error):
    severity = ERROR_SEVERITY.get(error.code, "error")
    logger.log(
        severity,
        "mcp_error",
        code=error.code,
        message=error.message,
        data=error.data
    )
```

## References

- [structlog Documentation](https://www.structlog.org/)
- [Structured Logging Best Practices](https://docs.datadoghq.com/logs/guide/best-practices-for-log-management/)
- [MCP Specification - Error Handling](https://modelcontextprotocol.io/specification/errors)
- [OpenTelemetry Logging](https://opentelemetry.io/docs/concepts/signals/logs/)

---

*Supersedes: N/A*
*Superseded by: N/A*
