# helMo Specification

## Overview

helMo is the Phenotype CLI framework, providing `agentkit`, a Rust library for building robust, extensible command-line applications using hexagonal (ports and adapters) architecture. It supplies a declarative command DSL, async handler dispatch, dynamic plugin loading, multi-format config loading, structured logging, and OpenTelemetry-compatible tracing so any Phenotype tool can be assembled without duplicating infrastructure code.

This document provides the comprehensive technical specification for helMo's architecture, components, interfaces, and behavior.

**Version:** 2.0.0  
**Status:** Active Development  
**Last Updated:** 2026-04-04  
**Maintainer:** Phenotype Core Team

---

## Table of Contents

1. [Mission and Tenets](#mission-and-tenets)
2. [Architecture Overview](#architecture-overview)
3. [Project Layout](#project-layout)
4. [Domain Layer Specification](#domain-layer-specification)
5. [Application Layer Specification](#application-layer-specification)
6. [Adapter Layer Specification](#adapter-layer-specification)
7. [Infrastructure Layer Specification](#infrastructure-layer-specification)
8. [Plugin System Specification](#plugin-system-specification)
9. [Configuration System](#configuration-system)
10. [Observability](#observability)
11. [CLI Interface](#cli-interface)
12. [Error Handling](#error-handling)
13. [Cross-Platform Support](#cross-platform-support)
14. [Testing Strategy](#testing-strategy)
15. [Performance Characteristics](#performance-characteristics)
16. [Security Considerations](#security-considerations)
17. [API Reference](#api-reference)
18. [Migration Guide](#migration-guide)
19. [Appendices](#appendices)

---

## Mission and Tenets

### Mission Statement

helMo provides the foundation for all Phenotype CLI tools, enabling teams to build portable, testable, and extensible command-line applications without duplicating common infrastructure concerns.

### Development Tenets

1. **Testability First**
   - Domain logic must be testable without I/O, network, or filesystem setup
   - Unit tests should execute in milliseconds without external dependencies
   - Adapters are swappable for testing purposes

2. **Explicit Over Implicit**
   - Dependencies, side effects, and configuration are always explicit
   - No hidden state or global singletons
   - Clear data flow through the application

3. **Minimal Dependencies**
   - Each dependency is justified with documented rationale
   - Prefer std library when sufficient
   - Audit dependencies for security and maintenance status

4. **Cross-Platform Native**
   - Compile to native binaries for all Tier 1 targets
   - No runtime, VM, or interpreter required
   - Consistent behavior across platforms

5. **Extensibility Without Recompilation**
   - Plugin architecture enables runtime extension
   - Core framework remains stable
   - Plugins are first-class citizens

6. **Ergonomic APIs**
   - Developer experience matters
   - Balance explicitness with ergonomics
   - Comprehensive documentation and examples

---

## Architecture Overview

### Hexagonal Architecture Pattern

helMo adopts hexagonal architecture (ports and adapters) to isolate domain logic from external concerns. This pattern provides:

- **Domain-Centric Design:** Business logic at the center
- **Port Abstraction:** Domain defines interfaces for external capabilities
- **Adapter Implementation:** External systems implement adapters
- **Dependency Inversion:** Domain depends on abstractions
- **Testability:** Domain logic testable without external systems

### Architecture Diagram

```
                        +-------------------------+
                        |   External Systems      |
                        | - Console               |
                        | - Config Files          |
                        | - Plugins               |
                        | - Logging Framework     |
                        +------------+------------+
                                     |
                        +------------v------------+
                        |      Adapters           |
                        |  +------------------+   |
                        |  | Primary (Inbound)|   |
                        |  | - CLI Parser     |   |
                        |  +------------------+   |
                        |  +------------------+   |
                        |  | Secondary (Out)  |   |
                        |  | - Config Loaders |   |
                        |  | - Loggers        |   |
                        |  +------------------+   |
                        +------------+------------+
                                     |
                        +------------v------------+
                        |     Application         |
                        |   (Orchestration)       |
                        | - CommandExecutor       |
                        | - Validation Pipeline   |
                        +------------+------------+
                                     |
                        +------------v------------+
                        |       Domain            |
                        |   (Business Logic)      |
                        |  +------------------+   |
                        |  |   Entities       |   |
                        |  | - Command        |   |
                        |  | - Argument       |   |
                        |  | - Input/Output   |   |
                        |  +------------------+   |
                        |  +------------------+   |
                        |  |   Ports          |   |
                        |  | - CommandHandler |   |
                        |  | - ConfigLoader   |   |
                        |  | - Plugin         |   |
                        |  +------------------+   |
                        |  +------------------+   |
                        |  |   Services       |   |
                        |  | - Registry       |   |
                        |  | - Validator      |   |
                        |  +------------------+   |
                        +-------------------------+
```

### Layer Responsibilities

#### Domain Layer
- Pure Rust with no I/O
- Business entities and value objects
- Port definitions (traits)
- Domain services
- No external crate dependencies beyond `thiserror` and `async-trait`

#### Application Layer
- Orchestrates domain logic
- Command execution flow
- Validation pipeline
- Transaction boundaries
- Coordinates multiple domain operations

#### Adapter Layer
- Primary (Inbound): CLI parsing, HTTP handlers
- Secondary (Outbound): Config loading, logging, persistence
- Translates external formats to domain types
- Implements port interfaces

#### Infrastructure Layer
- Concrete implementations of secondary adapters
- Tracing subscriber setup
- Plugin loading machinery
- External service clients

---

## Project Layout

### Directory Structure

```
helMo/
├── Cargo.toml              # Workspace manifest
├── Taskfile.yml            # Task runner configuration
├── SPEC.md                 # This document
├── ADR.md                  # Architecture Decision Records
├── RESEARCH.md             # SOTA research documentation
├── src/
│   ├── lib.rs              # Library exports
│   ├── prelude.rs          # Convenience re-exports
│   ├── bin/
│   │   └── main.rs         # Binary entry point
│   ├── domain/
│   │   ├── mod.rs          # Domain module exports
│   │   ├── entities/
│   │   │   ├── mod.rs      # Command, Argument, Option, Flag
│   │   │   └── types.rs    # Supporting types
│   │   ├── value_objects/
│   │   │   ├── mod.rs      # Input, Output, Context
│   │   │   └── builders.rs # Builder patterns
│   │   ├── ports/
│   │   │   ├── mod.rs      # Port trait definitions
│   │   │   ├── inbound.rs   # CommandHandler, Plugin
│   │   │   └── outbound.rs  # ConfigLoader, Logger, Telemetry
│   │   └── services/
│   │       ├── mod.rs      # CommandRegistry, InputValidator
│   │       └── registry.rs # Registry implementation
│   ├── application/
│   │   ├── mod.rs          # Application layer exports
│   │   ├── commands/
│   │   │   ├── mod.rs      # Command execution
│   │   │   ├── executor.rs # CommandExecutor
│   │   │   └── handlers.rs # DefaultHandler
│   │   └── validation/
│   │       ├── mod.rs      # Validation exports
│   │       └── pipeline.rs # ValidationPipeline
│   ├── adapters/
│   │   ├── mod.rs          # Adapter exports
│   │   ├── primary/
│   │   │   ├── mod.rs      # Primary adapters
│   │   │   └── cli.rs      # Clap-based CLI adapter
│   │   └── secondary/
│   │       ├── mod.rs      # Secondary adapters
│   │       ├── config.rs   # JSON/TOML config loaders
│   │       └── logging.rs  # Log adapters
│   ├── infrastructure/
│   │   ├── mod.rs          # Infrastructure exports
│   │   ├── tracing_setup.rs # Tracing initialization
│   │   └── plugin_loader.rs  # Dynamic plugin loading
│   └── plugins/
│       ├── mod.rs          # Plugin system exports
│       └── manager.rs      # PluginManager
├── tests/
│   ├── unit/               # Unit tests
│   ├── integration/          # Integration tests
│   └── fixtures/             # Test fixtures
├── examples/               # Example CLI applications
├── scripts/
│   ├── bootstrap-dev.sh    # Development setup
│   └── validate-governance.sh # Governance validation
└── docs/                   # Documentation site
```

### Crate Organization

**Library Target (`src/lib.rs`):**
```rust
pub mod domain;
pub mod application;
pub mod adapters;
pub mod infrastructure;
pub mod plugins;
pub mod prelude;
```

**Binary Target (`src/bin/main.rs`):**
```rust
use agentkit::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    run_cli().await
}
```

---

## Domain Layer Specification

### Entities

#### Command

A `Command` represents a CLI command with its metadata, arguments, options, and subcommands.

**Struct Definition:**
```rust
pub struct Command {
    pub name: String,
    pub description: String,
    pub version: String,
    pub arguments: Vec<Argument>,
    pub options: Vec<CliOption>,
    pub flags: Vec<Flag>,
    pub subcommands: Vec<Command>,
    pub handler: Option<Box<dyn CommandHandler>>,
}
```

**Builder Pattern:**
```rust
impl Command {
    pub fn new(name: impl Into<String>) -> Self;
    pub fn description(mut self, desc: impl Into<String>) -> Self;
    pub fn version(mut self, version: impl Into<String>) -> Self;
    pub fn argument(mut self, arg: Argument) -> Self;
    pub fn option(mut self, opt: CliOption) -> Self;
    pub fn flag(mut self, flag: Flag) -> Self;
    pub fn subcommand(mut self, cmd: Command) -> Self;
    pub fn handler(mut self, handler: impl CommandHandler) -> Self;
}
```

**Usage Example:**
```rust
let deploy_cmd = Command::new("deploy")
    .description("Deploy application to cloud")
    .version("1.0.0")
    .argument(Argument::new("environment").required())
    .option(CliOption::new("region")
        .short('r')
        .long("region")
        .default_value("us-east-1"))
    .flag(Flag::new("dry-run")
        .short('d')
        .long("dry-run"))
    .subcommand(
        Command::new("status")
            .description("Check deployment status")
    );
```

#### Argument

Represents a positional argument to a command.

**Struct Definition:**
```rust
pub struct Argument {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub multiple: bool,
    pub value_type: ArgType,
}
```

**Builder Methods:**
```rust
impl Argument {
    pub fn new(name: impl Into<String>) -> Self;
    pub fn description(mut self, desc: impl Into<String>) -> Self;
    pub fn required(mut self) -> Self;
    pub fn default_value(mut self, value: impl Into<String>) -> Self;
    pub fn multiple(mut self) -> Self;
    pub fn value_type(mut self, ty: ArgType) -> Self;
}
```

#### CliOption

Represents a named option (--key value or -k value).

**Struct Definition:**
```rust
pub struct CliOption {
    pub name: String,
    pub description: String,
    pub short: Option<char>,
    pub long: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub value_type: ArgType,
}
```

**Builder Methods:**
```rust
impl CliOption {
    pub fn new(name: impl Into<String>) -> Self;
    pub fn short(mut self, ch: char) -> Self;
    pub fn long(mut self, long: impl Into<String>) -> Self;
    pub fn required(mut self) -> Self;
    pub fn default_value(mut self, value: impl Into<String>) -> Self;
}
```

#### Flag

Represents a boolean flag (--flag or -f).

**Struct Definition:**
```rust
pub struct Flag {
    pub name: String,
    pub description: String,
    pub short: Option<char>,
    pub long: String,
    pub default: bool,
}
```

### Value Objects

#### Input

`Input` carries parsed CLI arguments as a typed value object.

**Struct Definition:**
```rust
pub struct Input {
    pub command: String,
    pub subcommand: Option<String>,
    pub args: HashMap<String, ArgValue>,
    pub opts: HashMap<String, Option<String>>,
    pub flags: HashMap<String, bool>,
    pub raw: Vec<String>,
}
```

**Accessor Methods:**
```rust
impl Input {
    pub fn get_str(&self, name: &str) -> Option<&str>;
    pub fn get_int(&self, name: &str) -> Option<i64>;
    pub fn get_bool(&self, name: &str) -> Option<bool>;
    pub fn get_flag(&self, name: &str) -> bool;
    pub fn get_opt(&self, name: &str) -> Option<&str>;
    pub fn get_arg(&self, name: &str) -> Option<&ArgValue>;
    pub fn get_multiple(&self, name: &str) -> Option<&[String]>;
}
```

#### ArgValue

Discriminated value type for arguments.

**Enum Definition:**
```rust
pub enum ArgValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Multiple(Vec<String>),
}
```

**Conversions:**
```rust
impl From<String> for ArgValue { ... }
impl From<&str> for ArgValue { ... }
impl From<i64> for ArgValue { ... }
impl From<bool> for ArgValue { ... }
impl From<Vec<String>> for ArgValue { ... }
```

#### Output

`Output` represents command results with content and exit metadata.

**Struct Definition:**
```rust
pub struct Output {
    pub content: OutputContent,
    pub exit_code: i32,
    pub should_exit: bool,
}
```

**Constructors:**
```rust
impl Output {
    pub fn text(content: impl Into<String>) -> Self;
    pub fn json<T: Serialize>(value: &T) -> Result<Self, serde_json::Error>;
    pub fn yaml<T: Serialize>(value: &T) -> Result<Self, serde_yaml::Error>;
    pub fn error(message: impl Into<String>) -> Self;
    pub fn none() -> Self;
}
```

**Modifiers:**
```rust
impl Output {
    pub fn exit_code(mut self, code: i32) -> Self;
    pub fn no_exit(mut self) -> Self;
}
```

#### OutputContent

Discriminated content type.

**Enum Definition:**
```rust
pub enum OutputContent {
    Text(String),
    Json(String),
    Yaml(String),
    Error(String),
    None,
}
```

#### Context

`Context` merges parsed `Input`, current working directory, process environment, and loaded config values.

**Struct Definition:**
```rust
pub struct Context {
    pub input: Input,
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
    pub config: HashMap<String, serde_json::Value>,
}
```

**Construction:**
```rust
impl Context {
    pub fn new(input: Input) -> Self {
        Self {
            input,
            cwd: std::env::current_dir().unwrap_or_default(),
            env: std::env::vars().collect(),
            config: HashMap::new(),
        }
    }
    
    pub fn with_config(mut self, config: HashMap<String, serde_json::Value>) -> Self;
}
```

**Delegation Methods:**
```rust
impl Context {
    pub fn get_flag(&self, name: &str) -> bool {
        self.input.get_flag(name)
    }
    
    pub fn get_opt(&self, name: &str) -> Option<&str> {
        self.input.get_opt(name)
    }
    
    pub fn get_str(&self, name: &str) -> Option<&str> {
        self.input.get_str(name)
    }
    
    pub fn get_arg(&self, name: &str) -> Option<&ArgValue> {
        self.input.get_arg(name)
    }
    
    pub fn config_get(&self, key: &str) -> Option<&serde_json::Value> {
        // Dot-notation traversal
    }
}
```

### Ports

#### CommandHandler (Inbound)

The primary inbound port for command execution.

**Trait Definition:**
```rust
#[async_trait]
pub trait CommandHandler: Send + Sync {
    /// Execute the command with given context
    async fn handle(&self, ctx: &Context) -> Result<Output>;
    
    /// Validate input before execution; default no-op
    fn validate(&self, input: &Input) -> Result<()> {
        Ok(())
    }
}
```

**Implementation Example:**
```rust
pub struct DeployHandler;

#[async_trait]
impl CommandHandler for DeployHandler {
    fn validate(&self, input: &Input) -> Result<()> {
        if input.get_str("environment").is_none() {
            return Err(DomainError::MissingArgument("environment"));
        }
        Ok(())
    }
    
    async fn handle(&self, ctx: &Context) -> Result<Output> {
        let env = ctx.get_str("environment").unwrap();
        let region = ctx.get_opt("region").unwrap_or("us-east-1");
        
        // Deployment logic...
        
        Ok(Output::text(format!("Deployed to {} in {}", env, region)))
    }
}
```

#### Plugin (Inbound)

Runtime extension mechanism.

**Trait Definition:**
```rust
pub trait Plugin: Send + Sync {
    /// Plugin identifier
    fn name(&self) -> &str;
    
    /// Plugin version (semver)
    fn version(&self) -> &str;
    
    /// Called immediately after load
    fn init(&self) -> Result<()>;
    
    /// Commands contributed by this plugin
    fn commands(&self) -> Vec<Command>;
    
    /// Called before plugin unload
    fn cleanup(&self) -> Result<()>;
}
```

#### ConfigLoader (Outbound)

Configuration loading abstraction.

**Trait Definition:**
```rust
pub trait ConfigLoader: Send + Sync {
    /// Load full configuration
    fn load(&self) -> Result<serde_json::Value>;
    
    /// Get value by dot-notation key (e.g., "server.port")
    fn get(&self, key: &str) -> Option<serde_json::Value>;
}
```

#### Logger (Outbound)

Structured logging abstraction.

**Trait Definition:**
```rust
pub trait Logger: Send + Sync {
    fn trace(&self, message: &str);
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
}
```

#### Telemetry (Outbound)

Metrics and tracing abstraction.

**Trait Definition:**
```rust
pub trait Telemetry: Send + Sync {
    fn record_execution(
        &self,
        command: &str,
        duration_ms: u64,
        success: bool,
    );
    
    fn record_error(
        &self,
        command: &str,
        error: &str,
    );
    
    fn record_metric(
        &self,
        name: &str,
        value: f64,
    );
}
```

### Services

#### CommandRegistry

Manages command registration and lookup.

**Struct Definition:**
```rust
pub struct CommandRegistry {
    commands: HashMap<String, Command>,
}
```

**Methods:**
```rust
impl CommandRegistry {
    pub fn new() -> Self;
    
    /// Register command; rejects duplicates
    pub fn register(&mut self, command: Command) -> Result<()>;
    
    /// Get command by exact name
    pub fn get(&self, name: &str) -> Option<&Command>;
    
    /// Find commands matching pattern in name or description
    pub fn find_matching(&self, pattern: &str) -> Vec<&Command>;
    
    /// List all registered commands
    pub fn list(&self) -> Vec<&Command>;
    
    /// Check if command exists
    pub fn contains(&self, name: &str) -> bool;
}
```

#### InputValidator

Validates input against command schema.

**Struct Definition:**
```rust
pub struct InputValidator;

impl InputValidator {
    pub fn validate(input: &Input, command: &Command) -> Result<()> {
        // Check required arguments
        for arg in &command.arguments {
            if arg.required && input.get_arg(&arg.name).is_none() {
                return Err(DomainError::MissingArgument(arg.name.clone()));
            }
        }
        
        // Check required options
        for opt in &command.options {
            if opt.required && input.get_opt(&opt.name).is_none() {
                return Err(DomainError::MissingOption(opt.name.clone()));
            }
        }
        
        Ok(())
    }
}
```

### Domain Errors

```rust
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    
    #[error("Missing required argument: {0}")]
    MissingArgument(String),
    
    #[error("Missing required option: {0}")]
    MissingOption(String),
    
    #[error("Invalid argument value for {name}: {value}")]
    InvalidArgumentValue { name: String, value: String },
    
    #[error("Plugin error: {0}")]
    PluginError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}
```

---

## Application Layer Specification

### CommandExecutor

Orchestrates command execution flow.

**Struct Definition:**
```rust
pub struct CommandExecutor {
    registry: CommandRegistry,
    validator: InputValidator,
    telemetry: Arc<dyn Telemetry>,
}
```

**Methods:**
```rust
impl CommandExecutor {
    pub fn new() -> Self;
    
    pub fn with_telemetry(mut self, telemetry: Arc<dyn Telemetry>) -> Self;
    
    /// Register a command
    pub fn register(&mut self, command: Command) -> Result<()>;
    
    /// Execute command by name with input
    pub async fn execute(
        &self,
        command_name: &str,
        input: Input,
    ) -> Result<Output>;
    
    /// Execute with full context
    pub async fn execute_with_context(
        &self,
        command_name: &str,
        ctx: Context,
    ) -> Result<Output>;
    
    /// List all registered commands
    pub fn list_commands(&self) -> Vec<&Command>;
}
```

**Execution Flow:**
1. Lookup command in registry
2. Validate input against command schema
3. Build Context from Input
4. Execute handler
5. Record telemetry
6. Return Output

### DefaultHandler

Wraps closures as handlers.

**Struct Definition:**
```rust
pub struct DefaultHandler<F, Fut>
where
    F: Fn(&Context) -> Fut + Send + Sync,
    Fut: Future<Output = Result<Output>> + Send,
{
    handler: F,
    _phantom: PhantomData<Fut>,
}
```

**Implementation:**
```rust
impl<F, Fut> CommandHandler for DefaultHandler<F, Fut>
where
    F: Fn(&Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Output>> + Send,
{
    async fn handle(&self, ctx: &Context) -> Result<Output> {
        (self.handler)(ctx).await
    }
}
```

### ValidationPipeline

Composes multiple validators.

**Struct Definition:**
```rust
pub struct ValidationPipeline {
    validators: Vec<Box<dyn Fn(&Input) -> Result<()>>>,
}

impl ValidationPipeline {
    pub fn new() -> Self;
    pub fn add(mut self, validator: impl Fn(&Input) -> Result<()> + 'static) -> Self;
    pub fn validate(&self, input: &Input) -> Result<()>;
}
```

---

## Adapter Layer Specification

### Primary Adapters

#### ClapCliAdapter

Translates between clap and domain types.

**Struct Definition:**
```rust
pub struct ClapCliAdapter {
    app: Command,
}
```

**Methods:**
```rust
impl ClapCliAdapter {
    pub fn new(app: Command) -> Self;
    
    /// Parse arguments and convert to Input
    pub fn parse(&self, args: &[String]) -> Result<Input>;
    
    /// Generate help text
    pub fn help(&self) -> String;
    
    /// Run the full CLI flow
    pub async fn run(&self) -> Result<Output>;
}
```

**Conversion Logic:**
```rust
impl From<clap::ArgMatches> for Input {
    fn from(matches: clap::ArgMatches) -> Self {
        // Extract values from clap matches
        // Populate Input.args, opts, flags
    }
}
```

### Secondary Adapters

#### JsonConfigLoader

**Struct Definition:**
```rust
pub struct JsonConfigLoader {
    path: PathBuf,
    cache: RwLock<Option<serde_json::Value>>,
}
```

**Implementation:**
```rust
impl JsonConfigLoader {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self>;
}

impl ConfigLoader for JsonConfigLoader {
    fn load(&self) -> Result<serde_json::Value> {
        if let Some(ref cached) = *self.cache.read().unwrap() {
            return Ok(cached.clone());
        }
        
        let content = std::fs::read_to_string(&self.path)?;
        let value: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| DomainError::ConfigError(e.to_string()))?;
        
        *self.cache.write().unwrap() = Some(value.clone());
        Ok(value)
    }
    
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        let value = self.load().ok()?;
        // Dot-notation traversal
        key.split('.').fold(Some(value), |v, k| {
            v?.get(k).cloned()
        })
    }
}
```

#### TomlConfigLoader

**Implementation:**
```rust
impl ConfigLoader for TomlConfigLoader {
    fn load(&self) -> Result<serde_json::Value> {
        let content = std::fs::read_to_string(&self.path)?;
        let toml_value: toml::Value = toml::from_str(&content)
            .map_err(|e| DomainError::ConfigError(e.to_string()))?;
        
        // Convert to serde_json::Value
        serde_json::to_value(toml_value)
            .map_err(|e| DomainError::Serialization(e.to_string()))
    }
    
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        let value = self.load().ok()?;
        key.split('.').fold(Some(value), |v, k| {
            v?.get(k).cloned()
        })
    }
}
```

#### TracingLogger

```rust
pub struct TracingLogger {
    target: String,
}

impl Logger for TracingLogger {
    fn trace(&self, message: &str) {
        tracing::trace!(target: &self.target, "{}", message);
    }
    
    fn debug(&self, message: &str) {
        tracing::debug!(target: &self.target, "{}", message);
    }
    
    fn info(&self, message: &str) {
        tracing::info!(target: &self.target, "{}", message);
    }
    
    fn warn(&self, message: &str) {
        tracing::warn!(target: &self.target, "{}", message);
    }
    
    fn error(&self, message: &str) {
        tracing::error!(target: &self.target, "{}", message);
    }
}
```

#### SimpleLogger

Zero-dependency logger for testing.

```rust
pub struct SimpleLogger {
    prefix: String,
}

impl Logger for SimpleLogger {
    fn info(&self, message: &str) {
        println!("[{} INFO] {}", self.prefix, message);
    }
    
    fn error(&self, message: &str) {
        eprintln!("[{} ERROR] {}", self.prefix, message);
    }
    
    // ... other levels
}
```

---

## Infrastructure Layer Specification

### Tracing Setup

```rust
pub fn init_tracing(level: &str) {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_level(true);
    
    let filter = tracing_subscriber::EnvFilter::new(level);
    
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}
```

### Plugin Loader

Handles dynamic library loading via `libloading`.

```rust
pub struct PluginLoader;

impl PluginLoader {
    pub unsafe fn load(path: &Path) -> Result<Box<dyn Plugin>> {
        type CreatePlugin = unsafe fn() -> *mut dyn Plugin;
        
        let lib = Library::new(path)?;
        let create: Symbol<CreatePlugin> = lib.get(b"create_plugin")?;
        
        let plugin = unsafe {
            let raw = create();
            Box::from_raw(raw)
        };
        
        // Library must be kept alive; handled by PluginManager
        Ok(plugin)
    }
}
```

---

## Plugin System Specification

### PluginManager

```rust
pub struct PluginManager {
    libraries: Vec<Library>,
    plugins: Vec<Box<dyn Plugin>>,
    command_cache: Vec<Command>,
}

impl PluginManager {
    pub fn new() -> Self;
    
    /// Load all plugins from directory
    pub fn load_from_dir(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(DomainError::PluginError(
                format!("Plugin directory not found: {}", path.display())
            ));
        }
        
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if is_plugin_file(&path) {
                self.load_plugin(&path)?;
            }
        }
        
        self.rebuild_command_cache();
        Ok(())
    }
    
    /// Load single plugin
    pub unsafe fn load_plugin(&mut self, path: &Path) -> Result<()> {
        let plugin = PluginLoader::load(path)?;
        plugin.init()?;
        self.plugins.push(plugin);
        Ok(())
    }
    
    /// Get all commands from all plugins
    pub fn get_commands(&self) -> &[Command] {
        &self.command_cache
    }
    
    /// List loaded plugin names
    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }
    
    fn rebuild_command_cache(&mut self) {
        self.command_cache.clear();
        for plugin in &self.plugins {
            self.command_cache.extend(plugin.commands());
        }
    }
}

fn is_plugin_file(path: &Path) -> bool {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    match std::env::consts::OS {
        "macos" | "ios" => ext == "dylib",
        "windows" => ext == "dll",
        _ => ext == "so",
    }
}
```

### Plugin Entry Point

Plugins must export this symbol:

```rust
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyPlugin::new()))
}
```

---

## Configuration System

### Configuration Hierarchy

Configuration is resolved in priority order (highest last):

1. **Built-in Defaults** - Hardcoded reasonable defaults
2. **Global Config** - `~/.config/agentkit/config.toml`
3. **Project Config** - `./agentkit.toml` or `./.agentkit/config.toml`
4. **Environment Variables** - `AGENTKIT_*`
5. **CLI Arguments** - Highest priority

### Configuration Merging

```rust
pub struct ConfigResolver {
    loaders: Vec<Box<dyn ConfigLoader>>,
}

impl ConfigResolver {
    pub fn resolve(&self, key: &str) -> Option<serde_json::Value> {
        // Iterate loaders in reverse (highest priority first)
        for loader in self.loaders.iter().rev() {
            if let Some(value) = loader.get(key) {
                return Some(value);
            }
        }
        None
    }
    
    pub fn resolve_all(&self) -> serde_json::Value {
        let mut merged = serde_json::Map::new();
        
        // Lowest to highest priority
        for loader in &self.loaders {
            if let Ok(value) = loader.load() {
                if let Some(obj) = value.as_object() {
                    for (k, v) in obj {
                        merged.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        
        serde_json::Value::Object(merged)
    }
}
```

### Environment Variable Mapping

Environment variables are mapped to config keys:

```
AGENTKIT_SERVER_PORT -> server.port
AGENTKIT_LOG_LEVEL -> log.level
AGENTKIT_PLUGIN_PATH -> plugin.path
```

Implementation:

```rust
pub struct EnvConfigLoader {
    prefix: String,
}

impl ConfigLoader for EnvConfigLoader {
    fn load(&self) -> Result<serde_json::Value> {
        let mut map = serde_json::Map::new();
        
        for (key, value) in std::env::vars() {
            if let Some(stripped) = key.strip_prefix(&self.prefix) {
                let config_key = stripped.to_lowercase().replace('_', ".");
                map.insert(config_key, serde_json::Value::String(value));
            }
        }
        
        Ok(serde_json::Value::Object(map))
    }
    
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        let env_key = format!("{}{}", self.prefix, key.to_uppercase().replace('.', "_"));
        std::env::var(&env_key).ok().map(serde_json::Value::String)
    }
}
```

---

## Observability

### Structured Logging

Log format (JSON):

```json
{
  "timestamp": "2026-04-04T12:34:56.789Z",
  "level": "INFO",
  "target": "agentkit::commands",
  "fields": {
    "message": "Starting deployment",
    "environment": "production",
    "version": "1.2.3"
  },
  "span": {
    "name": "deploy",
    "environment": "production"
  }
}
```

### Span-Based Tracing

```rust
pub async fn execute_with_tracing<F, Fut>(
    command_name: &str,
    f: F,
) -> Result<Output>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Output>>,
{
    let span = tracing::info_span!(
        "execute_command",
        command = command_name,
    );
    
    let start = Instant::now();
    
    let result = f().instrument(span).await;
    
    let duration = start.elapsed();
    
    match &result {
        Ok(_) => {
            tracing::info!(
                command = command_name,
                duration_ms = duration.as_millis(),
                "Command executed successfully"
            );
        }
        Err(e) => {
            tracing::error!(
                command = command_name,
                error = %e,
                duration_ms = duration.as_millis(),
                "Command execution failed"
            );
        }
    }
    
    result
}
```

### Telemetry Recording

```rust
pub struct TelemetryRecorder {
    backend: Arc<dyn Telemetry>,
}

impl TelemetryRecorder {
    pub async fn record_execution<F, Fut>(
        &self,
        command: &str,
        f: F,
    ) -> Result<Output>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Output>>,
    {
        let start = Instant::now();
        let result = f().await;
        let duration = start.elapsed();
        
        self.backend.record_execution(
            command,
            duration.as_millis() as u64,
            result.is_ok(),
        );
        
        if let Err(ref e) = result {
            self.backend.record_error(command, &e.to_string());
        }
        
        result
    }
}
```

---

## CLI Interface

### Argument Parsing Rules

**Positional Arguments:**
```
myapp deploy production --region us-west
       ^^^^^^^^^^ positional
```

**Short Options:**
```
myapp deploy -r us-west
            ^^ short option with value
            
myapp deploy -v
            ^^ short flag
            
myapp deploy -rf production
            ^^ combined short flags (both 'r' and 'f')
```

**Long Options:**
```
myapp deploy --region us-west
            ^^^^^^^^^^ long option with value
            
myapp deploy --region=us-west
            ^^^^^^^^^^^^^^^^^^ alternative form
            
myapp deploy --verbose
            ^^^^^^^^^^ long flag
```

**Subcommands:**
```
myapp git clone https://github.com/user/repo.git
      ^^^ subcommand
          ^^^^^ nested subcommand
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Command not found |
| 4 | Configuration error |
| 5 | Plugin error |
| 130 | Interrupted (Ctrl+C) |

### Help Generation

Auto-generated help format:

```
myapp 1.0.0
A sample CLI application

USAGE:
    myapp <COMMAND> [OPTIONS] [ARGS]

COMMANDS:
    deploy    Deploy application to cloud
    status    Check deployment status
    help      Print this message or help for a command

OPTIONS:
    -h, --help       Print help
    -v, --verbose    Enable verbose output
    -c, --config     Path to config file

ARGS:
    <environment>    Target environment [required]
```

---

## Error Handling

### Error Propagation Strategy

1. **Domain Errors:** Business logic failures, propagate via `Result<T, DomainError>`
2. **Adapter Errors:** External system failures, wrap in `DomainError::Io`, `DomainError::ConfigError`
3. **Plugin Errors:** Plugin-specific failures, `DomainError::PluginError`
4. **Fatal Errors:** Unrecoverable conditions, panic only for invariant violations

### Error Display

Human-readable error messages:

```
Error: Missing required argument: environment

Usage: myapp deploy <environment> [options]

For more information, run: myapp deploy --help
```

### Error Context

Using `thiserror` for automatic context:

```rust
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Failed to load configuration from {path}: {source}")]
    ConfigLoadError {
        path: PathBuf,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("Plugin '{name}' failed to initialize: {message}")]
    PluginInitError {
        name: String,
        message: String,
    },
}
```

---

## Cross-Platform Support

### Supported Targets

**Tier 1 (Fully Supported):**
- x86_64-unknown-linux-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin
- x86_64-pc-windows-msvc

**Tier 2 (Build Verified):**
- aarch64-unknown-linux-gnu
- aarch64-unknown-linux-musl
- aarch64-apple-ios
- aarch64-apple-ios-sim

**Tier 3 (Community):**
- armv7-linux-androideabi
- aarch64-linux-android
- wasm32-wasi

### Platform-Specific Code

```rust
#[cfg(target_os = "macos")]
mod macos {
    pub fn default_config_path() -> PathBuf {
        dirs::home_dir()
            .map(|h| h.join("Library/Application Support/agentkit"))
            .unwrap_or_default()
    }
}

#[cfg(target_os = "linux")]
mod linux {
    pub fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .map(|c| c.join("agentkit"))
            .unwrap_or_default()
    }
}

#[cfg(target_os = "windows")]
mod windows {
    pub fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .map(|c| c.join("agentkit"))
            .unwrap_or_default()
    }
}
```

### Path Handling

```rust
use std::path::PathBuf;

pub fn normalize_path(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    
    #[cfg(windows)]
    {
        // Normalize Windows paths
        path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
    }
    
    #[cfg(not(windows))]
    {
        // Unix paths
        if path.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                return home.join(path.strip_prefix("~").unwrap());
            }
        }
        path.to_path_buf()
    }
}
```

---

## Testing Strategy

### Unit Testing

Domain logic testing without I/O:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_builder() {
        let cmd = Command::new("deploy")
            .description("Deploy app")
            .version("1.0.0");
        
        assert_eq!(cmd.name, "deploy");
        assert_eq!(cmd.description, "Deploy app");
    }
    
    #[test]
    fn test_input_accessors() {
        let input = Input::builder()
            .arg("env", "production")
            .flag("verbose", true)
            .build();
        
        assert_eq!(input.get_str("env"), Some("production"));
        assert!(input.get_flag("verbose"));
    }
    
    #[test]
    fn test_handler_validation() {
        let handler = DeployHandler::new();
        let valid_input = Input::builder()
            .arg("environment", "staging")
            .build();
        
        assert!(handler.validate(&valid_input).is_ok());
        
        let invalid_input = Input::default();
        assert!(handler.validate(&invalid_input).is_err());
    }
}
```

### Integration Testing

Full CLI testing:

```rust
#[tokio::test]
async fn test_cli_end_to_end() {
    let args = vec![
        "myapp".to_string(),
        "deploy".to_string(),
        "production".to_string(),
        "--region".to_string(),
        "us-west-2".to_string(),
    ];
    
    let output = run_cli_with_args(args).await.unwrap();
    
    assert_eq!(output.exit_code, 0);
    assert!(output.content.to_string().contains("Deployed"));
}
```

### Mock Adapters

```rust
pub struct MockConfigLoader {
    data: HashMap<String, serde_json::Value>,
}

impl ConfigLoader for MockConfigLoader {
    fn load(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(&self.data)?)
    }
    
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.data.get(key).cloned()
    }
}

pub struct MockLogger {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logger for MockLogger {
    fn info(&self, message: &str) {
        self.logs.lock().unwrap().push(format!("INFO: {}", message));
    }
    // ...
}
```

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_input_preserves_values(value in "[a-zA-Z0-9]{1,20}") {
        let input = Input::builder()
            .arg("test", &value)
            .build();
        
        prop_assert_eq!(input.get_str("test"), Some(&*value));
    }
}
```

---

## Performance Characteristics

### Startup Time Targets

| Metric | Target | Maximum |
|--------|--------|---------|
| Binary size (release) | 2-5 MB | 10 MB |
| Cold start | < 50ms | 100ms |
| Hot start | < 10ms | 25ms |
| Help text generation | < 5ms | 10ms |

### Memory Footprint

| Scenario | Target |
|----------|--------|
| Base (no plugins) | 5-10 MB |
| With 5 plugins | 10-20 MB |
| Large config file | +1-5 MB |

### Throughput

| Operation | Target |
|-----------|--------|
| Command dispatch | 100,000+ ops/sec |
| Config loading | < 10ms per file |
| Plugin loading | < 50ms per plugin |

### Optimization Strategies

**Link-Time Optimization:**
```toml
[profile.release]
lto = true
codegen-units = 1
```

**Panic Strategy:**
```toml
[profile.release]
panic = "abort"
```

**Binary Stripping:**
```bash
cargo build --release
strip target/release/myapp
```

---

## Security Considerations

### Supply Chain

**Dependency Pinning:**
```toml
[dependencies]
clap = "=4.5.1"
tokio = "=1.36.0"
```

**Vulnerability Scanning:**
```bash
cargo audit
cargo deny check
```

### Input Sanitization

**Path Traversal Prevention:**
```rust
pub fn safe_join(base: &Path, user_path: &str) -> Option<PathBuf> {
    let path = base.join(user_path);
    let canonical_base = base.canonicalize().ok()?;
    let canonical_path = path.canonicalize().ok()?;
    
    if canonical_path.starts_with(&canonical_base) {
        Some(path)
    } else {
        None
    }
}
```

**Shell Injection Prevention:**
```rust
// BAD - never do this
std::process::Command::new("sh")
    .arg("-c")
    .arg(format!("git clone {}", url))
    .spawn();

// GOOD - use argument passing
std::process::Command::new("git")
    .arg("clone")
    .arg(url)  // Properly escaped
    .spawn();
```

### Plugin Security

**Current Limitations:**
- Plugins run in-process with full memory access
- No sandboxing for native plugins
- Trust model: plugins must be from trusted sources

**Future (WASM):**
- Capability-based sandboxing
- Memory isolation
- Defined interface boundary

---

## API Reference

### Prelude Exports

```rust
pub mod prelude {
    // Domain entities
    pub use crate::domain::entities::{Command, Argument, CliOption, Flag};
    
    // Value objects
    pub use crate::domain::value_objects::{Input, Output, Context, ArgValue};
    
    // Ports
    pub use crate::domain::ports::{
        CommandHandler,
        Plugin,
        ConfigLoader,
        Logger,
        Telemetry,
    };
    
    // Services
    pub use crate::domain::services::{CommandRegistry, InputValidator};
    
    // Application
    pub use crate::application::{
        CommandExecutor,
        DefaultHandler,
    };
    
    // Errors
    pub use crate::domain::DomainError;
}
```

### Public Functions

```rust
/// Run CLI with default configuration
pub async fn run_cli() -> Result<Output>;

/// Run CLI with custom configuration
pub async fn run_cli_with_config(config: ConfigResolver) -> Result<Output>;

/// Create default CLI adapter
pub fn default_cli_adapter(app: Command) -> ClapCliAdapter;

/// Initialize tracing with environment-based filtering
pub fn init_tracing();
```

---

## Migration Guide

### From clap-only Applications

**Before:**
```rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();
    if args.verbose {
        println!("Verbose mode");
    }
}
```

**After:**
```rust
use agentkit::prelude::*;

struct VerboseHandler;

#[async_trait]
impl CommandHandler for VerboseHandler {
    async fn handle(&self, ctx: &Context) -> Result<Output> {
        if ctx.get_flag("verbose") {
            Ok(Output::text("Verbose mode"))
        } else {
            Ok(Output::text("Normal mode"))
        }
    }
}

fn main() {
    let app = Command::new("myapp")
        .flag(Flag::new("verbose").short('v'))
        .handler(VerboseHandler);
    
    run_cli().await;
}
```

### From Shell Scripts

**Before:**
```bash
#!/bin/bash
set -e

ENVIRONMENT=$1
REGION=${2:-"us-east-1"}

echo "Deploying to $ENVIRONMENT in $REGION"
# deployment logic...
```

**After:**
```rust
struct DeployHandler;

#[async_trait]
impl CommandHandler for DeployHandler {
    fn validate(&self, input: &Input) -> Result<()> {
        if input.get_str("environment").is_none() {
            return Err(DomainError::MissingArgument("environment".into()));
        }
        Ok(())
    }
    
    async fn handle(&self, ctx: &Context) -> Result<Output> {
        let env = ctx.get_str("environment").unwrap();
        let region = ctx.get_opt("region").unwrap_or("us-east-1");
        
        // deployment logic...
        
        Ok(Output::text(format!("Deployed to {} in {}", env, region)))
    }
}
```

---

## Appendices

### Appendix A: Glossary

- **Adapter:** Implements a port for a specific technology
- **Command:** CLI command definition with metadata
- **Context:** Execution context combining input, env, and config
- **Domain:** Business logic layer, isolated from external concerns
- **Handler:** Implements business logic for a command
- **Hexagonal Architecture:** Ports and adapters pattern for isolation
- **Input:** Parsed CLI arguments as typed values
- **Output:** Command result with content and exit metadata
- **Port:** Interface defined by domain for external capabilities
- **Plugin:** Runtime extension providing additional commands

### Appendix B: Configuration File Example

```toml
# agentkit.toml
[server]
host = "0.0.0.0"
port = 8080

[logging]
level = "info"
format = "json"

[plugins]
directory = "./plugins"
auto_load = true

[telemetry]
enabled = true
endpoint = "https://telemetry.example.com"
```

### Appendix C: Plugin Development Guide

**Minimal Plugin:**

```rust
use agentkit::prelude::*;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn init(&self) -> Result<()> {
        println!("MyPlugin initialized");
        Ok(())
    }
    
    fn commands(&self) -> Vec<Command> {
        vec![
            Command::new("my-command")
                .description("A custom command")
                .handler(MyHandler),
        ]
    }
    
    fn cleanup(&self) -> Result<()> {
        println!("MyPlugin cleanup");
        Ok(())
    }
}

struct MyHandler;

#[async_trait]
impl CommandHandler for MyHandler {
    async fn handle(&self, _ctx: &Context) -> Result<Output> {
        Ok(Output::text("Hello from plugin!"))
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyPlugin))
}
```

**Build Plugin:**
```bash
# macOS
cargo build --release --crate-type cdylib
cp target/release/libmyplugin.dylib plugins/

# Linux
cargo build --release --crate-type cdylib
cp target/release/libmyplugin.so plugins/

# Windows
cargo build --release --crate-type cdylib
copy target/release/myplugin.dll plugins/
```

### Appendix D: Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `AGENTKIT_CONFIG` | Path to config file | `~/.config/agentkit/config.toml` |
| `AGENTKIT_LOG_LEVEL` | Logging level | `info` |
| `AGENTKIT_PLUGIN_DIR` | Plugin directory | `./plugins` |
| `AGENTKIT_NO_COLOR` | Disable colored output | `false` |
| `RUST_LOG` | Tracing filter | `agentkit=info` |

### Appendix E: Related Documentation

- [Architecture Decision Records](./ADR.md)
- [State of the Art Research](./RESEARCH.md)
- [API Documentation](https://docs.rs/agentkit)
- [Examples](./examples/)

---

## Appendix F: Advanced Topics

### F.1 Custom Argument Types

Implementing custom argument parsing:

```rust
use std::str::FromStr;
use std::net::SocketAddr;

pub struct HostPort {
    pub host: String,
    pub port: u16,
}

impl FromStr for HostPort {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Expected format: host:port".into());
        }
        
        let host = parts[0].to_string();
        let port = parts[1].parse::<u16>()
            .map_err(|e| format!("Invalid port: {}", e))?;
        
        Ok(HostPort { host, port })
    }
}

// Usage in command
let cmd = Command::new("serve")
    .argument(
        Argument::new("bind")
            .description("Bind address")
            .default_value("0.0.0.0:8080")
    );
```

### F.2 Async Runtime Configuration

Configuring tokio for optimal CLI performance:

```rust
#[tokio::main(
    flavor = "multi_thread",
    worker_threads = 4,
)]
async fn main() -> Result<()> {
    // Runtime configured for I/O-bound workloads
    run_cli().await
}

// For single-threaded use cases
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    run_cli().await
}
```

### F.3 Signal Handling

Graceful shutdown on SIGINT/SIGTERM:

```rust
use tokio::signal;

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    
    println!("Shutdown signal received, cleaning up...");
}

// Usage
pub async fn run_with_graceful_shutdown() -> Result<()> {
    let shutdown = shutdown_signal();
    let app = run_cli();
    
    tokio::select! {
        result = app => result,
        _ = shutdown => {
            Ok(Output::none().exit_code(130))
        }
    }
}
```

### F.4 Progress Indicators

Long-running command progress reporting:

```rust
use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressHandler {
    bar: ProgressBar,
}

impl ProgressHandler {
    pub fn new(total: u64) -> Self {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        Self { bar }
    }
    
    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }
    
    pub fn finish(&self, msg: &str) {
        self.bar.finish_with_message(msg);
    }
}

// Handler implementation
pub struct DeployWithProgress;

#[async_trait]
impl CommandHandler for DeployWithProgress {
    async fn handle(&self, ctx: &Context) -> Result<Output> {
        let progress = ProgressHandler::new(100);
        
        for i in 0..100 {
            // Deployment step
            tokio::time::sleep(Duration::from_millis(50)).await;
            progress.inc(1);
        }
        
        progress.finish("Deployment complete");
        Ok(Output::text("Deployed successfully"))
    }
}
```

### F.5 Shell Completion

Generating shell completion scripts:

```rust
use clap_complete::{generate, shells::*};

pub fn generate_completion(shell: &str, app: &mut clap::Command) {
    let shell = match shell {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => {
            eprintln!("Unknown shell: {}", shell);
            return;
        }
    };
    
    let bin_name = app.get_name().to_string();
    generate(shell, app, bin_name, &mut std::io::stdout());
}

// Usage in command
Command::new("completion")
    .argument(Argument::new("shell").required())
    .handler(CompletionHandler);
```

### F.6 Man Page Generation

Auto-generating man pages:

```rust
use clap_mangen::Man;

pub fn generate_man_page(app: &clap::Command) -> String {
    let man = Man::new(app.clone());
    let mut buf = Vec::new();
    man.render(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}
```

### F.7 Interactive Prompts

Using inquire for interactive input:

```rust
use inquire::{Text, Select, Confirm};

pub struct InteractiveHandler;

#[async_trait]
impl CommandHandler for InteractiveHandler {
    async fn handle(&self, _ctx: &Context) -> Result<Output> {
        let name = Text::new("Project name:").prompt()
            .map_err(|e| DomainError::Io(e.into()))?;
        
        let template = Select::new(
            "Choose template:",
            vec!["web", "api", "cli"],
        ).prompt()
            .map_err(|e| DomainError::Io(e.into()))?;
        
        let confirm = Confirm::new("Initialize git?")
            .with_default(true)
            .prompt()
            .map_err(|e| DomainError::Io(e.into()))?;
        
        Ok(Output::text(format!(
            "Created {} project '{}' with git={}",
            template, name, confirm
        )))
    }
}
```

### F.8 Configuration Validation Schema

JSON Schema validation for configs:

```rust
use jsonschema::{JSONSchema, ValidationError};
use serde_json::Value;

pub struct SchemaValidator {
    schema: JSONSchema,
}

impl SchemaValidator {
    pub fn new(schema: Value) -> Result<Self> {
        let schema = JSONSchema::compile(&schema)
            .map_err(|e| DomainError::ConfigError(e.to_string()))?;
        Ok(Self { schema })
    }
    
    pub fn validate(&self, config: &Value) -> Result<()> {
        let errors: Vec<_> = self.schema.validate(config)
            .collect();
        
        if errors.is_empty() {
            Ok(())
        } else {
            let msg = errors.iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("; ");
            Err(DomainError::ConfigError(msg))
        }
    }
}
```

### F.9 Rate Limiting

Protecting external API calls:

```rust
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;

pub struct RateLimitedClient {
    limiter: Arc<RateLimiter<direct::NotKeyed, InMemoryState, DefaultClock>>,
}

impl RateLimitedClient {
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(
            NonZeroU32::new(requests_per_second).unwrap()
        );
        let limiter = RateLimiter::direct(quota);
        
        Self {
            limiter: Arc::new(limiter),
        }
    }
    
    pub async fn request(&self, req: Request) -> Result<Response> {
        self.limiter.until_ready().await;
        // Make actual request
        http_client.request(req).await
    }
}
```

### F.10 Distributed Tracing

OpenTelemetry integration:

```rust
use opentelemetry::trace::Tracer;
use opentelemetry_otlp::WithExportConfig;

pub fn init_telemetry(endpoint: &str) -> Result<()> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;
    
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
    
    Ok(())
}

// Usage in handler
#[async_trait]
impl CommandHandler for TracedHandler {
    async fn handle(&self, ctx: &Context) -> Result<Output> {
        let span = tracing::info_span!("handle", command = ctx.input.command);
        let _enter = span.enter();
        
        tracing::info!("Starting operation");
        // ... work ...
        tracing::info!("Operation complete");
        
        Ok(Output::text("Done"))
    }
}
```

---

## Appendix G: Troubleshooting Guide

### G.1 Common Build Issues

**Issue:** Linker error on macOS
```
error: linking with `cc` failed: exit code: 1
```

**Solution:**
```bash
xcode-select --install
```

**Issue:** Missing OpenSSL on Linux
```
error: failed to run custom build command for `openssl-sys`
```

**Solution:**
```bash
# Debian/Ubuntu
sudo apt-get install libssl-dev pkg-config

# Fedora
sudo dnf install openssl-devel pkgconf
```

**Issue:** Windows MSVC toolchain not found
```
error: linker `link.exe` not found
```

**Solution:**
Install Visual Studio Build Tools or switch to GNU toolchain:
```bash
rustup target add x86_64-pc-windows-gnu
```

### G.2 Runtime Issues

**Issue:** Plugin fails to load
```
Error: Plugin error: cannot load library
```

**Diagnosis:**
```bash
# Linux: check dependencies
ldd plugins/myplugin.so

# macOS: check library dependencies
otool -L plugins/myplugin.dylib

# Check architecture match
file plugins/myplugin.so
file target/release/myapp
```

**Issue:** Config file not found
```
Error: Config error: No such file or directory
```

**Solution:**
```bash
# Check config search paths
myapp --config ./myconfig.toml

# Or set environment variable
export AGENTKIT_CONFIG=./myconfig.toml
```

**Issue:** Async runtime not found
```
error: there is no reactor running, must be called from the context of a Tokio 1.x runtime
```

**Solution:**
Ensure you use `#[tokio::main]` or wrap in `tokio::runtime::Runtime::new()`.

### G.3 Debugging Tips

**Enable Debug Logging:**
```bash
RUST_LOG=debug myapp
RUST_LOG=agentkit=trace myapp
```

**Trace System Calls:**
```bash
# Linux
strace -f myapp

# macOS
dtruss -f myapp
```

**Profile Binary:**
```bash
# CPU profiling
cargo flamegraph

# Memory profiling
valgrind --tool=massif ./target/release/myapp
```

---

## Appendix H: Contributing Guidelines

### Code Style

**Formatting:**
```bash
cargo fmt --check
cargo clippy --all-targets --all-features
```

**Documentation:**
- All public items must have doc comments
- Examples in doc comments are tested
- Architecture decisions recorded in ADRs

**Testing:**
- Unit tests for domain logic
- Integration tests for adapters
- Minimum 80% coverage for new code

### Commit Message Format

```
type(scope): subject

body

footer
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(plugins): add WASM plugin support

Add WebAssembly backend for plugin loading using wasmtime.
Enables sandboxed plugins with capability-based security.

Closes #123
```

### Release Process

1. Update CHANGELOG.md
2. Bump version in Cargo.toml
3. Create git tag: `git tag v1.2.3`
4. Push tag: `git push origin v1.2.3`
5. CI builds and publishes to crates.io

---

*Document Version: 2.1.0*  
*Last Updated: 2026-04-04*  
*Maintainer: Phenotype Core Team*  
*License: MIT*
