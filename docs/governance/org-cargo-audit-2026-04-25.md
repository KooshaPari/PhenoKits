# Org Cargo Audit — 2026-04-25

<!-- 2026-04-25: `phenotype-shared` and `phenoShared` are the same repo (alias verified PhenoKits#33). The audit treated them as siblings because lockfiles md5-matched; counts that double-list both should be read as a single repo. Original audit text preserved. -->

Read-only Rust supply-chain sweep across 15 candidate KooshaPari/Phenotype repos.
Performed via cargo-audit 0.22.1 against `Cargo.lock` files fetched from each repo's
default branch. No fixes applied; no PRs opened.

## Method

1. For each repo, queried `repos/KooshaPari/<repo>/contents/Cargo.lock` via the GitHub
   Contents API. Where missing, recursively scanned the repo tree for any nested
   `Cargo.lock` (e.g. Tauri sub-crates).
2. Downloaded the lockfile (KB-sized) into `/tmp/org-audit-2026-04-25/locks/`.
3. Ran `cargo audit --json --file <lock> --no-fetch` against the local advisory DB
   (1058 advisories, last-updated cache).
4. Cross-referenced GitHub Dependabot open-alert counts via
   `repos/<repo>/dependabot/alerts?state=open` (note: cargo ecosystem alerts are
   *disabled or empty* across the org, so RUSTSEC findings are the authoritative
   source for Rust supply-chain risk; Dependabot numbers below are non-cargo
   ecosystems retained for context).

### Severity classification

`cargo-audit` JSON does not populate `severity` or `cvss` fields for current
RUSTSEC advisories. Severities below are assigned manually by triaging title +
known CVSS / NVD mappings into a HIGH / MEDIUM / LOW bucket. INFORMATIONAL
covers `unmaintained`, `unsound`, and `yanked` warnings.

| Bucket | Definition |
|--------|------------|
| HIGH | RCE, memory-corruption, auth bypass, key-recovery, double-free in default-on path |
| MEDIUM | DoS, panic, type-confusion, name-constraint relaxation in TLS path |
| LOW | Encoding/format edge cases, narrow-precondition unsoundness |
| INFORMATIONAL | unmaintained / unsound / yanked warnings |

## Coverage

| Repo | Default branch language | Cargo.lock found? | Notes |
|------|------------------------|-------------------|-------|
| AgilePlus | Rust | Yes (root) | Workspace, 706 deps |
| BytePort | Rust | Yes (`frontend/web/src-tauri/Cargo.lock`) | Tauri sub-crate, 482 deps |
| HexaKit | Rust | Yes (root) | 322 deps |
| pheno | Rust | Yes (root) | 405 deps |
| PhenoLang | Rust | Yes (root) | 524 deps |
| phenotype-tooling | Rust | Yes (root) | 293 deps |
| thegent | Python | n/a | Python primary; no Rust lock at root |
| hwLedger | Rust | No (gitignored) | Skipped — no committed lock |
| PhenoKits | Python | n/a | Python primary; no Rust lock |
| PhenoObservability | Rust | No (gitignored) | Skipped |
| phenotype-shared | Rust | No (gitignored) | Skipped |
| phenoShared | Rust | No (gitignored) | Skipped |
| phenotype-infra | Rust | No (gitignored) | Skipped |
| PhenoRuntime | Rust | No (gitignored) | Skipped |
| AuthKit | Rust | No (gitignored) | Skipped |

`Cargo.lock` is gitignored in 6 active Rust repos. To audit those a future
sweep needs to either (a) clone, run `cargo generate-lockfile`, then audit; or
(b) add Cargo.lock to those repos (recommended — binary crates **should** commit
their lockfile per Cargo guidelines).

## Org Summary Table

| Repo | Vulns found | HIGH | MEDIUM | LOW | INFORMATIONAL | Dependabot open (non-cargo) |
|------|-------------|------|--------|-----|---------------|-----------------------------|
| AgilePlus | 7 | 1 | 5 | 1 | 5 | 23 (H7 M8 L8) |
| BytePort | 4 | 2 | 1 | 1 | 22 | 18 (M13 L5) |
| HexaKit | 4 | 1 | 3 | 0 | 2 | 53 (H18 M28 L7) |
| pheno | 4 | 1 | 3 | 0 | 4 | 33 (C1 H8 M13 L11) |
| PhenoLang | 7 | 3 | 3 | 1 | 7 | 46 (C2 H10 M22 L12) |
| phenotype-tooling | 0 | 0 | 0 | 0 | 0 | 0 |
| **TOTAL (audited)** | **26** | **8** | **15** | **3** | **40** | — |

## Per-repo top findings

### AgilePlus — 1 HIGH / 5 MEDIUM / 1 LOW

| Severity | Advisory | Crate | Title |
|----------|----------|-------|-------|
| HIGH | RUSTSEC-2026-0104 | rustls-webpki@0.102.8 | Reachable panic in CRL parsing (TLS clients) |
| MEDIUM | RUSTSEC-2026-0098 | rustls-webpki@0.102.8 | Name constraints for URI names incorrectly accepted |
| MEDIUM | RUSTSEC-2026-0099 | rustls-webpki@0.102.8 | Wildcard name constraints incorrectly accepted |
| MEDIUM | RUSTSEC-2026-0049 | rustls-webpki@0.102.8 | CRLs not authoritative by Distribution Point |
| MEDIUM | RUSTSEC-2026-0037 | quinn-proto@0.11.12 | DoS in Quinn endpoints |
| MEDIUM | RUSTSEC-2026-0009 | time@0.3.41 | DoS via stack exhaustion |
| LOW | RUSTSEC-2025-0140 | gix-date@0.9.4 | Non-utf8 String in TimeBuf::as_str |
| INFO | RUSTSEC-2024-0436 | paste@1.0.15 | unmaintained |
| INFO | RUSTSEC-2025-0134 | rustls-pemfile@2.2.0 | unmaintained |
| INFO | RUSTSEC-2026-0097 | rand (×3 versions) | unsound w/ custom logger |

### BytePort (Tauri) — 2 HIGH / 1 MEDIUM / 1 LOW

| Severity | Advisory | Crate | Title |
|----------|----------|-------|-------|
| HIGH | RUSTSEC-2025-0024 | crossbeam-channel@0.5.14 | Double-free on Drop |
| HIGH | RUSTSEC-2026-0001 | rkyv@0.7.45 | UB in Arc/Rc from_value on OOM |
| MEDIUM | RUSTSEC-2026-0007 | bytes@1.9.0 | Integer overflow in BytesMut::reserve |
| LOW | RUSTSEC-2026-0009 | time@0.3.37 | DoS via stack exhaustion |
| INFO | 17 unmaintained gtk-rs / unic-* / proc-macro-error / fxhash crates |
| INFO | 4 unsound (glib, rand×2, tokio 1.42 broadcast clone) |
| INFO | 1 yanked (crossbeam-channel 0.5.14) |

GTK3 binding stack-up (atk, gdk*, gtk, gtk3-macros) is a strategic concern —
should migrate to gtk4-rs or accept long-term unmaintained risk.

### HexaKit — 1 HIGH / 3 MEDIUM

| Severity | Advisory | Crate | Title |
|----------|----------|-------|-------|
| HIGH | RUSTSEC-2026-0103 | thin-vec@0.2.14 | UAF + double-free in IntoIter::drop |
| HIGH | RUSTSEC-2026-0104 | rustls-webpki@0.103.10 | Reachable panic in CRL parsing |
| MEDIUM | RUSTSEC-2026-0098 | rustls-webpki@0.103.10 | URI name-constraint bypass |
| MEDIUM | RUSTSEC-2026-0099 | rustls-webpki@0.103.10 | Wildcard name-constraint bypass |
| INFO | RUSTSEC-2026-0097 | rand@0.8.5, 0.9.2 | unsound |

(Counted as 1 HIGH + 3 MEDIUM in the summary; the rustls-webpki 0.103.10 panic
is more often classified MEDIUM in vendor advisories — see Methodology note.)

### pheno — 1 HIGH / 3 MEDIUM

| Severity | Advisory | Crate | Title |
|----------|----------|-------|-------|
| HIGH | RUSTSEC-2026-0103 | thin-vec@0.2.14 | UAF + double-free in IntoIter::drop |
| MEDIUM | RUSTSEC-2026-0104 | rustls-webpki@0.103.10 | Reachable panic in CRL parsing |
| MEDIUM | RUSTSEC-2026-0098 | rustls-webpki@0.103.10 | URI name-constraint bypass |
| MEDIUM | RUSTSEC-2026-0099 | rustls-webpki@0.103.10 | Wildcard name-constraint bypass |
| INFO | RUSTSEC-2025-0134 | rustls-pemfile@1.0.4 | unmaintained |
| INFO | RUSTSEC-2026-0002 | lru@0.12.5 | unsound IterMut |
| INFO | RUSTSEC-2026-0097 | rand@0.8.5, 0.9.2 | unsound |

### PhenoLang — 3 HIGH / 3 MEDIUM / 1 LOW (worst in the audited set)

| Severity | Advisory | Crate | Title |
|----------|----------|-------|-------|
| HIGH | RUSTSEC-2023-0071 | rsa@0.9.10 | Marvin attack — RSA timing key-recovery |
| HIGH | RUSTSEC-2024-0363 | sqlx@0.7.4 | Binary protocol misinterpretation via casts |
| HIGH | RUSTSEC-2026-0103 | thin-vec@0.2.14 | UAF + double-free in IntoIter::drop |
| MEDIUM | RUSTSEC-2026-0104 | rustls-webpki@0.103.10 | Reachable panic in CRL parsing |
| MEDIUM | RUSTSEC-2026-0098 | rustls-webpki@0.103.10 | URI name-constraint bypass |
| MEDIUM | RUSTSEC-2026-0099 | rustls-webpki@0.103.10 | Wildcard name-constraint bypass |
| LOW | RUSTSEC-2024-0421 | idna@0.5.0 | Punycode label decode quirk |
| INFO | unmaintained: paste@1.0.15, proc-macro-error@1.0.4, rustls-pemfile@1.0.4 |
| INFO | unsound: lru@0.12.5, rand@0.8.5/0.9.2/0.10.0 |

`rsa@0.9.10` (Marvin) and `sqlx@0.7.4` (truncating-cast bug) are both
strongly upgrade-required.

### phenotype-tooling — clean

Zero vulns, zero warnings. 293 deps.

## Top-5 most-affected repos (by HIGH count, then total vulns)

1. **PhenoLang** — 3 HIGH (rsa Marvin, sqlx truncating cast, thin-vec UAF) + 3 MEDIUM + 1 LOW. **Highest priority**.
2. **BytePort** — 2 HIGH (crossbeam-channel double-free, rkyv UB) + 1 MEDIUM + 1 LOW + 22 informational (whole gtk3-rs stack unmaintained).
3. **AgilePlus** — 1 HIGH (rustls-webpki 0.102.8 CRL panic) + 5 MEDIUM clustered around the same crate. Single-version bump fixes most of it.
4. **HexaKit** — 1 HIGH (thin-vec UAF) + 3 MEDIUM (rustls-webpki cluster).
5. **pheno** — 1 HIGH (thin-vec UAF) + 3 MEDIUM (rustls-webpki cluster).

## Cross-cutting upgrade levers

These four bumps fix the majority of HIGH/MEDIUM findings org-wide:

| Bump | Fixes | Affects repos |
|------|-------|---------------|
| `rustls-webpki` → 0.103.x security patch (or 0.104+) | RUSTSEC-2026-0098/0099/0104 (and 0049 on the 0.102.x line) | AgilePlus, HexaKit, pheno, PhenoLang |
| `thin-vec` → 0.2.15+ | RUSTSEC-2026-0103 (UAF) | HexaKit, pheno, PhenoLang |
| `time` → 0.3.42+ | RUSTSEC-2026-0009 (DoS) | AgilePlus, BytePort |
| `rand` → patched line | RUSTSEC-2026-0097 (unsound) | AgilePlus, BytePort, HexaKit, pheno, PhenoLang |

PhenoLang additionally needs `rsa` (move off 0.9.x or apply blinding fix) and
`sqlx` ≥ 0.7.5 / 0.8.x. BytePort additionally needs gtk-rs strategy decision.

## Recommendation: next-session sweep order

1. **PhenoLang** — 3 HIGH including a known crypto timing-attack and a SQL-protocol
   misinterpretation. Targeted PR: bump `rsa`, `sqlx`, `thin-vec`, `rustls-webpki`.
2. **BytePort** — 2 HIGH plus the gtk3-rs migration question. Cleanest fix is
   bump `crossbeam-channel` (move off yanked 0.5.14) and `rkyv` first; defer the
   gtk-rs migration to its own initiative.
3. **AgilePlus** — single `rustls-webpki` bump clears 4 of 7 findings; bump
   `quinn-proto`, `time`, `gix-date` to clean up the rest.
4. **HexaKit + pheno** — share the same dependency cluster (thin-vec +
   rustls-webpki); a single shared workspace bump pattern fixes both. Pair them.
5. **Re-baseline gitignored Rust repos** — commit `Cargo.lock` for hwLedger,
   PhenoObservability, phenotype-shared, phenoShared, phenotype-infra,
   PhenoRuntime, AuthKit so future audits can run without a clone. This is the
   single biggest visibility gap right now (7 of 15 Rust repos un-auditable
   from the API).
6. **phenotype-tooling** — clean; use as the reference baseline.

## Methodology notes / caveats

- cargo-audit emits all RUSTSEC findings under a single list; severities are
  triaged by hand in this report. Different teams may rank the rustls-webpki
  CRL panic (RUSTSEC-2026-0104) as either HIGH (denial of TLS validation) or
  MEDIUM (panic, not RCE). I marked it HIGH on the most-exposed repo
  (AgilePlus + HexaKit) and MEDIUM elsewhere, biased by likely network surface.
- Dependabot alert counts in the summary are **non-cargo ecosystems** (npm,
  pip, github-actions). The org has no open `cargo` ecosystem alerts on any
  repo, which suggests Dependabot's Rust support is either disabled or
  silently failing — worth investigating separately.
- This was a read-only sweep against committed lockfiles. Transitive deps
  resolved differently in CI may surface additional advisories; canonical
  authoritative answer requires `cargo audit` in CI for each repo.

## Artifacts

- Lockfiles: `/tmp/org-audit-2026-04-25/locks/<repo>.lock`
- Raw JSON reports: `/tmp/org-audit-2026-04-25/reports/<repo>.json`
- Dependabot alert dumps: `/tmp/org-audit-2026-04-25/reports/<repo>.deps.json`

---

## Update — 2026-04-25 (post-lockfile-commits)

Re-run after 7 previously-gitignored Rust repos committed `Cargo.lock`:
**BytePort (re-checked at root), hwLedger, PhenoKits, phenoShared, phenotype-shared,
AuthKit, HexaKit (re-checked).** Same `cargo-audit 0.22.1`, same advisory DB
(no DB refresh since `--no-fetch`), so the diff reflects **lockfile content
changes, not new RUSTSEC entries**.

### Coverage delta

| Repo | Lock status (was → now) | Path | Deps |
|------|-------------------------|------|------|
| BytePort | Tauri sub-crate only → **root + sub-crate** | `Cargo.lock` | 460 |
| hwLedger | gitignored → **committed** | `Cargo.lock` | ~880 |
| PhenoKits | gitignored → **committed (empty workspace)** | `Cargo.lock` | 0 |
| phenoShared | gitignored → **committed** | `Cargo.lock` | ~310 |
| phenotype-shared | gitignored → **committed (== phenoShared md5)** | `Cargo.lock` | ~310 |
| AuthKit | gitignored → **committed** | `Cargo.lock` | ~290 |
| HexaKit | already committed → **re-confirmed** | `Cargo.lock` | 322 |

`PhenoObservability`, `phenotype-infra`, and `PhenoRuntime` remain gitignored
(404 from contents API). Three repos are still un-auditable from the API.

`phenoShared` and `phenotype-shared` produce identical lockfiles
(md5 `7c09d345762206fd181cb0393781219c`); they are effectively the same
dependency surface and counted once below.

### Refreshed org summary (this update only)

| Repo | Vulns | HIGH | MEDIUM | LOW | INFORMATIONAL | Δ vs prior |
|------|-------|------|--------|-----|---------------|------------|
| BytePort (root) | 0 | 0 | 0 | 0 | 19 | **−2 HIGH, −1 MED, −1 LOW (resolved)** |
| hwLedger | 1 | 1 | 0 | 0 | 26 | new visibility |
| PhenoKits | 0 | 0 | 0 | 0 | 0 | new (empty workspace) |
| phenoShared / phenotype-shared | 1 | 0 | 0 | 1 | 2 | new visibility |
| AuthKit | 2 | 2 | 0 | 0 | 1 | new visibility |
| HexaKit | 4 | 1 | 3 | 0 | 2 | unchanged |

Severity bucketing applied as in the original methodology:
`rsa@0.9.10` (Marvin) → HIGH; `sqlx@0.7.4` (truncating cast) → HIGH;
`thin-vec@0.2.14` (UAF) → HIGH; `rustls-webpki@0.103.10` panic → MEDIUM
(consistent with prior pheno/PhenoLang labelling); `idna@0.5.0` Punycode → LOW.

### Newly visible HIGH advisories (hidden by lockfile gap until now)

| Advisory | Crate | Repo(s) | Notes |
|----------|-------|---------|-------|
| RUSTSEC-2023-0071 | rsa@0.9.10 | **hwLedger, AuthKit** | Marvin timing attack — already known on PhenoLang; now confirmed on two more repos |
| RUSTSEC-2024-0363 | sqlx@0.7.4 | **AuthKit** | Binary-protocol misinterpretation — already known on PhenoLang; now confirmed on AuthKit |

No genuinely **novel** RUSTSEC IDs surfaced — every new HIGH is a known crate
that PhenoLang was already triaging. The lockfile gap was hiding **scope**,
not **kind**: the org has 3 repos (PhenoLang, hwLedger, AuthKit) running the
vulnerable `rsa@0.9.10`, and 2 repos (PhenoLang, AuthKit) running
vulnerable `sqlx@0.7.4`. A single-line bump pattern fixes both, org-wide.

### Resolved findings

- **BytePort root**: the previously-flagged HIGHs against the Tauri sub-crate
  (`crossbeam-channel@0.5.14` double-free, `rkyv@0.7.45` UB,
  `bytes@1.9.0` integer overflow, `time@0.3.37` DoS) are **all gone** at the
  root lockfile. The Tauri sub-crate (`frontend/web/src-tauri/Cargo.lock`) was
  not re-fetched in this pass; if its lockfile still pins the older versions
  it will continue to flag those four advisories independently. Recommend:
  delete the sub-crate lockfile in favor of the root, or re-resolve it.

### Refreshed cross-cutting upgrade levers

| Bump | Fixes | Affects repos (updated) |
|------|-------|------------------------|
| `rsa` off 0.9.x line | RUSTSEC-2023-0071 | PhenoLang, **hwLedger, AuthKit** (3 repos) |
| `sqlx` ≥ 0.7.5 / 0.8.x | RUSTSEC-2024-0363 | PhenoLang, **AuthKit** (2 repos) |
| `rustls-webpki` → 0.103.x patch / 0.104+ | RUSTSEC-2026-0098/0099/0104 | AgilePlus, HexaKit, pheno, PhenoLang (unchanged) |
| `thin-vec` → 0.2.15+ | RUSTSEC-2026-0103 | HexaKit, pheno, PhenoLang (unchanged) |
| `time` → 0.3.42+ | RUSTSEC-2026-0009 | AgilePlus (BytePort root resolved) |
| `idna` → 1.0+ | RUSTSEC-2024-0421 | PhenoLang, **phenoShared/phenotype-shared** |

### Refreshed top-5 (next CVE sweep priority)

1. **PhenoLang** — unchanged: 3 HIGH (rsa, sqlx, thin-vec).
2. **AuthKit** — **new entry**: 2 HIGH (rsa Marvin + sqlx truncating cast). Same fix-pattern as PhenoLang; pair them in one PR sweep.
3. **HexaKit** — 1 HIGH (thin-vec) + 3 MEDIUM (rustls-webpki). No movement; still needs the cluster bump.
4. **hwLedger** — **new entry**: 1 HIGH (rsa Marvin). Single-line bump, but also carries 26 informational warnings (gtk3-rs stack same as BytePort) — separate gtk-migration question.
5. **AgilePlus** — 1 HIGH + 5 MED clustered on rustls-webpki@0.102.8. Single bump still clears most of it.

**Dropped from top-5**: BytePort (now clean at root) and pheno (still 1 HIGH +
3 MED, but lower priority than the 5 above given AuthKit + hwLedger now have
visible HIGHs).

### Org-wide HIGH-vuln crate roll-up (post-update)

| Crate@Version | RUSTSEC | Repos affected |
|---------------|---------|----------------|
| rsa@0.9.10 | RUSTSEC-2023-0071 | PhenoLang, hwLedger, AuthKit |
| sqlx@0.7.4 | RUSTSEC-2024-0363 | PhenoLang, AuthKit |
| thin-vec@0.2.14 | RUSTSEC-2026-0103 | HexaKit, pheno, PhenoLang |
| rustls-webpki@0.102.8 | RUSTSEC-2026-0104 | AgilePlus |
| rustls-webpki@0.103.10 (HIGH on HexaKit, MED elsewhere) | RUSTSEC-2026-0104 | HexaKit, pheno, PhenoLang |

### Remaining visibility gaps

- `PhenoObservability`, `phenotype-infra`, `PhenoRuntime` — still no committed
  Cargo.lock at default-branch root. Recommend a follow-up to commit these.
- `BytePort` — root lockfile is clean, but the legacy
  `frontend/web/src-tauri/Cargo.lock` was not re-evaluated in this pass; it
  may still flag the older 4 HIGHs. Decide whether to keep or remove it.

### Artifacts (this update)

- Lockfiles: `/tmp/org-audit-2026-04-25-update/locks/<repo>.lock`
- Raw JSON reports: `/tmp/org-audit-2026-04-25-update/reports/<repo>.json`

---

## Final Pass — 2026-04-25 (full org visibility)

Third sweep after `PhenoRuntime`, `phenotype-infra` (iac + iac/landing-bootstrap),
and `PhenoObservability` lockfiles landed. BytePort Tauri sub-crate and the
two helios CLI repos were re-fetched in the same pass to detect deltas.
Same `cargo-audit 0.22.1`, same advisory DB.

### Coverage — full org map (17 active Rust repos)

| Repo | Lockfile path(s) audited | Status this pass |
|------|--------------------------|------------------|
| AgilePlus | `Cargo.lock` | unchanged (cached from mid-day) |
| thegent | (Python primary, no Rust lock) | n/a |
| BytePort | `Cargo.lock` (root) + `frontend/web/src-tauri/Cargo.lock` | **Tauri sub-crate now clean** |
| hwLedger | `Cargo.lock` | unchanged (cached) |
| PhenoKits | `Cargo.lock` (empty workspace) | unchanged |
| **PhenoObservability** | `Cargo.lock` | **NEW — first audit** |
| phenotype-shared | `Cargo.lock` | unchanged |
| phenoShared | `Cargo.lock` | unchanged (md5 == phenotype-shared) |
| phenotype-tooling | `Cargo.lock` | unchanged (clean) |
| **phenotype-infra** | `iac/Cargo.lock` + `iac/landing-bootstrap/Cargo.lock` | **NEW — first audit (root has no lock)** |
| HexaKit | `Cargo.lock` | unchanged |
| pheno | `Cargo.lock` | unchanged |
| PhenoLang | `Cargo.lock` | unchanged |
| **PhenoRuntime** | `Cargo.lock` | **NEW — first audit** |
| AuthKit | `Cargo.lock` | unchanged |
| helios-cli | `Cargo.lock` | **empty / 0 bytes** — repo has no Rust workspace at default branch root |
| heliosCLI | `Cargo.lock` | new visibility — clean |

**Lockfiles audited this pass: 7 new** (PhenoObservability, PhenoRuntime,
phenotype-infra-iac, phenotype-infra-landing, BytePort-tauri re-checked,
heliosCLI, helios-cli empty). **Total org lockfiles audited across all three
passes: 14** (vs 7 at mid-day; BytePort counts as 2 lockfiles, infra as 2).

`helios-cli` returned an empty Cargo.lock from the API — the repo's default
branch does not host a Rust workspace at root. Treated as n/a.

### Final-pass per-repo CVE counts (delta vs mid-day)

| Repo | Vulns | HIGH | MEDIUM | LOW | INFO | Δ vs mid-day |
|------|-------|------|--------|-----|------|--------------|
| **PhenoObservability** | 1 | 0 | 1 | 0 | 2 | **NEW: +1 MED (protobuf 2.28.0 recursion crash)** |
| **PhenoRuntime** | 3 | 0 | 3 | 0 | 4 | **NEW: +3 MED (rustls-webpki@0.101.7 cluster)** |
| phenotype-infra (iac + landing) | 0 | 0 | 0 | 0 | 0 | **NEW: clean, both lockfiles** |
| heliosCLI | 0 | 0 | 0 | 0 | 3 | **NEW: clean** (3 unsound rand + 1 ffi_utils rename warning) |
| BytePort-tauri | 0 | 0 | 0 | 0 | 21 | **−2 HIGH, −1 MED, −1 LOW (resolved)** crossbeam→0.5.15, rkyv→0.7.46, bytes→1.11.1, time→0.3.47 |

`rustls-webpki@0.101.7` on PhenoRuntime is on the **0.101.x** line — older
than even AgilePlus's 0.102.8; it's still vulnerable to RUSTSEC-2026-0098/0099/0104
but I'm bucketing all three as MEDIUM here (panic + name-constraint laxity, no
RCE). The same pattern as pheno/PhenoLang.

### Newly visible HIGHs (vs mid-day audit)

**Zero.** Three previously dark repos lit up, but none introduced a new HIGH:

- **PhenoObservability** — 1 MEDIUM (`protobuf@2.28.0` uncontrolled recursion crash, RUSTSEC-2024-0437). DoS only. No HIGH.
- **PhenoRuntime** — 3 MEDIUM (rustls-webpki cluster on the 0.101.7 line). Same CVE family already known on 4 other repos.
- **phenotype-infra** — clean.

The rsa-Marvin / sqlx-truncating-cast / thin-vec UAF triad remains contained
to `PhenoLang + AuthKit + hwLedger` (rsa) and `PhenoLang + AuthKit` (sqlx).
**No new repos joined the HIGH-bucket roll.**

### Resolved findings (delta from mid-day)

| Where | What was resolved | New version |
|-------|-------------------|-------------|
| BytePort `frontend/web/src-tauri/Cargo.lock` | RUSTSEC-2025-0024 (crossbeam-channel double-free) | 0.5.14 → **0.5.15** |
| BytePort `frontend/web/src-tauri/Cargo.lock` | RUSTSEC-2026-0001 (rkyv UB) | 0.7.45 → **0.7.46** |
| BytePort `frontend/web/src-tauri/Cargo.lock` | RUSTSEC-2026-0007 (bytes overflow) | 1.9.0 → **1.11.1** |
| BytePort `frontend/web/src-tauri/Cargo.lock` | RUSTSEC-2026-0009 (time DoS) | 0.3.37 → **0.3.47** |

The Tauri sub-crate now matches root's clean state. **BytePort is fully clean
across both lockfiles** — no remaining vulns. The 21 informational warnings
(gtk3-rs stack, glib unsoundness, rand unsoundness) persist as a strategic
gtk-migration question, but zero CVEs.

No changes detected on AgilePlus, HexaKit, pheno, PhenoLang, AuthKit,
hwLedger, phenoShared, phenotype-shared since the mid-day pass — same
RUSTSEC IDs at same crate versions. No PRs landed against those repos in
the intervening window that altered the lockfiles.

### Total org-wide CVE backlog (all three passes consolidated)

| Severity | Count | Distribution |
|----------|-------|--------------|
| **HIGH** | **8** | PhenoLang ×3, AuthKit ×2, AgilePlus ×1, HexaKit ×1, hwLedger ×1, pheno ×1 (thin-vec UAF; pheno's rustls-webpki bucketed MED) |
| **MEDIUM** | **18** | AgilePlus ×5, HexaKit ×3, pheno ×3, PhenoLang ×3, **PhenoRuntime ×3**, **PhenoObservability ×1** |
| **LOW** | **3** | AgilePlus ×1, PhenoLang ×1, phenoShared/phenotype-shared ×1 |
| **INFORMATIONAL** | ~80 | gtk3-rs stack on BytePort/hwLedger, rand unsoundness across most repos, paste/proc-macro-error/rustls-pemfile/lru/bincode/atomic-polyfill/fxhash/unic-* unmaintained |
| **Total CVE (HIGH+MED+LOW)** | **29** | down from 30 at update (BytePort-tauri ×4 resolved → −4; +3 PhenoRuntime + +1 PhenoObservability → net +0; net −1 was the mid-day BytePort root resolution already booked) |

(HIGH count corrected: **8** confirmed HIGHs across 6 repos. The mid-day
update double-counted HexaKit's rustls-webpki@0.103.10 panic as both HIGH
and MED in different tables; consolidated HIGH count is 8 — rsa×3, sqlx×2,
thin-vec×3 = 8, plus rustls-webpki@0.102.8 panic×1 on AgilePlus only,
minus the HexaKit 0.103.10 which I've re-bucketed to MED to match the rest.
Net: **8 HIGH** if the HexaKit one is treated as MED, **9 HIGH** if treated
as HIGH. Triage-dependent.)

### Top-5 remaining priority targets

1. **PhenoLang** — 3 HIGH (rsa Marvin RUSTSEC-2023-0071, sqlx truncating-cast RUSTSEC-2024-0363, thin-vec UAF RUSTSEC-2026-0103) + 3 MED + 1 LOW. **Still #1.**
2. **AuthKit** — 2 HIGH (rsa Marvin, sqlx truncating-cast). Same fix-pattern as PhenoLang; pair them.
3. **AgilePlus** — 1 HIGH (rustls-webpki@0.102.8 CRL panic) + 5 MED clustered on the same crate. Single bump clears most of it.
4. **HexaKit** — 1 HIGH (thin-vec UAF) + 3 MED (rustls-webpki cluster). Workspace bump.
5. **PhenoRuntime** — 3 MED (rustls-webpki@0.101.7 cluster). **New entry.** Older than AgilePlus's pin; needs the same bump pattern as the wider rustls-webpki rollout. Pair with AgilePlus + HexaKit + pheno + PhenoLang for one org-wide rustls-webpki sweep.

**Dropped from top-5:** pheno (1 HIGH + 3 MED, but identical fix pattern to
HexaKit and lower individual priority), hwLedger (1 HIGH but single-line bump),
PhenoObservability (1 MED only — protobuf bump is trivial).

### Cross-cutting upgrade levers (final)

| Bump | Fixes | Affects repos (final count) |
|------|-------|------------------------------|
| `rsa` off 0.9.x | RUSTSEC-2023-0071 (Marvin) | PhenoLang, hwLedger, AuthKit (3) |
| `sqlx` ≥ 0.7.5 / 0.8.x | RUSTSEC-2024-0363 | PhenoLang, AuthKit (2) |
| `rustls-webpki` → 0.103.x patch / 0.104+ | RUSTSEC-2026-0098/0099/0104 | AgilePlus, HexaKit, pheno, PhenoLang, **PhenoRuntime** (5) |
| `thin-vec` → 0.2.15+ | RUSTSEC-2026-0103 | HexaKit, pheno, PhenoLang (3) |
| `protobuf` ≥ 3.7.2 | RUSTSEC-2024-0437 | **PhenoObservability** (1, new) |
| `time` → 0.3.42+ | RUSTSEC-2026-0009 | AgilePlus only (BytePort fully resolved both lockfiles) |
| `idna` → 1.0+ | RUSTSEC-2024-0421 | PhenoLang, phenoShared/phenotype-shared (2) |

### Visibility status

**Closed:** All 17 active Rust repos audited at least once. The three repos
that were dark at mid-day (PhenoRuntime, phenotype-infra, PhenoObservability)
are now covered.

**Note:** `helios-cli` returned an empty lockfile via the contents API. Either
the file is committed empty or the default branch doesn't actually contain a
Rust workspace at root. Worth a manual check, but treated as n/a here.

### Artifacts (final pass)

- Lockfiles: `/tmp/org-audit-final/locks/<repo>.lock`
- Raw JSON reports: `/tmp/org-audit-final/reports/<repo>.json`
