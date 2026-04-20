# Scripting Language Hierarchy — Phenotype Org Canonical Policy

**Status:** Active
**Scope:** All Phenotype-org repositories (AgilePlus, thegent, phenotype-infrakit, hwLedger, phenotype-shared, phenotype-journeys, heliosCLI, and every project under `Phenotype/repos/`).
**Canonical path:** `Phenotype/repos/docs/governance/scripting_policy.md`
**Governance pointers:** referenced from `~/.claude/CLAUDE.md` (symlinked to `thegent/CLAUDE.md`), `Phenotype/CLAUDE.md`, `Phenotype/repos/CLAUDE.md`, `thegent/dotfiles/governance/CLAUDE.base.md`, `AgilePlus/CLAUDE.md`, and per-repo CLAUDE.md files.

---

## TL;DR

**Write Rust. If not Rust, then Zig/Mojo/Go with a one-liner justification. Python/TS only when embedded in an existing Python/TS runtime. Bash only as a ≤5-line wrapper with an inline justification comment. No new shell scripts in Phenotype-org repos — period.**

---

## 1. Hierarchy

| Tier | Languages | When | Required justification |
|------|-----------|------|------------------------|
| 1 (default) | **Rust** | Any script, tool, codegen step, pipeline component, CI helper, dev-env tool, build-time utility, file walker, config munger, migrator, hook. | None — this is the default. Use `clap` + `anyhow` + `serde` + `tokio` + `walkdir` as the baseline toolbelt. Add `thiserror`, `tracing`, `reqwest`, `sqlx`, `indicatif` as needed. |
| 2 (acceptable alternates) | **Zig, Mojo, Go** | When the language is materially better for the specific job (e.g., Go's `os/exec` + `filepath.Walk` for an ffmpeg CLI driver; Zig for a tight C-FFI wrapper; Mojo for numeric kernels). | One-line comment at the top of the main source file: `// <lang> because <concrete technical reason>`. |
| 3 (runtime-embedded fallback) | **Python, TypeScript** | Permitted **only** when the script must live inside an existing Python/TS runtime: Streamlit page scripts, Playwright specs, Jupyter notebooks, Vite/VitePress config, Next.js API routes, pytest fixtures, vitest specs. | Not for standalone CLI tools. If you reach for `argparse`, `click`, `typer`, or `commander` — stop and write Rust instead. |
| 4 (last-resort shell glue) | **Bash, sh, zsh, fish, PowerShell** | Only for ≤5-line platform glue where a real-language replacement is a net loss — e.g., a wrapper that sources the user's shell rc before `exec`ing a Rust binary. | Every surviving shell script **must** carry a top-of-file comment stating (a) the rule it's invoking, (b) why Rust/Go/etc. are worse for this specific case. No exceptions. |

---

## 2. Rationale

Shell is a footgun at scale:

- **Quoting and word-splitting** produce silent correctness bugs (`$var` vs `"$var"` vs `"${var[@]}"`).
- **Portability** drifts across GNU/BSD/macOS (`sed -i`, `readlink -f`, `date -d`, `grep -P`).
- **Error handling** is easy to mask (missing `set -euo pipefail`, pipefail semantics, subshell exit codes, `|| true` swallowing failures).
- **Testing** is poor: no unit-test ergonomics, no type system, no mocking, no coverage.
- **IDE support** is weak: no LSP-quality completion, no borrow checker, no rename refactor.
- **Dependencies** are global and implicit: every `jq`, `yq`, `gh`, `fd`, `rg` pulled in is an uninstalled-binary runtime failure waiting to happen.
- **Concurrency** is fragile: GNU parallel / xargs / `&` + `wait` is not a substitute for `tokio::join!` or a real work-pool.

Rust solves all of these with a single static binary. A 300-line Rust tool with `clap` + `anyhow` is cheaper to maintain than a 50-line bash script that everyone is afraid to touch.

Previous project patterns across hwLedger, AgilePlus, thegent, and phenotype-infrakit accumulated **dozens** of `.sh` scripts for CI, sync, codegen, and hooks. Those are now legacy. **New work must not add shell; existing shell must be migrated when touched.**

---

## 3. Permitted Shell Exceptions (with examples)

A shell script is permitted only if **all** of:

1. It is ≤5 lines of meaningful code (ignoring shebang/license comment).
2. Replacing it with a real-language tool would be a net loss (usually because the script exists to set up the environment for that tool).
3. It has a top-of-file comment that cites this policy and the exemption reason.

### Allowed example — shell rc wrapper

```bash
#!/usr/bin/env bash
# Scripting policy (Phenotype/repos/docs/governance/scripting_policy.md) tier 4 exception:
# Shell must source the user's rc so PATH/asdf/mise/direnv shims are resolved
# before exec'ing the Rust binary. Rewriting in Rust would require
# re-implementing shell init, which is a net loss.
set -euo pipefail
[[ -f "$HOME/.zshrc" ]] && source "$HOME/.zshrc"
exec hwledger-cli "$@"
```

### Allowed example — git hook dispatcher

```bash
#!/usr/bin/env bash
# Scripting policy tier 4 exception: lefthook/pre-commit require executable
# scripts, and this 2-line dispatcher only execs the real Rust quality-gate.
set -euo pipefail
exec "$(git rev-parse --show-toplevel)/target/release/phenotype-quality-gate" "$@"
```

### Not allowed — rewrite in Rust

- Anything that loops over files and processes them (use `walkdir` + `rayon`).
- Anything that parses JSON/TOML/YAML (use `serde_json` / `toml` / `serde_yaml`).
- Anything that calls `jq`, `yq`, or `awk` for non-trivial transforms.
- Anything that handles errors with `|| true`, `|| echo`, or ignored exit codes.
- Anything that spawns more than one subprocess with interdependent state.
- Anything that has "retry", "timeout", or "parallel" in its logic.
- CI glue that invokes cargo/pytest/pnpm with flag matrices — use `just`, `cargo xtask`, or a Rust `xtask` crate.

---

## 4. Migration Guidance

**When you touch a shell script, replace it with Rust** unless the ≤5-line-glue exception applies. Concretely:

1. **Inventory:** `rg --files-with-matches --glob '*.sh'` in the repo gives the current footprint.
2. **Classify:** for each script, decide glue-exemption vs. migration candidate.
3. **Migrate:**
   - Create a `tools/<name>/` crate (or add to an existing `xtask` crate) with `clap` + `anyhow`.
   - Port logic with tests. `tempfile` + `assert_cmd` + `predicates` covers most CLI testing.
   - Wire the new binary into CI/hooks (lefthook, pre-commit, justfile, Taskfile).
   - Delete the `.sh`. Do not leave it as a fallback — that defeats the migration.
4. **Worklog:** add an entry to `worklogs/GOVERNANCE.md` tagged `[<repo>]` describing the LOC delta.

Target cadence: migrate any shell script you are already editing; do not open "migrate all shell" mega-PRs unless explicitly scoped.

---

## 5. PR Review Rubric

Reviewers of any PR adding or modifying scripts must check:

| Check | Pass criteria |
|-------|---------------|
| Language tier | Tier 1 (Rust) used unless justified in PR description. |
| Alternate-tier justification | If Tier 2 (Zig/Mojo/Go), the top-of-file one-liner explains the concrete technical reason. |
| Fallback-tier justification | If Tier 3 (Python/TS), the script is provably embedded in an existing runtime (Streamlit page, Playwright spec, etc.), not a standalone CLI. |
| Shell exception | If Tier 4 (shell), the file is ≤5 lines AND carries the policy comment AND the replacement-in-Rust is demonstrably a net loss. |
| No new shell for CI/hooks/codegen | CI pipelines, git hooks, codegen, and sync tools must not introduce new `.sh`. Use `cargo xtask`, `just`, or a standalone Rust binary. |
| Error handling | No `|| true`, no silent `2>/dev/null`, no ignored exit codes. Errors must be loud (see the Optionality and Failure Behavior policy in `~/.claude/CLAUDE.md`). |
| Testing | Rust/Go scripts have at least smoke tests (`assert_cmd` / `testing`). Shell exemptions are exempt from test requirement only because they're ≤5 lines. |
| Worklog entry | For migrations, a `worklogs/GOVERNANCE.md` entry is present. |

A PR failing any of these is blocked until resolved. "Pre-existing shell" is **not** a valid excuse — see the Suppression/Ignore Rules (STRICT) section of `~/.claude/CLAUDE.md`.

---

## 6. Tooling Baseline (Rust)

Default crates for a new script/tool — workspace-pin these at the monorepo root:

```toml
clap     = { version = "4",  features = ["derive", "env"] }
anyhow   = "1"
thiserror = "2"
serde    = { version = "1",  features = ["derive"] }
serde_json = "1"
toml     = "0.8"
tokio    = { version = "1",  features = ["full"] }
walkdir  = "2"
tracing  = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
indicatif = "0.17"
reqwest  = { version = "0.12", features = ["json", "rustls-tls"] }
```

For xtask-style repo-local tools, a `tools/xtask` crate with `cargo xtask <cmd>` wiring via `.cargo/config.toml` aliases is the preferred pattern.

---

## 7. See Also

- Global baseline: `~/.claude/CLAUDE.md` (symlink to `Phenotype/repos/thegent/CLAUDE.md`).
- Wrap-over-handroll mandate: `~/.claude/CLAUDE.md` — "Wrap/Fork/Integrate over Hand-Roll".
- Optionality / loud-failure policy: `~/.claude/CLAUDE.md` — "Optionality and Failure Behavior".
- Suppression rules: `~/.claude/CLAUDE.md` — "Suppression/Ignore Rules (STRICT)".
- Architectural governance: `Phenotype/repos/thegent/docs/governance/23_ARCHITECTURAL_GOVERNANCE.md`.
- Language/framework selection: `Phenotype/repos/docs/engineering/language_governance_framework.md` (when deciding on a language for a new repo — this policy governs *scripts*; that framework governs *services and apps*).
