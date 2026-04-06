//! Basic Logging Example
//!
//! Demonstrates simple logging to stderr with different levels.

const std = @import("std");
const stderr_adapter = @import("phenotype-logging/adapters/stderr");

pub fn main() !void {
    // Create stderr adapter with colors
    var adapter = stderr_adapter.StderrAdapter.init(.{
        .use_color = true,
        .show_timestamp = true,
        .show_fields = true,
    });

    // Log at different levels
    try adapter.log(.trace, "Trace message (very detailed)");
    try adapter.log(.debug, "Debug message (development)");
    try adapter.log(.info, "Info message (general information)");
    try adapter.log(.warn, "Warning message (attention needed)");
    try adapter.log(.err, "Error message (something failed)");
    try adapter.log(.fatal, "Fatal message (application dying)");
}
