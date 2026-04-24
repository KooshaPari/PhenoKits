# INTEGRATION Worklog

External integration notes for Phenotype-org repositories. See `worklogs/README.md`
for category conventions and `worklogs/AGENT_ONBOARDING.md` for writing guidance.

---

## [phenotype-ops-mcp] 2026-04-23 — Fork of nanovms/ops-mcp (Task #59)

Forked [`nanovms/ops-mcp`](https://github.com/nanovms/ops-mcp) (Apache-2.0, Go,
~106 KiB) into the KooshaPari namespace as
[`phenotype-ops-mcp`](https://github.com/KooshaPari/phenotype-ops-mcp) and cloned
to `repos/phenotype-ops-mcp` with `upstream` remote pointing back at
`nanovms/ops-mcp`. Rationale: ops-mcp is the canonical MCP server for the nanoVMs
`ops` unikernel toolchain, which makes it the natural bridge between Phenotype's
MCP-first agent stack and nanoVMs unikernels. We already promoted
`phenotype-nanovms-client` (bare-cua -> phenoShared, PR #82 merged) as the
typed Rust client for the `ops` CLI; forking the MCP server lets us extend the
tool surface (auth, multi-tenant instance isolation, observability hooks,
Phenotype policy-engine integration) while tracking upstream. Planned
integration points: (1) wire `phenotype-ops-mcp` tool handlers to call
`phenotype-nanovms-client` instead of shelling out to `ops` directly where
feasible, (2) expose Phenotype auth/tenant context via MCP server metadata,
(3) add observability (phenotype-observability) around each tool invocation,
(4) upstream any non-Phenotype-specific improvements back to
`nanovms/ops-mcp` via PR before landing in the fork. Fork attribution landed on
branch `chore/fork-attribution` (README NOTICE + this worklog entry).
