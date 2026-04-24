# ADR — phenotype-cli-extensions

## ADR-001: Dynamic Module Loading Strategy
**Status:** Accepted
**Context:** Extensions must be loaded at runtime without requiring recompilation of the host CLI.
**Decision:** Use Node.js `require`/ESM dynamic import with a sandboxed module registry. Extensions are CommonJS or ESM packages resolved from the extension path.
**Rationale:** Native module system provides isolation, tree-shaking, and version pinning without custom VM overhead.
**Alternatives considered:** WASM plugins (too much overhead for CLI use), subprocess IPC (too slow for command dispatch).

## ADR-002: Extension Manifest Schema
**Status:** Accepted
**Context:** A stable contract is needed between the host CLI and extensions.
**Decision:** JSON manifest (`phenotype-ext.json`) with fields: `name`, `version`, `apiVersion`, `entry`, `capabilities[]`, `permissions[]`.
**Rationale:** JSON Schema allows validation tooling; `apiVersion` enables forward-compatibility gating.

## ADR-003: Registry Transport
**Status:** Accepted
**Context:** Extensions must be distributable without a central server dependency.
**Decision:** Primary distribution via npm / GitHub Packages. Local path installs (`file:./`) also supported.
**Rationale:** Reuses existing package infrastructure; no new registry service needed.

## ADR-004: Error Policy
**Status:** Accepted
**Context:** A broken extension must not silently degrade the CLI.
**Decision:** Any extension load failure throws immediately and surfaces the full error to the user. The CLI exits non-zero when a required extension fails to load.
**Rationale:** Aligns with Phenotype fail-clearly mandate.

## ADR-005: Capability Discovery and Capability Declaration
**Status:** Accepted
**Context:** Extensions must advertise their capabilities for the CLI to route commands and middleware appropriately.
**Decision:** Extensions declare capabilities in `phenotype-ext.json` under `capabilities[]` using a `type:action` format (e.g., `command:deploy`, `middleware:auth`, `provider:cloud-storage`). The CLI maintains a capability registry at startup.
**Rationale:** Declarative capability registration enables efficient command routing, middleware pipeline composition, and provider discovery without requiring extension instantiation. The `type:action` format provides namespace separation and clear intent.

## ADR-006: Platform-Specific Binary Resolution
**Status:** Accepted
**Context:** Extensions may bundle platform-specific binaries (e.g., shell executables) that must be resolved based on the host platform and architecture.
**Decision:** Use a target triple system (e.g., `x86_64-unknown-linux-musl`) with a vendor directory structure. Resolution happens at extension load time by matching `process.platform` and `process.arch` to known triples. Linux additionally uses OS release detection (`/etc/os-release`) to select distribution-specific variants (debian, alpine, default).
**Rationale:** Target triples provide a deterministic, portable naming scheme for platform-specific assets. OS release detection handles Linux's fragmentation (Debian vs Alpine vs Arch) without requiring separate packages per distribution.
