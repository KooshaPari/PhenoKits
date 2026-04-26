# cliproxyapi-plusplus Security Issue Triage — 2026-04

**Repo:** `KooshaPari/cliproxyapi-plusplus`
**Date:** 2026-04-25
**Open issues sampled:** ~125 total open; ~15 CodeQL bot-filed + 25 human SEC-N tracker + many `[OPT-N]`/feature/CI-failure issues.
**Scope:** Triage only — no code changes; no issue closures.

---

## 1. Categorization

### A. Real CodeQL findings (bot-filed, `app/github-actions`, label `source:codeql`)

These are auto-synced from GitHub code-scanning alerts. Each has a body that is a thin pointer to `https://github.com/.../security/code-scanning/<N>` — the actual finding lives in the Security tab, not in the issue body. Sampled #853, #848, #816 — all confirm the pattern (one-liner with `alert-sync-key: codeql:<N>`).

Distribution of the visible window:

| Rule | Count (sample) | Severity | Issue numbers |
|---|---|---|---|
| `go/path-injection` | 11 | high | #853, #852, #851, #850, #849, #846, #845, #844, #843, #842, #813, #807 |
| `go/request-forgery` | 2 | critical | #848, #808 |
| `go/allocation-size-overflow` | 1 | high | #816 |

Top 5:
1. **#848 [critical] go/request-forgery** — SSRF surface; user-controlled URL flows into outbound HTTP. Maps to SEC-1/2/3 (`antigravity_executor.go`, `sso_oidc.go`, `api_tools.go`).
2. **#808 [critical] go/request-forgery** — second SSRF instance, same family.
3. **#853 / #852 / #851 [high] go/path-injection** — file-path injection in auth/token/oauth/store paths. Maps to SEC-7..13.
4. **#816 [high] go/allocation-size-overflow** — likely `kiro_websearch.go` (matches SEC-17). User-controlled `make([]T, n)` size.
5. **#813 / #807 [high] go/path-injection** — older syncs of the same path-injection finding cluster.

### B. Human `[SEC-N]` tracker issues (#373–#397, author KooshaPari)

These are deliberate human-curated grouping/triage tickets. Bodies are stub: `*Auto-populated from CodeQL security alerts*` — they were created as a per-file/per-rule planning queue layered on top of the raw bot alerts.

Top 5 (highest leverage):
1. **#385 [SEC-13]** Fix path-injection in `auth_files.go` (**4 alerts**) — biggest concentrated cluster; one fix likely closes 4 CodeQL alerts.
2. **#383 [SEC-11]** Fix path-injection in `token.go` (2 alerts) — auth-critical module.
3. **#374 [SEC-2]** Fix request-forgery in `sso_oidc.go` (**3 alerts**) — critical severity, OIDC flow.
4. **#375 [SEC-3]** Fix request-forgery in `api_tools.go` (2 alerts) — critical severity.
5. **#373 [SEC-1]** Fix request-forgery in `antigravity_executor.go` — critical severity.

Other SEC tracker issues: #376–#378 weak-sensitive-data-hashing (3 files), #379–#384 path-injection across store layer (postgres/object/git/oauth-sessions/token/request_logger), #386–#388 reflected-xss (oauth_server, response_rewriter, response_writer), #389 allocation-size-overflow, #390–#391 unvalidated-url-redirection / bad-redirect-check in auth_files, #392–#394 clear-text-logging, #395 "resolve to 0", #396 add SAST to CI, #397 docs.

---

## 2. Deduplication finding

The bot-filed CodeQL issues and the human SEC-N issues are **two views of the same underlying CodeQL alert set**. Roughly:

- ~14 visible CodeQL bot issues ≈ subset of ~25 SEC-N tracker entries (each SEC-N can wrap 1–4 CodeQL alerts).
- SEC-N tickets give file-level grouping (the actionable unit). Bot issues give 1:1 alert mirrors (the verification unit).
- **Recommended:** treat SEC-N as the workstream unit. When a SEC-N is closed by a fix PR, the corresponding bot issues will auto-close via the alert-sync workflow once code-scanning re-runs.
- **Do NOT manually close bot issues** — the `auto-alert-sync` workflow owns their lifecycle.

---

## 3. Recommended fix order (by leverage × severity)

| # | Ticket | Why | Est. agent effort |
|---|---|---|---|
| 1 | **SEC-1/2/3** (#373, #374, #375) — request-forgery | Only `critical`-severity bucket. SSRF is the highest blast-radius class here (auth + tool execution). 6 alerts across 3 files. | 1 focused PR, ~3–6 tool calls per file → 1 agent batch (~8–15 tool calls total) using a shared URL-allowlist helper. |
| 2 | **SEC-13** (#385) — path-injection in `auth_files.go` (4 alerts) | Single-file, 4 alerts; highest dedup ratio. `filepath.Clean` + base-dir containment is a known pattern. | 1 PR, ~3–5 tool calls. |
| 3 | **SEC-9..12** (#381–#384) — path-injection in store layer (gitstore, objectstore, postgresstore, oauth_sessions, token) | Same pattern as SEC-13; can share a `safeJoin(base, untrusted) (string, error)` helper. | 1–2 PRs, ~10 tool calls; introduces a shared `internal/pathsafe` util. |
| 4 | **SEC-17** (#389) — allocation-size-overflow in `kiro_websearch.go` | Single hit; cap with `if n > MAX { return ErrTooLarge }`. | 1 PR, 2–3 tool calls. |
| 5 | **SEC-14/15/16** (#386–#388) — reflected-xss (oauth_server, response_rewriter, response_writer) | XSS in OAuth UI surface. Requires `html/template` or escape-on-write; touches response pipeline. | 1 PR, 5–8 tool calls. Higher risk of regression — needs response-shape tests. |
| 6 | **SEC-4/5/6** (#376–#378) — weak-sensitive-data-hashing | `sha1`/`md5` for tokens/IDs → swap to `sha256`. Mostly mechanical; verify no on-disk format break. | 1 PR, 3–5 tool calls. |
| 7 | **SEC-18/19** (#390, #391) — open-redirect / bad-redirect-check in `auth_files.go` | Allowlist redirect targets. | 1 PR, 3–4 tool calls. |
| 8 | **SEC-20/21/22** (#392–#394) — clear-text-logging | Redact secrets in log lines. Mechanical. | 1 PR, 3–5 tool calls. |
| 9 | **SEC-24** (#396) — add security linting to CI | After backlog drains, lock the gate so regressions can't land. **Note:** Actions billing constraint — must run `gosec`/`semgrep` on a Linux runner only, no macOS/Windows matrix. | 1 PR, 4–6 tool calls. |
| 10 | **SEC-23** (#395) — "Resolve remaining CodeQL alerts to 0" + **SEC-25** (#397) — docs | Closeout step after #1–#9. | 1–2 tool calls each. |

**Note on coupling with `auth_files.go`:** SEC-13, SEC-18, SEC-19 all touch the same file. **Bundle into a single auth_files hardening PR** (≈6 alerts in one diff) rather than 3 PRs.

---

## 4. Effort summary

| Bucket | Tickets | Est. wall-clock | Est. tool calls |
|---|---|---|---|
| Critical SSRF (request-forgery) | SEC-1/2/3 | 5–10 min | 8–15 |
| Path injection cluster | SEC-7/8/9/10/11/12/13 | 10–15 min | 15–25 (one shared helper PR + per-file follow-ups) |
| Memory safety | SEC-17 | 1–2 min | 2–3 |
| XSS | SEC-14/15/16 | 5–10 min | 5–10 |
| Hashing | SEC-4/5/6 | 3–5 min | 3–5 |
| Redirects | SEC-18/19 (bundle into auth_files PR) | 2–3 min | merged above |
| Logging | SEC-20/21/22 | 3–5 min | 3–5 |
| CI gate | SEC-24 | 3–5 min | 4–6 |
| Closeout/docs | SEC-23/25 | 1–2 min | 2 |
| **Total** | **25 SEC issues + ~14 CodeQL mirrors** | **~35–60 min agent time** | **~45–75 tool calls** |

---

## 5. Constraints / non-blocking observations

- **CI billing:** Repo CI is currently failing (#702–#709) due to the org-wide GitHub Actions billing block. Security-fix PRs must be verified locally; do not gate merges on CI status. Any new SAST in SEC-24 must run only on standard Linux runners.
- **Bot issue lifecycle:** Do not close `auto-alert-sync`-labeled issues by hand. They reflect live CodeQL alert state.
- **Shared helpers wanted:** at minimum `internal/pathsafe.SafeJoin` and `internal/urlguard.AllowedOutbound` would deduplicate fixes across SSRF + path-injection clusters. Worth introducing in PR #1 of the campaign.
- **Out of scope here:** the `[OPT-N]` (#361–#372) decomposition tickets and `Cliproxy Feature N` / `Task: cliproxy++ item N` (#325–#359) backlog — none are security issues.

---

## 6. Top-3 priority items (executive summary)

1. **SEC-1 / SEC-2 / SEC-3 (issues #373, #374, #375)** — fix the 6 `go/request-forgery` (critical) alerts in `antigravity_executor.go`, `sso_oidc.go`, `api_tools.go` via a shared outbound-URL allowlist helper. Closes both critical-severity CodeQL bot issues (#848, #808).
2. **SEC-13 (issue #385) — path-injection in `auth_files.go` (4 alerts)** — single-file, highest dedup ratio. Bundle with **SEC-18 (#390)** and **SEC-19 (#391)** since all three touch the same file.
3. **SEC-9..12 (issues #381–#384) — path-injection in storage layer** — gitstore / objectstore / postgresstore / oauth_sessions / token / request_logger. Introduce `internal/pathsafe.SafeJoin` here; reuse for SEC-13 cluster.
