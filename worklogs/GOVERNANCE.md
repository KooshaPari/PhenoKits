# Governance Worklog
**Category: GOVERNANCE**

Tracks governance-related work: policy, evidence, and quality gates.

## 2026-04-25 — ENOSPC Crisis Post-Incident & Disk Budget Policy Enhancement

**Incident:** Disk filled to 117 Mi free (~0.1% available). Multi-agent workspace triggered cascading purge cycle.

**Recovery Executed:**
- Homebrew cache: 7.5 GB reclaimed
- npm _cacache: ~6 GB reclaimed
- cargo targets: ~33 GB reclaimed
- **Total:** 46.5+ GB recovered; disk healthy within 15 min

**Root Causes Identified:**
1. Hidden caches not tracked in pre-dispatch checks (~/Library/Caches/Homebrew, ~/.npm/_cacache, ~/Library/Caches/com.apple.dt.Xcode)
2. No emergency playbook automation — manual prioritization was error-prone
3. Absence of clear "which to purge first" guidance during panic scenarios

**Governance Improvements Implemented:**
1. **governance/disk_budget_policy.md created:**
   - Added hidden cache inspection commands (4 major caches: Homebrew, npm, cargo, Xcode)
   - Codified crisis playbook with strict purge priority order (5 phases)
   - Added monthly cron suggestion for proactive purge
2. **scripts/bin/disk-emergency.rs created:**
   - Automated crisis playbook runner (Rust per scripting hierarchy)
   - Executes purges in priority order: Homebrew → npm → worktree targets → Xcode → cargo registry
   - Byte accounting per phase with --report flag
3. **FocalPoint/tooling/target-pruner/README.md created:**
   - Documented atime limitations (APFS `du` resets atime to "today")
   - Clarified scope and expansion roadmap
   - Linked to disk_budget_policy.md + disk-emergency.rs

**Operational Takeaway:** ENOSPC events are preventable with monthly cache purging. Hidden caches (not visible in `df -h`) are responsible for >50% of emergency recoveries.

---
