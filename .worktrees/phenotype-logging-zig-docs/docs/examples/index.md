# Examples

Complete working examples for common use cases.

## Basic Logging

Simple logging to stderr:

```zig
const std = @import("std");
const logging = @import("phenotype-logging");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    // Simple colored output to stderr
    var adapter = logging.adapters.stderr.StderrAdapter.init(.{
        .use_color = true,
    });

    try adapter.log(.info, "Server starting...");
    try adapter.log(.debug, "Debug mode enabled");
    try adapter.log(.warn, "High memory usage detected");
    try adapter.log(.err, "Failed to connect to database");
}
```

## Structured Logging

Logging with key-value fields:

```zig
const std = @import("std");
const logging = @import("phenotype-logging");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var adapter = logging.adapters.stderr.StderrAdapter.init(.{});

    // HTTP request logging
    try adapter.logWithFields(.info, "HTTP request", .{
        .method = "GET",
        .path = "/api/v1/users",
        .status = 200,
        .duration_ms = 15,
        .request_id = "req-abc-123",
    }, allocator);

    // Error with context
    try adapter.logWithFields(.err, "Database query failed", .{
        .query = "SELECT * FROM users WHERE id = ?",
        .error_code = "ECONNREFUSED",
        .retry_count = 3,
    }, allocator);
}
```

## File Rotation

Writing logs to file with automatic rotation:

```zig
const std = @import("std");
const logging = @import("phenotype-logging");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize file adapter with 1MB rotation threshold
    var adapter = try logging.adapters.file.FileAdapter.init(allocator, .{
        .base_path = "app.log",
        .max_size = 1024 * 1024, // 1 MB
        .max_files = 3,
    });
    defer adapter.deinit();

    // Simulate many log entries
    var i: usize = 0;
    while (i < 10000) : (i += 1) {
        var entry = try logging.Entry.init(.info, "Processing item", allocator);
        defer entry.deinit(allocator);

        try entry.addField(allocator, "item_id", try std.fmt.allocPrint(allocator, "{d}", .{i}));
        try entry.addField(allocator, "status", "ok");

        try adapter.write(entry);
    }

    // Files created:
    // - app.log (current)
    // - app.log.1 (most recent rotated)
    // - app.log.2 (older)
    // - app.log.3 (oldest, will be deleted on next rotation)
}
```

## Custom Adapter

Implementing a custom transport adapter:

```zig
const std = @import("std");
const logging = @import("phenotype-logging");

// Custom adapter that sends logs to a network endpoint
const NetworkAdapter = struct {
    endpoint: []const u8,
    socket: ?std.net.Stream,

    pub fn init(endpoint: []const u8) NetworkAdapter {
        return .{
            .endpoint = endpoint,
            .socket = null,
        };
    }

    pub fn connect(self: *NetworkAdapter) !void {
        // Parse endpoint and connect
        _ = self;
    }

    pub fn disconnect(self: *NetworkAdapter) void {
        if (self.socket) |sock| {
            sock.close();
            self.socket = null;
        }
    }

    // Required by Transport interface
    pub fn write(self: *NetworkAdapter, entry: logging.Entry) !void {
        _ = self;
        _ = entry;
        // Serialize entry to JSON and send over network
        // Implementation omitted for brevity
    }
};

// Verify interface at compile time
comptime {
    logging.transport.Transport.check(NetworkAdapter);
}

pub fn main() !void {
    var adapter = NetworkAdapter.init("localhost:8080");
    try adapter.connect();
    defer adapter.disconnect();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    // Use with GenericLogger
    const Logger = logging.transport.GenericLogger(NetworkAdapter);
    var logger = Logger.init(.info, adapter);

    try logger.log(.info, "Connected to remote log server", gpa.allocator());
}
```

## Using GenericLogger

Working with the comptime interface:

```zig
const std = @import("std");
const logging = @import("phenotype-logging");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create any adapter that implements Transport
    var stderr_adapter = logging.adapters.stderr.StderrAdapter.init(.{});

    // Create generic logger with comptime adapter type
    const Logger = logging.transport.GenericLogger(logging.adapters.stderr.StderrAdapter);
    var logger = Logger.init(.info, stderr_adapter);

    // Log with automatic filtering
    try logger.log(.debug, "this is filtered", allocator); // Won't print
    try logger.log(.info, "this appears", allocator);     // Will print

    // With fields
    try logger.logWithFields(.warn, "Rate limit approaching", .{
        .current_rps = 950,
        .limit_rps = 1000,
    }, allocator);
}
```

## Build All Examples

To build and run examples:

```bash
# Build the library
zig build

# Run tests (includes inline examples)
zig build test

# Run a specific example
zig run examples/basic.zig
```
