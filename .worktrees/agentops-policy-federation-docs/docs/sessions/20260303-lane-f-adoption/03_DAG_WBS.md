# 03_DAG_WBS — Lane F (Adoption/Rollout)

## F37–F40 (continuation from F33–F36)

| Task | Focus | Artifacts | Dependencies | Done Criteria |
|---|---|---|---|---|
| **F37 – Identify First: KPI governance anomaly response** | Identify-current-kpi anomalies across policy and operations telemetry, then define owned owners, severity, and response owners before remediation tasks begin. | `artifacts/f37-kpi-governance-control-baseline.tsv`, `artifacts/f37-kpi-anomaly-sop.md` | F33–F36 | Baseline includes all in-scope KPI families, includes alert owner + escalation path for every open anomaly class, and documents response SLAs for Sev-1/2/3. |
| **F38 – Identify First: Recertification debt burn-down plan** | Identify recertification debt by domain, age, and risk, then define a staged burn-down plan with strict carry-cost and recovery commitments. | `artifacts/f38-recert-debt-portfolio.tsv`, `artifacts/f38-recert-burndown-plan.md` | F33–F36, F37 | Every recertification debt item is classified by criticality and owner, with a 90-day reduction target and weekly execution cadence captured in a burn-down plan. |
| **F39 – Identify First: Ownership succession readiness scoring** | Identify key ownership chains and backup coverage, then score readiness for mission-critical roles and controls before any handoff or expansion actions. | `artifacts/f39-ownership-succession-scorecard.schema.json`, `artifacts/f39-ownership-readiness-snapshot.md` | F33–F36, F37, F38 | Scorecard covers all critical functions, flags any owner with single-point-of-failure risk, and records remediation owners with evidence of tested handover paths. |
| **F40 – Identify First: Annual operating model recalibration playbook** | Identify recurring annual recalibration triggers and controls, then publish a gated playbook that updates targets, controls, and owner handoff mechanics on a fixed schedule. | `artifacts/f40-annual-operating-model-playbook.md`, `artifacts/f40-recalibration-runbook-calendar.yml` | F33–F36, F37, F38, F39 | Playbook is approved, runbook calendar is published, and first annual recalibration drill passes with at least 1 evidence set per domain and zero unresolved critical risks. |

## F41–F48

| Task | Focus | Artifacts | Dependencies | Done Criteria |
|---|---|---|---|---|
| **F41 – Identify First: KPI anomaly automation hardening** | Identify weak links in KPI anomaly collection, triage, and closure automation, then harden detection-to-action controls before scaling policy rollout. | `artifacts/f41-kpi-anomaly-automation-baseline.tsv`, `artifacts/f41-kpi-anomaly-automation-rules.md` | F37–F40 | Baseline documents all high-priority KPI families, defines runbook steps for automatic triage/escalation, and requires zero untriaged Sev-1 anomalies for 30 days in the dry-run window. |
| **F42 – Identify First: Recert debt SLA enforcement** | Identify every open recertification debt item and map explicit SLA by criticality, then enforce deadline and reminder controls automatically. | `artifacts/f42-recert-debt-sla-policy.yml`, `artifacts/f42-recert-sla-enforcement-dashboard.md` | F38, F41 | 100% of tracked recert items have SLA tier + owner; SLA violations trigger automatic reminders/escalations and produce weekly exception reports with no missed critical deadlines. |
| **F43 – Identify First: Succession plan execution audits** | Identify planned succession workflows and execute sample audits of each transition path to confirm readiness evidence and update risk ratings. | `artifacts/f43-succession-audit-checklist.md`, `artifacts/f43-succession-audit-snapshots/` | F39, F42 | At least one audit run per critical role completed per quarter, all findings have remediation owners, and readiness scorecards refresh only after evidence closure. |
| **F44 – Identify First: Annual recalibration outcome tracking** | Identify outcomes from annual recalibration cycles and build repeatable telemetry, acceptance checks, and drift controls to prevent untracked policy drift. | `artifacts/f44-recalibration-outcome-metrics.tsv`, `artifacts/f44-recalibration-outcome-review-board.md` | F40, F43 | Recalibration outcomes are recorded per domain, each outcome has a closure owner, and quarterly trend reporting shows no stale action items past 10 business days. |
| **F45 – Identify First: KPI anomaly closure governance** | Identify weak KPI anomaly closure paths, define SLA-tiered closure ownership, and enforce mandatory evidence-based closure steps before closing loops. | `artifacts/f45-kpi-anomaly-closure-sop.md`, `artifacts/f45-kpi-anomaly-closure-rules.tsv` | F41, F44 | 100% of severe anomalies (P1/P2/P3) have closure playbooks, escalation paths, and closure evidence requirements; no Sev-1 anomaly remains open without a completion owner and closure timestamp. |
| **F46 – Identify First: Recert debt exception policy controls** | Identify all recertification exception cases and codify approval, renewal, and rollback controls to eliminate unmanaged policy exceptions. | `artifacts/f46-recert-exception-policy.yml`, `artifacts/f46-recert-exception-audit-log.csv` | F42 | Every recert exception has a policy record, approver, expiry window, and automatic hold/review action; no exception older than 60 days remains unreviewed. |
| **F47 – Identify First: Succession audit remediation tracking** | Identify recurring succession-audit findings and create a tracked remediation log with enforced owner refresh and re-audit checkpoints. | `artifacts/f47-succession-remediation-registry.md`, `artifacts/f47-succession-remediation-sla.yml` | F43, F46 | Each audit finding has a tagged owner, due date, and status; all "open" findings are reviewed biweekly, with 100% of critical risks assigned to live remediation plans. |
| **F48 – Identify First: Recalibration governance board handoff cadence** | Identify governance board handoff needs and publish fixed recurring cadence, quorum, and evidence packet requirements for recalibration decisions. | `artifacts/f48-recalibration-board-handoff-calendar.md`, `artifacts/f48-recalibration-board-brief-template.md` | F44, F47 | Quarterly board handoff calendar exists, includes quorum and evidence packet requirements, and no recalibration decision is accepted without refreshed KPI + succession readiness evidence. |

## Execution order

1. F37
2. F38
3. F39
4. F40
5. F41
6. F42
7. F43
8. F44
9. F45
10. F46
11. F47
12. F48
