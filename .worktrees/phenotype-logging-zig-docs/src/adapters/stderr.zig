//! Stderr Adapter with ANSI Colors
//!
//! Implements FR-LOG-007: Stderr Adapter Default
//! Writes colorised human-readable output to stderr with ANSI color codes.

const std = @import("std");
const lib = @import("../lib.zig");

/// ANSI color codes for terminal output
pub const Colors = struct {
    pub const reset = "\x1b[0m";
    pub const black = "\x1b[30m";
    pub const red = "\x1b[31m";
    pub const green = "\x1b[32m";
    pub const yellow = "\x1b[33m";
    pub const blue = "\x1b[34m";
    pub const magenta = "\x1b[35m";
    pub const cyan = "\x1b[36m";
    pub const white = "\x1b[37m";
    pub const bold = "\x1b[1m";
    pub const dim = "\x1b[2m";

    /// Get color for log level
    pub fn forLevel(level: lib.Level) []const u8 {
        return switch (level) {
            .trace => dim ++ white,
            .debug => cyan,
            .info => green,
            .warn => yellow,
            .err => red,
            .fatal => bold ++ red,
        };
    }
};

/// Configuration for stderr output
pub const StderrConfig = struct {
    /// Enable ANSI color codes
    use_color: bool = true,
    /// Include timestamp in output
    show_timestamp: bool = true,
    /// Include structured fields
    show_fields: bool = true,
    /// Timestamp format (RFC 3339 style)
    timestamp_format: []const u8 = "{d:0>4}-{d:0>2}-{d:0>2} {d:0>2}:{d:0>2}:{d:0>2}",
};

/// Stderr adapter for human-readable colored output
pub const StderrAdapter = struct {
    config: StderrConfig,
    stderr: std.fs.File,
    mutex: std.Thread.Mutex,

    /// Initialize with default stderr
    pub fn init(config: StderrConfig) StderrAdapter {
        return StderrAdapter{
            .config = config,
            .stderr = std.io.getStdErr(),
            .mutex = .{},
        };
    }

    /// Initialize with custom writer (for testing)
    pub fn initWithWriter(config: StderrConfig, file: std.fs.File) StderrAdapter {
        return StderrAdapter{
            .config = config,
            .stderr = file,
            .mutex = .{},
        };
    }

    /// Write a log entry to stderr (implements Transport interface)
    pub fn write(self: *StderrAdapter, entry: lib.Entry) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        const writer = self.stderr.writer();

        // Format timestamp
        if (self.config.show_timestamp) {
            const ts = entry.timestamp;
            const epoch_seconds = std.time.epoch.EpochSeconds{ .secs = @intCast(ts) };
            const ymd = epoch_seconds.getEpochDay().calculateYearMonthDay();
            const day_seconds = epoch_seconds.getDaySeconds();

            if (self.config.use_color) {
                try writer.print(Colors.dim, .{});
            }
            try writer.print(self.config.timestamp_format, .{
                ymd.year,      ymd.month,          ymd.day,
                day_seconds.getHoursIntoDay(),
                day_seconds.getMinutesIntoHour(),
                day_seconds.getSecondsIntoMinute(),
            });
            try writer.writeAll(" ");
        }

        // Format level with color
        if (self.config.use_color) {
            const level_color = Colors.forLevel(entry.level);
            try writer.print("{s}[{s:>5}]{s} ", .{ level_color, entry.level.toString(), Colors.reset });
        } else {
            try writer.print("[{s:>5}] ", .{entry.level.toString()});
        }

        // Format message
        try writer.print("{s}", .{entry.message});

        // Format fields
        if (self.config.show_fields and entry.fields.count() > 0) {
            try writer.writeAll(" {");
            var it = entry.fields.iterator();
            var first = true;
            while (it.next()) |kv| {
                if (!first) try writer.writeAll(", ");
                first = false;

                if (self.config.use_color) {
                    try writer.print("{s}{s}{s}={s}\"{s}\"{s}", .{
                        Colors.cyan,    kv.key_ptr.*,
                        Colors.reset,   Colors.yellow,
                        kv.value_ptr.*, Colors.reset,
                    });
                } else {
                    try writer.print("{s}=\"{s}\"", .{ kv.key_ptr.*, kv.value_ptr.* });
                }
            }
            try writer.writeAll("}");
        }

        try writer.writeAll("\n");
    }

    /// Log a message directly (convenience method)
    pub fn log(self: *StderrAdapter, level: lib.Level, message: []const u8) !void {
        var entry = try lib.Entry.init(level, message, std.heap.page_allocator);
        defer entry.deinit(std.heap.page_allocator);
        try self.write(entry);
    }

    /// Log with structured fields (convenience method)
    pub fn logWithFields(self: *StderrAdapter, level: lib.Level, message: []const u8, fields: anytype, allocator: std.mem.Allocator) !void {
        var entry = try lib.Entry.init(level, message, allocator);
        defer entry.deinit(allocator);

        inline for (std.meta.fields(@TypeOf(fields))) |f| {
            const value = @field(fields, f.name);
            const value_str = try std.fmt.allocPrint(allocator, "{s}", .{value});
            defer allocator.free(value_str);
            try entry.addField(allocator, f.name, value_str);
        }

        try self.write(entry);
    }
};

// ============================================================================
// TDD Tests for Stderr Adapter
// ============================================================================

test "StderrAdapter.init creates valid adapter" {
    const adapter = StderrAdapter.init(.{
        .use_color = false,
        .show_timestamp = false,
    });
    _ = adapter;
}

test "StderrAdapter.write outputs message without color" {
    const allocator = std.testing.allocator;

    // Create temp file to capture output
    const test_path = "/tmp/test_stderr.log";
    defer std.fs.cwd().deleteFile(test_path) catch {};

    const file = try std.fs.cwd().createFile(test_path, .{});
    defer file.close();

    var adapter = StderrAdapter.initWithWriter(.{
        .use_color = false,
        .show_timestamp = false,
        .show_fields = false,
    }, file);

    var entry = try lib.Entry.init(.info, "hello world", allocator);
    defer entry.deinit(allocator);

    try adapter.write(entry);

    // Read and verify output
    const content = try std.fs.cwd().readFileAlloc(allocator, test_path, 4096);
    defer allocator.free(content);

    try std.testing.expect(std.mem.indexOf(u8, content, "hello world") != null);
    try std.testing.expect(std.mem.indexOf(u8, content, "INFO") != null);
}

test "StderrAdapter.write includes fields when enabled" {
    const allocator = std.testing.allocator;

    const test_path = "/tmp/test_stderr_fields.log";
    defer std.fs.cwd().deleteFile(test_path) catch {};

    const file = try std.fs.cwd().createFile(test_path, .{});
    defer file.close();

    var adapter = StderrAdapter.initWithWriter(.{
        .use_color = false,
        .show_timestamp = false,
        .show_fields = true,
    }, file);

    var entry = try lib.Entry.init(.warn, "request processed", allocator);
    defer entry.deinit(allocator);

    try entry.addField(allocator, "request_id", "abc-123");
    try entry.addField(allocator, "duration_ms", "42");

    try adapter.write(entry);

    const content = try std.fs.cwd().readFileAlloc(allocator, test_path, 4096);
    defer allocator.free(content);

    try std.testing.expect(std.mem.indexOf(u8, content, "request_id") != null);
    try std.testing.expect(std.mem.indexOf(u8, content, "abc-123") != null);
    try std.testing.expect(std.mem.indexOf(u8, content, "duration_ms") != null);
}

test "Colors.forLevel returns appropriate colors" {
    try std.testing.expectEqualStrings(Colors.dim ++ Colors.white, Colors.forLevel(.trace));
    try std.testing.expectEqualStrings(Colors.cyan, Colors.forLevel(.debug));
    try std.testing.expectEqualStrings(Colors.green, Colors.forLevel(.info));
    try std.testing.expectEqualStrings(Colors.yellow, Colors.forLevel(.warn));
    try std.testing.expectEqualStrings(Colors.red, Colors.forLevel(.err));
    try std.testing.expectEqualStrings(Colors.bold ++ Colors.red, Colors.forLevel(.fatal));
}

test "BDD: Given color enabled, When logging ERROR, Then output contains ANSI red code" {
    const allocator = std.testing.allocator;

    const test_path = "/tmp/test_color.log";
    defer std.fs.cwd().deleteFile(test_path) catch {};

    const file = try std.fs.cwd().createFile(test_path, .{});
    defer file.close();

    // Given: color enabled
    var adapter = StderrAdapter.initWithWriter(.{
        .use_color = true,
        .show_timestamp = false,
        .show_fields = false,
    }, file);

    // When: logging ERROR
    var entry = try lib.Entry.init(.err, "something failed", allocator);
    defer entry.deinit(allocator);

    try adapter.write(entry);

    // Then: output contains red ANSI code
    const content = try std.fs.cwd().readFileAlloc(allocator, test_path, 4096);
    defer allocator.free(content);

    try std.testing.expect(std.mem.indexOf(u8, content, Colors.red) != null);
    try std.testing.expect(std.mem.indexOf(u8, content, "ERROR") != null);
}
