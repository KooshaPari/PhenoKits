# src/ Charter

## Mission

The `src/` directory provides foundational infrastructure for AI-native tooling across the Phenotype ecosystem. It implements the Model Context Protocol (MCP) specification, enabling secure, standardized integration between AI systems and external tools, data sources, and services.

## Tenets (unless you know better ones)

These tenets guide development:

### 1. Simplicity

Keep the core small and focused. Each component has a single, well-defined responsibility. Prefer composition over inheritance. Avoid premature abstraction.

### 2. Performance

Optimize for the common case. Stdio transport adds <1ms overhead. HTTP transport handles >1000 req/s per core.

### 3. Security

Never expose internal error details to clients. All inputs validated against schemas. Authentication required for network transports.

### 4. Observability

Every operation must be traceable. Distributed tracing IDs propagate through all layers. Structured logging for all significant events.

### 5. Compatibility

Follow the MCP specification exactly. No protocol extensions without specification updates. Version negotiation on initialization.

## Contributions & Project Roles

All contributions must align with this charter.

---

*See SPEC.md for technical details.*
*See ROADMAP.md for planned features.*
