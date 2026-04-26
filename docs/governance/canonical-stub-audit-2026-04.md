# Canonical Stub Audit — 2026-04-25

**Scope:** Top-level directories under `/Users/kooshapari/CodeProjects/Phenotype/repos/`
that are *expected* to be standalone canonical Git checkouts of an org repo.

**Trigger:** PhenoLang investigation found `repos/PhenoLang/` is an empty stub
(`python/` only, no `.git`) while `repos/.archive/PhenoLang-actual/` carries the
real `PhenoLang.git` remote. We audited the org for the same pattern.

---

## Critical context discovered during audit

`repos/` itself is a Git repo: `git@github.com:KooshaPari/PhenoKits.git` on
branch `docs/alert-sync-policy-main`. That means **any subdirectory without its
own `.git/` is just tracked content inside PhenoKits**, not a "stub canonical
repo with the wrong remote." `git -C <subdir> config remote.origin.url` walks
up and reports PhenoKits' remote — that is normal, not a smell.

The genuine smell is therefore narrower than the prompt's hypothesis:

1. A directory whose **name matches an org repo** but is not its own checkout
   (no `.git/`), and the real repo content lives elsewhere
   (`.archive/<name>-actual/` or a sibling with the real remote).
2. A directory with `.git/` but checked out on a non-main branch where main is
   the canonical integration target.

No directory in `repos/` had `.git/` pointing at a *wrong* GitHub repo. The
`HexaKit/PhenoLang/phenotype-shared` claim from the prompt does not reproduce
under the current tree — those names are content dirs inside PhenoKits, not
mis-remoted standalone clones.

---

## Pattern 1 — Empty-stub canonical with real code in `.archive/<name>-actual/`

| Name | Has `.git` | Files at root | On `main` | Archive sibling | Status |
|------|-----------|---------------|-----------|-----------------|--------|
| `PhenoLang` | N (PhenoKits content) | 1 (`python/`) | n/a | `.archive/PhenoLang-actual/` (own `.git`, remote `PhenoLang.git`) | **STUB — known case** |

`.archive/PhenoLang-actual/` carries the canonical `PhenoLang.git` remote and
its own commit history (`ae1c643 chore(archive): final snapshot before cold
storage`). The directory content overlaps heavily with `repos/pheno/`,
suggesting PhenoLang was rehomed into the `pheno` monorepo and the original
clone was cold-archived. The remaining `repos/PhenoLang/python/` shell is a
PhenoKits-tracked stub.

**No other repo matches this pattern.** Search included every top-level dir.

---

## Pattern 2 — `.git`-bearing canonical with deprecated `.archive/<name>/` sibling

These are *not* stubs; the canonical is real and the archive is an older
snapshot intentionally cold-stored (each archive carries `DEPRECATION.md`).
Listed for completeness.

| Name | Has `.git` | Files | On `main` | Archive sibling | Status |
|------|-----------|-------|-----------|-----------------|--------|
| `DevHex` | Y | 8 | Y | `.archive/DevHex/` (has `DEPRECATION.md`) | leave alone |
| `GDK` | Y | 25 | Y | `.archive/GDK/` (has `DEPRECATION.md`) | leave alone |
| `pheno` | Y | 242 | Y | `.archive/pheno/` (has `DEPRECATION.md`) | leave alone |
| `phenodocs` | Y | 28 | Y | `.archive/phenodocs/` (has `DEPRECATION.md`) | leave alone |
| `PhenoProject` | Y | 8 | Y | `.archive/PhenoProject/` (has `DEPRECATION.md`) | leave alone |
| `PhenoRuntime` | Y | 24 | Y | `.archive/PhenoRuntime/` (has `DEPRECATION.md`) | leave alone |

`PhenoProject` is small (8 files, all skeleton) but is a real `.git`-backed
canonical on main. Confirm intent next time it is touched, but no action now.

---

## Pattern 3 — `.git` exists but checked out on non-main branch

These are real canonicals that violate the worktree commandment ("canonical
folders track main only"). They are not stubs — they are dirty canonicals.
Recommend: `git stash && git checkout main && git pull` next time the agent
visits, *unless* there is an active integration in progress.

| Name | Branch | Recommendation |
|------|--------|----------------|
| `agent-user-status` | `user-status-next-dag-hardening` | switch back to main after current work |
| `agentapi-plusplus` | `chore/infrastructure-push` | switch back to main |
| `agileplus-landing` | `feat/tier3-path-microfrontends` | switch back to main |
| `bare-cua` | `master` | repo's default is `master` (not main) — leave alone |
| `DINOForge-UnityDoorstop` | `master` | repo's default is `master` — leave alone |
| `kmobile` | `chore/dead-code-phase1-kmobile` | switch back to main |
| `nanovms` | `docs` | switch back to main |
| `PlayCua` | `master` | repo's default is `master` — leave alone |
| `phenotype-infra` | `audit/cross-repo-loc-dedup-followup` | switch back to main |
| `phenotype-ops-mcp` | `chore/fork-attribution` | switch back to main |
| `PhenoVCS` | `chore/sync-state` | switch back to main |
| `phenoXdd` | `docs/productization` | switch back to main |
| `portage` | `fix/integration-test-utils` | switch back to main |
| `thegent` | `chore/sync-working-tree-state` | switch back to main |
| `Tracely` | `chore/dead-code-phase1-tracely` | switch back to main |
| `Tracera-recovered` | `fix/main-workflow-syntax` | switch back to main |

---

## Pattern 4 — PhenoKits-tracked content directories (NOT canonical repos)

These directories live inside `repos/` (which itself is the PhenoKits repo) and
have no `.git/` of their own. They look like repo names but are PhenoKits
sub-trees. Not stubs of a canonical clone — they are first-class PhenoKits
content. **No action needed.**

`apps`, `artifacts`, `bifrost-extensions`, `eyetracker`, `hexagon`, `HexaKit`,
`Observably`, `PhenoContracts`, `PhenoEvents`, `PhenoKit`, `PhenoSchema`,
`phenotype-previews-smoketest`, `phenotype-shared`, `phenotype-skills`,
`portage-adapter-core`, `Stashly`, `thegent-jsonl`, `thegent-utils`, `Tracera`,
`ValidationKit`.

If any of these *should* be a standalone clone (e.g. `phenotype-shared` is
documented as its own Cargo workspace at
`/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-shared` per
`repos/CLAUDE.md`), then PhenoKits has absorbed the content and the standalone
clone is missing. Worth verifying owner intent for:

- `phenotype-shared` — repos/CLAUDE.md treats this as a standalone Rust
  workspace, but on disk it has no `.git/` and is tracked under PhenoKits.
  Either re-clone it as a sibling, or update the CLAUDE.md to say "lives inside
  PhenoKits."
- `bifrost-extensions` — 20 files of real content (cmd/, db/, costengine/),
  but no `.git/`. Same question.
- `Tracera` — 10 files including `backend/`, `frontend/`, `__pycache__/` —
  active content but no `.git/`. The active checkout is at
  `Tracera-recovered` (which has `.git`).
- `HexaKit` — 10 files including `crates/`, `platforms/`, `vendor/` — large
  content without `.git/`.

These four warrant follow-up; the rest are clearly small placeholder folders.

---

## Recommendation Summary

| Action | Count |
|--------|-------|
| Empty-stub canonicals to re-clone (Pattern 1) | **1** (`PhenoLang`) |
| Real canonicals with deprecated archives — leave alone (Pattern 2) | **6** |
| Canonicals on non-main branch — switch back when free (Pattern 3) | **13** (excluding 3 master-default repos) |
| PhenoKits-tracked content folders that *might* need own clone (Pattern 4) | **4** to verify (`phenotype-shared`, `bifrost-extensions`, `Tracera`, `HexaKit`) |
| All other top-level dirs — real `.git`, on main, healthy | **~85** |

**Top-line:** stub-vs-real ratio is **1 stub (PhenoLang) : ~95 real
canonicals**. The PhenoLang case is genuinely unique. The hypothesised
"HexaKit/PhenoLang/phenotype-shared wrong-remote" smell is actually the
PhenoKits monorepo absorbing those names as content directories — a different
governance concern (should they be standalone clones?) but not a wrong-remote
bug.

## Method

- Listed top-level dirs under `repos/`, excluded `.archive/`, `.worktrees/`,
  `*-wtrees`, `worklogs/`, `docs/`, `kitty*`, file artifacts.
- Per dir: checked `.git/` presence, root file count, `git rev-parse
  --abbrev-ref HEAD`, `git config remote.origin.url`, presence of
  `.archive/<name>/` and `.archive/<name>-actual/`.
- Cross-checked suspect dirs by listing content and comparing to archive
  siblings, looking for `DEPRECATION.md` markers.
- Audit was read-only — no clones, fetches, or commits.
