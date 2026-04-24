# Resolution Algorithm

1. Validate input scopes.
2. Resolve each policy path in deterministic order:
   - `system/base`
   - `user/org-default`
   - `harness/{name}`
   - `repo/{name}`
   - `task_domain/{name}`
   - optional `task_instance/{name}`
   - optional `task_overlay/{name}`
3. For each existing file:
   - validate against `schemas/policy-contract.schema.json`
   - merge `policy` payload with the configured strategy (`replace`, `append`, `append_unique`, `merge_map`)
4. Return immutable artifact with:
   - `policy_hash` of the resolved policy
   - full `scope_chain`
   - `source_files` (including missing optional files for auditability)
   - `conflicts` when merge types can’t be reconciled
5. Resolve extension candidates from `extensions/registry.yaml` and include them in `extensions` output.
6. Persist `policy_hash`, `scope_chain`, and selected extension state into runtime metadata.
