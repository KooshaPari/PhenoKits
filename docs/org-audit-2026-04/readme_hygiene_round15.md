# README Hygiene Audit — Round 15

**Audit Date:** 2026-04-25  
**Scope:** 10 recently-discovered repos  
**Criteria:** Install command, 1-line description, License section, Status badge/note  

## Checklist by Repo

| Repo | Install | 1-Line Desc | License | Status | Notes |
|------|---------|-----------|---------|--------|-------|
| **chatta** | ✅ | ✅ | ❌ | ✅ | WebRTC chat; has status indicators (✅/🔄) but no formal badge |
| **pheno** | ❌ | ✅ | ❌ | ✅ | Organizational shelf; no install (not consumable package) |
| **AtomsBot** | ❌ | ✅ | ❌ | ❌ | Discord bot; has setup steps but no `install` or `npm i` block |
| **agent-user-status** | ✅ | ✅ | ❌ | ✅ | Python CLI with `./scripts/install.sh` and `--help` documented |
| **agent-devops-setups** | ❌ | ✅ | ❌ | ✅ | Deprecated; marked "read-only reference" in first para |
| **McpKit** | ✅ | ✅ | ❌ | ✅ | Multi-language; shows language-specific install (pip, cargo, npm) |
| **kwality** | ❌ | ✅ | ❌ | ✅ | Archived; ⚠️ ARCHIVED banner; no install (deprecated) |
| **MCPForge** | ❌ | ✅ | ❌ | ❌ | Fork of isaacphi/mcp-language-server; has `go install` in upstream |
| **Configra** | ✅ | ✅ | ❌ | ✅ | `cargo install --path pheno-cli` clear; has feature flags status (✅/🔄) |
| **DataKit** | ❌ | ✅ | ❌ | ✅ | Multi-language (Python primary); shows `pip install -e .` but not as standalone install block |

## Summary

- **Install command:** 4/10 ✅ (chatta, agent-user-status, McpKit, Configra)
- **1-line description:** 10/10 ✅ (all have clear opening statement)
- **License section:** 0/10 ❌ (none include explicit License heading)
- **Status badge/note:** 7/10 ✅ (most have status indicators or deprecation notes)

**Recommended Actions:** Add `## License` section to all active repos; clarify non-consumable repos (pheno, AtomsBot). Promote install blocks in DataKit/MCPForge.
