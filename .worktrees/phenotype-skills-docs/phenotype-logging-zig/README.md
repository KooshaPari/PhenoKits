# phenotype-logging-zig

Zig structured logging with minimal allocations. Zero-copy logging for performance-critical applications.

## Features

- Structured logging (JSON)
- Level-based filtering
- Minimal allocations
- Async-safe
- Custom formatters
- Multiple outputs (stderr, file, syslog)

## Installation

Add to `build.zig.zon`:

```zig
.phenotype_logging = .{
    .url = "https://github.com/KooshaPari/phenotype-logging-zig",
    .hash = "<commit-hash>",
},
```

## Usage

### Basic Logging

```zig
const log = @import("phenotype_logging");

pub fn main() !void {
    try log.init(.{
        .level = .info,
        .format = .json,
    });
    defer log.deinit();

    log.info("Server started", .{
        .port = 8080,
        .host = "localhost",
    });

    log.err("Connection failed", .{
        .error = @errorName(err),
    });
}
```

### Structured Fields

```zig
log.info("Request processed", .{
    .method = "GET",
    .path = "/api/users",
    .status = 200,
    .duration_ms = 42,
});
```

Output:
```json
{"level":"info","ts":1699999999,"msg":"Request processed","method":"GET","path":"/api/users","status":200,"duration_ms":42}
```

## Architecture

```zig
src/
├── logger.zig         # Core logger
├── level.zig         # Log levels
├── formatter.zig     # JSON/Text formatters
├── writer.zig        # Output writers
└── async.zig         # Async support
```

## Performance

- < 1KB heap allocation per log call
- Lock-free for single-threaded
- Batch writes for high-throughput

## License

MIT
