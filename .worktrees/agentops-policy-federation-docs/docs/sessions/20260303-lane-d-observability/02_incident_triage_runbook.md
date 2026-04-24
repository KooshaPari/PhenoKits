# Incident Triage Runbook (Lane D Observability)

## Purpose
Provide a repeatable top-5 incident playbook template for rapid response in the first 10 minutes, including mitigation, escalation, and communications.

## Scope
Use for production incidents affecting availability, latency, data integrity, auth/access, or critical integrations.

## Roles (assign immediately)
- `Incident Commander (IC)`: owns decisions and timeline.
- `Technical Lead (TL)`: drives diagnosis/mitigation.
- `Comms Lead (CL)`: handles internal/external updates.
- `Scribe`: keeps incident log, timestamps, actions.

## Severity Guide
- `SEV1`: Major outage / critical customer impact.
- `SEV2`: Partial degradation / high-impact feature failure.
- `SEV3`: Limited impact / workaround exists.

---

## 10-Minute Triage Checklist

### 0-2 minutes: Declare and contain
- Declare incident in on-call channel and page IC/TL/CL.
- Open incident bridge and assign roles.
- Capture first signal: alert name, time detected, affected service(s).
- Freeze risky deploys/releases for affected scope.

### 2-5 minutes: Bound blast radius
- Confirm customer impact: who/what/where/how many.
- Identify symptom class: outage, latency, errors, data, auth, dependency.
- Check recent changes (deploys, config, infra, feature flags).
- Classify tentative severity (`SEV1/SEV2/SEV3`).

### 5-10 minutes: First mitigation decision
- Choose fastest safe mitigation path (rollback, failover, flag off, rate-limit).
- Execute one primary mitigation owner/action.
- Record expected effect and verification signal.
- Post first status update with next ETA.

---

## Top-5 Incident Playbook Template

## 1) Service Outage / Hard Down
### Triggers
- Global 5xx spike, health checks failing, endpoint unreachable.
### First actions
- Verify scope (single region/service vs global).
- Roll back latest deployment or switch to last-known-good.
- Route traffic to healthy region/cluster if available.
### Mitigation targets
- Restore baseline availability ASAP, then investigate root cause.
### Escalation
- Escalate to platform/SRE lead if no recovery signal in 10 minutes.
### Comms
- Internal update every 10 minutes; external status page for SEV1.

## 2) Latency/Timeout Degradation
### Triggers
- P95/P99 latency regression, timeout rate increasing.
### First actions
- Identify bottleneck layer (app, DB, cache, network, dependency).
- Enable protective controls: rate limiting, queue backpressure, load shedding.
- Disable expensive non-critical paths via flag/config.
### Mitigation targets
- Bring latency/error budget back within SLO guardrails.
### Escalation
- Escalate to owning service team + infra team when sustained >15 minutes.
### Comms
- Share impacted endpoints, current latency/error trend, mitigation ETA.

## 3) Elevated Errors from Dependency Failure
### Triggers
- Sudden upstream 4xx/5xx, auth/provider outages, webhook failures.
### First actions
- Confirm external dependency status and error signatures.
- Activate fallback path (cached responses, degraded mode, retries with jitter).
- Reduce retry storms and protect shared resources.
### Mitigation targets
- Stabilize core flows while isolating dependency blast radius.
### Escalation
- Escalate vendor/on-call contact and internal integration owner.
### Comms
- Call out dependency impact explicitly and customer-facing limitations.

## 4) Data Integrity / Corruption Risk
### Triggers
- Incorrect writes, duplicate/missing records, schema mismatch side effects.
### First actions
- Stop further harmful writes (feature flag, write gate, job pause).
- Preserve forensic evidence (logs, request IDs, job IDs, query snapshots).
- Determine safe read mode and affected time window.
### Mitigation targets
- Prevent additional corruption before any repair.
### Escalation
- Immediate escalation to data owner + security/compliance as needed.
### Comms
- Tight, factual updates; avoid speculative impact claims.

## 5) Security/Auth Incident (Access, Token, Permission)
### Triggers
- Unauthorized access patterns, auth outage, token validation failures.
### First actions
- Contain: revoke keys/tokens, disable affected auth path, tighten allowlists.
- Confirm if incident is availability-only vs potential compromise.
- Preserve evidence and maintain chain-of-custody notes.
### Mitigation targets
- Block abuse path and restore secure access.
### Escalation
- Immediate escalation to security lead; involve legal/compliance if required.
### Comms
- Security-approved comms only; separate internal technical vs external statements.

---

## Escalation Matrix
- `T+10 min`: If no improving signal, escalate to next-tier owner.
- `T+20 min`: If SEV1 persists, engage incident exec sponsor.
- `Any time`: Escalate immediately for suspected data breach/compliance exposure.

## Communications Cadence
- Internal channel updates: every 10 minutes minimum until stable.
- Status page/customer updates: every 30 minutes for SEV1/SEV2, or on material change.
- Each update includes:
  - current impact
  - actions in progress
  - next update time
  - explicit asks/blockers

## Mitigation Patterns (fast-safe defaults)
- Rollback recent change.
- Disable suspect feature flag.
- Fail over traffic/region.
- Rate-limit or shed non-critical load.
- Pause async consumers/jobs causing amplification.

## Incident Log Template
- `Time (UTC)`:
- `Signal/Observation`:
- `Decision`:
- `Action Owner`:
- `Action Taken`:
- `Result/Metric Change`:
- `Next Step`:

## Exit Criteria (leave active incident mode when all true)
- Service health stable across key metrics for agreed window.
- Customer impact materially resolved or contained.
- No uncontrolled error growth or data risk.
- Hand-off created for root cause analysis and follow-up actions.
