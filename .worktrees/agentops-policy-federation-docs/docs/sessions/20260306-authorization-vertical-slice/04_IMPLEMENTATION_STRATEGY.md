# Implementation Strategy

## Chosen approach

- Keep the existing resolver as the source of merged policy truth.
- Add a typed authorization layer on top of the merged policy.
- Compile only what target runtimes can faithfully enforce natively.
- Surface all lossy cases as shim requirements.
- Add one real harness shim now, rather than waiting for all harnesses to converge.

## Rejected approach

- Encoding complex path-aware rules directly into harness-native allowlists and denylists.

Reason:

- That would create false confidence and silently weaken enforcement.

## Runtime delivery in this slice

- `policyctl intercept` provides deterministic policy enforcement with stable exit codes.
- `policyctl exec` wraps subprocess execution behind the same decision path.
- `scripts/runtime/codex_exec_guard.sh` is the first runnable Codex-oriented shell wrapper using that path.
