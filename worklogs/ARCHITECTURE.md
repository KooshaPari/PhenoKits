# ARCHITECTURE

Entries tracking architecture decisions, decomposition patterns, shared crate extraction, and structural refactoring.

**Last updated:** 2026-04-25  
**Entries:** 1

---

## 2026-04-25 AgilePlus API Router Overlap (P0 Regression)

**Summary:** Duplicate route registration in agileplus-api causes Axum panic on router merge. Protected router (router.rs:92) and dashboard router (routes.rs:2228) both register `GET /api/v1/stream`; when merged, build panics immediately. Blocks all 14 api integration tests.

**Impact:** Classification (a) — real bug, not test issue. Regression indicates architecture boundary loss (unclear who owns `/api/v1/stream` surface).

**Fix:** Decide surface ownership (dashboard owns `/dashboard/stream` or api owns full `/api/v1/*`), update route prefix, regenerate OpenAPI fixtures. Est. 10–20 min.

**Decision Required:** Review route hierarchy and consolidate overlapping surfaces before next api crate push.

**Source:** [AGILEPLUS_TEST_FAILURE_TRIAGE_2026_04_25.md#agileplus-api](./AGILEPLUS_TEST_FAILURE_TRIAGE_2026_04_25.md) (lines 37–74)
