# 02_SPECIFICATIONS

## Scope and Precedence

Federated policy resolution follows strict precedence:
1. `global` (organization-wide baseline; mandatory defaults)
2. `harness` (runtime or environment-level overlays)
3. `task-domain` (task-specific policy refinements)

Conflict rule: higher-precedence scope overrides lower-precedence scope on the same key.

Merge rule:
- Scalar values: replace by precedence.
- Object values: deep-merge with higher-precedence overwrite.
- Lists: explicit strategy required (`replace` or `append_unique`), default is `replace`.

Non-overridable controls:
- Any control marked `enforced=true` at `global` cannot be weakened by lower scopes.

## Trust Boundaries

Boundary 1: Policy Authoring
- Trusted: authenticated policy maintainers with scoped write permissions.
- Untrusted: all anonymous or unsigned policy changes.

Boundary 2: Policy Distribution
- Trusted: signed policy artifacts and verified transport channels.
- Untrusted: in-transit payloads without signature/expiry validation.

Boundary 3: Policy Evaluation Runtime
- Trusted: evaluator service and immutable, versioned policy snapshot.
- Untrusted: task input payloads and user-supplied metadata.

Boundary 4: Execution Agents
- Trusted: agents only after identity attestation and policy fetch authz.
- Untrusted: any agent without valid tenant binding and token claims.

Security invariants:
- Fail closed on missing/invalid policy.
- Enforce signature, version, and freshness checks before evaluation.
- Log every policy decision with scope provenance.

## Tenancy Model

Model: strict multi-tenant isolation with optional shared global baseline.

Tenant hierarchy:
- `org_id` (hard isolation boundary)
- `harness_id` (sub-boundary within org)
- `task_domain` (domain-level policy namespace)

Isolation requirements:
- No cross-tenant policy reads/writes.
- Tenant ID required in all policy API operations.
- Cache keys must include full tenant context (`org_id:harness_id:task_domain`).

Delegation model:
- Global policies may be centrally managed and inherited.
- Tenant-local overrides allowed only within explicit allowlist.
- Deny escalation: tenant overrides cannot broaden privileges beyond global enforced controls.

## ARUs (Assumptions, Risks, Uncertainties)

Assumptions:
- Policy artifacts are versioned and cryptographically signed.
- Identity tokens include stable claims for org/harness/task-domain scoping.
- Consumers can tolerate fail-closed behavior during transient policy fetch failures.

Risks:
- Misconfigured precedence could unintentionally weaken intended controls.
- Stale caches may apply outdated policies.
- Incomplete provenance logging may reduce incident diagnosability.

Uncertainties:
- Final source of truth for signature key rotation cadence.
- Required latency SLO for policy evaluation at peak load.
- Final list of non-overridable global controls.

Mitigations:
- Add precedence conformance tests for canonical conflict cases.
- Enforce TTL + version pinning + cache invalidation hooks.
- Require decision logs to include policy version, scope chain, and override map.

## Acceptance Criteria

1. Precedence is deterministic and documented as `global > harness > task-domain`.
2. `enforced=true` global controls cannot be weakened by lower scopes.
3. Policy evaluation fails closed on invalid signature, expired artifact, or missing required scope.
4. Tenant isolation is enforced for storage, retrieval, and cache lookups.
5. Every decision record includes tenant context, evaluated scope chain, final resolved policy hash, and timestamp.
6. Merge behavior for scalars/objects/lists is implemented and consistent with this spec.
7. Policy API rejects requests missing required tenant identifiers.
8. Operational runbooks include key rotation, cache invalidation, and incident triage for policy mismatch.
