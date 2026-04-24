# Architecture — phenotype-cli-extensions

> CLI Extensions for the Phenotype ecosystem

**Version**: 1.0 | **Status**: Active | **Last Updated**: 2026-04-04

---

## Table of Contents

1. [Overview](#overview)
2. [System Architecture](#system-architecture)
3. [CLI Extension Architecture](#cli-extension-architecture)
4. [Plugin System Design](#plugin-system-design)
5. [Command Registration and Execution Flow](#command-registration-and-execution-flow)
6. [Configuration Management](#configuration-management)
7. [Module Structure](#module-structure)
8. [Extension Points](#extension-points)
9. [Error Handling Strategy](#error-handling-strategy)
10. [Platform Support](#platform-support)
11. [Performance Targets](#performance-targets)
12. [Security Considerations](#security-considerations)

---

## Overview

The phenotype-cli-extensions project provides CLI extension capabilities for the Phenotype ecosystem's helios-cli fork (derived from OpenAI Codex CLI). It implements a plugin-based architecture that allows dynamic loading of extensions at runtime without requiring recompilation of the host CLI.

### Goals

- Enable runtime extension loading with sandboxed module isolation
- Provide a stable contract (manifest schema) between host CLI and extensions
- Support multiple distribution channels (npm, GitHub Packages, local paths)
- Maintain fail-clearly error semantics

### Non-Goals

- WASM-based plugin isolation (overhead too high for CLI use cases)
- Subprocess IPC for command dispatch (too slow)
- Custom VM implementation

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Host CLI (helios-cli)                     │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Command   │  │  Extension  │  │    Configuration        │  │
│  │  Registry   │  │   Loader    │  │      Manager            │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                    Plugin System Layer                           │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │              phenotype-ext.json Manifest Schema             │  │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌────────────┐  │  │
│  │  │   name    │ │  version  │ │apiVersion │ │capabilities│  │  │
│  │  └───────────┘ └───────────┘ └───────────┘ └────────────┘  │  │
│  └─────────────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│  ┌───────────────┐  ┌───────────────┐  ┌────────────────────┐   │
│  │shell-tool-mcp│  │sdk-typescript  │  │  kitty-specs/      │   │
│  │               │  │               │  │  (AgilePlus specs) │   │
│  └───────────────┘  └───────────────┘  └────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Extension Ecosystem                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   npm/      │  │  GitHub     │  │   Local      │             │
│  │  Registry   │  │  Packages   │  │  file://     │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
└─────────────────────────────────────────────────────────────────┘
```

---

## CLI Extension Architecture

### Core Principles

1. **Dynamic Module Loading**: Extensions are loaded at runtime using Node.js `require` (CommonJS) or ESM dynamic `import()` without recompiling the host CLI.

2. **Sandboxed Module Registry**: Each extension operates within a sandboxed module registry that provides isolation between extensions and the host process.

3. **Manifest-Driven Contract**: Extensions declare their capabilities and requirements via a `phenotype-ext.json` manifest file.

4. **Fail-Clearly Policy**: Any extension load failure throws immediately with full error context; the CLI exits non-zero when required extensions fail.

### Extension Lifecycle

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Discover   │────▶│   Validate   │────▶│     Load     │────▶│   Initialize  │
└──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘
      │                    │                    │                    │
      ▼                    ▼                    ▼                    ▼
  Find extensions    Verify manifest    Import module      Call init()
  in extension       schema, API        with sandbox      with config
  path directories   version compat     isolation
```

### Manifest Schema (phenotype-ext.json)

```typescript
interface ExtensionManifest {
  name: string;           // Unique extension identifier
  version: string;        // Semantic version
  apiVersion: string;     // Phenotype API version (for forward compat)
  entry: string;          // Path to main module entry point
  capabilities: string[]; // List of extension capabilities
  permissions: string[];  // Required permissions (e.g., "file:read")
  dependencies?: Record<string, string>; // Extension dependencies
}
```

---

## Plugin System Design

### Architecture

The plugin system consists of three main components:

#### 1. Extension Loader (`ExtensionLoader`)

Responsible for discovering, validating, and loading extensions.

```typescript
class ExtensionLoader {
  private manifestSchema: ManifestSchema;
  private moduleRegistry: ModuleRegistry;
  private errorHandler: ErrorHandler;

  async load(extensionPath: string): Promise<LoadedExtension>;
  async loadAll(extensionPaths: string[]): Promise<LoadedExtension[]>;
  async validateManifest(manifest: unknown): Promise<ExtensionManifest>;
  async importModule(entryPoint: string): Promise<ExtensionModule>;
}
```

#### 2. Module Registry (`ModuleRegistry`)

Provides sandboxed module resolution and caching.

```typescript
class ModuleRegistry {
  private cache: Map<string, LoadedModule>;
  private sandboxPolicy: SandboxPolicy;

  async resolve(moduleId: string, context: LoadContext): Promise<LoadedModule>;
  async preload(moduleIds: string[]): Promise<void>;
  invalidate(moduleId: string): void;
  clear(): void;
}
```

#### 3. Extension Manager (`ExtensionManager`)

Orchestrates extension lifecycle and provides the public API.

```typescript
class ExtensionManager {
  private loader: ExtensionLoader;
  private registry: ModuleRegistry;
  private configManager: ConfigManager;

  async install(extensionId: string, source: ExtensionSource): Promise<void>;
  async uninstall(extensionId: string): Promise<void>;
  async enable(extensionId: string): Promise<void>;
  async disable(extensionId: string): Promise<void>;
  async list(): Promise<ExtensionInfo[]>;
  async get(extensionId: string): Promise<LoadedExtension | null>;
}
```

### Plugin Types

The system supports three extension types:

| Type | Description | Use Case |
|------|-------------|----------|
| `command` | Adds new CLI commands | `pheno deploy`, `pheno init` |
| `middleware` | Intercepts and transforms operations | Logging, caching, auth |
| `provider` | Supplies external services | Cloud integrations, APIs |

### Capability Declaration

Extensions declare capabilities in their manifest:

```json
{
  "name": "my-extension",
  "version": "1.0.0",
  "apiVersion": "1.0",
  "entry": "./dist/index.js",
  "capabilities": [
    "command:deploy",
    "middleware:auth",
    "provider:cloud-storage"
  ],
  "permissions": [
    "file:read",
    "file:write",
    "net:http"
  ]
}
```

---

## Command Registration and Execution Flow

### Registration Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    Command Registration Flow                      │
└─────────────────────────────────────────────────────────────────┘

1. CLI Startup
   │
   ▼
2. Extension Discovery
   │  - Scan extension directories
   │  - Load phenotype-ext.json manifests
   │  - Validate API version compatibility
   │
   ▼
3. Manifest Validation
   │  - JSON Schema validation
   │  - Required fields check (name, version, entry)
   │  - Semantic version compatibility
   │
   ▼
4. Module Loading
   │  - Dynamic import of entry module
   │  - Sandbox execution context
   │  - Dependency resolution
   │
   ▼
5. Command Registration
   │  - Call extension's register() hook
   │  - Register commands with CLI router
   │  - Register middlewares with pipeline
   │
   ▼
6. CLI Ready
   - Commands available in CLI
   - Middlewares wired in execution pipeline
```

### Execution Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    Command Execution Flow                         │
└─────────────────────────────────────────────────────────────────┘

User Input: pheno deploy --target production

1. CLI Parser
   │  - Parse command line arguments
   │  - Resolve command: "deploy"
   │
   ▼
2. Middleware Pipeline (in order)
   │
   ▼ [+Auth Middleware]
   │  - Check authentication
   │  - Validate permissions
   │
   ▼ [+Logging Middleware]
   │  - Log command invocation
   │  - Capture timing metadata
   │
   ▼ [+Cache Middleware]
   │  - Check cache for repeated ops
   │  - Return cached if hit
   │
   ▼
3. Command Handler
   │  - Dispatch to extension's handler
   │  - Pass parsed arguments
   │  - Handle async operations
   │
   ▼
4. Result Processing
   │  - Transform output format
   │  - Apply formatting rules
   │
   ▼
5. Error Handling (if failure)
   │  - Catch extension errors
   │  - Apply error policy (fail-clearly)
   │  - Surface full error to user
   │
   ▼
6. CLI Exit
   - Exit code: 0 (success) or non-zero (failure)
```

### Command Handler Interface

```typescript
interface CommandHandler {
  name: string;
  description: string;
  args: ArgumentSchema[];
  options: OptionSchema[];
  
  execute(ctx: CommandContext): Promise<CommandResult>;
}

interface CommandContext {
  args: Record<string, unknown>;
  options: Record<string, unknown>;
  config: ConfigManager;
  logger: Logger;
  env: Environment;
}

interface CommandResult {
  success: boolean;
  data?: unknown;
  error?: CommandError;
  metadata?: {
    durationMs: number;
    cacheHit?: boolean;
  };
}
```

---

## Configuration Management

### Configuration Hierarchy

Configuration is resolved following this precedence (highest to lowest):

```
1. Command-line flags / options
   │
   ▼
2. Environment variables (PHENO_* prefix)
   │
   ▼
3. Project-level config (.phenotyperc, phenotype.config.ts)
   │
   ▼
4. User-level config (~/.phenotype/config)
   │
   ▼
5. Default values in extension manifests
```

### Config Schema

```typescript
interface PhenotypeConfig {
  // Core settings
  version: string;                    // Config format version
  apiVersion: string;                 // API version requirement
  
  // Extension settings
  extensions: {
    enabled: string[];                // Enabled extension IDs
    disabled: string[];              // Disabled extension IDs
    paths: string[];                 // Additional extension search paths
    autoUpdate: boolean;              // Auto-update extensions
  };
  
  // Execution settings
  execution: {
    timeout: number;                 // Default command timeout (ms)
    retries: number;                 // Retry count on failure
    parallel: boolean;               // Allow parallel execution
    sandboxPolicy: SandboxLevel;     // Isolation level
  };
  
  // Logging settings
  logging: {
    level: LogLevel;                 // Log verbosity
    format: LogFormat;               // Output format
    destinations: string[];          // Log destinations
  };
  
  // Platform settings
  platform: {
    os: OSOverride;                  // OS override (for testing)
    arch: ArchOverride;             // Architecture override
    bashVariant: string;             // Preferred Bash variant
  };
}
```

### Config Resolution

```typescript
class ConfigManager {
  private sources: ConfigSource[];
  private cache: Map<string, ResolvedValue>;
  
  async resolve<T>(key: string, defaultValue?: T): Promise<T>;
  async resolveAll(): Promise<PhenotypeConfig>;
  async set(key: string, value: unknown): Promise<void>;
  async merge(partial: Partial<PhenotypeConfig>): Promise<void>;
  
  // Environment variable binding
  bindEnv(key: string, envVar: string): void;
  
  // Validation
  validate(config: unknown): ValidationResult;
}
```

### Extension Configuration

Extensions receive configuration via the `init()` hook:

```typescript
interface ExtensionModule {
  // Called once during extension load
  init?(config: ExtensionConfig): Promise<void> | void;
  
  // Called when config changes
  onConfigChange?(changes: ConfigChanges): void;
  
  // Called before unloading
  destroy?(): void | Promise<void>;
}

interface ExtensionConfig {
  // Extension-specific config merged from all sources
  [key: string]: unknown;
  
  // Built-in context
  __context__: {
    extensionId: string;
    extensionPath: string;
    apiVersion: string;
    platform: PlatformInfo;
  };
}
```

---

## Module Structure

```
phenotype-cli-extensions/
├── src/
│   ├── shell-tool-mcp/           # Shell tool MCP server
│   │   ├── src/
│   │   │   ├── index.ts          # Entry point
│   │   │   ├── platform.ts       # Target triple resolution
│   │   │   ├── bashSelection.ts  # Bash variant selection
│   │   │   ├── osRelease.ts      # Linux OS release detection
│   │   │   ├── constants.ts      # Platform constants
│   │   │   └── types.ts          # Type definitions
│   │   ├── vendor/               # Bundled Bash binaries
│   │   │   └── {target-triple}/
│   │   │       └── bash/
│   │   │           ├── debian/
│   │   │           ├── alpine/
│   │   │           └── default/
│   │   └── package.json
│   │
│   ├── sdk-typescript/           # TypeScript SDK additions
│   │   ├── exec.ts               # CodexExec class
│   │   ├── responsesProxy.ts      # SSE test proxy
│   │   ├── codexOptions.ts       # Config types
│   │   └── threadOptions.ts      # Thread options
│   │
│   └── kitty-specs/               # AgilePlus specs
│       ├── 001-codex-tui-renderer-optimization/
│       └── 002-phenotype-modular-arch/
│
├── docs/                          # Documentation
│   ├── index.md
│   ├── journeys/
│   ├── stories/
│   └── traceability/
│
├── tests/                         # Test suite
│
├── docs/                          # VitePress documentation
│   └── .vitepress/
│       └── config.ts
│
├── package.json                   # Root package (docs)
├── SPEC.md                        # Specification
├── ARCHITECTURE.md                # This document
├── ADR.md                         # Architecture Decision Records
├── SOTA.md                        # State of the Art
└── CLAUDE.md                      # Development guidelines
```

---

## Extension Points

### 1. Command Extension Point

```typescript
// Extension registers commands via register()
export function register(cli: CliInstance): void {
  cli.commands.register({
    name: 'deploy',
    description: 'Deploy to target environment',
    args: [
      { name: 'target', type: 'string', required: true }
    ],
    options: [
      { name: 'force', type: 'boolean', short: 'f' }
    ],
    handler: async (ctx) => {
      // Implementation
    }
  });
}
```

### 2. Middleware Extension Point

```typescript
// Extension provides middleware
export function registerMiddlewares(pipeline: MiddlewarePipeline): void {
  pipeline.use(authMiddleware);
  pipeline.use(loggingMiddleware);
  pipeline.use(cacheMiddleware);
}
```

### 3. Provider Extension Point

```typescript
// Extension provides external service integration
export function registerProviders(registry: ProviderRegistry): void {
  registry.register('cloud-storage', new CloudStorageProvider(config));
  registry.register('auth', new AuthProvider(config));
}
```

---

## Error Handling Strategy

### Error Policy (ADR-004)

Per the architecture decision record, the system follows a **fail-clearly** policy:

1. **Load Failures**: Any extension load failure throws immediately with full context
2. **Required Extensions**: CLI exits non-zero when required extensions fail
3. **Optional Extensions**: Warning logged, CLI continues
4. **Runtime Errors**: Full error surfaced to user with stack trace

### Error Categories

```typescript
enum ErrorCategory {
  MANIFEST_INVALID = 'MANIFEST_INVALID',        // phenotype-ext.json validation failed
  MANIFEST_MISSING = 'MANIFEST_MISSING',        // No manifest found
  API_VERSION_INCOMPATIBLE = 'API_INCOMPATIBLE', // API version mismatch
  MODULE_NOT_FOUND = 'MODULE_NOT_FOUND',        // Entry module cannot be loaded
  DEPENDENCY_MISSING = 'DEPENDENCY_MISSING',    // Required dep not installed
  PERMISSION_DENIED = 'PERMISSION_DENIED',      // Insufficient permissions
  INITIALIZATION_FAILED = 'INIT_FAILED',        // Extension init() threw
  COMMAND_FAILED = 'COMMAND_FAILED',            // Command handler threw
}
```

### Error Shape

```typescript
interface ExtensionError {
  category: ErrorCategory;
  extensionId: string;
  message: string;
  cause?: Error;
  context?: Record<string, unknown>;
  timestamp: string;
  stack?: string;
}
```

---

## Platform Support

### Supported Platforms

| Platform | Architecture | Bash Variants |
|----------|-------------|---------------|
| Linux (musl) | x86_64, aarch64 | debian, alpine, default |
| macOS | x86_64, arm64 | (system Bash) |
| Windows | x86_64, arm64 | (via WSL) |

### Target Triple Resolution

```typescript
function resolveTargetTriple(platform: NodeJS.Platform, arch: NodeJS.Architecture): string {
  switch (platform) {
    case 'linux':
      return arch === 'x64' 
        ? 'x86_64-unknown-linux-musl'
        : 'aarch64-unknown-linux-musl';
    case 'darwin':
      return arch === 'x64'
        ? 'x86_64-apple-darwin'
        : 'aarch64-apple-darwin';
    default:
      throw new Error(`Unsupported platform: ${platform} (${arch})`);
  }
}
```

### Bash Selection Strategy

- **Linux**: OS release detection via `/etc/os-release`; variant selection based on ID and version ID
- **macOS**: Darwin major version comparison against variant minimums

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Extension load time | < 50ms per extension | Cold load, no cache |
| Command invocation overhead | < 5ms | Excluding command execution |
| Memory per extension | < 5MB | Baseline + extension heap |
| Concurrent extensions | 10+ | Without performance degradation |
| Manifest validation | < 1ms | Per manifest file |

---

## Security Considerations

### Sandbox Policy

Extensions run in a sandboxed context with:

- **File Access**: Controlled via permissions array in manifest
- **Network Access**: Explicit opt-in per extension
- **Environment Variables**: Filtered, allowlist-based
- **Module Access**: Scoped to declared dependencies

### Permission Levels

```typescript
enum Permission {
  FILE_READ = 'file:read',
  FILE_WRITE = 'file:write',
  NET_HTTP = 'net:http',
  NET_UV = 'net:uv',
  ENV_READ = 'env:read',
  ENV_WRITE = 'env:write',
  SPAWN_PROCESS = 'spawn:process',
}
```

### Trust Model

1. **npm Registry**: Extensions signed, verified via package integrity
2. **GitHub Packages**: Verified origin, org membership required
3. **Local Path**: Developer mode only, warned on install

---

## Related Documentation

- [SPEC.md](./SPEC.md) — Project specification
- [ADR.md](./ADR.md) — Architecture decision records
- [SOTA.md](./SOTA.md) — State of the art analysis
- [FORK_MAINTENANCE.md](./FORK_MAINTENANCE.md) — Fork sync and maintenance
- [UPSTREAM_SYNC.md](./UPSTREAM_SYNC.md) — Upstream merge procedures
