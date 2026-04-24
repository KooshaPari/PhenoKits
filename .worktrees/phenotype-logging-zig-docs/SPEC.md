# Phenotype Logging Zig Specification

> Logging Library

**Version**: 1.0 | **Status**: Draft | **Last Updated**: 2026-04-02

## Overview

Logging Library for the Phenotype ecosystem.

**Language**: Zig

**Dependencies**: None (pure Zig standard library)

**Key Features**:
- Structured logging with zero-allocation hot path
- Compile-time log level filtering
- JSON and ANSI-colored output formats
- File adapter with automatic rotation
- Pluggable transport adapter interface

## Architecture

```
phenotype-logging-zig/
├── src/
│   ├── lib.zig           # Core types and Logger
│   ├── adapters/
│   │   ├── stderr.zig    # ANSI-colored stderr output
│   │   └── file.zig      # File output with rotation
│   └── interface.zig     # Comptime transport interface
├── tests/                # Integration tests
├── docs/                 # VitePress documentation
├── examples/             # Usage examples
└── build.zig            # Zig build configuration
```

## Quick Start

```bash
# Add as a dependency in build.zig.zon
.{
    .name = "my-project",
    .version = "0.1.0",
    .dependencies = .{
        .phenotype_logging = .{
            .url = "https://github.com/KooshaPari/phenotype-logging-zig/archive/refs/heads/main.tar.gz",
        },
    },
}

# In your build.zig
const phenotype_logging = b.dependency("phenotype_logging", .{});
exe.root_module.addImport("phenotype-logging", phenotype_logging.module("phenotype-logging"));
```

## Usage

See the `examples/` directory for complete usage examples.

## Performance Targets

| Metric | Target |
|--------|--------|
| Init time | < 10ms |
| Memory | < 10MB |
| Throughput | 10K ops/sec |

## License

MIT
