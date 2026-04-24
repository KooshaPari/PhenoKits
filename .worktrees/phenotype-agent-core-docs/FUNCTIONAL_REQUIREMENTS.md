# Functional Requirements — phenotype-agent-core

## FR-AGENT — Agent Model

| ID | Requirement |
|----|-------------|
| FR-AGENT-001 | The library SHALL define an Agent protocol with a run(task, ctx) -> Result interface. |
| FR-AGENT-002 | The library SHALL define a Task type with input fields, expected output schema, and metadata. |
| FR-AGENT-003 | The library SHALL define an AgentContext carrying session ID, memory, and tool registry. |
| FR-AGENT-004 | AgentContext SHALL be immutable from the agent perspective; mutations go through explicit methods. |

## FR-TOOL — Tool Registry

| ID | Requirement |
|----|-------------|
| FR-TOOL-001 | The library SHALL define a Tool protocol with name(), description(), and execute(args) methods. |
| FR-TOOL-002 | The library SHALL provide a ToolRegistry supporting dynamic registration and name-based lookup. |
| FR-TOOL-003 | The library SHALL validate tool arguments against a JSON Schema before execution. |
| FR-TOOL-004 | Tool execution failures SHALL produce a typed ToolError, not a panic or generic exception. |

## FR-MEM — Memory

| ID | Requirement |
|----|-------------|
| FR-MEM-001 | The library SHALL define a Memory port with store, retrieve, and search methods. |
| FR-MEM-002 | The library SHALL provide an in-memory Memory adapter for development and tests. |
| FR-MEM-003 | The Memory port SHALL support semantic search as an optional capability. |

## FR-LOOP — Execution Loop

| ID | Requirement |
|----|-------------|
| FR-LOOP-001 | The library SHALL implement a ReAct-style execution loop (reason, act, observe). |
| FR-LOOP-002 | The execution loop SHALL respect a configurable max_iterations limit. |
| FR-LOOP-003 | Exceeding max_iterations SHALL terminate the loop with a MaxIterationsError. |
| FR-LOOP-004 | Each loop step SHALL emit a step event observable by subscribers. |

## FR-OBS — Observability

| ID | Requirement |
|----|-------------|
| FR-OBS-001 | The library SHALL emit a trace span for each agent step using helix-tracing. |
| FR-OBS-002 | The library SHALL emit structured log events for task start, tool call, memory access, and task end. |
| FR-OBS-003 | The library SHALL track step count, tool call count, and wall-clock duration as metrics. |
