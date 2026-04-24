# FUNCTIONAL_REQUIREMENTS — portkey

## FR-REG-001: Declarative Port Registry
**SHALL** parse a `portkey.yaml` (or `portkey.toml`) config declaring `service: port` mappings.
Traces to: E1.1

## FR-REG-002: Conflict Detection
**SHALL** fail with a non-zero exit and list all conflicting port assignments if two services claim the same port.
Traces to: E1.2

## FR-REG-003: Auto-allocation
**SHALL** support `port: auto` in config; assign next free port in configured range `[base, base+range)`.
Traces to: E1.3

## FR-ROUTE-001: Routing Rules
**SHALL** support routing rules with fields: `host`, `path_prefix`, `method`, `upstream`.
Traces to: E2.1

## FR-ROUTE-002: Load Balancing
**SHALL** support `strategy: round_robin | least_conn | sticky_session` per route.
Traces to: E2.2

## FR-ROUTE-003: Health Check
**SHALL** perform HTTP health checks on upstreams at configurable intervals; remove unhealthy upstreams from rotation.
Traces to: E2.3

## FR-KEY-001: Key Issuance
**SHALL** issue HMAC-signed API keys with embedded scope (route pattern) and expiry.
Traces to: E3.1

## FR-KEY-002: Key Validation
**SHALL** validate keys on every request; return HTTP 401 with `{"error":"invalid_key"}` on failure.
Traces to: E3.2

## FR-KEY-003: Key Rotation
**SHALL** support overlapping validity windows during rotation (old key valid for configurable grace period).
Traces to: E3.3

## FR-CLI-001: CLI Commands
**SHALL** provide `portkey list`, `portkey assign <service> <port>`, `portkey revoke <key-id>`.
Traces to: E4.1

## FR-CLI-002: Environment Export
**SHALL** support `portkey env` that outputs `EXPORT SERVICE_PORT=N` lines suitable for shell eval.
Traces to: E4.3
