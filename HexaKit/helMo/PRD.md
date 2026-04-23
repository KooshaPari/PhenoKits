# Product Requirements Document -- helMo (agentkit)

## Product Vision

helMo provides `agentkit`, a Rust library for building robust, extensible command-line
applications using hexagonal (ports and adapters) architecture. It supplies a declarative
command DSL, async handler dispatch, dynamic plugin loading, multi-format config loading,
structured logging, and OpenTelemetry-compatible tracing so any Phenotype tool can be
assembled without duplicating infrastructure code.

---

## E1: Command Definition and Parsing

### E1.1: Declarative Command DSL

As a Phenotype tool author, I can define commands, subcommands, positional arguments,
named options, and boolean flags using a fluent builder API without writing boilerplate
argument parsing.

**Acceptance Criteria:**
- `Command::new(name).description(text).version(v)` constructs a root command.
- `.subcommand(cmd)`, `.argument(arg)`, `.option(opt)`, `.flag(flag)` compose the tree.
- `Argument::new(name).required()`, `.default_value(v)`, `.multiple()` cover all
  positional argument shapes.
- `CliOption` and `Flag` support both short (`-v`) and long (`--verbose`) forms.
- `test_command_builder` in `agentkit/src/domain/entities/mod.rs` is green.

### E1.2: Typed Input Value Object

As a command author, parsed CLI arguments are delivered as a typed `Input` value object
so handler code never touches raw argument strings.

**Acceptance Criteria:**
- `Input` carries `command` (String), optional `subcommand`, `args: HashMap<String, ArgValue>`,
  `opts: HashMap<String, Option<String>>`, and `flags: HashMap<String, bool>`.
- `ArgValue` discriminates `String`, `Integer`, `Boolean`, and `Multiple(Vec<String>)`.
- `Input::get_str(name)`, `get_flag(name)`, `get_opt(name)`, `get_arg(name)` return
  typed `Option<_>` accessors.
- `ParsedInput` (adapter-layer) provides the equivalent surface for clap-derived parsing.
- `test_parsed_input` in `domain/entities/mod.rs` is green.

### E1.3: Execution Context

As a handler, I receive a `Context` that merges parsed `Input`, current working directory,
process environment, and loaded config values so handlers are pure functions over context
with no global state access.

**Acceptance Criteria:**
- `Context::new(input)` populates `cwd` from `std::env::current_dir()` and `env` from
  `std::env::vars()`.
- `config: HashMap<String, serde_json::Value>` is populated by secondary adapters before
  dispatch.
- `get_flag`, `get_opt`, `get_str`, `get_arg` delegate transparently to the inner `Input`.

---

## E2: Async Handler Dispatch

### E2.1: CommandHandler Inbound Port

As a command author, I implement the `CommandHandler` trait to write async business
logic that returns typed `Output` without coupling to CLI parsing.

**Acceptance Criteria:**
- `CommandHandler::handle(&Context) -> Result<Output>` is the sole inbound port for
  command execution.
- `validate(&Input) -> Result<()>` runs before `handle` and may reject early.
- `DefaultHandler<F, Fut>` wraps any `async Fn(&Context) -> Result<Output>` closure
  and satisfies `Send + Sync + 'static`.

### E2.2: Output Value Object

As a consumer, command output carries content and exit metadata so callers render or
forward results deterministically.

**Acceptance Criteria:**
- `Output::text(s)`, `Output::json(&T)`, `Output::error(s)` are the three constructors.
- `Output::json` calls `serde_json::to_string_pretty` and propagates errors loudly.
- `exit_code` defaults to 0 for success, 1 for error.
- `should_exit` controls process termination after dispatch; `Output::no_exit()` suppresses it.
- `OutputContent` discriminates `Text`, `Json`, `Yaml`, `Error`, and `None`.

### E2.3: Command Registry and Executor

As a framework consumer, I register commands by name, look them up by exact or pattern
match, and dispatch them through `CommandExecutor`.

**Acceptance Criteria:**
- `CommandRegistry::register` rejects duplicate names with `DomainError::InvalidCommand`.
- `CommandRegistry::get(name)` returns `Option<&Command>`.
- `CommandRegistry::find_matching(pattern)` returns all commands whose name or
  description contains pattern.
- `CommandExecutor::register(command)` chains commands; `list_commands()` returns all.

---

## E3: Dynamic Plugin System

### E3.1: Runtime Plugin Loading

As an operator, I load agentkit plugins at runtime from native shared libraries
(`.so`/`.dylib`/`.dll`) so the CLI is extended without recompilation.

**Acceptance Criteria:**
- `PluginManager::load_from_dir(path)` scans for shared library files by platform
  extension and loads each.
- Each library exports `create_plugin: fn() -> Box<dyn Plugin>` as the entry symbol.
- `Plugin::init()` is called immediately after load; errors abort that plugin load.
- Loaded plugins and their library handles are retained for the process lifetime.
- Non-existent plugin directory returns `DomainError::PluginError` with the path.

### E3.2: Plugin Command Contribution

As a plugin author, I implement `Plugin::commands()` to contribute additional commands
that appear in the host application's command tree at runtime.

**Acceptance Criteria:**
- `PluginManager::get_commands()` aggregates commands from all loaded plugins in load order.
- `PluginManager::list_plugins()` returns plugin names in load order.
- `Plugin::cleanup()` is called on plugin unload.
- `test_plugin_trait` in `domain/ports/mod.rs` verifies the `Plugin` trait surface.

---

## E4: Configuration Loading

### E4.1: JSON Configuration Adapter

As a developer, I load application configuration from a JSON file using the `ConfigLoader`
outbound port with dot-notation key traversal.

**Acceptance Criteria:**
- `JsonConfigLoader::from_file(path)` reads and deserializes on construction; fails
  loudly on missing file or invalid JSON via `DomainError::ConfigError`.
- `load()` returns the full `serde_json::Value`.
- `get("a.b.c")` traverses nested objects, returning `None` if any segment is absent.

### E4.2: TOML Configuration Adapter

As a developer, I load configuration from a TOML file through the same `ConfigLoader`
port so config format is swappable without changing handler code.

**Acceptance Criteria:**
- `TomlConfigLoader::from_file(path)` fails loudly on missing file or invalid TOML.
- Both `load()` and `get(key)` produce `serde_json::Value` for uniform downstream use.
- `TomlConfigLoader` implements the identical `ConfigLoader` trait as `JsonConfigLoader`.

---

## E5: Observability Infrastructure

### E5.1: Structured Logging

As an operator, log messages are emitted at `info`, `debug`, `warn`, and `error` levels
through a swappable `Logger` port so observability is not hardcoded.

**Acceptance Criteria:**
- `TracingLogger` routes to `tracing::{info!, debug!, warn!, error!}` with target
  `"agentkit"` enabling per-crate log filtering.
- `SimpleLogger(prefix)` prints `[PREFIX LEVEL] message` to stdout/stderr for
  environments without a tracing subscriber.
- Both satisfy the `Logger` outbound port and are `Send + Sync`.

### E5.2: Telemetry Port

As an integrator, I implement the `Telemetry` outbound port to forward execution
metrics to any observability backend without modifying handler code.

**Acceptance Criteria:**
- `record_execution(command, duration_ms, success)` is the primary execution hook.
- `record_error(command, error)` is invoked on handler failure.
- `record_metric(name, value)` supports arbitrary business metrics.

### E5.3: Input Validation Service

As a framework consumer, invalid inputs are rejected before handler dispatch so handlers
never receive structurally incomplete data.

**Acceptance Criteria:**
- `InputValidator::validate(&Input)` checks all required arguments against the
  registered command schema.
- Missing required argument returns `DomainError::MissingArgument(name)`.
- Unknown command returns `DomainError::CommandNotFound(name)`.

---

## E6: Phenotype Ecosystem Integration

### E6.1: Clap CLI Adapter

As a developer, the primary inbound adapter wraps `clap` derive-macro parsing and
routes parsed args into the domain `Input` type via `run_cli()`.

**Acceptance Criteria:**
- `Cli` struct uses `#[derive(Parser)]` from clap 4.
- `--config <path>` option routes to secondary config adapters at startup.
- `run_cli()` is the public entry point called from `src/bin/main.rs`.

### E6.2: Prelude and Crate API

As a Phenotype tool maintainer, adding `agentkit` as a Cargo dependency gives access
to the full command framework via a single `use agentkit::prelude::*` import.

**Acceptance Criteria:**
- `prelude` re-exports: `Command`, `Argument`, `CliOption`, `Flag`, `Input`, `Output`,
  `Context`, `CommandHandler`, `CommandExecutor`, `DefaultHandler`, `ConfigLoader`,
  `Logger`, `Plugin`, `PluginLoader`, `Telemetry`, `CliParser`, `Persistence`.
- Binary crate at `src/bin/main.rs` demonstrates minimal wiring without panics.
- `cargo test --workspace` passes with zero failures.
