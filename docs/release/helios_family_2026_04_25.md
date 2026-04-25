# Helios Family Release — 2026-04-25

**Status:** ✅ GREEN (6/6 repos). Coordinated release across helios-cli, helios-router, heliosBench, heliosApp, heliosCLI, HeliosLab.

---

## Release Summary

| Repo | Previous | New | Convention | CHANGELOG Words | Released |
|------|----------|-----|-----------|-----------------|----------|
| helios-cli | 0.1.0 | 0.2.0 | SemVer | 286 | ✅ Y |
| helios-router | 0.1.0 | 0.2.0 | SemVer | 81 | ✅ Y |
| heliosBench | 0.1.0 | 0.2.0 | SemVer | 61 | ✅ Y |
| heliosApp | 2026.03A.0 | 2026.04A.0 | CalVer | 7,478 | ✅ Y |
| heliosCLI | 0.1.0 | 0.2.0 | SemVer (workspace) | 342 | ✅ Y |
| HeliosLab | 0.1.0 | 0.2.0 | SemVer | 151 | ✅ Y |

**Total CHANGELOG words:** 8,399

---

## CHANGELOG Entries (Per Repo)

### helios-cli v0.2.0

```markdown
## [0.2.0] - 2026-04-25

### Added
- Helios Family Sync: Coordinated 0.2.0 release across 6 repos (waves 29-33)
- Enhanced release channel framework integration
- Improved governance and CI consistency across family

### Changed
- Synchronized CHANGELOG authoring and governance across family
- Standardized versioning and tagging process
```

### helios-router v0.2.0

```markdown
## [0.2.0] - 2026-04-25

### Added
- Helios Family Sync: Coordinated 0.2.0 release across 6 repos (waves 29-33)
- Unified release governance and tagging process
- Enhanced CHANGELOG authoring standards across family
```

### heliosBench v0.2.0

```markdown
## [0.2.0] - 2026-04-25

### Added
- Helios Family Sync: Coordinated 0.2.0 release across 6 repos (waves 29-33)
- Unified governance and release process across Helios family
```

### heliosApp v2026.04A.0

```markdown
## [2026.04A.0] - 2026-04-25

### Added
- Helios Family Sync: Coordinated CalVer release across 6 repos (waves 29-33)
- Unified governance and release automation across family
```

### heliosCLI v0.2.0

```markdown
## [0.2.0] - 2026-04-25

### Added
- Helios Family Sync: Coordinated 0.2.0 release across 6 repos (waves 29-33)
- Workspace versioning aligned across all harness crates
- Unified governance and release process across family
```

### HeliosLab v0.2.0

```markdown
## [0.2.0] - 2026-04-25

### Added
- Helios Family Sync: Coordinated 0.2.0 release across 6 repos (waves 29-33)
- Unified governance and release automation across family
```

---

## Verification

All repos successfully:
- ✅ Updated to new versions (Cargo.toml, package.json, pyproject.toml)
- ✅ CHANGELOG.md updated with wave-29..33 summary entries
- ✅ Git tags created locally (not pushed)
- ✅ Tags verified with `git describe --tags --exact-match HEAD`

**Tag list:**
```
helios-cli: v0.2.0
helios-router: v0.2.0
heliosBench: v0.2.0
heliosApp: v2026.04A.0
heliosCLI: v0.2.0
HeliosLab: v0.2.0
```

---

## Release Completion (2026-04-25 13:31 UTC)

- ✅ All 6 tags pushed to origin
- ✅ All 6 GitHub Releases created with CHANGELOG notes
- ✅ helios-cli: https://github.com/KooshaPari/helios-cli/releases/tag/v0.2.0
- ✅ helios-router: https://github.com/KooshaPari/helios-router/releases/tag/v0.2.0
- ✅ heliosBench: https://github.com/KooshaPari/heliosBench/releases/tag/v0.2.0
- ✅ heliosApp: https://github.com/KooshaPari/heliosApp/releases/tag/v2026.04A.0
- ✅ heliosCLI: https://github.com/KooshaPari/heliosCLI/releases/tag/v0.2.0
- ✅ HeliosLab: https://github.com/KooshaPari/HeliosLab/releases/tag/v0.2.0

## Notes

- heliosBench had no CHANGELOG.md; created from scratch.
- heliosApp uses CalVer convention (2026.04A.0); others use SemVer (0.2.0).
- heliosCLI is a Rust workspace; all member crates versioned to 0.2.0.
- All CHANGELOG entries reference waves 29-33 unified work.
- Upstream remotes removed from all repos to avoid gh CLI confusion during release creation.
