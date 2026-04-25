# GitHub PR Triage: Dependabot/Renovate Auto-Merge (2026-04-25)

**Target:** Bulk-merge 34 identified Dependabot/Renovate PRs to unblock CI and resolve CodeQL alerts.

## Status Summary

| Status | Count | Details |
|--------|-------|---------|
| MERGED | 0 | Pending merge attempts |
| CONFLICT | 0 | Merge conflicts detected |
| REJECTED | 0 | Branch protection / CI failures |
| SKIPPED-MAJOR | 0 | Major version bumps for review |
| TOTAL | 34 | PRs in scope |

## Merge Log

### phenotype-infra (7 PRs)

| # | Title | Base | Status | Notes |
|---|-------|------|--------|-------|
| 7 | chore(deps): bump actions/checkout from 4 to 6 | main | PENDING | GitHub Actions version bump |
| 6 | build(deps): bump hashicorp/setup-terraform from 3 to 4 | main | PENDING | Terraform setup bump |
| 5 | build(deps): bump DavidAnson/markdownlint-cli2-action from 16 to 23 | main | PENDING | Markdown linter bump |
| 4 | build(deps): update hashicorp/aws requirement from ~> 5.50 to ~> 6.42 in /iac/terraform | main | PENDING | Major AWS provider bump |
| 3 | build(deps): update hashicorp/google requirement from ~> 5.30 to ~> 7.29 in /iac/terraform | main | PENDING | Major GCP provider bump |
| 2 | build(deps): update cloudflare/cloudflare requirement from ~> 4.30 to ~> 5.18 in /iac/terraform | main | PENDING | Cloudflare provider bump |
| 1 | build(deps): update oracle/oci requirement from ~> 5.40 to ~> 8.10 in /iac/terraform | main | PENDING | Oracle OCI provider bump |

### AgilePlus (6 PRs)

| # | Title | Base | Status | Notes |
|---|-------|------|--------|-------|
| 331 | chore(deps): update protobuf requirement from >=5.0 to >=7.34.1 in /python | main | PENDING | Python protobuf bump |
| 330 | chore(deps): update grpcio-tools requirement from >=1.62 to >=1.80.0 in /python | main | PENDING | Python gRPC tools bump |
| 329 | chore(deps): update grpcio requirement from >=1.62 to >=1.80.0 in /python | main | PENDING | Python gRPC bump |
| 328 | chore(deps): update grpcio-health-checking requirement from >=1.62 to >=1.80.0 in /python | main | PENDING | Python gRPC health bump |
| 327 | chore(deps): update fastmcp requirement from >=2.0 to >=3.2.3 in /python | main | PENDING | FastMCP bump |
| 323 | chore(deps): bump the uv group across 2 directories with 1 update | main | PENDING | UV package manager bump |
| 310 | chore(deps): bump codecov/codecov-action from 4 to 6 | main | PENDING | Codecov action bump |
| 308 | chore(deps): bump actions/cache from 4 to 5 | main | PENDING | GitHub Actions cache bump |
| 307 | chore(deps): bump actions/checkout from 4 to 6 | main | PENDING | GitHub Actions checkout bump |

### thegent (1 PR)

| # | Title | Base | Status | Notes |
|---|-------|------|--------|-------|
| 909 | chore(deps): bump aiohttp from 3.13.3 to 3.13.4 in the uv group across 1 directory | main | PENDING | aiohttp patch bump |

### phenotype-tooling (5 PRs)

| # | Title | Base | Status | Notes |
|---|-------|------|--------|-------|
| 5 | chore(deps): Bump cargo_metadata from 0.18.1 to 0.23.1 | main | PENDING | Rust cargo_metadata bump |
| 4 | chore(deps): Bump pulldown-cmark from 0.9.6 to 0.13.3 | main | PENDING | Rust markdown parser bump |
| 3 | chore(deps): Bump toml from 0.8.23 to 1.1.2+spec-1.1.0 | main | PENDING | Rust TOML parser bump |
| 2 | chore(deps): bump thiserror from 1.0.69 to 2.0.18 | main | PENDING | Rust error lib bump |
| 1 | chore(deps): Bump reqwest from 0.11.27 to 0.13.2 | main | PENDING | Rust HTTP client bump |

### PhenoPlugins (1 PR)

| # | Title | Base | Status | Notes |
|---|-------|------|--------|-------|
| 10 | chore(deps): bump actions/checkout from 4 to 6 | main | PENDING | GitHub Actions checkout bump |

### heliosApp (2 PRs)

| # | Title | Base | Status | Notes |
|---|-------|------|--------|-------|
| 398 | chore(deps): bump ky from 1.14.3 to 2.0.2 | main | PENDING | HTTP client major bump |
| 392 | chore(deps-dev): bump typescript from 6.0.2 to 6.0.3 | main | PENDING | TypeScript patch bump |

### AuthKit (3 PRs)

| # | Title | Base | Status | Notes |
|---|-------|------|--------|-------|
| 13 | build(deps): Bump actions/setup-node from 4 to 6 | main | PENDING | GitHub Actions setup-node bump |
| 3 | build(deps): Update base64 requirement from 0.21 to 0.22 | main | PENDING | Rust base64 lib bump |
| 2 | build(deps-dev): Update black requirement from ^24.0 to ^26.3 | main | PENDING | Python formatter major bump |

### cliproxyapi-plusplus (6 PRs)

| # | Title | Base | Status | Notes |
|---|-------|------|--------|-------|
| 951 | build(deps): bump github.com/go-git/go-git/v6 from 6.0.0-20260328145551-a93bccd59f82 to 6.0.0-alpha.2 in the go_modules group across 1 directory | main | PENDING | Go git lib bump |
| 950 | build(deps): bump modernc.org/sqlite from 1.48.0 to 1.48.1 | main | PENDING | SQLite lib patch bump |
| 949 | build(deps): bump github.com/router-for-me/CLIProxyAPI/v6 from 6.9.6 to 6.9.15 | main | PENDING | CLIProxyAPI bump |
| 948 | build(deps): bump github.com/fxamacker/cbor/v2 from 2.9.0 to 2.9.1 | main | PENDING | CBOR lib patch bump |
| 947 | build(deps): bump github.com/go-git/go-git/v6 from 6.0.0-20260328145551-a93bccd59f82 to 6.0.0-alpha.1 | main | PENDING | Go git lib bump |
| 946 | build(deps): bump github.com/andybalholm/brotli from 1.2.0 to 1.2.1 | main | PENDING | Brotli compression lib patch bump |

## Execution Log

_To be updated as merges are attempted._
