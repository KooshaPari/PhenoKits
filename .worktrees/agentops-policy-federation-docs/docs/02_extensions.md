# Extension System

Extensions are optional packs registered in `extensions/registry.yaml`.
Each extension manifest defines:
- kind
- scope selector
- required fields
- entrypoint

Disabled extensions must never affect default resolution.
