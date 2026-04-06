# ADR — portkey

## ADR-001: Config Format
**Status:** Accepted
**Context:** Need a human-readable, VCS-friendly format for port and route declarations.
**Decision:** YAML as primary format with TOML as secondary. JSON schema validation on load.
**Rationale:** YAML is idiomatic in Kubernetes/cloud tooling; JSON Schema provides machine-readable contract.

## ADR-002: Key Signing Algorithm
**Status:** Accepted
**Context:** API keys must be tamper-proof and efficiently validated without a database lookup on every request.
**Decision:** HMAC-SHA256 with a per-environment secret. Keys are JWT-structured with `scope` and `exp` claims.
**Rationale:** Stateless validation; no round-trip to a key store on hot path.
**Alternatives considered:** Asymmetric (RSA/EC) — overkill for internal service mesh; database-backed tokens — too slow.

## ADR-003: Port Conflict Policy
**Status:** Accepted
**Context:** Port conflicts cause hard-to-debug runtime failures.
**Decision:** Conflicts are a startup-time hard error. The process exits non-zero and lists all conflicts before any port is bound.
**Rationale:** Fail-early, fail-clearly. No partial startup with conflicting ports.

## ADR-004: Routing Engine
**Status:** Accepted
**Context:** Need a routing layer that can be embedded or run as a sidecar.
**Decision:** Implement routing as a library with an optional standalone HTTP proxy binary. Consumers embed the library; the binary is for standalone deployments.
**Rationale:** Library-first keeps the codebase composable; the binary is a thin wrapper.
