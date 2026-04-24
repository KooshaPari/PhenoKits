# Phenotype Publishing Guide — crates.io / npm / PyPI via Trusted Publishing (OIDC)

**Canonical publishing strategy for all Phenotype-org libraries.** Uses
OIDC-based Trusted Publishing — **no long-lived tokens stored in GitHub
Secrets**, no token rotation, no leakage risk. This is the default for every
publishable crate/package/module in the Phenotype organization.

## 1. Decision rationale

Three registries, one pattern:

| Registry  | Mechanism                    | Since     |
|-----------|------------------------------|-----------|
| crates.io | Trusted Publishing (OIDC)    | 2025-07   |
| npmjs.com | Trusted Publishers + `--provenance` (OIDC) | 2025-07 |
| PyPI      | Trusted Publishers (OIDC)    | 2023      |

Why this over `CARGO_REGISTRY_TOKEN` / `NPM_TOKEN` / `PYPI_API_TOKEN`
secrets:

- No secrets to rotate, leak, or accidentally commit.
- Per-workflow scoping: the token is minted only for the exact repo +
  workflow + environment that registered with the registry.
- Provenance (npm) is emitted automatically; crates.io + PyPI attach a
  signed attestation that the artifact was built from a specific commit.
- Aligns with global `~/.claude/CLAUDE.md` "Optionality and Failure
  Behavior" — loud failures, no silent fallback to bearer tokens.

## 2. What NOT to publish

Leave these private / non-publishable:

- **Apps, not libraries:** AgilePlus, heliosApp, heliosCLI, thegent,
  Tracera, BytePort, AgentMCP, Civis, AppGen, Conft. These are end-user
  applications, not consumable packages.
- **Anything marked `publish = false` in `Cargo.toml`** — keep it private.
- **Anything marked `"private": true` in `package.json`** — keep it
  private. Notably `AuthKit/typescript/package.json` (root workspace) is
  private; only `@phenotype/*` sub-packages are candidates.
- **Internal-only contract crates** (e.g. `phenotype-contracts` used only
  by sibling services): leave `publish = false` until a real external
  consumer exists.

## 3. Bellwether matrix

Three reference repos — configure these first; copy the pattern to
remaining libraries as needed.

| Language | Repo | Package(s) | Registry |
|----------|------|------------|----------|
| Rust     | `AuthKit` | `phenotype-policy-engine` (workspace member, library) | crates.io |
| Python   | `DataKit` | `pheno-database` (src layout, hatchling) | PyPI |
| Python   | `ResilienceKit` | `pheno-resilience` | PyPI |
| TypeScript | *(deferred)* | No bellwether — most TS is app code or private workspace; revisit when a genuine shared lib lands. | npm |

## 4. Registry-side setup — **user action required**

Trusted Publishing requires the *publisher* (you, on the registry dashboard)
to register the GitHub repo + workflow filename *before* the first
publish run succeeds. GitHub Actions cannot do this side — it's a manual
click-through per registry, per package.

**Action checklist (once, per package):**

### 4.1 crates.io — Rust

1. Log in at https://crates.io.
2. Publish a first manual release of each crate under your personal
   token (Trusted Publishing can only be added to *existing* crates).
3. For each crate (`phenotype-policy-engine`, etc.):
   - Crate page → **Settings** → **Trusted Publishing** → **Add**.
   - Repository owner: `KooshaPari`
   - Repository name: `AuthKit`
   - Workflow filename: `release.yml`
   - Environment: `release` (recommended; must match `environment:`
     key in workflow job).
4. Revoke the personal `CARGO_REGISTRY_TOKEN` from GitHub Secrets once
   Trusted Publishing is verified to work.

### 4.2 PyPI — Python

1. Log in at https://pypi.org.
2. For each project (`pheno-database`, `pheno-resilience`):
   - Account → **Publishing** → **Add a new pending publisher**
     (if project doesn't exist yet) **or** project page →
     **Publishing** → **Add trusted publisher**.
   - Owner: `KooshaPari`
   - Repository name: `DataKit` / `ResilienceKit`
   - Workflow filename: `release.yml`
   - Environment: `release`
3. Use `pending publisher` flow so the first publish on a new project
   works without uploading via personal token first.

### 4.3 npm — TypeScript (when a shared lib lands)

1. Log in at https://npmjs.com.
2. Package page → **Settings** → **Publishing access** → **Require
   two-factor authentication and trusted publisher**.
3. Add Trusted Publisher → GitHub Actions → fill repo/workflow/env.
4. Ensure the workflow calls `npm publish --provenance --access public`
   (provenance is the OIDC trust handshake).

## 5. Canonical workflow templates

Place each as `.github/workflows/release.yml` in the target repo. Each
triggers on tag push (`v*`) and uses an `environment: release` gate so
the OIDC token is minted with the correct audience.

### 5.1 Rust → crates.io

```yaml
name: release-crates
on:
  push:
    tags: ['v*']
permissions:
  id-token: write   # required for OIDC
  contents: read
jobs:
  publish:
    runs-on: ubuntu-latest
    environment: release
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: rust-lang/crates-io-auth-action@v1
        id: auth
      - name: Publish workspace crates
        env:
          CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}
        run: |
          cargo publish -p phenotype-policy-engine
          # Add more -p <crate> lines as each becomes public
```

Notes:
- `rust-lang/crates-io-auth-action` is the official OIDC exchange action.
- If the workspace has inter-crate dependencies, publish leaves first.
- Keep `publish = false` on crates that are not yet registered.

### 5.2 Python → PyPI

```yaml
name: release-python
on:
  push:
    tags: ['v*']
permissions:
  id-token: write
  contents: read
jobs:
  publish:
    runs-on: ubuntu-latest
    environment: release
    strategy:
      matrix:
        package:
          - python/pheno-database
          # Add more as registered
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'
      - name: Build
        working-directory: ${{ matrix.package }}
        run: |
          python -m pip install --upgrade build
          python -m build
      - name: Publish
        uses: pypa/gh-action-pypi-publish@release/v1
        with:
          packages-dir: ${{ matrix.package }}/dist
```

Notes:
- `pypa/gh-action-pypi-publish` handles OIDC exchange internally when
  no `password:` is set.
- Each package has its own version; tag format `v<pkg>-<ver>` is
  optional; the simpler approach is per-repo synchronized releases.

### 5.3 TypeScript → npm (template, deferred)

```yaml
name: release-npm
on:
  push:
    tags: ['v*']
permissions:
  id-token: write
  contents: read
jobs:
  publish:
    runs-on: ubuntu-latest
    environment: release
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '22'
          registry-url: 'https://registry.npmjs.org'
      - run: npm ci
      - run: npm run build
      - run: npm publish --provenance --access public
```

## 6. Tag conventions

- Single-package repo: `v0.2.1` publishes the one package.
- Multi-package workspace: either
  - (a) bump all packages together on a single `v0.2.1` tag, or
  - (b) use `<pkg>-v0.2.1` tags + filter in the workflow.
- Keep CHANGELOG.md at repo root; reference in release notes via
  `gh release create` in a downstream step (optional).

## 7. Rollout order

1. **AuthKit** (Rust): publish `phenotype-policy-engine` first. It's
   the cleanest standalone library in the workspace.
2. **DataKit** (Python): publish `pheno-database`. `pheno-storage` and
   `pheno-events` follow once their FRs stabilize.
3. **ResilienceKit** (Python): publish `pheno-resilience`.
4. Audit remaining repos for publishable libraries; apply the same
   pattern. Do **not** publish apps masquerading as libraries.

## 8. Anti-patterns

- Storing `CARGO_REGISTRY_TOKEN`, `NPM_TOKEN`, `PYPI_API_TOKEN` in
  GitHub Secrets. If a legacy workflow has these, migrate and
  revoke — do not keep them "as a fallback" (global policy:
  "no silent fallbacks").
- Publishing from a developer laptop. All releases flow through the
  `release.yml` workflow so provenance is verifiable.
- Skipping the `environment: release` gate. The environment gate lets
  you attach required reviewers later without touching the workflow.
- Publishing app-shaped repos (heliosApp, AgilePlus, thegent, Tracera,
  heliosCLI, BytePort). These are products, not libraries.

## 9. Cross-references

- Global policy: `~/.claude/CLAUDE.md` → "Dependency & Technology
  Preferences", "Optionality and Failure Behavior".
- Phenotype Git & Delivery Workflow Protocol: `~/.claude/CLAUDE.md`.
- Release registry schema: `docs/governance/release_registry_schema.md`.
- Scripting policy: `docs/governance/scripting_policy.md`.
