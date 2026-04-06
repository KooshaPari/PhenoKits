# Alert Inventory

## Purpose

This document defines a shared inventory structure for observability alerts, including severity classification and actionability rules. Use it as the canonical template for collecting and triaging alert definitions.

## Severity Map

| Severity | Incident Impact | Expected Response Window | Typical Trigger Profile | Escalation Guidance |
|---|---|---|---|---|
| `SEV1` | Critical customer-visible outage or hard data integrity/security risk | Immediate (page on-call now) | Service unavailable, severe latency saturation, failed critical control | Escalate to incident commander and platform leadership immediately |
| `SEV2` | Major degradation with material user impact, partial outage, or high error budget burn | 15 minutes | Elevated error rates, sustained dependency failures, regional impact | Escalate to owning team + secondary on-call; prepare incident channel |
| `SEV3` | Moderate degradation, operational risk trending toward customer impact | 1 business hour | Capacity pressure, intermittent failures, unusual retry amplification | Create/track remediation issue; escalate if duration/impact increases |
| `SEV4` | Low-impact anomaly, hygiene/compliance warning, informational risk | Next business day | Non-urgent drift, noisy/low-confidence signal, non-critical threshold breach | Route to backlog with owner and due date; monitor for recurrence |

## Actionability Taxonomy

| Category | Definition | Operator Action | Paging Eligibility | Example Signals |
|---|---|---|---|---|
| `PAGE_NOW` | Requires immediate human intervention to prevent/limit active impact | Acknowledge, triage, mitigate, communicate incident status | Yes | Hard downtime, rapid error-rate spikes, auth/system-wide failures |
| `URGENT_TICKET` | Action needed soon but does not require immediate page wake-up | Open high-priority ticket, assign owner, schedule near-term fix | Conditional (business-hours page optional) | Sustained degradation with workaround available |
| `SCHEDULED_REMEDIATION` | Valid signal with clear fix path; low immediate risk | Create scoped task with SLA and verification date | No | Capacity nearing limits, recurring transient dependency warnings |
| `INFO_ONLY` | Observability context with no direct intervention needed now | Record trend and review during ops hygiene cycle | No | Deployment notices, low-risk state transitions |
| `TUNE_OR_SUPPRESS` | Non-actionable or noisy signal needing threshold/query cleanup | Adjust alert logic, suppress duplicates, add dedupe/inhibition rules | No | Flapping alerts, false positives, duplicate symptom alerts |

## Alert Inventory Table Template

Use one row per alert definition.

| alert_name | owner_team | owner_primary | owner_secondary | source_system | metric_or_query | threshold_condition | evaluation_window | severity | actionability | paging_policy | runbook_url | dashboard_url | suppression_rules | escalation_path | notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `<alert identifier>` | `<team>` | `<person/rotation>` | `<backup>` | `<datadog/prometheus/cloudwatch/etc.>` | `<metric or query ref>` | `<e.g. error_rate > 5%>` | `<e.g. 5m>` | `<SEV1-4>` | `<PAGE_NOW/...>` | `<pager route>` | `<link>` | `<link>` | `<mute/inhibit logic>` | `<who/when to escalate>` | `<freeform context>` |

## Minimum Field Requirements

- `owner_team`
- `source_system`
- `threshold_condition`
- `severity`
- `actionability`
- `runbook_url`

## Usage Notes

- Prefer symptom alerts for paging and dependency alerts for diagnosis.
- Avoid duplicate pages for the same underlying failure mode; use inhibition where possible.
- Every page-eligible alert must have a tested runbook and explicit escalation path.
- Re-review thresholds after major architecture, traffic, or SLO changes.
