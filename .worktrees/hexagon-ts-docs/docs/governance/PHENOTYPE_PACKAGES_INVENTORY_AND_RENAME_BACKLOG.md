# Phenotype `phenotype-*` Package Inventory & Rename Backlog

**Version:** 1.0.0  
**Updated:** 2026-03-25  
**Policy:** [POLYREPO_PACKAGE_NAMING_AND_PRODUCTIZATION.md](./POLYREPO_PACKAGE_NAMING_AND_PRODUCTIZATION.md)

This inventory scans the **`Phenotype/repos` hub** (top-level and selected nested paths). **Tier A** = keep `phenotype-*` (org-bound). **Tier B** = candidate for **neutral, marketable** repo name in a future ADR.

---

## Tier A — Keep `phenotype-*` (org / product semantics)

| Path | Rationale |
|------|-----------|
| `phenotype-config` | Org deployment and config contract surface |
| `phenotype-design` | Phenotype-owned design system / tokens |
| `phenotype-config-adapter` | Adapters to org config model |
| `phenotype-config-ts` | TS mirror of org config (if tied to same schema) |
| `phenotype-agent-core` | Agent orchestration **if** product-specific semantics |
| `phenotype-cli-core` | Org CLI platform (if product anchor) |
| `phenotype-shared` | Intentional monorepo/shared kit for org |
| `template-commons/phenotype-go-kit` | Central Go kit for Phenotype templates |
| `template-commons/phenotype-go-config` | Same, config slice |
| `crates/phenotype-config`, `crates/phenotype-design` | If same semantics as above (dedupe with root copies via ADR) |

---

## Tier B — Candidate neutral / “marketable” names (ADR before rename)

Generic **hexagonal**, **middleware**, **xDD**, **docs engine**, **evaluation** patterns—useful outside Phenotype branding.

| Current path | Suggested neutral name (illustrative) | Notes |
|--------------|---------------------------------------|--------|
| `phenotype-go-hexagonal` | `hexkit-go` or `ports-adapters-go` | Go hexagonal helpers |
| `phenotype-ts-hexagonal` | `hexkit-ts` | TS ports/adapters |
| `phenotype-py-hexagonal` | `hexkit-py` | Python |
| `phenotype-hexagonal` | Deprecate in favor of language-specific hexkits | Avoid duplicate “generic” roots |
| `phenotype-middleware-py` | `http-middleware-kit-py` or `otel-middleware-py` | If not org-specific |
| `phenotype-infrakit` | `infrakit` or `cloud-adapters` | If adapters are generic |
| `phenotype-docs-engine` | `static-docs-engine` or `doc-pipeline-kit` | VitePress/docs automation |
| `phenotype-research-engine` | `research-pipeline` or `lit-review-kit` | If domain-agnostic |
| `phenotype-evaluation` | `eval-harness` or `benchmark-kit` | If generic |
| `phenotype-task-engine` | `workflow-engine` or `task-runner-kit` | If generic DAG/tasks |
| `phenotype-xdd` / `phenotype-xdd-lib` | `xdd-spec-kit`, `traceability-kit` | Methodology tooling |
| `phenotype-logging-zig` | `structured-log-zig` or `logkit-zig` | If not org-tied |
| `phenotype-redis-adapter` | `redis-cache-adapter` | If generic Redis port |
| `phenotype-auth-ts` | `oauth-kit-ts` or `auth-middleware-ts` | If not org IdP |
| `packages/phenotype-sdk` | `client-sdk` + scope (`@org/*`) | Version as npm scope |
| `packages/phenotype-config-client` | `config-client` under `@phenotype/*` scope only if API is org-specific |
| `phenotype-agents` | Keep or rename to product line (`helios-agents`) if product-bound |
| `phenotype-cli-extensions` | `cli-plugins` or product-prefixed | If generic extension host |
| `phenotype-colab-extensions` | `colab-adapters` | If generic |
| `phenotype-skills-clone` | Archive or rename to `skills-template` | Clone is not a product name |
| `crates/phenotype-clean-arch` | `clean-arch-rs` | Patterns crate |
| `crates/phenotype-hexagonal` | `hexagonal-rs` | Avoid duplicate with Go/TS |
| `crates/phenotype-flags` | `feature-flags-rs` | If generic |
| `crates/phenotype-xdd` | Merge with `phenotype-xdd-lib` or `xdd-kit` | Dedupe |
| `template-commons/phenotype-api` | `api-contracts` or OpenAPI package name | If generic |
| `template-commons/phenotype-go-auth` | `go-auth-adapters` | If generic middleware |
| `template-commons/phenotype-go-cli` | `go-cli-kit` | Cobra/urfave helpers |
| `template-commons/phenotype-go-middleware` | `go-http-middleware` | Generic |
| `template-commons/phenotype-id` | `idgen` or `snowflake-id` | If generic IDs |
| `template-commons/phenotype-logging` | `structured-log-go` | If generic |
| `template-commons/phenotype-py-kit` | `py-kit` or domain name | Python twin of go-kit |
| `template-commons/phenotype-testing` | `testkit-go` | Test helpers |
| `template-commons/phenotype-toolkit` | Split by concern or `devtoolkit` | Too broad—split ADR |

---

## Dedup / consolidate (high priority)

| Issue | Action |
|-------|--------|
| Multiple `phenotype-config` (root vs `template-commons` vs `crates`) | Single source of truth ADR; archive or submodule |
| Multiple `phenotype-hexagonal` / `phenotype-xdd*` | Merge crates and libs under one neutral umbrella + language ports |

---

## Next steps

1. **ADR per rename** (old import path → new module path, timeline, consumers).  
2. **Forward-only** migration; no silent `replace` hacks in committed `go.mod`.  
3. **Inventory refresh**: re-run `find … -name 'phenotype-*'` quarterly or on each new package.

---

## Revision history

| Date | Change |
|------|--------|
| 2026-03-25 | Initial inventory from `repos` hub scan |
