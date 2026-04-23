# Code Entity Map — Bifrost Extensions

## Forward Map (Code -> Requirements)

| Entity | Type | FRs |
|--------|------|-----|
| `account/account.go` | Module | FR-CFG-002 |
| `cmd/bifrost-enhanced/` | CLI | FR-CLI-001..003 |
| `config/config.go` | Module | FR-CFG-001 |
| `plugins/intelligentrouter/` | Plugin | FR-RTR-001 |
| `plugins/smartfallback/` | Plugin | FR-RTR-002 |
| `plugins/learning/` | Plugin | FR-RTR-003 |
| `providers/oauthproxy/` | Module | FR-AUTH-001..003 |
| `providers/agentcli/` | Module | FR-AGT-001..002 |
| `server/server.go` | Module | FR-SRV-002 |
| `server/handlers.go` | Module | FR-SRV-001 |

## Reverse Map (Requirements -> Code)

| FR ID | Code Entities |
|-------|---------------|
| FR-PLG-001..003 | `plugins/` |
| FR-RTR-001 | `plugins/intelligentrouter/` |
| FR-RTR-002 | `plugins/smartfallback/` |
| FR-RTR-003 | `plugins/learning/` |
| FR-CLI-001..003 | `cmd/bifrost-enhanced/` |
| FR-CFG-001 | `config/config.go` |
| FR-CFG-002 | `account/account.go` |
| FR-AUTH-001..003 | `providers/oauthproxy/` |
| FR-AGT-001..002 | `providers/agentcli/` |
| FR-SRV-001 | `server/handlers.go` |
| FR-SRV-002 | `server/server.go` |
