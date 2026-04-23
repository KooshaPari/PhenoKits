---
title: Shared-process policy (first-to-need)
scope: cross-repo (all Phenotype projects, all concurrent agent sessions)
owner: engineering (Argis + Helios sessions share this file)
---

# Shared-process policy

**Rule:** When multiple agents or CC sessions need the same long-running process (dev server, database, cache, ollama, orchestrator), the **first to need it launches; the rest reuse**. Only split-process when there is a real multi-tenancy concern; in that case, use the strategy below for that class of service.

This exists because port conflicts on :5432 / :6379 / :9000 / :3000 have bit multiple sessions, and restarts of shared dev stacks kill in-flight agent work.

## Discovery before launch (required)

Before launching any listed process, agents **must** probe for an existing instance:

```bash
# 1. Port probe — cheap, no false negatives.
lsof -nP -iTCP:<port> -sTCP:LISTEN 2>/dev/null

# 2. Process-name probe — catches background-bound stacks with no open port.
pgrep -af '<process-name-pattern>' | head

# 3. Project orchestrator — ask the source of truth if one exists.
process-compose process list --output table 2>/dev/null
pm2 list 2>/dev/null
```

If any probe finds a matching process, **attach / reuse**. Do not launch a second copy, do not restart, do not force-kill.

## Per-service strategy

| Service | Strategy | Notes |
|---|---|---|
| **VitePress `bun run dev`** (docs-site) | First-to-need launches on :5173. Rest check `lsof -iTCP:5173 -sTCP:LISTEN`; if present, just open browser to the existing URL. | Hot reload broadcasts to all connected clients — one server feeds many agent observations. |
| **Streamlit `streamlit run`** (journey apps) | First-to-need launches on :8501/:8502/etc. Rest check + reuse. If a specific journey requires an isolated app state (e.g., destructive tests), run on `--server.port=<ephemeral>` with tear-down. | Streamlit is single-tenant per session-state; isolation needed only when mutating `st.session_state`. |
| **Postgres** | Exactly ONE: native install (`brew services start postgresql@16`) OR OrbStack/Docker. Not both. Detect via `lsof -iTCP:5432`. If two bound, alert + ask user. | Native postgres conflicts with OrbStack postgres hit multiple times this week. Pick one platform per machine, document in repo README. |
| **Redis / DragonFly** | ONE bound on :6379. Mirror the postgres rule. | OrbStack redis vs dev-redis collision. |
| **MinIO / SonarQube** | One-off dev stacks. :9000 is hotly contested — OrbStack defaults to it. Move SonarQube to a different port via env var. | Detect via `lsof -iTCP:9000` + `curl -s localhost:9000/_health` to fingerprint which service is listening. |
| **Ollama / LMStudio / llama.cpp** | First-to-need launches on :11434 (ollama default). Rest probe + reuse. Model loads are expensive — never restart a running ollama to load a different model; instead use its API to load a new tag. | Per-model weights can be 10-100GB — restart-to-load is catastrophic. |
| **Process orchestrator (process-compose, pm2, supervisord)** | Source of truth for its children. Agents consult the CLI; they do not touch individual children directly unless the orchestrator's docs say that's safe. | `process-compose` has an HTTP API; prefer that over `kill`. |
| **Cargo target dirs** | Per-worktree is the default. Session-wide `sccache` helps; install once, set `RUSTC_WRAPPER=sccache`, shared across all repos. | Orphaned target/ dirs caused 5 ENOSPC rounds this session. Purge completed-push targets after the matching branch lands. |
| **Node / Bun module caches** | Global `~/.bun/install/cache` or `~/.local/share/pnpm/store` is shared automatically. | Fine as-is. Don't force per-project node_modules to persist across sessions. |

## True multi-tenancy strategies

When a service genuinely cannot be shared (e.g., destructive test mutates shared state), use ONE of:

1. **Ephemeral port**: launch on `0.0.0.0:0` and read back the assigned port. Advertise via pidfile + JSON (`{"port":N,"pid":P}`).
2. **Namespace isolation**: postgres schema per agent, redis DB index per agent, bucket prefix per agent. The shared process serves multiple isolated tenants.
3. **Sidecar process**: launch a dedicated instance with a scoped config file + separate data dir. Clean up in a deferred/atexit hook — not on SIGINT (agent can be killed before cleanup runs).
4. **Queue + broker**: for mutation-heavy workloads, serialize via a broker (postmaster, lock file, redis lock). First agent acquires; others wait or fail-fast.

## Agent etiquette

- **Probe first.** Every time, even if you launched one earlier in the same session (another session may have killed it).
- **Write a pidfile** when you launch: `<repo>/.run/<service>.pid` with `{pid, port, launcher, launched_at}`. Reading agents inherit this info.
- **Never `pkill`** unless the pidfile's owner is provably this session AND the user asked for a restart. For cross-session shared processes, notify via `.argis-helios-bus/*.jsonl` before touching.
- **Don't restart on file-change.** Hot-reload is the norm. If the service doesn't support hot-reload, file an issue before adding "restart on change" hooks.
- **SIGTERM not SIGKILL.** Give the service a chance to flush state. Fall through to KILL only after a 5s timeout.

## Known hot conflicts (as of 2026-04-22)

- `:5432` native postgres (pid 906) vs OrbStack postgres — user must `brew services stop postgresql@*` or stop the OrbStack stack before `task dev:up` on AgilePlus.
- `:6379` OrbStack redis vs dev-redis — same pattern.
- `:9000` OrbStack sonarqube reserving port conflicts with MinIO — move MinIO to `:9001` or stop sonarqube.

## Enforcement

- Pre-dispatch probe is a governance rule, not a hook yet. Violations leave the user fighting phantom port conflicts.
- Future work: a `scripts/pre-dispatch-check.sh` (Rust binary per scripting policy) that runs the probes above and returns a table. Agents run it before launching any service. See also `disk_budget_policy.md` for the equivalent disk-side check.

## See also

- `multi_session_coordination.md` — Argis↔Helios bus.
- `disk_budget_policy.md` — pre-dispatch disk check.
- `long_push_pattern.md` — nohup pushes.
- `enospc_playbook.md` — emergency purge sequence.
