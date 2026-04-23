# Phenotype Logging Zig

Structured logging library for Zig with minimal allocations.

## Features

- **Zero-allocation hot path** — Log calls below active level generate no machine code in `ReleaseFast` builds
- **Compile-time level filtering** — Filter at compile time, pay no runtime cost for disabled levels
- **Structured key-value fields** — Pass anonymous structs with fields that serialize to JSON
- **Multiple output formats** — JSON for machines, ANSI-colored text for humans
- **File rotation** — Automatic rotation when file size exceeds threshold (default 10MB)
- **Pluggable adapters** — Swap transports without touching call sites via comptime interface

## Quick Start

```zig
const std = @import("std");
const logging = @import("phenotype-logging");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create a stderr adapter with colors
    var adapter = logging.adapters.stderr.StderrAdapter.init(.{
        .use_color = true,
    });

    // Log with structured fields
    try adapter.logWithFields(.info, "Request processed", .{
        .method = "GET",
        .path = "/api/users",
        .duration_ms = 42,
    }, allocator);
}
```

## Installation

Add to your `build.zig.zon`:

```zig
.{
    .name = "my-project",
    .version = "0.1.0",
    .dependencies = .{
        .phenotype_logging = .{
            .url = "https://github.com/KooshaPari/phenotype-logging-zig/archive/refs/heads/main.tar.gz",
        },
    },
}
```

Then in `build.zig`:

```zig
const phenotype_logging = b.dependency("phenotype_logging", .{});
exe.root_module.addImport("phenotype-logging", phenotype_logging.module("phenotype-logging"));
```

## Documentation

- [Getting Started](/guide/getting-started) — Installation and basic usage
- [API Reference](/api/) — Complete API documentation
- [Examples](/examples/) — Code examples for common use cases

## Architecture

This library follows hexagonal architecture principles:

- **Domain** (`lib.zig`): Core types (Level, Entry, Logger)
- **Ports** (`interface.zig`): Comptime Transport interface
- **Adapters** (`adapters/`): Concrete implementations (stderr, file)

## Status

**Archived** — This repository has been migrated to `libs/logging-zig` as a neutral Zig logging library. See [ARCHIVED.md](https://github.com/KooshaPari/phenotype-logging-zig/blob/main/ARCHIVED.md) for migration details.

## License

MIT
