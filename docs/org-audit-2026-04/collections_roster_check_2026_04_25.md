# Collections Roster Verification — 2026-04-25

## Summary

Eidolon and Paginary collections verified against directory structure and governance files. One critical drift found: **KVirtualStage does not exist** — memory references it as an Eidolon member, but directory is missing.

---

## Eidolon Collection

**Memory Roster:** KDesktopVirt, KVirtualStage, kmobile, PlayCua, bare-cua

| Member | Status | Last Commit | CLAUDE.md | AGENTS.md | Issue |
|--------|--------|-------------|-----------|-----------|-------|
| KDesktopVirt | ✓ ACTIVE | 2026-04-25 18:07 fix(security) | ✓ | ✓ | — |
| KVirtualStage | ✗ MISSING | — | — | — | Directory does not exist in `/repos/` |
| kmobile | ✓ ACTIVE | 2026-04-24 17:15 docs(agents) | ✓ | ✓ | — |
| PlayCua | ✓ ACTIVE | 2026-04-24 17:15 docs(agents) | ✓ | ✓ | — |
| bare-cua | ✓ ACTIVE | 2026-04-25 00:13 docs(readme) | ✓ | ✓ | — |

---

## Paginary Collection

**Memory Roster:** phenodocs, PhenoHandbook, PhenoSpecs, phenoXdd (consolidated docs collection)

| Member | Status | Last Commit | CLAUDE.md | AGENTS.md | Issue |
|--------|--------|-------------|-----------|-----------|-------|
| phenodocs | ✓ ACTIVE | 2026-04-25 07:23 docs(worklog) | ✓ | ✓ | — |
| PhenoHandbook | ✓ ACTIVE | 2026-04-25 08:34 docs(readme) | ✓ | ✓ | — |
| PhenoSpecs | ✓ ACTIVE | 2026-04-24 17:15 docs(agents) | ✓ | ✓ | — |
| phenoXdd | ✓ ACTIVE | 2026-04-25 07:23 docs(worklog) | ✓ | ✗ | Missing AGENTS.md; has CLAUDE.md |

---

## Roster Accuracy

### Eidolon

**README.md roster** (lines 132–138):
- Lists: KDesktopVirt, kmobile, PlayCua, bare-cua, KVirtualStage
- **DRIFT**: KVirtualStage referenced but directory missing

**Impact**: Extraction plan references non-existent sibling; integration matrix blocked.

### Paginary

**README.md roster** (lines 106–110):
- Lists: PhenoHandbook, PhenoSpecs, phenoXdd, phenotype-journeys (implicit)
- **STATUS**: All four exist and are ACTIVE

---

## Governance Gaps

1. **phenoXdd**: Missing AGENTS.md (has CLAUDE.md only)
   - Mitigation: Run `agileplus specify --title "phenoXdd AGENTS.md" --description "Add thin-pointer agent instructions"`

2. **KVirtualStage**: Missing entirely
   - Action: Either locate archived worktree or update Eidolon README.md to remove reference
   - Check: `git log --all --grep=KVirtualStage` or `find .worktrees -name "*Virtual*" -type d`

---

## Recommendations

1. Locate KVirtualStage (search worktrees, archived branches)
2. If unrecoverable: Update Eidolon/README.md (line 50) and extraction plan
3. Add phenoXdd/AGENTS.md (thin pointer to CLAUDE.md)
4. Verify all collection members have both CLAUDE.md and AGENTS.md going forward

---

**Audit Date**: 2026-04-25  
**Scope**: Eidolon (5 members), Paginary (4 members)  
**Status**: 8/9 directories present; 8/9 have full governance (phenoXdd missing AGENTS.md)
