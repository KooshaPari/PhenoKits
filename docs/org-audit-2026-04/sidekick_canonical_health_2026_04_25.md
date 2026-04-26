# Sidekick Canonical Health Audit — 2026-04-25

## Summary
Three canonical Sidekick repos audited. **Agent-imessage does not exist** (noted as TBD in CUTOVER_STEPS.md).

| Repo | Tests | Coverage | FR Doc | FR Traces | CI Workflows | Status |
|------|-------|----------|--------|-----------|--------------|--------|
| cheap-llm-mcp | 38 | Partial | YES | 1/38 (3%) | 3 | PROD-READY |
| agent-user-status | 67 | Good | YES | 67/67 (100%) | 4 | EXCELLENT |
| agent-devops-setups | 0 | None | YES | 0/0 (—) | 10 | AT-RISK |

## Key Findings

**agent-user-status** (Last: 872c5ad, 2026-04-25 17:25) — **Gold Standard**
- 67 test functions, all with FR-trace tags (100% traceability)
- Complete spec coverage; FUNCTIONAL_REQUIREMENTS.md in place
- 4 CI workflows (unit, integration, lint, security)

**cheap-llm-mcp** (Last: ea6102d, 2026-04-25 08:15) — **Production-Ready**
- 38 tests; only 1 has FR trace (3% coverage)
- FUNCTIONAL_REQUIREMENTS.md exists but low trace density
- 3 CI workflows; needs FR audit to lift trace coverage

**agent-devops-setups** (Last: 34e99a2, 2026-04-25 17:12) — **AT-RISK**
- Zero test code; FUNCTIONAL_REQUIREMENTS.md present but no test coverage
- 10 CI workflows suggest active development, but no tests backing specs
- Immediate action required: bootstrap test suite & FR traces

## Recommendations
1. **cheap-llm-mcp**: Add FR traces to 37 tests (9 minutes, 1 tool call)
2. **agent-devops-setups**: Scaffold test harness + FR traces (45 minutes, agent delegation)
3. **agent-imessage**: Does not exist; verify if it's planned or archived

