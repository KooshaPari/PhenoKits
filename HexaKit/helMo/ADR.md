# Architecture Decision Records -- helMo (agentkit)

---

## ADR-001: Hexagonal Architecture as the Structural Pattern

**Status:** Accepted
**Date:** 2026-03

### Context

CLI frameworks often conflate parsing, business logic, and I/O, making them hard to
test and extend. Phenotype tools need a pattern that isolates domain logic from adapters
so handlers are testable without spawning a process.

### Decision

`agentkit` adopts hexagonal (ports and adapters) architecture.

- The `domain` module is pure Rust with no I/O and no external crate dependencies beyond
  `thiserror` and `async-trait`.
- The `application` module orchestrates domain logic.
- The `adapters` module contains `primary` (inbound: CLI parsing) and `secondary` (outbound:
  config loading) adapters.
- The `infrastructure` module contains logging and tracing concrete implementations.

### Alternatives Considered

- Flat module structure (simpler but leads to tangled dependencies).
- Traits-only approach without layer enforcement (enforcement relies on convention, not structure).

### Consequences

- Domain tests do not require I/O setup.
- New adapters can be wired without changing domain code.
- Plugin implementations must satisfy `Plugin: Send + Sync`, enforced at compile time.

**Code Reference:** `agentkit/src/lib.rs` module tree; `agentkit/ADR/001-hexagonal-architecture.md`

---

## ADR-002: Dynamic Plugin Loading via libloading

**Status:** Accepted
**Date:** 2026-03

### Context

Phenotype tools need extensibility at runtime without requiring recompilation of the host binary.
Compile-time feature flags or monolithic command registration do not satisfy this requirement.

### Decision

Use `libloading` (version 0.8) to load native shared libraries (`.so`/`.dylib`/`.dll`) at runtime.
Each plugin exports a `create_plugin: fn() -> Box<dyn Plugin>` C-compatible symbol.
`PluginManager` owns the loaded `Library` handles for the process lifetime to prevent
symbol invalidation.

### Alternatives Considered

- Compile-time feature flags (no runtime extensibility).
- WASM/Extism plugins (cross-platform, safer, but higher complexity and latency at this stage).
- Scripted plugins via embedded interpreter (not idiomatic for Rust ecosystem).

### Consequences

- Plugin authors must target the same platform and Rust ABI as the host.
- `unsafe` is required for `Library::new` and `lib.get`; this is unavoidable for FFI.
- Future migration to Extism/WASM is documented as a follow-up path.

**Code Reference:** `agentkit/src/plugins/mod.rs`; `agentkit/ADR/002-plugin-system.md`

---

## ADR-003: clap 4 as the Primary CLI Adapter

**Status:** Accepted
**Date:** 2026-03

### Context

Raw argument parsing is error-prone and duplicated across Phenotype tools.
The inbound adapter must handle argument parsing without leaking parsing concerns
into domain entities.

### Decision

Use `clap` version 4 with the `derive` feature for the primary inbound adapter.
`clap` derives produce a `Cli` struct in `adapters/primary/cli.rs` that is converted
to the domain `Input` value object before dispatch to handlers.

### Alternatives Considered

- `pico-args` (minimal but no derive, no help generation).
- `argh` (derive-based but less ecosystem support).
- Custom parser (unnecessary duplication).

### Consequences

- `clap`'s auto-generated help and version flags are available for free.
- Consumers do not interact with `clap` types; they always receive `Input`.
- Help formatting is delegated to `CliParser::format_help` port for testability.

**Code Reference:** `agentkit/src/adapters/primary/cli.rs`; `agentkit/Cargo.toml`

---

## ADR-004: serde_json as the Universal Config Value Type

**Status:** Accepted
**Date:** 2026-03

### Context

`agentkit` supports multiple config formats (JSON, TOML). Callers should not need to
know which format was loaded; config values should be queryable uniformly.

### Decision

All `ConfigLoader` implementations normalize their format to `serde_json::Value` before
returning from `load()` and `get(key)`.
`TomlConfigLoader` internally parses to `toml::Value` then converts to `serde_json::Value`
via `serde_json::to_value`.

### Alternatives Considered

- Returning a format-specific enum type (forces callers to match on format).
- Using `toml::Value` as the universal type (does not generalize beyond TOML).
- A custom `ConfigValue` enum (unnecessary indirection).

### Consequences

- TOML types that have no JSON equivalent (e.g., datetime) are serialized as strings.
- Callers work with `serde_json::Value` navigation regardless of config format.

**Code Reference:** `agentkit/src/adapters/secondary/config.rs`

---

## ADR-005: tracing Crate for Structured Logging

**Status:** Accepted
**Date:** 2026-03

### Context

Phenotype tools share infrastructure. Logging must be structured, filterable, and
compatible with OpenTelemetry exporters.

### Decision

Use `tracing` + `tracing-subscriber` for the `TracingLogger` implementation.
The `Logger` outbound port abstracts the concrete tracing implementation so tests
can inject a `SimpleLogger` without a subscriber installed.
`tracing-appender` is available for file-based log rotation if needed.

### Alternatives Considered

- `log` + `env_logger` (not structured; poor async support).
- `slog` (structured but less ecosystem integration than `tracing`).

### Consequences

- `TracingLogger` requires a `tracing-subscriber` to be installed in `main`.
- `SimpleLogger` is provided as a zero-dependency alternative for tests and simple consumers.

**Code Reference:** `agentkit/src/infrastructure/logging.rs`; `agentkit/Cargo.toml`

---

## ADR-006: Cargo Workspace with Separate Library and Binary Crates

**Status:** Accepted
**Date:** 2026-03

### Context

`agentkit` is primarily a library consumed by other Phenotype tools.
A standalone binary is needed for demonstration and for running the CLI adapter directly.

### Decision

Structure as a Cargo workspace with a `[lib]` target at `src/lib.rs` and a `[[bin]]`
target at `src/bin/main.rs` within the same crate.
The `agentkit` library name matches the crate name, making `cargo add agentkit` the
consumer workflow.

### Alternatives Considered

- Separate crate for binary (unnecessary package split at this stage).
- No binary (makes demonstrations and integration testing harder).

### Consequences

- `cargo build` produces both a library and a binary.
- `cargo test --workspace` covers all library tests.
- Release profile (`opt-level=3`, `lto=true`, `codegen-units=1`) applies to the binary.

**Code Reference:** `agentkit/Cargo.toml`; `agentkit/src/lib.rs`; `agentkit/src/bin/main.rs`

---

## ADR-007: Cross-Platform Mobile Compilation Strategy

**Status:** Accepted
**Date:** 2026-04-04

### Context

helMo targets a diverse ecosystem including desktop (macOS, Linux, Windows), mobile (iOS, Android),
and emerging platforms (WASM, edge compute). Each platform has distinct compilation targets,
runtime constraints, and distribution requirements. A unified strategy is required to maintain
consistency while leveraging platform-specific optimizations.

### Decision

Establish a tiered compilation strategy based on Rust's target tiers:

**Tier 1 (Full CI/CD):**
- x86_64-unknown-linux-gnu
- x86_64-apple-darwin / aarch64-apple-darwin
- x86_64-pc-windows-msvc

**Tier 2 (Build Verification):**
- aarch64-unknown-linux-gnu
- aarch64-apple-ios (iOS device)
- aarch64-apple-ios-sim (iOS simulator)
- armv7-linux-androideabi / aarch64-linux-android

**Tier 3 (Community/Experimental):**
- wasm32-wasi (edge functions)
- wasm32-unknown-unknown (browser)

Cross-compilation uses `cargo-zigbuild` for most targets, with platform-specific toolchains
(ios-deploy, cargo-ndk) for mobile targets.

### Alternatives Considered

- **Docker-based cross compilation (cross-rs)**: Rejected due to slower build times and
  Docker dependency. Preferred zigbuild for lightweight cross-compilation.

- **Separate CI per platform**: Rejected due to maintenance overhead. GitHub Actions matrix
  builds cover all Tier 1 and Tier 2 targets in unified workflow.

- **LLVM-based universal binaries for Apple**: Considered but rejected for initial release.
  Fat binaries will be constructed in packaging stage, not build stage.

### Consequences

- All core functionality must compile without platform-specific code for Tier 1 targets.
- Mobile targets may require conditional compilation for platform APIs (behind cfg flags).
- Plugin ABI must match host compilation target; cross-architecture plugins are not supported.
- WASM targets require WASI Preview 2 for file system and networking capabilities.

**Code Reference:** `.github/workflows/ci.yml`; `Cargo.toml` target configuration

---

## ADR-008: WebAssembly Component Model for Future Plugin Architecture

**Status:** Proposed
**Date:** 2026-04-04

### Context

Current plugin system (ADR-002) uses native shared libraries via libloading. While this
provides maximum performance, it has limitations:
- Platform-specific binaries required for each target
- No sandboxing (plugins have full process access)
- Rust ABI instability concerns
- Distribution complexity (multiple artifacts per plugin)

WebAssembly Component Model (WIT) offers a path to portable, sandboxed, language-agnostic
plugins with defined interfaces.

### Decision

Design the plugin system for eventual WASM Component migration without breaking current
native plugin support.

**Phase 1 (Current):** Native shared library plugins via libloading.

**Phase 2 (Future):** WASM component plugins using wasmtime runtime.

The Plugin trait is designed to be implementable by both native and WASM hosts:

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> Cow<'_, str>;
    fn version(&self) -> Cow<'_, str>;
    fn commands(&self) -> Vec<Command>;
    fn execute(&self, command: &str, input: &Input) -> Result<Output>;
}
```

The `PluginManager` will gain a `PluginBackend` abstraction:
- `NativeBackend`: Current libloading implementation
- `WasmBackend`: wasmtime-based component execution

### Alternatives Considered

- **Immediate WASM-only**: Rejected due to wasmtime binary size (~3MB) and startup latency
  for simple CLI tools. Native plugins remain optimal for trusted, performance-critical extensions.

- **Extism**: A higher-level WASM plugin framework. Considered as alternative to direct
  wasmtime integration. Decision deferred to Phase 2 evaluation.

- **Scripting languages (Lua, JavaScript)**: Rejected as not idiomatic for Rust ecosystem
  and significant embedding complexity.

### Consequences

- Plugin trait design must accommodate WASM limitations (no references across host boundary,
  serialization for complex types).
- Host functions (logging, config access) must be explicitly defined for WASM guests.
- WASI Preview 2 required for file system and environment access in WASM plugins.
- Binary size tradeoff: WASM runtime adds ~2-5MB to host binary; optional feature flag.

**Code Reference:** `agentkit/src/domain/ports/mod.rs` (Plugin trait); `agentkit/ADR/008-wasm-components.md`

---

## ADR-009: Async Runtime Agnosticism via Generic Executor

**Status:** Accepted
**Date:** 2026-04-04

### Context

helMo's agentkit uses async/await for non-blocking I/O in command handlers. However,
different Phenotype tools may have different runtime requirements:
- CLI tools: tokio (mature, full-featured)
- Embedded/edge: smol or embassy (minimal footprint)
- Testing: futures::executor::block_on (no runtime dependency)

Hard-coding tokio would force this dependency on all consumers.

### Decision

Design the `CommandHandler` trait to be runtime-agnostic using the `async-trait` crate
or native async traits (Rust 1.75+). The trait definition:

```rust
#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle(&self, ctx: &Context) -> Result<Output>;
}
```

The binary crate (demonstration/main.rs) uses tokio by default, but library consumers
can use any executor:

```rust
// With tokio
#[tokio::main]
async fn main() { run_cli().await }

// With smol
fn main() { smol::block_on(run_cli()) }

// With async-std
#[async_std::main]
async fn main() { run_cli().await }
```

Runtime-specific adapters (timers, file I/O) are implemented behind the `Context` port,
allowing domain logic to remain executor-agnostic.

### Alternatives Considered

- **Standardize on tokio**: Rejected due to binary size concerns and ecosystem lock-in.
  Many embedded Rust projects avoid tokio's ~1MB overhead.

- **Poll-based futures**: Rejected as ergonomically unacceptable for 2026 Rust development.
  Async/await is the standard.

- **blocking I/O only**: Rejected as insufficient for network-dependent CLI tools
  (deploy, sync operations) where concurrent execution matters.

### Consequences

- Handler implementations must be `Send` (typically automatic for 'static data).
- Context provides async I/O via trait objects, introducing minimal dynamic dispatch overhead.
- Testing can use `futures::executor::block_on` without tokio dependency.
- Documentation must show examples for multiple runtimes.

**Code Reference:** `agentkit/src/domain/ports/mod.rs`; `agentkit/src/application/commands/mod.rs`

---

## ADR-010: Governance Validation as a Core Capability

**Status:** Accepted
**Date:** 2026-03-26

### Context

Multiple Phenotype repositories independently maintain bootstrap scripts, linter configurations,
and governance validation tooling. As the organization grows, these files diverge: different repos
pin different tool versions, enforce different rules, and have different levels of completeness.
Updating an org-wide convention requires touching dozens of repos individually.

### Decision

Treat helMo as a two-concern repository: (1) developer bootstrap tooling (scripts that set up
environments), and (2) governance validation tooling (scripts that verify other repos meet
org standards). Both are equally first-class.

`scripts/validate-governance.sh` is elevated to a P0 deliverable alongside `bootstrap-dev.sh`.
The validation script maintains a canonical list of required governance files that must be
kept in sync with Phenotype org governance updates.

### Alternatives Considered

- **Monorepo**: Merge all Phenotype repos into one. Rejected: too disruptive to existing
delivery workflows and CI pipelines; cross-language monorepo tooling overhead is significant.

- **Copy-paste convention**: Each repo copies files and updates them independently. Rejected:
drift is the root problem we are solving.

- **NPM/pip package**: Publish helMo as a versioned package. Considered for a future iteration;
rejected for the current phase because the tooling is shell scripts and config files that do
not benefit from a package runtime.

### Consequences

- helMo becomes the single source of truth for org-wide developer tooling
- Changes to helMo propagate to consuming repos on pull or copy-update
- helMo must be kept stable and backward-compatible; breaking changes require consumer migration
- Consumers can pin to a helMo git tag to avoid unintended updates
- CI in other Phenotype repos may invoke helMo's validation script as a check

**Code Reference:** `scripts/validate-governance.sh`; `scripts/bootstrap-dev.sh`

---

## Tenets

These tenets guide helMo development and supersede any individual decision:

1. **Testability First**: Domain logic must be testable without I/O, network, or filesystem setup.

2. **Explicit Over Implicit**: Dependencies, side effects, and configuration are always explicit,
   never hidden behind macros or global state.

3. **Minimal Dependencies**: Each dependency is justified; prefer std library when sufficient.

4. **Cross-Platform Native**: Compile to native binaries for all Tier 1 targets; no runtime required.

5. **Extensibility Without Recompilation**: Plugin architecture enables runtime extension while
   maintaining core stability.

6. **Ergonomic APIs**: Developer experience matters; balance explicitness with ergonomics.

---

*Document Version: 2.0*
*Last Updated: 2026-04-04*
*Maintainer: helMo Core Team*
