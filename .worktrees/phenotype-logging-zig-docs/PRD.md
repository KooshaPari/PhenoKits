# PRD — phenotype-logging-zig

**Version:** 1.0.0
**Stack:** Zig 0.15.x
**Repo:** `KooshaPari/phenotype-logging-zig`
**Status:** Archived (2026-03-25) — candidate for migration to `libs/hexagonal-rs` or neutral crate

---

## Overview

`phenotype-logging-zig` is a structured logging library for Zig applications in the Phenotype
ecosystem. It provides zero-allocation log formatting, compile-time log-level filtering,
JSON-serialisable log records, and a port/adapter interface so the transport backend
(stderr, file, network sink) can be swapped without touching call sites.

---

## Epics

### E1 — Core Logging Engine
**Goal:** Zero-overhead structured logging that compiles away at `ReleaseFast`.

#### E1.1 Log Levels
- As a developer, I want standard log levels (trace, debug, info, warn, err, fatal) so I can
  filter signal from noise.
- **Acceptance:** Levels compile-time filtered via `std.log.Level`; disabled levels emit no code.

#### E1.2 Structured Fields
- As an operator, I want key-value fields attached to log records so log aggregators can index them.
- **Acceptance:** `log.info("msg", .{ .request_id = id, .latency_ms = ms })` emits structured JSON.

#### E1.3 Zero-Allocation Hot Path
- **Acceptance:** Log call in hot path allocates 0 bytes when output is below active level.

### E2 — Transport Adapters
**Goal:** Pluggable output sinks via the hexagonal port interface.

#### E2.1 Stderr Adapter
- **Acceptance:** Default adapter writes colourised human-readable output to stderr.

#### E2.2 JSON File Adapter
- **Acceptance:** File adapter writes newline-delimited JSON to a configured path with rotation.

#### E2.3 Custom Adapter
- As a library consumer, I want to implement a custom transport by satisfying a comptime interface.
- **Acceptance:** `comptime` interface check at compile time; mismatched adapter causes descriptive error.
