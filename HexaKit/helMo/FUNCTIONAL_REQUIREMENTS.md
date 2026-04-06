# Functional Requirements -- helMo (agentkit)

---

## FR-CMD: Command Definition

### FR-CMD-001: Command Builder Pattern
The `Command` type SHALL support a fluent builder pattern with `.description()`, `.version()`,
`.subcommand()`, `.argument()`, `.option()`, and `.flag()` methods, each returning `Self`.
**Traces to:** E1.1
**Code:** `agentkit/src/domain/entities/mod.rs` -- `Command` struct

### FR-CMD-002: Argument Shapes
`Argument` SHALL support `required()`, `default_value(v)`, and `multiple()` modifiers.
**Traces to:** E1.1
**Code:** `agentkit/src/domain/entities/mod.rs` -- `Argument` struct

### FR-CMD-003: Option Short and Long Forms
`CliOption` and `Flag` SHALL each accept an optional short single-char form and a mandatory
long string form. At least one form MUST be specified when constructing the option.
**Traces to:** E1.1
**Code:** `agentkit/src/domain/entities/mod.rs` -- `CliOption`, `Flag` structs

### FR-CMD-004: Input ArgValue Discrimination
`ArgValue` SHALL discriminate `String(String)`, `Integer(i64)`, `Boolean(bool)`,
and `Multiple(Vec<String>)`. Conversions from `String`-like types SHALL be provided via `From`.
**Traces to:** E1.2
**Code:** `agentkit/src/domain/value_objects/mod.rs` -- `ArgValue` enum

### FR-CMD-005: Input Accessor Methods
`Input` SHALL expose `get_str(name)`, `get_flag(name)`, `get_opt(name)`, and `get_arg(name)`
returning typed `Option` values without panicking on missing keys.
**Traces to:** E1.2
**Code:** `agentkit/src/domain/value_objects/mod.rs` -- `Input` impl

### FR-CMD-006: Context Construction
`Context::new(input)` SHALL populate `cwd` from `std::env::current_dir()` and `env` from
`std::env::vars()` at construction time. Failure to read cwd SHALL fall back to `PathBuf::default()`.
**Traces to:** E1.3
**Code:** `agentkit/src/domain/value_objects/mod.rs` -- `Context` struct

---

## FR-HDL: Handler Dispatch

### FR-HDL-001: CommandHandler Trait
The `CommandHandler` trait SHALL declare `async fn handle(&self, ctx: &Context) -> Result<Output>`
and `fn validate(&self, input: &Input) -> Result<()>` with a default no-op `validate` impl.
**Traces to:** E2.1
**Code:** `agentkit/src/domain/ports/mod.rs` -- `CommandHandler`

### FR-HDL-002: DefaultHandler Closure Wrapper
`DefaultHandler<F, Fut>` SHALL wrap any `Fn(&Context) -> Fut` where `Fut: Future<Output=Result<Output>>`
and SHALL satisfy `Send + Sync + 'static` for dispatch to async runtimes.
**Traces to:** E2.1
**Code:** `agentkit/src/application/commands/mod.rs` -- `DefaultHandler`

### FR-HDL-003: Output Constructors
`Output::text(s)` SHALL produce `OutputContent::Text` with `exit_code=0`.
`Output::json(&T)` SHALL serialize with `serde_json::to_string_pretty` and propagate errors.
`Output::error(s)` SHALL produce `OutputContent::Error` with `exit_code=1`.
**Traces to:** E2.2
**Code:** `agentkit/src/domain/value_objects/mod.rs` -- `Output` impl

### FR-HDL-004: Output Exit Control
`Output::exit_code(code)` SHALL override the default exit code.
`Output::no_exit()` SHALL set `should_exit = false` to support streaming or multi-step commands.
**Traces to:** E2.2
**Code:** `agentkit/src/domain/value_objects/mod.rs` -- `Output` impl

### FR-HDL-005: CommandRegistry Uniqueness
`CommandRegistry::register(command)` SHALL reject duplicate command names by returning
`DomainError::InvalidCommand` rather than silently overwriting.
**Traces to:** E2.3
**Code:** `agentkit/src/domain/services/mod.rs` -- `CommandRegistry::register`

### FR-HDL-006: CommandRegistry Search
`CommandRegistry::find_matching(pattern)` SHALL return all commands whose `name` or
`description` contains the pattern substring (case-sensitive).
**Traces to:** E2.3
**Code:** `agentkit/src/domain/services/mod.rs` -- `CommandRegistry::find_matching`

### FR-HDL-007: InputValidator Required Argument Check
`InputValidator::validate(&Input)` SHALL iterate `command.arguments` and return
`DomainError::MissingArgument(name)` for any required argument absent from `input.args`.
**Traces to:** E5.3
**Code:** `agentkit/src/domain/services/mod.rs` -- `InputValidator::validate`

---

## FR-PLG: Plugin System

### FR-PLG-001: Plugin Directory Scan
`PluginManager::load_from_dir(path)` SHALL scan `path` for files with extensions
`so`, `dll`, or `dylib` and attempt to load each as a plugin.
**Traces to:** E3.1
**Code:** `agentkit/src/plugins/mod.rs` -- `PluginManager::load_from_dir`

### FR-PLG-002: Plugin Entry Symbol
Each plugin shared library SHALL export a `create_plugin: fn() -> Box<dyn Plugin>` symbol.
Loading SHALL fail with `DomainError::PluginError` if the symbol is absent.
**Traces to:** E3.1
**Code:** `agentkit/src/plugins/mod.rs` -- `PluginManager::load_plugin`

### FR-PLG-003: Plugin Lifecycle
`Plugin::init()` SHALL be called immediately after load; `Plugin::cleanup()` SHALL be called
before the plugin handle is dropped. Both SHALL propagate errors as `DomainError::PluginError`.
**Traces to:** E3.1, E3.2
**Code:** `agentkit/src/domain/ports/mod.rs` -- `Plugin` trait

### FR-PLG-004: Plugin Command Aggregation
`PluginManager::get_commands()` SHALL return a flat `Vec<Command>` collecting commands
from all loaded plugins in load order with no deduplication.
**Traces to:** E3.2
**Code:** `agentkit/src/plugins/mod.rs` -- `PluginManager::get_commands`

### FR-PLG-005: Non-Existent Plugin Directory Error
`PluginManager::load_from_dir(path)` SHALL return `DomainError::PluginError` when
`path` does not exist, including the path in the error message.
**Traces to:** E3.1
**Code:** `agentkit/src/plugins/mod.rs` -- `PluginManager::load_from_dir`

---

## FR-CFG: Configuration Loading

### FR-CFG-001: JSON File Loading
`JsonConfigLoader::from_file(path)` SHALL read and parse the file as `serde_json::Value`.
Missing file or parse failure SHALL produce `DomainError::ConfigError` (not panic).
**Traces to:** E4.1
**Code:** `agentkit/src/adapters/secondary/config.rs` -- `JsonConfigLoader::from_file`

### FR-CFG-002: JSON Key Traversal
`JsonConfigLoader::get("a.b.c")` SHALL traverse nested objects by splitting on `.` and
return `None` if any segment is absent, `Some(Value)` if the full path resolves.
**Traces to:** E4.1
**Code:** `agentkit/src/adapters/secondary/config.rs` -- `JsonConfigLoader::get`

### FR-CFG-003: TOML File Loading
`TomlConfigLoader::from_file(path)` SHALL read and parse TOML, with failure producing
`DomainError::ConfigError`.
**Traces to:** E4.2
**Code:** `agentkit/src/adapters/secondary/config.rs` -- `TomlConfigLoader::from_file`

### FR-CFG-004: TOML to JSON Normalization
`TomlConfigLoader::load()` and `get(key)` SHALL convert internal `toml::Value` to
`serde_json::Value`, making both loaders interchangeable from callers' perspective.
**Traces to:** E4.2
**Code:** `agentkit/src/adapters/secondary/config.rs` -- `TomlConfigLoader` impl

---

## FR-OBS: Observability

### FR-OBS-001: TracingLogger Routing
`TracingLogger` SHALL route all four log levels through the `tracing` crate with
target `"agentkit"`, enabling per-crate filtering via `RUST_LOG`.
**Traces to:** E5.1
**Code:** `agentkit/src/infrastructure/logging.rs` -- `TracingLogger`

### FR-OBS-002: SimpleLogger Prefix Formatting
`SimpleLogger::new(prefix)` SHALL format messages as `[PREFIX LEVEL] message`
printed to stdout (info/debug) or stderr (warn/error).
**Traces to:** E5.1
**Code:** `agentkit/src/infrastructure/logging.rs` -- `SimpleLogger`

### FR-OBS-003: Telemetry Port Surface
The `Telemetry` outbound port SHALL declare `record_execution(command, duration_ms, success)`,
`record_error(command, error)`, and `record_metric(name, value)` as the three required methods.
**Traces to:** E5.2
**Code:** `agentkit/src/domain/ports/mod.rs` -- `Telemetry` trait

### FR-OBS-004: Persistence Port Surface
The `Persistence` outbound port SHALL declare `save(key, &[u8])`, `load(key)`, and
`delete(key)` covering all lifecycle operations for persistent state.
**Traces to:** E6.1
**Code:** `agentkit/src/domain/ports/mod.rs` -- `Persistence` trait

---

## FR-INT: Integration and Build

### FR-INT-001: Prelude Re-Exports
The `agentkit::prelude` module SHALL re-export all commonly-used entities, ports, and
service types so consumers can write `use agentkit::prelude::*` for one-import access.
**Traces to:** E6.2
**Code:** `agentkit/src/lib.rs` -- `pub mod prelude`

### FR-INT-002: Workspace Test Suite
`cargo test --workspace` SHALL complete with zero test failures.
Unit tests SHALL cover `Command` builder, `Input` builder, `Output` builder,
`test_plugin_trait`, and `DefaultHandler` creation.
**Traces to:** E6.2
**Code:** Tests in `domain/entities/mod.rs`, `domain/ports/mod.rs`, `domain/value_objects/mod.rs`,
`application/commands/mod.rs`

### FR-INT-003: Clap Adapter Entry Point
`run_cli()` in `agentkit/src/adapters/primary/cli.rs` SHALL parse args via
`clap::Parser::parse()` and produce output without panicking.
**Traces to:** E6.1
**Code:** `agentkit/src/adapters/primary/cli.rs`

### FR-INT-004: Release Build Profile
The `Cargo.toml` release profile SHALL set `opt-level=3`, `lto=true`, and
`codegen-units=1` to produce an optimized binary suitable for production use.
**Traces to:** E6.2
**Code:** `agentkit/Cargo.toml` -- `[profile.release]`
