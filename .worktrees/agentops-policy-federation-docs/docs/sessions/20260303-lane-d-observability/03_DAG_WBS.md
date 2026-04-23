# 03_DAG_WBS — Lane D (Observability/Incident Ops)

## D29–D32 (continuation from D25–D28)

| Task | Focus | Artifacts | Dependencies | Done Criteria |
|---|---|---|---|---|
| **D29 – Identify First: SLO canary and incident-ops baseline** | Identify-first pass over current SLO gates, alerts, synthetic checks, escalation drills, and recurrence signal sources so D30-D32 can be applied safely. | `artifacts/d29-d30-baseline-catalog.tsv`, `artifacts/d29-canary-incident-ops-sanity-notes.md` | D25–D28 | Baseline captures >=12 in-scope controls; each control has owner, source system, and last verification timestamp; no duplicated canary or drill pathways. |
| **D30 – SLO gate canary promotion policy** | Define policy for synthetic + production-canary promotion between severity classes, with explicit failure budgets, max concurrent canaries, and rollback thresholds. | `artifacts/d30-slo-gate-policy.yaml`, `artifacts/d30-canary-promotion-decision-tree.md`, `docs/sessions/20260303-lane-d-observability/00_SESSION_OVERVIEW.md` | D29 | Policy file with machine-readable thresholds is committed; runbook references policy sections for all rollout and rollback states; at least 3 testable scenarios documented (PASS/FAIL). |
| **D31 – Synthetic drift suppression governance** | Codify how to classify, suppress, and re-enable synthetic checks after repeated false positives, including ownership and suppression-duration rules. | `artifacts/d31-synthetic-drift-suppression-policy.md`, `artifacts/d31-suppression-exceptions.tsv`, `artifacts/d31-drill-drift-review-log.md` | D29, D30 | Every synthetic alert in scope maps to one governance status (`active`, `suppressed`, `parked`); suppression requests expire automatically (time-boxed) and require post-suppression evidence in review log. |
| **D32 – Escalation drill failure recovery loops and recurrence prevention efficacy** | Build recovery-loop coverage for failed escalation drills and measure whether recurrence is actually reduced over two consecutive drill cycles. | `artifacts/d32-escalation-recovery-loop.tmpl.md`, `artifacts/d32-drill-failure-runbook-checklist.md`, `artifacts/d32-recurrence-efficacy-dashboard.json`, `artifacts/d32-recurrence-metrics-targets.md` | D29, D30, D31 | Drill failure playbook executed end-to-end at least once; recovery loop includes automatic re-notify + owner rotation logic; recurrence metric shows negative trend (incident class recurrence -15% or better across two 30-day windows). |

## Execution order

1. D29
2. D30
3. D31
4. D32
