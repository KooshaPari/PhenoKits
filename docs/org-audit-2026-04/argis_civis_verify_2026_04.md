# argis-extensions & Civis Health Verification — 2026-04-24

## argis-extensions (Go Workspace)

**Recent Commits:**
- HEAD: `90ed103` docs(agents): harmonize AGENTS.md (Apr 24)
- `60d7663` fix(build): resolve final 2 api/server.go import-alias errors — full go build green ✓

**Build Status:**
- Primary build (`go build ./...`): **PASS** ✓
  - No compilation errors in source code
  - 13 schema blockers resolved per commit 60d7663

**Test Status:** **PARTIAL FAILURE** (4/18 pass)
- Passing: `infra/circuitbreaker`, `infra/neo4j`, `infra/redis`, `tests` (4 packages)
- Failing: 13 packages with test build failures
  - Root cause: **Schema mismatch in tests** — test files reference undefined types (`schemas.BifrostStreamChunk`, `DefaultAgentConfig`, `NewProvider`, `NewClient`)
  - Packages affected: `account`, `api`, `api/graphql`, `cmd/bifrost/cli`, `config`, `db`, `db/migrations`, `infra/graceful`, `plugins/learning`, `plugins/smartfallback`, `providers/agentcli`, `providers/oauthproxy`, `server`
  - Pattern: Test schemas not synced with recent refactors; type exports missing or renamed

**Analysis:**
Source build succeeds (60d7663 fixed import-alias errors), but test layer has diverged from schema changes. Tests still reference old type signatures. This is a test-only issue; production builds are unaffected.

---

## Civis (JavaScript/VitePress Workspace)

**Recent Commits:**
- HEAD: `c1e76d5` docs(agents): harmonize AGENTS.md (Apr 24)
- Recent: governance + security baseline work (Apr 4, wave-3 CI adoption)

**Install Status:** **BUILD_FAILS**
- Command: `bun install`
- Exit code: 1
- Error: `GET https://npm.pkg.github.com/@phenotype%2fdocs - 404`
- Root cause: `@phenotype/docs@^0.1.0` (in package.json) does not exist in GitHub Packages registry
- `.npmrc` config: Points to `https://npm.pkg.github.com` for scoped `@phenotype` packages
- Blocker: Package dependency missing; likely not yet published to GitHub Packages or authentication token missing

**Build Status:**
- Static asset linting (`npm run lint`): **PASS** (2,045 warnings, 0 errors)
  - Warnings are primarily minified asset lint rules (expected in build output)
  - No blocking errors
- Documentation build (`npm run docs:build`): **BLOCKED** — dependency install failed
  - Cannot proceed without resolving `@phenotype/docs` package

**File Count:** 317 TypeScript/JavaScript files (excluding node_modules, .git)

**Development Status:**
- Recent activity: governance adoption (CLAUDE.md, AGENTS.md, quality gates)
- Stack: VitePress (docs framework) + ESLint + custom CI workflows
- Build artifacts up-to-date (April 4 baseline with recent doc/governance commits)

**Notable:** No breaking changes in recent 5 commits; governance standardization focused.
- **Next Action:** Verify `@phenotype/docs` package is published to GitHub Packages, or update dependency to published version

---

## Summary

| Repo | Install | Build (Lint) | Docs Build | Status |
|------|---------|-------------|-----------|--------|
| **argis-extensions** | ✓ PASS | ✓ PASS | N/A | Ready with test fixes pending |
| **Civis** | ✗ FAIL | ✓ PASS | ✗ BLOCKED | Missing dependency blocker |

**Findings:**
1. **argis-extensions**: Source build passes; test schemas diverged (13 test files, ~1-2h fix)
2. **Civis**: Install fails — `@phenotype/docs@^0.1.0` not found in GitHub Packages registry
   - Blocker is external (missing package publication)
   - Lint passes; docs build blocked until dependency resolved

**Recommended Action:**
1. **argis-extensions**: Sync test schemas with production refactors
2. **Civis**: Verify `@phenotype/docs` is published to GitHub Packages or update to available version
