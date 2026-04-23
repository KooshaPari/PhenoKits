# ADR-003: Plugin System Contract Design

**Date**: 2026-04-04  
**Status**: Proposed  
**Deciders**: Phenotype Architecture Team  
**Related**: [SOTA Research](./SOTA-RESEARCH.md), [ADR-001](./ADR-001-api-contract-format.md)

## Context

The Phenotype Contracts system includes a plugin architecture for extending functionality through dynamically loaded modules. The plugin system requires:

1. **Plugin Interface Contracts**: Standard interfaces all plugins must implement
2. **Manifest Schema**: Declarative configuration for plugin metadata and dependencies
3. **Lifecycle Management**: Standard lifecycle hooks (init, start, stop, health)
4. **Type Safety**: Compile-time and runtime type checking for plugin interfaces
5. **Version Compatibility**: Version negotiation and compatibility checking
6. **Isolation**: Sandboxed execution where possible

## Decision Drivers

| Driver | Weight | Description |
|--------|--------|-------------|
| **Type Safety** | Critical | Plugins must conform to interfaces at compile time |
| **Security** | Critical | Plugins must be sandboxed and validated |
| **Version Compatibility** | High | Graceful handling of version mismatches |
| **Performance** | High | Minimal overhead for plugin operations |
| **Extensibility** | Medium | Easy to add new plugin types |
| **Cross-Language** | Medium | Support plugins in multiple languages |

## Options Considered

### Option A: Go Interface-Based Plugins

**Description**: Native Go plugin system using `plugin` package or RPC-based plugins.

**Pros**:
- Native Go interfaces
- Type safety at compile time
- Good performance
- Simple for Go-only ecosystem

**Cons**:
- Limited to Go (no cross-language)
- `plugin` package has limitations (no Windows, build constraints)
- Version compatibility issues with shared libraries
- Hard to sandbox

```go
// Go plugin interface
package main

import "github.com/Phenotype/contracts/plugins"

type MyPlugin struct{}

func (p *MyPlugin) Metadata() *plugins.Metadata {
    return &plugins.Metadata{
        ID: "example.my-plugin",
        Name: "My Plugin",
        Version: "1.0.0",
    }
}

func (p *MyPlugin) Init(ctx context.Context, config map[string]any) error {
    return nil
}

func (p *MyPlugin) Start(ctx context.Context) error {
    return nil
}

func (p *MyPlugin) Stop(ctx context.Context) error {
    return nil
}

func (p *MyPlugin) Health(ctx context.Context) (*plugins.HealthStatus, error) {
    return &plugins.HealthStatus{Healthy: true}, nil
}
```

### Option B: gRPC-Based Plugin System (HashiCorp Model)

**Description**: RPC-based plugins using gRPC for cross-language support, inspired by HashiCorp's go-plugin.

**Pros**:
- Cross-language support (Go, Rust, Python, etc.)
- Strong type safety via Protocol Buffers
- Process isolation (security)
- Version negotiation via gRPC
- Hot reloading possible

**Cons**:
- RPC overhead
- More complex setup
- Requires gRPC infrastructure
- Debugging complexity

```protobuf
// Plugin interface definition
syntax = "proto3";

service Plugin {
    rpc GetMetadata(Empty) returns (Metadata);
    rpc Init(InitRequest) returns (InitResponse);
    rpc Start(Empty) returns (StartResponse);
    rpc Stop(Empty) returns (StopResponse);
    rpc Health(Empty) returns (HealthStatus);
    rpc Execute(ExecuteRequest) returns (ExecuteResponse);
}

message Metadata {
    string id = 1;
    string name = 2;
    string version = 3;
    PluginType type = 4;
}

enum PluginType {
    BUS = 0;
    CACHE = 1;
    DATABASE = 2;
    AUTH = 3;
    STORAGE = 4;
    LOGGER = 5;
    METRICS = 6;
    TRACER = 7;
    CUSTOM = 8;
}
```

### Option C: WebAssembly (WASM) Plugins

**Description**: WASM-based plugins for near-native performance with sandboxing.

**Pros**:
- Sandboxed by design (security)
- Near-native performance
- Cross-language (any language compiling to WASM)
- Small binary size
- Hot reloading

**Cons**:
- Limited system access (WASM sandbox)
- Host function complexity
- Tooling maturity
- Debugging challenges

```rust
// WASM plugin in Rust
use wit_bindgen_rust::exports;

exports!(Component);

struct Component;

impl exports::phenotype::plugin::Plugin for Component {
    fn metadata() -> Metadata {
        Metadata {
            id: "example.wasm-plugin".to_string(),
            name: "WASM Plugin".to_string(),
            version: "1.0.0".to_string(),
            plugin_type: PluginType::Cache,
        }
    }
    
    fn init(config: Vec<(String, String)>) -> Result<(), String> {
        Ok(())
    }
    
    fn start() -> Result<(), String> {
        Ok(())
    }
    
    fn stop() -> Result<(), String> {
        Ok(())
    }
    
    fn health() -> HealthStatus {
        HealthStatus { healthy: true }
    }
}
```

### Option D: Hybrid Plugin System (Recommended)

**Description**: Multi-tier plugin system supporting different isolation levels:

1. **In-Process**: Go interfaces for trusted, high-performance plugins
2. **gRPC**: For cross-language plugins requiring process isolation
3. **WASM**: For untrusted plugins requiring strong sandboxing

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Hybrid Plugin System Architecture                          │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Plugin Registry                              │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                 │   │
│  │  │   In-Proc   │  │   gRPC      │  │    WASM     │                 │   │
│  │  │   Plugins   │  │   Plugins   │  │   Plugins   │                 │   │
│  │  │   (Go)      │  │ (Any Lang)  │  │  (Sandbox)  │                 │   │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘                 │   │
│  │         │                │                │                        │   │
│  │         ▼                ▼                ▼                        │   │
│  │  ┌──────────────────────────────────────────────────────────┐     │   │
│  │  │              Unified Plugin Interface                     │     │   │
│  │  │  • Metadata()  • Init()  • Start()  • Stop()  • Health()  │     │   │
│  │  └──────────────────────────────────────────────────────────┘     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  Isolation Level:    Low ◄─────────────────────────────► High              │
│  Performance:        High ◄────────────────────────────► Low             │
│  Security:           Low ◄─────────────────────────────► High              │
│  Cross-Language:     No  ◄─────────────────────────────► Yes               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Decision

**Adopt a Hybrid Plugin System** with three tiers:

1. **Native Plugins** (Go interfaces): For core infrastructure, maximum performance
2. **gRPC Plugins**: For cross-language support, process isolation
3. **WASM Plugins**: For untrusted code, maximum sandboxing

### Plugin Type Assignment

| Plugin Type | Recommended Implementation | Rationale |
|-------------|---------------------------|-----------|
| **Database adapters** | Native | Performance critical |
| **Cache implementations** | Native or gRPC | Performance critical |
| **Message bus** | gRPC | May be external service |
| **Auth providers** | gRPC | Often external (OAuth, SAML) |
| **Metrics exporters** | gRPC | Non-critical path |
| **Custom business logic** | WASM | Untrusted, needs sandbox |
| **Data transformers** | WASM | Compute-heavy, sandboxed |
| **Validation rules** | Native | Performance critical |

### Plugin Manifest Schema

```yaml
# plugin.yaml - Declarative plugin configuration
apiVersion: phenotype.io/v1
kind: Plugin

metadata:
  id: com.example.my-plugin
  name: My Example Plugin
  version: 1.2.3
  author: Example Inc
  license: Apache-2.0
  
spec:
  type: cache
  runtime: wasm  # native | grpc | wasm
  
  # For WASM plugins
  wasm:
    module: plugin.wasm
    memory: 128Mi
    timeout: 30s
    
  # For gRPC plugins  
  grpc:
    address: localhost:50051
    tls: true
    
  # For native plugins
  native:
    package: github.com/example/my-plugin
    symbol: NewPlugin
  
  # Configuration schema
  config:
    schema:
      type: object
      required: [endpoint]
      properties:
        endpoint:
          type: string
          format: uri
        timeout:
          type: integer
          default: 5000
        retries:
          type: integer
          default: 3
          maximum: 10
  
  # Dependencies
  dependencies:
    - id: phenotype.logger
      version: ">=1.0.0"
      optional: false
    - id: phenotype.metrics
      version: ">=1.0.0"
      optional: true
  
  # Capabilities
  capabilities:
    - read
    - write
    - streaming
  
  # Resource limits
  resources:
    cpu: "100m"
    memory: "256Mi"
    requestsPerSecond: 1000
```

### Plugin Interface Contract

```go
// contracts/plugins/plugin.go
package plugins

import "context"

// PluginType defines the type/category of a plugin.
type PluginType string

const (
    PluginTypeBus        PluginType = "bus"
    PluginTypeCache      PluginType = "cache"
    PluginTypeDatabase   PluginType = "database"
    PluginTypeAuth       PluginType = "auth"
    PluginTypeStorage    PluginType = "storage"
    PluginTypeLogger     PluginType = "logger"
    PluginTypeMetrics    PluginType = "metrics"
    PluginTypeTracer     PluginType = "tracer"
    PluginTypeCustom     PluginType = "custom"
)

// Plugin is the base interface that all plugins must implement.
type Plugin interface {
    // Metadata returns the plugin metadata.
    Metadata() *Metadata
    
    // Init initializes the plugin with the given configuration.
    Init(ctx context.Context, config map[string]any) error
    
    // Start starts the plugin.
    Start(ctx context.Context) error
    
    // Stop stops the plugin gracefully.
    Stop(ctx context.Context) error
    
    // Health returns the plugin health status.
    Health(ctx context.Context) (*HealthStatus, error)
}

// Metadata contains plugin metadata information.
type Metadata struct {
    ID          string       `json:"id" validate:"required"
    Name        string       `json:"name" validate:"required"`
    Version     string       `json:"version" validate:"required,semver"`
    Description string       `json:"description"`
    Type        PluginType   `json:"type" validate:"required"`
    Author      string       `json:"author,omitempty"`
    License     string       `json:"license,omitempty"`
    Homepage    string       `json:"homepage,omitempty" validate:"omitempty,url"`
    Repository  string       `json:"repository,omitempty" validate:"omitempty,url"`
    Dependencies []Dependency `json:"dependencies,omitempty"`
    Tags        []string     `json:"tags,omitempty"`
}

// Dependency represents a plugin dependency.
type Dependency struct {
    ID       string `json:"id" validate:"required"`
    Version  string `json:"version" validate:"required"`
    Optional bool   `json:"optional,omitempty"`
}

// HealthStatus represents the health status of a plugin.
type HealthStatus struct {
    State     PluginState `json:"state"`
    Healthy   bool        `json:"healthy"`
    Message   string      `json:"message,omitempty"`
    Error     string      `json:"error,omitempty"`
    LastCheck string      `json:"last_check,omitempty"`
}

// PluginState represents the current state of a plugin.
type PluginState string

const (
    PluginStateRegistered   PluginState = "registered"
    PluginStateLoaded       PluginState = "loaded"
    PluginStateInitialized  PluginState = "initialized"
    PluginStateRunning      PluginState = "running"
    PluginStateStopping     PluginState = "stopping"
    PluginStateStopped      PluginState = "stopped"
    PluginStateError        PluginState = "error"
)
```

### Plugin Registry Contract

```go
// contracts/plugins/registry.go
package plugins

import "context"

// PluginFactory creates plugin instances.
type PluginFactory interface {
    Create(ctx context.Context, config map[string]any) (Plugin, error)
}

// PluginRegistry manages plugin registration and discovery.
type PluginRegistry interface {
    // Register registers a plugin factory.
    Register(pluginType PluginType, factory PluginFactory) error
    
    // Unregister unregisters a plugin factory.
    Unregister(pluginType PluginType) error
    
    // GetFactory returns the factory for a plugin type.
    GetFactory(pluginType PluginType) (PluginFactory, error)
    
    // List returns all registered plugin types.
    List() []PluginType
    
    // Create creates a new plugin instance by type.
    Create(ctx context.Context, pluginType PluginType, config map[string]any) (Plugin, error)
    
    // ResolveDependencies resolves and orders dependencies.
    ResolveDependencies(plugin *Metadata) ([]PluginType, error)
}

// PluginLoader loads plugins from various sources.
type PluginLoader interface {
    Load(ctx context.Context, source string) ([]Plugin, error)
    LoadFromFile(ctx context.Context, path string) ([]Plugin, error)
    LoadFromDir(ctx context.Context, dir string) ([]Plugin, error)
    LoadFromRegistry(ctx context.Context, registryURL string, names []string) ([]Plugin, error)
}
```

### Type-Specific Plugin Interfaces

```go
// contracts/plugins/ports.go
package plugins

import (
    "context"
    "contracts/models"
    "contracts/ports/outbound"
)

// CachePlugin extends Plugin with cache operations.
type CachePlugin interface {
    Plugin
    outbound.Cache
}

// DatabasePlugin extends Plugin with database operations.
type DatabasePlugin interface {
    Plugin
    outbound.QueryExecutor
    outbound.ConnectionPool
}

// AuthPlugin extends Plugin with authentication operations.
type AuthPlugin interface {
    Plugin
    Authenticate(ctx context.Context, credentials models.Credentials) (*models.AuthResult, error)
    Authorize(ctx context.Context, token string, resource string, action string) (bool, error)
}

// BusPlugin extends Plugin with message bus operations.
type BusPlugin interface {
    Plugin
    Publish(ctx context.Context, topic string, message any) error
    Subscribe(ctx context.Context, topic string, handler func(message any)) error
}
```

## Implementation Plan

### Phase 1: Core Interfaces (Weeks 1-2)
- [ ] Define Plugin, Registry, and Loader interfaces
- [ ] Create manifest schema (JSON Schema)
- [ ] Implement manifest validation
- [ ] Create plugin lifecycle manager

### Phase 2: Native Plugin Support (Weeks 3-4)
- [ ] Implement Go interface-based plugins
- [ ] Create plugin build tooling
- [ ] Implement hot reloading for native plugins
- [ ] Add native plugin examples

### Phase 3: gRPC Plugin Support (Weeks 5-7)
- [ ] Define Protocol Buffers service definitions
- [ ] Implement gRPC plugin host
- [ ] Create gRPC plugin SDK (Go, Rust, Python)
- [ ] Add TLS/mTLS support

### Phase 4: WASM Plugin Support (Weeks 8-10)
- [ ] Implement WASM runtime integration (Wasmtime)
- [ ] Define WIT (WASM Interface Types) interfaces
- [ ] Create WASM plugin SDK (Rust, Go via TinyGo)
- [ ] Implement resource limits and sandboxing

### Phase 5: Registry & Discovery (Weeks 11-12)
- [ ] Create plugin registry service
- [ ] Implement dependency resolution
- [ ] Add version compatibility checking
- [ ] Create plugin marketplace (optional)

### Phase 6: Tooling & Documentation (Ongoing)
- [ ] Plugin CLI for development
- [ ] Plugin testing framework
- [ ] Documentation generation
- [ ] Performance profiling tools

## Version Compatibility

### Semantic Versioning for Plugins

```go
// contracts/plugins/version.go
package plugins

import "github.com/Masterminds/semver"

// VersionConstraint represents version requirements.
type VersionConstraint struct {
    MinVersion *semver.Version
    MaxVersion *semver.Version
    Constraint string // e.g., ">=1.0.0 <2.0.0"
}

// IsCompatible checks if a plugin version satisfies constraints.
func (vc *VersionConstraint) IsCompatible(version string) (bool, error) {
    v, err := semver.NewVersion(version)
    if err != nil {
        return false, err
    }
    
    c, err := semver.NewConstraint(vc.Constraint)
    if err != nil {
        return false, err
    }
    
    return c.Check(v), nil
}

// CheckBreakingChanges detects breaking changes between versions.
func CheckBreakingChanges(oldManifest, newManifest *Manifest) []BreakingChange {
    var changes []BreakingChange
    
    // Check interface changes
    if oldManifest.Spec.Runtime != newManifest.Spec.Runtime {
        changes = append(changes, BreakingChange{
            Type:    "runtime",
            Field:   "spec.runtime",
            Old:     string(oldManifest.Spec.Runtime),
            New:     string(newManifest.Spec.Runtime),
            Breaking: true,
        })
    }
    
    // Check required config changes
    oldRequired := getRequiredConfig(oldManifest)
    newRequired := getRequiredConfig(newManifest)
    
    for _, req := range newRequired {
        if !contains(oldRequired, req) {
            changes = append(changes, BreakingChange{
                Type:     "config",
                Field:    req,
                Breaking: true,
                Message:  "New required config field",
            })
        }
    }
    
    return changes
}
```

## Security Model

```go
// contracts/plugins/security.go
package plugins

import "context"

// SecurityContext provides security information for plugin execution.
type SecurityContext struct {
    // Capabilities granted to the plugin
    Capabilities []Capability
    
    // Resource limits
    ResourceLimits ResourceLimits
    
    // Network policy
    NetworkPolicy NetworkPolicy
    
    // File system access
    FilesystemPolicy FilesystemPolicy
}

// Capability represents a permission granted to a plugin.
type Capability string

const (
    CapabilityNetwork    Capability = "network"
    CapabilityFileRead   Capability = "file:read"
    CapabilityFileWrite  Capability = "file:write"
    CapabilityExec       Capability = "exec"
    CapabilityEnv        Capability = "env"
)

// Enforce SecurityContext based on plugin type and manifest.
func (sc *SecurityContext) Enforce(ctx context.Context, operation Operation) error {
    for _, cap := range operation.RequiredCapabilities {
        if !sc.HasCapability(cap) {
            return ErrCapabilityNotGranted{Capability: cap}
        }
    }
    return nil
}
```

## Consequences

### Positive
- **Flexibility**: Three plugin types for different use cases
- **Security**: WASM provides strong sandboxing for untrusted code
- **Performance**: Native plugins for hot paths
- **Cross-language**: gRPC enables polyglot plugins
- **Type safety**: Strong interfaces at all levels

### Negative
- **Complexity**: Three different plugin systems to maintain
- **Learning curve**: Developers must understand which type to use
- **Debugging**: gRPC and WASM plugins harder to debug
- **Overhead**: gRPC has network overhead, WASM has runtime overhead

## Testing Strategy

```go
// Plugin testing framework
package plugins_test

func TestPluginLifecycle(t *testing.T) {
    ctx := context.Background()
    
    // Create mock plugin
    plugin := &mockPlugin{}
    
    // Test lifecycle
    t.Run("init", func(t *testing.T) {
        err := plugin.Init(ctx, map[string]any{"key": "value"})
        assert.NoError(t, err)
    })
    
    t.Run("start", func(t *testing.T) {
        err := plugin.Start(ctx)
        assert.NoError(t, err)
    })
    
    t.Run("health", func(t *testing.T) {
        status, err := plugin.Health(ctx)
        assert.NoError(t, err)
        assert.True(t, status.Healthy)
    })
    
    t.Run("stop", func(t *testing.T) {
        err := plugin.Stop(ctx)
        assert.NoError(t, err)
    })
}

func TestPluginRegistry(t *testing.T) {
    registry := plugins.NewRegistry()
    
    // Register factory
    factory := &mockFactory{}
    err := registry.Register(plugins.PluginTypeCache, factory)
    assert.NoError(t, err)
    
    // Create plugin
    ctx := context.Background()
    plugin, err := registry.Create(ctx, plugins.PluginTypeCache, nil)
    assert.NoError(t, err)
    assert.NotNil(t, plugin)
}
```

## References

- [HashiCorp go-plugin](https://github.com/hashicorp/go-plugin)
- [WASM Component Model](https://component-model.bytecodealliance.org/)
- [WIT (WASM Interface Types)](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)
- [Extism](https://extism.org/) - WASM plugin framework
- [SOTA Research: Cross-Language Contract Sharing](./SOTA-RESEARCH.md#cross-language-contract-sharing)

---

*This ADR will be updated as plugin system implementation progresses.*
