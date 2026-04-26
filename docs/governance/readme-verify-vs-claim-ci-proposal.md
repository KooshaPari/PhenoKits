# README Verify-vs-Claim CI Sweep — Governance Proposal

**Status:** Proposal (planner-only; no implementation code in this doc)
**Date:** 2026-04-25
**Owner:** Forge / governance
**Related:**
- `docs/governance/userstory-aggregation-2026-04-26.md` (P1 — README↔reality contradiction in 12/19 repos)
- `docs/governance/org-readme-quality-audit-2026-04-26.md`
- Memory: *verify-then-write*, *identity-verification*, *audit-freshness-decay*

---

## 1. Problem

The org-wide user-story aggregation (2026-04-26) found that **63% of audited repos (12/19)** ship a README that contradicts the on-disk tree. **74% (14/19)** have a broken or fictional install/quickstart path. Cold visitors cannot run the documented commands; agents downstream consume false claims as ground truth and propagate them.

Representative failures:
- `phenoshared` — README advertises 4 headline crates + 15 workspace members; only `phenotype-migrations` exists; no root `Cargo.toml`.
- `phenoobservability` — Quick Start imports `pheno_tracing::init_tracing()`; the crate does not exist.
- `cliproxyapi` — Quick-start references `cmd/proxy` and `config/endpoints.yaml.example`; the real binary is `cmd/cliproxyapi`, the real file is `config.example.yaml`.
- `mcpkit` — Quick Start runs `cd python && pip install -e .`; no `pyproject.toml` is present.

Net effect: README is treated as marketing copy, decays silently, and there is no programmatic gate keeping it honest.

## 2. Solution (one line)

A CI check — `readme-verify` — that parses the README for **falsifiable claims**, verifies each claim against repo state, and **fails the build with line-numbered diffs** when reality drifts from documentation.

This is the README equivalent of `cargo check`: a fast, dumb, mechanical sanity gate. It does not judge prose. It does not require the README to be exhaustive. It requires that every concrete claim the README *does* make is true *today*.

## 3. Falsifiable claim taxonomy

The check extracts and verifies these patterns. Anything outside the taxonomy is ignored.

| Claim type | Extraction pattern (informal) | Verification |
|------------|-------------------------------|--------------|
| File / directory references | Backtick paths matching `\`[\w./-]+\`` that contain `/` or end in a known extension | `Path::exists()` from repo root |
| Cargo crate references | Fenced `cargo install <name>`, `cargo run -p <name>`, `cargo build -p <name>` | Crate present in workspace `Cargo.toml` `members`/`package.name`, **or** resolvable on crates.io |
| Cargo workspace members table | "N crates" / numbered crate lists | Count matches `cargo metadata --no-deps` package count |
| Module / use-statement examples | Fenced ` ```rust ` blocks containing `use foo::bar` | `foo` resolvable as a workspace crate or declared dependency |
| Shell quick-starts | Fenced ` ```bash ` / ` ```sh ` blocks | Dry-run lint: each `cd <path>` resolves; each binary mentioned exists in `PATH` of a known toolchain (`cargo`, `bun`, `pnpm`, `python`, `go`) |
| Test counts | Phrases like "N tests", "N test cases" | Compare to `cargo test --workspace -- --list` count (±10% tolerance) |
| Submodule references | `git submodule` mentions or `<repo>/<sub>` paths | `.gitmodules` entry exists and SHA is reachable |
| Language matrix claims | "Python ✅ / Go ✅ / TS ✅" tables | Each ticked language has a manifest (`pyproject.toml`, `go.mod`, `package.json`) at the claimed path |
| Binary / command names | `\$ <cmd> [args]` blocks, `agileplus <subcmd>` | `<cmd>` builds from a workspace target **or** is on the documented install path |
| Hosted-URL claims | `https://<sub>.kooshapari.com` | HEAD request returns 2xx/3xx (skipped on offline runs; warn-only) |

Patterns that are **explicitly excluded**: prose, screenshots, design rationale, roadmap items marked "planned" / "TBD", anything inside a `<!-- verify:skip -->` block.

## 4. Reference implementation sketch (pseudocode only)

Per planner rule, this is **shape, not code**. Implementation lives in a follow-up work package.

```
# Pseudocode — do not copy as production
for repo in workspace:
    readme = read("README.md")
    claims = extract_claims(readme, taxonomy)        # → [(line, kind, value)]
    failures = []
    for (line, kind, value) in claims:
        result = verify(kind, value, repo_state)     # → Ok | Err{actual, hint}
        if result is Err:
            failures.append(line, kind, value, result.actual, result.hint)
    if failures:
        emit_report(failures)                         # human + machine readable
        exit 1
```

Recommended language: **Rust** (per Phenotype scripting policy). Suggested crate stack:
- `pulldown-cmark` — Markdown AST
- `cargo_metadata` — workspace introspection
- `regex` — pattern matchers
- `clap` + `anyhow` + `serde_json` — CLI shell
- `walkdir` — path resolution

Single binary, `readme-verify`, lives in `phenotype-shared/crates/readme-verify` (or as a `tools/` member of `phenotype-infrakit`). Invoked as `readme-verify --repo .` in CI.

### Failure-mode contract

Every failure must report:
1. **Line number** in `README.md`
2. **Claim kind** (one of the taxonomy rows)
3. **Claimed value** (verbatim)
4. **Actual state** ("path does not exist", "crate `foo` not in workspace", "config file is `config.example.yaml`, not `config/endpoints.yaml.example`")
5. **Suggested fix** ("did you mean `cmd/cliproxyapi`?" using nearest-match heuristic)

This makes the check *self-explaining*: a junior agent or contributor can fix the failure without reading additional docs.

### Escape hatches (used sparingly)

- `<!-- verify:skip --> ... <!-- /verify:skip -->` — skip a block (must include a one-line justification comment, enforced by the linter itself).
- `<!-- verify:planned -->` — mark a section as roadmap, exempt from existence checks but still subject to syntax validation.
- Per-repo `.readme-verify.toml` — allowlist of intentionally fictional commands (e.g., illustrative examples). Capped at 5 entries; CI fails if exceeded without an ADR link.

## 5. Adoption path

Three-stage rollout, agent-driven, no human approval gates.

| Stage | Scope | Gate | Estimated effort |
|-------|-------|------|------------------|
| 0. Bootstrap | Build `readme-verify` binary; smoke-test against 3 known-broken repos (`phenoshared`, `phenoobservability`, `cliproxyapi`) | Tool reproduces all P1 findings from the aggregation doc | 1 agent batch (~15 min wall-clock) |
| 1. Opt-in | Each repo adds `.github/workflows/readme-verify.yml` (Linux runner only, per billing constraint); failures are **warn-only** | All Tier-1 repos opted in; baseline failure counts captured | 1 agent batch per repo, parallelizable across ~10 subagents |
| 2. Mandatory (Tier 1) | Tier-1 repos flip to **fail-on-error**. Branch protection requires green check on `main` | `userstory-aggregation` re-run shows P1 contradictions ≤ 2/19 | 1 agent batch per repo + fix work |
| 3. Mandatory (org-wide) | All non-archived repos enforce | P1 contradictions = 0 in next aggregation | Continuous |

Tier 1 (initial mandatory cohort): `phenotype-shared`, `phenotype-infrakit`, `AgilePlus`, `thegent`, `heliosCLI` — repos other agents depend on as ground truth.

Archived repos (`KaskMan`, `pheno`, etc.) are exempt; the workflow file is not added.

## 6. Acceptance criteria

The proposal is considered shipped when:

1. `readme-verify` binary exists, has unit tests for every taxonomy row, and has a documented CLI contract.
2. Running it against each of the 12 P1-flagged repos from the aggregation doc reproduces **at least one failure per repo** (proves the tool actually catches the documented drift).
3. Tier-1 repos have the workflow installed and passing **after** their READMEs are corrected — i.e., the gate is real, not bypassed via skip-blocks.
4. The next quarterly user-story aggregation reports **P1 contradiction count ≤ 2/19** (down from 12/19).
5. A short ADR records the decision and links this proposal.

## 7. Non-goals

- **Not** a prose linter. Vale already covers tone and style.
- **Not** a doc-completeness gate. Empty READMEs pass, as long as their few claims are true. (Completeness is governed separately by `org-readme-quality-audit`.)
- **Not** a substitute for integration tests. A README quick-start that *exists* and *builds* may still be wrong at runtime; that's `cargo test`'s job.
- **Not** opinionated about layout. Any heading order, any tone, any length passes.

## 8. Open questions

1. Should `readme-verify` walk other root-level docs (`AGENTS.md`, `CLAUDE.md`, `00_START_HERE.md`)? **Recommended:** yes, gated behind `--include-agent-docs`; default off in stage 1, on in stage 3.
2. Should it verify `<project>.kooshapari.com` URLs against the Org Pages coverage matrix? **Recommended:** yes, warn-only forever (network flakiness is real).
3. Where does the binary live — `phenotype-shared` or `phenotype-infrakit/tools/`? **Recommended:** `phenotype-shared/crates/readme-verify` so every repo can vendor or `cargo install` it without pulling infrakit's heavier deps.

## 9. References

- `docs/governance/userstory-aggregation-2026-04-26.md` § P1, § P2, § P4
- `docs/governance/org-readme-quality-audit-2026-04-26.md`
- Memory: *verify origin/main not local canonical* — same failure mode, different surface
- Memory: *audit freshness decays fast* — motivates running this in CI, not as a quarterly audit
- Phenotype scripting policy: `docs/governance/scripting_policy.md` (Rust default)
- Billing constraint: Linux runners only

---

*Planner-only document. No production code. Implementation tracked separately under AgilePlus once a feature ID is assigned.*
