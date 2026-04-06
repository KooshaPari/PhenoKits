# Code Entity Map — AgentOps Policy Federation

## Forward Map (Code -> Requirements)

| Entity | Type | FRs |
|--------|------|-----|
| `policy_federation/` | Package | FR-RES-001, FR-RES-002, FR-AUTH-001..003, FR-EXT-002 |
| `cli/` | CLI | FR-RES-003, FR-AUTH-004, FR-CMP-001..002, FR-RT-001, FR-RT-003..006, FR-VAL-001..002 |
| `scripts/runtime/codex_exec_guard.sh` | Script | FR-RT-002 |
| `scripts/runtime/codex_write_guard.sh` | Script | FR-RT-002 |
| `scripts/runtime/codex_network_guard.sh` | Script | FR-RT-002 |
| `scripts/runtime/claude_pretool_hook.py` | Script | FR-RT-002 |
| `scripts/runtime/*_runtime_manifest.json` | Config | FR-RT-002 |
| `extensions/registry.yaml` | Config | FR-EXT-001 |
| `policies/` | Config | FR-RES-001 |
| `schemas/` | Schema | FR-VAL-001 |
| `contracts/` | Config | FR-AUTH-001 |

## Reverse Map (Requirements -> Code)

| FR ID | Code Entities |
|-------|---------------|
| FR-RES-001 | `policy_federation/`, `policies/` |
| FR-RES-002 | `policy_federation/` |
| FR-RES-003 | `cli/` |
| FR-AUTH-001 | `policy_federation/`, `contracts/` |
| FR-AUTH-002 | `policy_federation/` |
| FR-AUTH-003 | `policy_federation/` |
| FR-AUTH-004 | `cli/` |
| FR-CMP-001 | `cli/` |
| FR-CMP-002 | `cli/` |
| FR-RT-001 | `cli/` |
| FR-RT-002 | `scripts/runtime/` |
| FR-RT-003 | `cli/` |
| FR-RT-004 | `cli/` |
| FR-RT-005 | `cli/` |
| FR-RT-006 | `cli/` |
| FR-EXT-001 | `extensions/registry.yaml` |
| FR-EXT-002 | `policy_federation/` |
| FR-VAL-001 | `cli/`, `schemas/` |
| FR-VAL-002 | `cli/` |
