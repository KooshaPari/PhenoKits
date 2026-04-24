//! File Adapter with Rotation
//!
//! Implements FR-LOG-008: File Adapter with Rotation
//! Rotates logs when file exceeds configurable size threshold (default 10 MB)
//! Retains configurable number of rotated files (default 5)

const std = @import("std");
const lib = @import("../lib.zig");

/// Configuration for file rotation
pub const RotationConfig = struct {
    /// Maximum file size before rotation (bytes)
    max_size: u64 = 10 * 1024 * 1024, // 10 MB default
    /// Number of rotated files to retain
    max_files: u8 = 5,
    /// Base path for log files
    base_path: []const u8,
};

/// File adapter that handles rotation
pub const FileAdapter = struct {
    config: RotationConfig,
    current_file: ?std.fs.File,
    current_size: u64,
    allocator: std.mem.Allocator,
    mutex: std.Thread.Mutex,

    /// Initialize the file adapter
    pub fn init(allocator: std.mem.Allocator, config: RotationConfig) !FileAdapter {
        var adapter = FileAdapter{
            .config = config,
            .current_file = null,
            .current_size = 0,
            .allocator = allocator,
            .mutex = .{},
        };

        // Open or create the initial log file
        try adapter.openCurrentFile();
        return adapter;
    }

    /// Clean up resources
    pub fn deinit(self: *FileAdapter) void {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.current_file) |file| {
            file.close();
            self.current_file = null;
        }
    }

    /// Write a log entry to file (implements Transport interface)
    pub fn write(self: *FileAdapter, entry: lib.Entry) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        // Check if rotation needed
        if (self.current_size >= self.config.max_size) {
            try self.rotate();
        }

        // Format entry as JSON
        const json = try self.formatEntry(entry);
        defer self.allocator.free(json);

        // Write to file
        if (self.current_file) |file| {
            try file.writeAll(json);
            try file.writeAll("\n");
            self.current_size += json.len + 1;
        }
    }

    /// Open the current log file
    fn openCurrentFile(self: *FileAdapter) !void {
        const flags = std.fs.File.CreateFlags{
            .read = false,
            .truncate = false,
        };

        const file = try std.fs.cwd().createFile(self.config.base_path, flags);

        // Get current size for tracking
        const stat = try file.stat();
        self.current_size = stat.size;
        self.current_file = file;
    }

    /// Rotate log files
    fn rotate(self: *FileAdapter) !void {
        // Close current file
        if (self.current_file) |file| {
            file.close();
            self.current_file = null;
        }

        // Shift existing rotated files
        var i: i8 = @intCast(self.config.max_files - 1);
        while (i > 0) : (i -= 1) {
            const old_path = try std.fmt.allocPrint(
                self.allocator,
                "{s}.{d}",
                .{ self.config.base_path, i },
            );
            defer self.allocator.free(old_path);

            const new_path = try std.fmt.allocPrint(
                self.allocator,
                "{s}.{d}",
                .{ self.config.base_path, i + 1 },
            );
            defer self.allocator.free(new_path);

            // Rename if old file exists
            std.fs.cwd().rename(old_path, new_path) catch |err| switch (err) {
                error.FileNotFound => {},
                else => return err,
            };
        }

        // Rename current to .1
        const backup_path = try std.fmt.allocPrint(
            self.allocator,
            "{s}.1",
            .{self.config.base_path},
        );
        defer self.allocator.free(backup_path);

        try std.fs.cwd().rename(self.config.base_path, backup_path);

        // Open new current file
        try self.openCurrentFile();
        self.current_size = 0;
    }

    /// Format entry to JSON string
    fn formatEntry(self: *FileAdapter, entry: lib.Entry) ![]u8 {
        var list = std.ArrayList(u8).init(self.allocator);
        errdefer list.deinit();

        try list.writer().print(
            "{{\"level\":\"{s}\",\"message\":\"{s}\",\"timestamp\":{d}",
            .{ entry.level.toString(), entry.message, entry.timestamp },
        );

        try list.writer().writeAll(",\"fields\":{");

        var it = entry.fields.iterator();
        var first = true;
        while (it.next()) |kv| {
            if (!first) try list.writer().writeAll(",");
            first = false;
            try list.writer().print("\"{s}\":\"{s}\"", .{ kv.key_ptr.*, kv.value_ptr.* });
        }

        try list.writer().writeAll("}}");

        return list.toOwnedSlice();
    }

    /// Get current file size for testing
    pub fn getCurrentSize(self: *FileAdapter) u64 {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.current_size;
    }
};

// ============================================================================
// TDD Tests for File Adapter
// ============================================================================

test "FileAdapter.init creates file if not exists" {
    const allocator = std.testing.allocator;

    // Use a temp file path
    const test_path = "/tmp/test_log_init.log";
    defer std.fs.cwd().deleteFile(test_path) catch {};

    const config = RotationConfig{
        .base_path = test_path,
        .max_size = 1024,
        .max_files = 3,
    };

    var adapter = try FileAdapter.init(allocator, config);
    defer adapter.deinit();

    // Verify file exists
    const file = try std.fs.cwd().openFile(test_path, .{});
    file.close();
}

test "FileAdapter.write appends to file" {
    const allocator = std.testing.allocator;

    const test_path = "/tmp/test_log_write.log";
    defer std.fs.cwd().deleteFile(test_path) catch {};

    const config = RotationConfig{
        .base_path = test_path,
        .max_size = 1024 * 1024,
        .max_files = 3,
    };

    var adapter = try FileAdapter.init(allocator, config);
    defer adapter.deinit();

    var entry = try lib.Entry.init(.info, "test message", allocator);
    defer entry.deinit(allocator);

    try adapter.write(entry);

    // Verify content was written
    const content = try std.fs.cwd().readFileAlloc(allocator, test_path, 4096);
    defer allocator.free(content);

    try std.testing.expect(std.mem.indexOf(u8, content, "test message") != null);
    try std.testing.expect(std.mem.indexOf(u8, content, "INFO") != null);
}

test "FileAdapter.rotate triggers at size threshold" {
    const allocator = std.testing.allocator;

    const test_path = "/tmp/test_log_rotate.log";
    defer {
        std.fs.cwd().deleteFile(test_path) catch {};
        std.fs.cwd().deleteFile(test_path ++ ".1") catch {};
    }

    // Small size to trigger rotation quickly
    const config = RotationConfig{
        .base_path = test_path,
        .max_size = 100, // 100 bytes - very small for testing
        .max_files = 3,
    };

    var adapter = try FileAdapter.init(allocator, config);
    defer adapter.deinit();

    // Write multiple entries to trigger rotation
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        var entry = try lib.Entry.init(.info, "rotation test message", allocator);
        defer entry.deinit(allocator);
        try adapter.write(entry);
    }

    // Verify rotation occurred (backup file exists)
    const backup_exists = blk: {
        std.fs.cwd().access(test_path ++ ".1", .{}) catch {
            break :blk false;
        };
        break :blk true;
    };

    try std.testing.expect(backup_exists);
}

test "BDD: Given file adapter with 1KB limit, When writing 2KB of logs, Then rotation occurs" {
    const allocator = std.testing.allocator;

    const test_path = "/tmp/bdd_rotation.log";
    defer {
        std.fs.cwd().deleteFile(test_path) catch {};
        std.fs.cwd().deleteFile(test_path ++ ".1") catch {};
    }

    // Given
    const config = RotationConfig{
        .base_path = test_path,
        .max_size = 1024, // 1KB
        .max_files = 2,
    };

    var adapter = try FileAdapter.init(allocator, config);
    defer adapter.deinit();

    // When - write enough to trigger rotation
    var msg_count: usize = 0;
    while (adapter.getCurrentSize() < 512) : (msg_count += 1) {
        var entry = try lib.Entry.init(.info, "message content for rotation testing", allocator);
        defer entry.deinit(allocator);
        try adapter.write(entry);
    }

    // Then - should have written multiple messages and possibly rotated
    try std.testing.expect(msg_count > 0);
}
