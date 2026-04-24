//! Structured Logging Example
//!
//! Demonstrates logging with structured key-value fields.

const std = @import("std");
const stderr_adapter = @import("phenotype-logging/adapters/stderr");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var adapter = stderr_adapter.StderrAdapter.init(.{
        .use_color = true,
    });

    // HTTP request logging
    try adapter.logWithFields(.info, "HTTP request received", .{
        .method = "GET",
        .path = "/api/v1/users",
        .request_id = "req-abc-123-def",
        .remote_addr = "192.168.1.100",
    }, allocator);

    // Request completion
    try adapter.logWithFields(.info, "HTTP request completed", .{
        .method = "GET",
        .path = "/api/v1/users",
        .status = 200,
        .duration_ms = 15,
        .response_size = 2048,
    }, allocator);

    // Database operation
    try adapter.logWithFields(.debug, "Database query executed", .{
        .query = "SELECT id, name, email FROM users WHERE active = true",
        .duration_ms = 5,
        .rows_returned = 42,
    }, allocator);

    // Error with context
    try adapter.logWithFields(.err, "Failed to process payment", .{
        .user_id = "user-789",
        .order_id = "order-456",
        .error_code = "CARD_DECLINED",
        .gateway = "stripe",
        .amount_usd = 99.99,
    }, allocator);
}
