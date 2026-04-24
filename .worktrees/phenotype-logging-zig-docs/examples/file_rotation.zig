//! File Rotation Example
//!
//! Demonstrates writing logs to file with automatic rotation.

const std = @import("std");
const logging = @import("phenotype-logging");
const file_adapter = @import("phenotype-logging/adapters/file");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create file adapter with small rotation threshold for demo
    var adapter = try file_adapter.FileAdapter.init(allocator, .{
        .base_path = "example.log",
        .max_size = 4096, // 4 KB for quick rotation
        .max_files = 3,
    });
    defer adapter.deinit();

    std.debug.print("Writing logs with rotation (4KB threshold)...\n", .{});

    // Write many log entries to trigger rotation
    var i: usize = 0;
    while (i < 500) : (i += 1) {
        var entry = try logging.Entry.init(.info, "Processing batch job iteration", allocator);
        defer entry.deinit(allocator);

        try entry.addField(allocator, "iteration", try std.fmt.allocPrint(allocator, "{d}", .{i}));
        try entry.addField(allocator, "status", "ok");
        try entry.addField(allocator, "processed_items", "100");

        try adapter.write(entry);
    }

    std.debug.print("Done! Check example.log and rotated files:\n", .{});
    std.debug.print("  - example.log (current)\n", .{});
    std.debug.print("  - example.log.1 (most recent rotated)\n", .{});
    std.debug.print("  - example.log.2\n", .{});
    std.debug.print("  - example.log.3 (oldest, deleted on next rotation)\n", .{});
}
