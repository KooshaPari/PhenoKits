# PRD — phenotype-agent-core

## Overview

`phenotype-agent-core` is the foundational agent framework library for the Phenotype platform. It provides the core abstractions for building, composing, and running AI agents: task graphs, tool registries, memory stores, and execution loops.

## Goals

- Define the canonical agent model used across all Phenotype agent implementations.
- Enable composable agent pipelines with reusable tool and memory adapters.
- Provide observability hooks (tracing, logging) for all agent operations.

## Epics

### E1 — Agent Model
- E1.1 `Agent` trait/protocol with `run(task) -> Result`.
- E1.2 `Task` type representing a unit of agent work with inputs and expected outputs.
- E1.3 `AgentContext` carrying session state, memory, and tool registry.

### E2 — Tool Registry
- E2.1 `Tool` protocol: `name()`, `description()`, `execute(args) -> Result`.
- E2.2 Dynamic tool registration and lookup by name.
- E2.3 Tool argument validation using JSON Schema.

### E3 — Memory
- E3.1 `Memory` port: `store(key, value)`, `retrieve(key)`, `search(query)`.
- E3.2 In-memory adapter for development and testing.
- E3.3 Semantic search adapter stub for vector-store integration.

### E4 — Execution Loop
- E4.1 ReAct-style (Reason + Act) execution loop.
- E4.2 Configurable max iterations with graceful termination.
- E4.3 Step-level event emission for observability.

### E5 — Observability
- E5.1 Emit span for each agent step using helix-tracing.
- E5.2 Structured log events for task start, tool call, memory access, and task end.
- E5.3 Metrics: step count, tool call count, tokens used, wall-clock duration.

## Acceptance Criteria

- An agent with a single tool can complete a task using the ReAct loop.
- Tool calls are traced as child spans of the agent task span.
- Max iteration limit terminates the loop with a clear error, not a hang.
