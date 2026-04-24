# Getting Started

## Installation

### Using build.zig.zon

Add the dependency to your `build.zig.zon`:

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

### In build.zig

Import the module in your `build.zig`:

```zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "my-app",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Add phenotype-logging dependency
    const phenotype_logging = b.dependency("phenotype_logging", .{});
    exe.root_module.addImport("phenotype-logging", phenotype_logging.module("phenotype-logging"));

    b.installArtifact(exe);
}
```

## Basic Usage

### Stderr Adapter

The stderr adapter outputs colored, human-readable logs to standard error:

```zig
const std = @import("std");
const logging = @import("phenotype-logging");
const stderr_adapter = @import("phenotype-logging/adapters/stderr");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    // Create adapter with colors enabled
    var adapter = stderr_adapter.StderrAdapter.init(.{
        .use_color = true,
        .show_timestamp = true,
        .show_fields = true,
    });

    // Simple log
    try adapter.log(.info, "Application started");

    // Structured log with fields
    try adapter.logWithFields(.info, "Request completed", .{
        .method = "POST",
        .path = "/api/users",
        .status = 201,
        .duration_ms = 42,
    }, gpa.allocator());
}
```

Output:
```
2026-04-02 14:30:25 [ INFO] Application started
2026-04-02 14:30:25 [ INFO] Request completed {method="POST", path="/api/users", status="201", duration_ms="42"}
```

### File Adapter

The file adapter writes JSON logs with automatic rotation:

```zig
const std = @import("std");
const logging = @import("phenotype-logging");
const file_adapter = @import("phenotype-logging/adapters/file");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create file adapter with rotation
    var adapter = try file_adapter.FileAdapter.init(allocator, .{
        .base_path = "/var/log/myapp.log",
        .max_size = 10 * 1024 * 1024, // 10 MB
        .max_files = 5,
    });
    defer adapter.deinit();

    // Log entry
    var entry = try logging.Entry.init(.info, "Database connection established", allocator);
    defer entry.deinit(allocator);

    try entry.addField(allocator, "host", "localhost");
    try entry.addField(allocator, "port", "5432");

    try adapter.write(entry);
}
```

Output (in `/var/log/myapp.log`):
```json
{"level":"INFO","message":"Database connection established","timestamp":1712050225,"fields":{"host":"localhost","port":"5432"}}
```

## Configuration

### Stderr Adapter Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `use_color` | `bool` | `true` | Enable ANSI color codes |
| `show_timestamp` | `bool` | `true` | Include timestamp in output |
| `show_fields` | `bool` | `true` | Include structured fields |

### File Adapter Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `base_path` | `[]const u8` | (required) | Path to log file |
| `max_size` | `u64` | `10485760` (10MB) | Size threshold for rotation |
| `max_files` | `u8` | `5` | Number of rotated files to keep |

## Next Steps

- Learn about [custom adapters](/guide/custom-adapters)
- Explore [performance optimizations](/guide/performance)
- See more [examples](/examples/)
