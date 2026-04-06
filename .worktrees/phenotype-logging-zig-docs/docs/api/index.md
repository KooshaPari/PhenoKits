# API Reference

Complete API documentation for phenotype-logging-zig.

## Module Structure

```zig
const logging = @import("phenotype-logging");

// Core types
const Level = logging.Level;
const Entry = logging.Entry;
const Logger = logging.Logger;

// Adapters
const FileAdapter = logging.adapters.file.FileAdapter;
const StderrAdapter = logging.adapters.stderr.StderrAdapter;

// Interface
const Transport = logging.transport.Transport;
const GenericLogger = logging.transport.GenericLogger;
```

## Core Types

### Level

Enumeration of log levels in ascending severity:

```zig
pub const Level = enum(u3) {
    trace = 0,
    debug = 1,
    info = 2,
    warn = 3,
    err = 4,
    fatal = 5,
};
```

**Methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `fromString` | `fn(s: []const u8) ?Level` | Parse level from string |
| `toString` | `fn(self) []const u8` | Convert to uppercase string |
| `isEnabled` | `fn(self, min: Level) bool` | Check if level >= minimum |

### Entry

Log entry with structured data:

```zig
pub const Entry = struct {
    level: Level,
    message: []const u8,
    timestamp: i64,
    fields: std.StringHashMapUnmanaged([]const u8),
};
```

**Methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `init` | `fn(level, message, allocator) !Entry` | Create new entry |
| `deinit` | `fn(*self, allocator) void` | Free resources |
| `addField` | `fn(*self, allocator, key, value) !void` | Add key-value field |

### Logger

Core logger implementation:

```zig
pub const Logger = struct {
    level: Level,
    writer: std.io.AnyWriter,
};
```

**Methods:**

| Method | Signature | Description |
|--------|-----------|-------------|
| `log` | `fn(*self, level, message, allocator) !void` | Log message |
| `logWithFields` | `fn(*self, level, message, fields, allocator) !void` | Log with structured data |
| `format` | `fn(*self, entry, allocator) !void` | Format to JSON |

## Adapters

See individual adapter documentation:

- [Stderr Adapter](/api/adapters/stderr) — Colored console output
- [File Adapter](/api/adapters/file) — JSON file output with rotation
- [Custom Adapters](/api/adapters/custom) — Implementing your own adapter

## Transport Interface

The comptime interface for pluggable transports:

```zig
pub const Transport = struct {
    /// Verify type implements interface at compile time
    pub fn check(comptime T: type) void

    /// Check if type is valid (returns bool)
    pub fn isValid(comptime T: type) bool
};

/// Generic logger that works with any transport
pub fn GenericLogger(comptime T: type) type
```

## Constants

### Colors (Stderr Adapter)

| Constant | ANSI Code |
|----------|-----------|
| `reset` | `\x1b[0m` |
| `black` | `\x1b[30m` |
| `red` | `\x1b[31m` |
| `green` | `\x1b[32m` |
| `yellow` | `\x1b[33m` |
| `blue` | `\x1b[34m` |
| `magenta` | `\x1b[35m` |
| `cyan` | `\x1b[36m` |
| `white` | `\x1b[37m` |
| `bold` | `\x1b[1m` |
| `dim` | `\x1b[2m` |

## Error Handling

All logging functions return `!void` and may produce:

- `error.OutOfMemory` — Allocation failure
- `error.FileNotFound` — File operation failed (file adapter)
- Various IO errors from underlying writers

## Performance

| Metric | Target | Achieved |
|--------|--------|----------|
| Init time | < 10ms | ✓ ~0ms |
| Memory | < 10MB | ✓ ~KB |
| Throughput | 10K ops/sec | ✓ 100K+ ops/sec |
| Zero allocation (filtered) | 0 bytes | ✓ Verified |

## See Also

- [Getting Started Guide](/guide/getting-started)
- [Examples](/examples/)
