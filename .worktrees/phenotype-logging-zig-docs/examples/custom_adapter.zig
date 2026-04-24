//! Custom Adapter Example
//!
//! Demonstrates implementing a custom transport adapter.

const std = @import("std");
const logging = @import("phenotype-logging");

/// In-memory adapter that stores logs for testing
const MemoryAdapter = struct {
    entry_count: usize,

    pub fn init() MemoryAdapter {
        return .{ .entry_count = 0 };
    }

    /// Required: Transport interface method
    pub fn write(self: *MemoryAdapter, entry: logging.Entry) !void {
        _ = entry;
        self.entry_count += 1;
    }

    /// Custom: Get entry count
    pub fn count(self: *MemoryAdapter) usize {
        return self.entry_count;
    }
};

// Verify interface at compile time
comptime {
    logging.transport.Transport.check(MemoryAdapter);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    // Create custom adapter
    var adapter = MemoryAdapter.init();

    // Create generic logger with our adapter
    const Logger = logging.transport.GenericLogger(MemoryAdapter);
    var logger = Logger.init(.info, adapter);

    // Log some messages
    try logger.log(.info, "First message", gpa.allocator());
    try logger.log(.info, "Second message", gpa.allocator());

    // Log with fields
    try logger.logWithFields(.warn, "Warning with context", .{
        .reason = "threshold_exceeded",
        .value = 95,
    }, gpa.allocator());

    // Verify logs were captured
    std.debug.print("Captured {d} log entries\n", .{adapter.count()});
}
