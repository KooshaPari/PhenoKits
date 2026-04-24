# Implementation Plan — AgentOps Policy Federation

## Phase 1: Core Resolution Engine (Done)

| Task | Description | Depends On | Status |
|------|-------------|------------|--------|
| P1.1 | Define seven-scope resolution model | — | Done |
| P1.2 | Implement `policyctl resolve` | P1.1 | Done |
| P1.3 | Create JSON schemas for policy validation | P1.1 | Done |
| P1.4 | Implement `policyctl check` | P1.3 | Done |

## Phase 2: Authorization DSL (Done)

| Task | Description | Depends On | Status |
|------|-------------|------------|--------|
| P2.1 | Define authorization rule format | P1.1 | Done |
| P2.2 | Implement `policyctl evaluate` | P2.1 | Done |
| P2.3 | Implement `policyctl intercept` | P2.2 | Done |
| P2.4 | Implement `policyctl compile` | P2.1 | Done |

## Phase 3: Runtime Guards (Done)

| Task | Description | Depends On | Status |
|------|-------------|------------|--------|
| P3.1 | Codex exec/write/network guard scripts | P2.3 | Done |
| P3.2 | Cursor/Factory/Claude runtime manifests | P3.1 | Done |
| P3.3 | Claude pre-tool hook bridge | P3.2 | Done |

## Phase 4: Runtime Installation (Done)

| Task | Description | Depends On | Status |
|------|-------------|------------|--------|
| P4.1 | Implement `policyctl install-runtime` | P3.2 | Done |
| P4.2 | Implement `policyctl uninstall-runtime` | P4.1 | Done |
| P4.3 | Launcher wrappers for codex/cursor/droid/claude | P4.1 | Done |

## Phase 5: Extensions (Done)

| Task | Description | Depends On | Status |
|------|-------------|------------|--------|
| P5.1 | Extension registry and scope selectors | P1.2 | Done |
| P5.2 | Extension activation in resolve output | P5.1 | Done |

## Phase 6: Future (Planned)

| Task | Description | Depends On | Status |
|------|-------------|------------|--------|
| P6.1 | Real extension entrypoint implementations | P5.2 | Planned |
| P6.2 | Machine-generated `policy_version` manifests | P1.2 | Planned |
| P6.3 | Hook `resolve` into runtime start of thegent/Factory/Cursor | P4.3 | Planned |
