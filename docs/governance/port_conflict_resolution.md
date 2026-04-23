---
title: Port conflict resolution (2026-04-22 observed state)
scope: AgilePlus dev stack + OrbStack + native services coexistence
companion: shared_process_policy.md
---

# Port conflict resolution

Live snapshot from agent audit 2026-04-22 03:28:

```
:5432  postgres  10405 (native)   LISTEN
       OrbStack  10462            LISTEN  ← double-bind
:6379  OrbStack  10462            LISTEN
:9000  OrbStack  10462            LISTEN

brew services: postgresql@17 started
OrbStack containers running: only `headscale` (VPN, not postgres/redis/minio)
```

**Root observation:** OrbStack is holding 5432/6379/9000 even though none of its running containers need them. These are phantom host-port forwards left by previously-stopped containers (probably an earlier plane-postgres / dev-redis / sonarqube container that exited without releasing its forwards).

**Double-bind on 5432** is accepted by the kernel because the two sockets bind to different addresses (native `[::1]` + `127.0.0.1` loopback only; OrbStack `*` wildcard). The native postgres serves localhost clients fine; the OrbStack binding captures any non-loopback traffic. `task dev:up` spinning a new plane-postgres container on the default port will fail to bind because OrbStack already holds `*:5432`.

## Resolution paths (pick one)

### 1. Preferred — AgilePlus dev reuses native postgres (first-to-need)

Update `AgilePlus` plane-postgres config to probe `lsof -iTCP:5432` at start; if native `postgres` is bound, skip launching the container and connect to the existing instance with a different DB name (`CREATE DATABASE agileplus_dev;`). This honors `shared_process_policy.md` and stops fighting the port. Per-project WIP, not done here.

### 2. Prune OrbStack phantom forwards

```bash
orb restart                          # cleanest — drops stale port-forwards
# confirm:
lsof -nP -iTCP:5432 -sTCP:LISTEN
lsof -nP -iTCP:6379 -sTCP:LISTEN
lsof -nP -iTCP:9000 -sTCP:LISTEN
# expect only: native postgres on 5432, nothing on 6379/9000
```

Restarts headscale (user's VPN) — ~5 second outage. Non-destructive.

### 3. Move AgilePlus plane-postgres to a non-standard port

Edit AgilePlus `process-compose.yml` (or equivalent): `plane-postgres.port: 5433`. Adjust plane-api + agileplus-api connection strings accordingly. Harder to maintain than #1; choose only if #2 isn't acceptable.

## Why agent did not auto-stop

- `brew services stop postgresql@17` would terminate a service other dev tools and migrations on this host may actively depend on. Destructive, no way to know without asking user.
- Touching OrbStack kills the active `headscale` container briefly; user may have clients attached via that VPN.
- Scripting-policy + safety rules: no unilateral destructive services actions.

## Recommended next action for user

```bash
orb restart                          # clears phantom forwards, brief headscale blip
task dev:up                          # should now succeed
```

If plane-postgres still can't bind, fall to resolution #1 or #3.
