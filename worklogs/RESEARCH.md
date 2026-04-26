# RESEARCH

Entries tracking research summaries, audit findings, analysis, and investigation results.

**Last updated:** 2026-04-25  
**Entries:** 1

---

## 2026-04-25 Post-PR Cleanup Audit: Local Drift Survey & Branch Hygiene

**Summary:** Audit of 218 local repos found: 186 with state drift, 126 dirty working trees, 108 branches with ahead/behind divergence. GitHub PR count is zero (PR cleanup complete). Top-risk repos: agentapi-plusplus (4,681 changes), phenoSDK (2,200 changes), PhenoProc (gitlink-heavy, ahead 8/behind 12), shelf root (doc deletions + untracked spillover).

**Key Findings:**
- Stale no-PR branches: pr/211 (139d), pr/683 (61d) on cliproxyapi-plusplus; calver-migration (27d) on heliosApp
- High-risk dirty repos require split/discard decisions before sync
- Governance enforcement gap: local docs (CODEOWNERS, branch protection) not reflected on GitHub for 7+ repos
- Worklogs scattered across shelf root and per-repo dirs (consolidation pending)

**Deliverables:** 3 tables (local drift manifest, stale branches, governance gaps) + 7-task WBS (P0: preserve, P1: branch hygiene + enforcement, P2: docs)

**Source:** [PHENOTYPE_LOCAL_DRIFT_MANIFEST_2026_04_25.md](./PHENOTYPE_LOCAL_DRIFT_MANIFEST_2026_04_25.md) + [PHENOTYPE_POST_PR_CLEANUP_AUDIT_WBS_2026_04_25.md](./PHENOTYPE_POST_PR_CLEANUP_AUDIT_WBS_2026_04_25.md)
