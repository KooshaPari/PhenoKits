# Architecture Decision Records — phenotype-logging-zig

---

## ADR-001 — Zig as Implementation Language

**Date:** 2025-09-01
**Status:** Accepted

### Context
Phenotype required a logging library for Zig-based services. The Zig standard library provides
`std.log` but it lacks structured fields and pluggable transports.

### Decision
Implement a thin wrapper over `std.log` that adds structured key-value fields and a comptime
transport adapter interface. Use Zig 0.15.x as the baseline.

### Consequences
- Library is usable in any Zig project via `build.zig` module import.
- No external dependencies; pure Zig.
- Compile-time filtering is idiomatic and zero-overhead.

---

## ADR-002 — Hexagonal Port for Transport

**Date:** 2025-09-10
**Status:** Accepted

### Context
Per Phenotype hexagonal architecture mandate, I/O concerns (writing logs to disk, network, etc.)
must be isolated behind a port interface so the domain logic (log formatting) is testable without
real I/O.

### Decision
Define a `LogTransport` comptime interface with a single method `write(record: LogRecord) void`.
The logging engine calls only through this interface; adapters implement it.

### Consequences
- Tests inject a no-op or capture transport.
- New transports (syslog, OpenTelemetry) can be added without modifying the core.

---

## ADR-003 — build.zig Module Export

**Date:** 2025-09-15
**Status:** Accepted

### Context
Zig's package manager (introduced in 0.12) supports `build.zig.zon` for dependency declaration.
Older projects consumed libraries via path imports.

### Decision
Export the library as a named Zig module in `build.zig` so consumers can import it either via
`build.zig.zon` or direct path reference. Both paths are tested in CI.

### Consequences
- Consumers on 0.12+ use the package manager; older toolchains use path import.
- `zig build test` in this repo validates both the library and the module export.
