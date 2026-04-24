# Settly Technical Specification

**Version:** 1.0.0  
**Status:** Draft  
**Date:** 2026-04-05  
**Owner:** Phenotype Team  

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Problem Statement](#2-problem-statement)
3. [Architecture Overview](#3-architecture-overview)
4. [Core Components](#4-core-components)
5. [Data Models](#5-data-models)
6. [API Specifications](#6-api-specifications)
7. [Configuration System](#7-configuration-system)
8. [Layer System](#8-layer-system)
9. [Validation Framework](#9-validation-framework)
10. [Migration Support](#10-migration-support)
11. [Integration Points](#11-integration-points)
12. [Performance Requirements](#12-performance-requirements)
13. [Testing Strategy](#13-testing-strategy)
14. [Implementation Roadmap](#14-implementation-roadmap)
15. [Glossary](#15-glossary)

---

## 1. Executive Summary

### 1.1 Purpose

Settly is a universal configuration management framework for Rust applications, providing layered configs, validation, and environment support following hexagonal architecture principles.

### 1.2 Scope

| Component | Description | Priority |
|-----------|-------------|----------|
| Layered Configuration | Merge configs from multiple sources with priority | P0 |
| Format Support | TOML, YAML, JSON, ENV file parsing | P0 |
| Validation | Schema-based validation with custom validators | P0 |
| Environment Support | Dev, staging, production overrides | P0 |
| Hot Reload | Watch and reload configuration files | P1 |
| Type Safety | Strongly-typed configuration access | P0 |
| Migration | Configuration versioning and migration | P2 |
| Secret Management | Integration with secret stores | P2 |

### 1.3 Key Differentiators

| Feature | Settly | fig | gcfg | config-rs |
|---------|--------|-----|------|-----------|
| Layered Configs | Yes | No | Limited | No |
| Schema Validation | Yes | No | No | No |
| Hot Reload | Yes | No | No | No |
| Environment Override | Yes | Yes | No | Limited |
| Type Safety | Strong | None | None | Some |
| Hexagonal Architecture | Yes | No | No | No |

### 1.4 Target Users

1. **Application Developers**: Configure applications with type-safe access
2. **Platform Engineers**: Standardize configuration across services
3. **DevOps**: Manage environment-specific configurations
4. **Library Authors**: Make libraries configurable without coupling

---

## 2. Problem Statement

### 2.1 Current Challenges

| Challenge | Impact | Frequency |
|-----------|--------|-----------|
| Configuration Scattered | Hard to track all config sources | Daily |
| No Validation | Runtime errors from bad config | Weekly |
| Environment Confusion | Prod bugs from dev settings | Monthly |
| Type Mismatch | Runtime panics from wrong types | Weekly |
| No Hot Reload | Need restart for config changes | Daily |

### 2.2 Business Requirements

| Requirement | Description | Priority |
|-------------|-------------|----------|
| BR-001 | Unified config from files, env, CLI | P0 |
| BR-002 | Validate config before use | P0 |
| BR-003 | Environment-specific overrides | P0 |
| BR-004 | Type-safe config access | P0 |
| BR-005 | Hot reload without restart | P1 |

---

## 3. Architecture Overview

### 3.1 Hexagonal Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Driving Adapters                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │     CLI      │  │   REST API   │  │  Framework   │  │   Library    │  │
│  │   Adapter    │  │   Adapter    │  │   Adapter    │  │   Consumer   │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Application Core                                │
│  ┌──────────────────────────────────────────────────────────────────────┐ │
│  │                         Ports (Interfaces)                             │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │ │
│  │  │  ConfigPort  │  │ LoaderPort   │  │ ValidatorPort│              │ │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────────────────────────┐ │
│  │                      Domain (Business Logic)                          │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │ │
│  │  │   Config     │  │   Layer      │  │   Builder    │              │ │
│  │  │   Entity     │  │   Manager    │  │   Service    │              │ │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Driven Adapters                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │    TOML      │  │    YAML      │  │     ENV      │  │     CLI      │  │
│  │   Loader     │  │   Loader     │  │   Loader     │  │   Args       │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   JSON       │  │  Validator   │  │   Secret     │  │   Hot       │  │
│  │   Loader     │  │   Adapter    │  │   Store      │  │   Reload    │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Module Hierarchy

```
settly/
├── src/
│   ├── lib.rs                    # Public API exports
│   │
│   ├── domain/                   # Core domain
│   │   ├── mod.rs
│   │   ├── config.rs             # Configuration entity
│   │   ├── layer.rs              # Layer definition
│   │   ├── value.rs              # Config value types
│   │   ├── error.rs              # Domain errors
│   │   └── path.rs               # Dot-notation paths
│   │
│   ├── application/              # Application services
│   │   ├── mod.rs
│   │   ├── builder.rs            # Config builder
│   │   ├── loader.rs             # Loading orchestration
│   │   ├── validator.rs          # Validation runner
│   │   └── hot_reload.rs         # Hot reload service
│   │
│   ├── ports/                    # Port interfaces
│   │   ├── mod.rs
│   │   ├── config_source.rs      # Config source trait
│   │   ├── validator.rs          # Validator trait
│   │   └── secrets.rs            # Secret store trait
│   │
│   ├── adapters/                 # Adapter implementations
│   │   ├── mod.rs
│   │   ├── loaders/             # Format loaders
│   │   │   ├── mod.rs
│   │   │   ├── toml.rs
│   │   │   ├── yaml.rs
│   │   │   ├── json.rs
│   │   │   └── env.rs
│   │   ├── validators/           # Built-in validators
│   │   │   ├── mod.rs
│   │   │   ├── json_schema.rs
│   │   │   └── custom.rs
│   │   └── secrets/              # Secret integrations
│   │       ├── mod.rs
│   │       ├── env.rs
│   │       └── vault.rs
│   │
│   └── infrastructure/           # Cross-cutting
│       ├── mod.rs
│       ├── watcher.rs            # File watching
│       └── merge.rs             # Deep merge logic
```

### 3.3 Configuration Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Default   │────▶│    File     │────▶│  Environment│────▶│    CLI      │
│   Values    │     │   Config    │     │   Vars     │     │    Args     │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
       │                  │                  │                   │
       │                  │                  │                   │
       ▼                  ▼                  ▼                   ▼
   ┌─────────────────────────────────────────────────────────────────┐
   │                      Layer Merge (priority: CLI > ENV > File > Default)
   └─────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
   ┌─────────────────────────────────────────────────────────────────┐
   │                      Validation (optional)
   └─────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
   ┌─────────────────────────────────────────────────────────────────┐
   │                   Type-Safe Config Access
   └─────────────────────────────────────────────────────────────────┘
```

---

## 4. Core Components

### 4.1 Configuration Entity

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub values: Map<String, ConfigValue>,
    pub metadata: ConfigMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(Map<String, ConfigValue>),
    Null,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigMetadata {
    pub source: ConfigSource,
    pub path: Option<PathBuf>,
    pub layer_index: u8,
    pub loaded_at: DateTime<Utc>,
}
```

### 4.2 Layer Manager

```rust
pub struct LayerManager {
    layers: Vec<Layer>,
    merger: Arc<dyn ConfigMerger>,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub source: Arc<dyn ConfigSource>,
    pub priority: LayerPriority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LayerPriority {
    Default = 0,
    File = 10,
    Environment = 20,
    Cli = 30,
}

impl LayerManager {
    pub fn add_layer(&mut self, layer: Layer) -> &mut Self;
    pub fn merge(&self) -> Result<Config, MergeError>;
    pub fn get_value(&self, path: &str) -> Option<&ConfigValue>;
}
```

### 4.3 Config Builder

```rust
pub struct ConfigBuilder {
    layers: Vec<Layer>,
    validators: Vec<Arc<dyn ConfigValidator>>,
    post_build: Option<Box<dyn FnOnce(&Config)>>,
}

impl ConfigBuilder {
    pub fn new() -> Self;
    
    pub fn with_default<T: Into<ConfigValue>>(mut self, path: &str, value: T) -> Self;
    
    pub fn with_file<P: AsRef<Path>>(mut self, path: P) -> Result<Self, ConfigError>;
    
    pub fn with_env_prefix<T: AsRef<str>>(mut self, prefix: T) -> Result<Self, ConfigError>;
    
    pub fn with_cli_args<T: Iterator<Item = String>>(mut self, args: T) -> Result<Self, ConfigError>;
    
    pub fn with_validator(mut self, validator: Arc<dyn ConfigValidator>) -> Self;
    
    pub fn build(&self) -> Result<Config, ConfigError>;
}
```

---

## 5. Data Models

### 5.1 Config Source Types

| Type | Description | Priority | Merge Strategy |
|------|-------------|----------|----------------|
| Default | Hard-coded fallback values | Lowest | Override |
| File | TOML/YAML/JSON files | Low | Override |
| Environment | ENV variables | Medium | Override |
| CLI | Command-line arguments | Highest | Override |

### 5.2 Layer Merge Rules

| Source Type | Destination Exists | Action |
|-------------|-------------------|--------|
| Object | Object | Recursive merge |
| Array | Any | Replace |
| String | Any | Replace |
| Number | Any | Replace |
| Boolean | Any | Replace |
| Null | Any | Delete/ignore |

### 5.3 Validation Error Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub validator: String,
    pub value: Option<ConfigValue>,
    pub context: Map<String, ConfigValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}
```

---

## 6. API Specifications

### 6.1 Builder API

```rust
// Basic usage
let config = ConfigBuilder::new()
    .with_file("config.toml")?
    .with_env_prefix("APP_")?
    .with_cli_args(std::env::args())?
    .build()?;

// Type-safe access
let port: u16 = config.get("server.port")?;
let host: String = config.get("server.host").unwrap_or_else(|| "0.0.0.0".to_string());
```

### 6.2 Macro API

```rust
use settly::config;

#[config]
struct ServerConfig {
    #[settly(default = "0.0.0.0")]
    host: String,
    
    #[settly(default = 8080)]
    port: u16,
    
    #[settly(validate = "port_range")]
    max_connections: Option<u32>,
}

fn port_range(port: &u32) -> Result<(), String> {
    if *port > 65535 {
        Err("Port must be <= 65535".to_string())
    } else {
        Ok(())
    }
}
```

### 6.3 Builder Pattern API

```rust
let config = ConfigBuilder::new()
    .runtime(|r| r
        .max_memory_mb(8192)
        .worker_threads(16)
        .log_level("info".to_string())
    )
    .database(|d| d
        .host("localhost".to_string())
        .port(5432)
        .name("myapp".to_string())
    )
    .build()?;
```

---

## 7. Configuration System

### 7.1 Configuration Schema

```toml
# settly configuration
[settings]
log_level = "info"
debug = false

[settings.validation]
enabled = true
strict = true
fail_fast = true

[settings.layers]
default_priority = 0
file_priority = 10
env_priority = 20
cli_priority = 30

[settings.merge]
arrays_replace = true
null_ignore = true

[settings.reload]
enabled = true
debounce_ms = 500
```

### 7.2 Environment Variable Convention

| Variable | Config Path | Example |
|----------|-------------|---------|
| APP_HOST | server.host | APP_HOST=0.0.0.0 |
| APP_PORT | server.port | APP_PORT=8080 |
| APP__DB__HOST | db.host | APP__DB__HOST=localhost |

### 7.3 CLI Argument Convention

| Argument | Config Path | Example |
|----------|-------------|---------|
| --server.host | server.host | --server.host=0.0.0.0 |
| --server.port | server.port | --server.port=8080 |
| --db.host | db.host | --db.host=localhost |

---

## 8. Layer System

### 8.1 Layer Priority

```
┌─────────────────────────────────────────────────────────────┐
│  CLI Args              Priority: 30  (Applied last)         │
│  Environment Variables  Priority: 20                         │
│  File Config           Priority: 10                         │
│  Default Values        Priority: 0   (Applied first)        │
└─────────────────────────────────────────────────────────────┘
```

### 8.2 Merge Algorithm

```rust
pub fn deep_merge(base: &mut Map<String, ConfigValue>, overlay: Map<String, ConfigValue>) {
    for (key, overlay_value) in overlay {
        match (base.get(&key), overlay_value) {
            (Some(base_obj), ConfigValue::Object(overlay_obj)) 
                if matches!(base_obj, ConfigValue::Object(_)) => {
                // Recursive merge for objects
                let base_obj = base.get_mut(&key).unwrap();
                if let ConfigValue::Object(base_map) = base_obj {
                    deep_merge(base_map, overlay_obj);
                }
            }
            _ => {
                // Replace for all other cases
                base.insert(key, overlay_value);
            }
        }
    }
}
```

### 8.3 Environment-Specific Layers

```rust
// Load environment-specific config
let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

let config = ConfigBuilder::new()
    .with_file("config/default.toml")?
    .with_file(format!("config/{}.toml", env))?
    .with_env_prefix("APP_")?
    .build()?;
```

---

## 9. Validation Framework

### 9.1 Built-in Validators

| Validator | Description | Example |
|-----------|-------------|---------|
| required | Field must exist | `validate.required("db.host")` |
| type_check | Field must be specific type | `validate.type_check::<String>("server.host")` |
| range | Number in range | `validate.range("server.port", 1..=65535)` |
| pattern | String matches regex | `validate.pattern("email", r"^[\w-\.]+@([\w-]+\.)+[\w-]{2,}$")` |
| enum | Value in list | `validate.enum("log.level", &["debug", "info", "warn", "error"])` |
| custom | Custom function | `validate.custom("custom_rule", my_validator)` |

### 9.2 JSON Schema Validation

```rust
use jsonschema::JSONSchema;

let schema = serde_json::json!({
    "type": "object",
    "properties": {
        "host": { "type": "string" },
        "port": { "type": "integer", "minimum": 1, "maximum": 65535 }
    },
    "required": ["host", "port"]
});

let compiled = JSONSchema::compile(&schema).unwrap();

config.validate_with_schema(&compiled)?;
```

### 9.3 Custom Validators

```rust
use settly::validator::ConfigValidator;

pub struct PortValidator;

#[async_trait::async_trait]
impl ConfigValidator for PortValidator {
    async fn validate(&self, config: &Config) -> Result<ValidationResult, ConfigError> {
        let port = config.get::<u16>("server.port")?;
        
        if port == 0 {
            return Ok(ValidationResult {
                valid: false,
                errors: vec![ValidationError {
                    path: "server.port".to_string(),
                    message: "Port cannot be 0".to_string(),
                    validator: "port_validator".to_string(),
                    value: None,
                    context: Map::new(),
                }],
                warnings: vec![],
            });
        }
        
        Ok(ValidationResult { valid: true, errors: vec![], warnings: vec![] })
    }
}
```

---

## 10. Migration Support

### 10.1 Configuration Versioning

```rust
pub struct ConfigMigration {
    pub from_version: Version,
    pub to_version: Version,
    pub steps: Vec<MigrationStep>,
}

pub enum MigrationStep {
    RenameKey { from: String, to: String },
    ChangeType { path: String, from_type: Type, to_type: Type },
    MoveKey { from: String, to: String },
    DeleteKey { path: String },
}
```

### 10.2 Automatic Migration

```rust
let config = ConfigBuilder::new()
    .with_migration_from_version("1.0.0")?
    .with_file("config.toml")?
    .build()?;

if let Some(migration) = config.migration_applied() {
    info!("Migrated from {} to {}", migration.from_version, migration.to_version);
}
```

---

## 11. Integration Points

### 11.1 Framework Integration

| Framework | Integration | Status |
|-----------|-------------|--------|
| Axum | Settly extractor | Planned |
| Actix-web | Settly extractor | Planned |
| Tower | Settly layer | Planned |
| Tokio | Settly runtime | Implemented |

### 11.2 Secret Store Integration

| Store | Integration | Status |
|-------|-------------|--------|
| Environment | Built-in | Implemented |
| Vault | HashiCorp Vault | Planned |
| AWS Secrets | AWS SSM | Planned |
| Kubernetes | K8s Secrets | Planned |

---

## 12. Performance Requirements

### 12.1 Latency Targets

| Operation | Target | Maximum |
|-----------|--------|---------|
| Config build (3 sources) | < 10ms | 50ms |
| Config get (primitive) | < 1μs | 10μs |
| Config get (nested) | < 5μs | 50μs |
| Hot reload | < 100ms | 500ms |

### 12.2 Memory Usage

| Scenario | Memory |
|----------|--------|
| Empty config | 10KB |
| 100 keys, shallow | 50KB |
| 100 keys, nested | 100KB |
| 1000 keys, complex | 1MB |

---

## 13. Testing Strategy

### 13.1 Test Categories

| Category | Focus | Tools |
|----------|-------|-------|
| Unit | Individual functions | `#[test]` |
| Property | Invariants | `proptest` |
| Integration | Layer merging | `#[tokio::test]` |
| Format Parsing | TOML/YAML/JSON/ENV | `#[test]` |
| Validation | Custom validators | `#[test]` |

### 13.2 Test Pyramid

```
           ┌─────────┐
           │   E2E    │  10% - Full config flow
           └────┬────┘
       ┌────────┴────────┐
       │   Integration    │  30% - Layer merging
       └────────┬────────┘
   ┌───────────┴───────────┐
   │        Unit Tests       │  60% - Individual functions
   └───────────────────────┘
```

---

## 14. Implementation Roadmap

### 14.1 Phase 1: Core (v0.1)

- [x] Project structure
- [ ] Config entity
- [ ] Layer manager
- [ ] TOML/YAML/JSON loaders
- [ ] Basic builder

### 14.2 Phase 2: Validation (v0.2)

- [ ] JSON Schema validator
- [ ] Built-in validators
- [ ] Custom validator support
- [ ] Error reporting

### 14.3 Phase 3: Ecosystem (v0.3)

- [ ] ENV loader
- [ ] CLI args loader
- [ ] Hot reload
- [ ] Secret store integration

### 14.4 Phase 4: Polish (v1.0)

- [ ] Performance optimization
- [ ] Documentation
- [ ] Examples
- [ ] Release

---

## 15. Glossary

| Term | Definition |
|------|------------|
| Layer | A configuration source with a specific priority |
| Merge | Combining configurations from multiple layers |
| Validator | Component that checks configuration values |
| Hot Reload | Updating configuration without restart |
| Config Source | Origin of configuration values (file, env, etc.) |
| Config Value | A typed value in the configuration |
| Path | Dot-notation address to a config value |

---

**End of Specification**
