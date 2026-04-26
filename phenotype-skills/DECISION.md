# Decision: phenotype-skills Repository Status

**Date:** 2026-04-25  
**Decision Owner:** Forge  
**Reference:** [Wave 68 Sidekick Candidates Audit](../docs/org-audit-2026-04/wave68_sidekick_candidates_audit.md)

---

## Current State

- **LOC:** 0 (empty stub, no source code)
- **Tests:** 0
- **CI:** None
- **Consumers:** 0 across Phenotype org
- **Purpose:** Placeholder for skills registry (unfunded, undocumented)

---

## Two Options

### Option 1: ARCHIVE (Recommended)

**Rationale:**
- Repository is a dead stub with no implementation path.
- Skills system was speculative; agentkit now provides the framework foundation.
- Revive as dedicated project **only if**:
  - Skills become a priority initiative (spec + champion assigned)
  - agentkit stabilizes (60%+ coverage, 2+ adopters)
  - Explicit skill registry design is approved

**Action:** Mark repo as archived. If skills are needed later, rebuild from agentkit base with clear requirements.

### Option 2: ABSORB

**Rationale:**
- agentkit already has a skill system (src/domain/skills/); phenotype-skills could extend it.
- Consolidates agent-related code under one framework.

**Trade-off:** Requires agentkit maturation first; phenotype-skills would not exist as separate project.

---

## Recommendation: **OPTION 1 — ARCHIVE**

**Reasoning:**
- Zero evidence of active use or integration need
- agentkit skill module (src/domain/skills/) is the canonical location
- Reduces organizational dead weight
- Preserves option to rebuild if strategic need emerges

**Next Steps:**
- Archive repo (mark as read-only, add archive notice)
- Update phenotype-org portfolio to remove dead link
- Link agentkit skill system as the canonical path for skill development

---

## Related Decisions

- **agentkit Maturation Path:** See [docs/PATH_TO_SIDEKICK.md](../agentkit/docs/PATH_TO_SIDEKICK.md) for roadmap to production readiness.
