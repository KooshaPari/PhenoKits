# Architecture Decision Records — phenotype-agent-core

## ADR-001 — ReAct Execution Loop

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Agent architectures vary (ReAct, Plan-and-Execute, MRKL, etc.). ReAct (Reason + Act) is the most widely validated pattern for tool-using agents and maps cleanly to LLM prompting patterns.

### Decision
The default execution loop is ReAct: Reason (generate next action from context), Act (execute tool), Observe (record result), repeat.

### Consequences
- Plan-and-Execute can be layered on top as a higher-order agent.
- The loop is generic over the LLM backend; the reasoning step is a port.

---

## ADR-002 — Tool Protocol as Interface, Not Base Class

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Base class inheritance for tools creates brittle hierarchies. Protocol/interface-based design is more composable.

### Decision
`Tool` is a protocol (Python) or trait (Rust). Any object implementing `name()`, `description()`, and `execute()` is a valid tool without inheritance.

### Consequences
- External tools (from other packages) work without inheriting from a base class.
- Adapter wrapping is simple: implement the protocol.

---

## ADR-003 — helix-tracing for Agent Observability

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Agent steps must be observable for debugging and performance analysis. helix-tracing is the standard observability library for the Phenotype ecosystem.

### Decision
Every agent step emits a span via helix-tracing. Tool calls are child spans. The session ID is propagated as a trace attribute.

### Consequences
- Agent execution is fully traceable in any OTel-compatible backend.
- helix-tracing is a required runtime dependency of phenotype-agent-core.
