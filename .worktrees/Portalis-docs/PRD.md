# PRD — portkey

## Overview
portkey is an API routing and port management key system for the Phenotype platform. It provides deterministic, declarative routing rules for service ports, API gateway configuration, and key-based access control for internal service mesh communication.

## Epics

### E1 — Port Registry
**E1.1** Maintain a declarative registry of service ports across Phenotype services.
**E1.2** Conflict detection: fail loudly if two services claim the same port.
**E1.3** Port allocation: auto-assign free ports from a configured range when not explicitly declared.

### E2 — API Routing Rules
**E2.1** Define routing rules (host, path prefix, method) to downstream services.
**E2.2** Load balancing strategies per route (round-robin, least-conn, sticky).
**E2.3** Route health checking and automatic failover.

### E3 — Key-Based Access Control
**E3.1** Issue signed API keys scoped to specific routes or services.
**E3.2** Key validation middleware: reject requests with invalid or expired keys with HTTP 401.
**E3.3** Key rotation with zero-downtime overlap window.

### E4 — Developer Tooling
**E4.1** CLI: `portkey list`, `portkey assign`, `portkey revoke`.
**E4.2** Config file format (YAML/TOML) for declarative port and route definitions.
**E4.3** Export port map to environment variable format for service startup.

## Acceptance Criteria
- Port conflicts are detected at config load time, not at runtime.
- Invalid API keys produce HTTP 401 with a clear error body; no silent pass-through.
- Port registry exports are deterministic and idempotent.
