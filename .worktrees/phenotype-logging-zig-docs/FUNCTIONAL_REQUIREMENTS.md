# Functional Requirements — phenotype-logging-zig

**Version:** 1.0.0
**Traces to:** PRD epics E1–E2

---

## FR-LOG-001 — Compile-Time Level Filtering
**SHALL** filter log levels at compile time so that log calls below the active level generate
no machine code in `ReleaseFast` builds.
**Traces to:** E1.1

## FR-LOG-002 — Level Hierarchy
**SHALL** implement levels in ascending severity: trace < debug < info < warn < err < fatal.
A call at level N is emitted only if N >= active level.
**Traces to:** E1.1

## FR-LOG-003 — Structured Key-Value Fields
**SHALL** accept an anonymous struct of key-value pairs as the second argument to each log
function and serialise them as part of the log record.
**Traces to:** E1.2

## FR-LOG-004 — JSON Serialisation
**SHALL** serialise log records to valid JSON when the JSON adapter is active, including
timestamp (RFC 3339), level, message, and all structured fields.
**Traces to:** E1.2

## FR-LOG-005 — Zero Allocation on Filtered Call
**SHALL** allocate zero heap bytes when a log call is filtered out by the active level.
**Traces to:** E1.3

## FR-LOG-006 — Build System Integration
**SHALL** expose the library via `build.zig` as a Zig module so consumers add it with
`b.addModule("phenotype-logging", .{ .root_source_file = .{ .path = "src/lib.zig" } })`.
**Traces to:** E1.1

## FR-LOG-007 — Stderr Adapter Default
**SHALL** default to the stderr adapter with ANSI colour codes when no adapter is configured.
**Traces to:** E2.1

## FR-LOG-008 — File Adapter with Rotation
**SHALL** provide a file adapter that rotates logs when the file exceeds a configurable size
threshold (default 10 MB) and retains a configurable number of rotated files (default 5).
**Traces to:** E2.2

## FR-LOG-009 — Comptime Adapter Interface
**SHALL** enforce the transport adapter interface at compile time using `comptime` duck-typing,
producing a descriptive compile error if the interface is not satisfied.
**Traces to:** E2.3
