//! Transport Interface - Comptime Duck Typing
//!
//! Implements FR-LOG-009: Comptime Adapter Interface
//! Enforces the transport adapter interface at compile time using comptime duck-typing,
//! producing a descriptive compile error if the interface is not satisfied.

const std = @import("std");
const lib = @import("lib.zig");

/// Transport interface requirements
/// Any type implementing this interface must have:
/// - `write(self, entry: lib.Entry) !void` method
pub const Transport = struct {
    /// Verify a type implements the Transport interface at compile time
    pub fn check(comptime T: type) void {
        // Check for write method
        if (!@hasDecl(T, "write")) {
            @compileError("Transport interface violation: type '" ++ @typeName(T) ++ "' missing 'write' method");
        }

        // Get the write function type
        const write_decl = @field(T, "write");
        const write_info = @typeInfo(@TypeOf(write_decl));

        // Must be a function
        if (write_info != .Fn) {
            @compileError("Transport interface violation: 'write' in '" ++ @typeName(T) ++ "' must be a function");
        }

        const fn_info = write_info.Fn;

        // Check return type is an error union
        const return_type_info = @typeInfo(fn_info.return_type.?);
        if (return_type_info != .ErrorUnion) {
            @compileError("Transport interface violation: 'write' must return an error union (e.g., '!void'), found: " ++ @typeName(fn_info.return_type.?));
        }

        // Check first parameter is a pointer to the type (self)
        if (fn_info.params.len < 2) {
            @compileError("Transport interface violation: 'write' must take at least 2 parameters: self and entry");
        }

        const self_param = fn_info.params[0];
        const self_type_info = @typeInfo(self_param.type.?);

        // Self must be a pointer
        if (self_type_info != .Pointer) {
            @compileError("Transport interface violation: first parameter of 'write' must be a pointer to self");
        }

        // Check second parameter is Entry
        const entry_param = fn_info.params[1];
        if (entry_param.type.? != lib.Entry) {
            @compileError("Transport interface violation: second parameter of 'write' must be 'Entry', found: " ++ @typeName(entry_param.type.?));
        }
    }

    /// Returns true if type implements Transport, false otherwise (for runtime checks)
    pub fn isValid(comptime T: type) bool {
        comptime {
            if (!@hasDecl(T, "write")) return false;

            const write_decl = @field(T, "write");
            const write_info = @typeInfo(@TypeOf(write_decl));

            if (write_info != .Fn) return false;

            const fn_info = write_info.Fn;
            if (fn_info.params.len < 2) return false;

            const return_type_info = @typeInfo(fn_info.return_type.?);
            if (return_type_info != .ErrorUnion) return false;

            if (fn_info.params[1].type.? != lib.Entry) return false;

            return true;
        }
    }
};

/// Generic Logger that works with any Transport
pub fn GenericLogger(comptime T: type) type {
    // Enforce interface at compile time
    comptime Transport.check(T);

    return struct {
        const Self = @This();

        level: lib.Level,
        transport: T,

        pub fn init(level: lib.Level, transport: T) Self {
            return .{
                .level = level,
                .transport = transport,
            };
        }

        pub fn log(self: *Self, level: lib.Level, message: []const u8, allocator: std.mem.Allocator) !void {
            if (!level.isEnabled(self.level)) return;

            var entry = try lib.Entry.init(level, message, allocator);
            defer entry.deinit(allocator);

            try self.transport.write(entry);
        }

        pub fn logWithFields(self: *Self, level: lib.Level, message: []const u8, fields: anytype, allocator: std.mem.Allocator) !void {
            if (!level.isEnabled(self.level)) return;

            var entry = try lib.Entry.init(level, message, allocator);
            defer entry.deinit(allocator);

            inline for (std.meta.fields(@TypeOf(fields))) |f| {
                const value = @field(fields, f.name);
                try entry.addField(allocator, f.name, value);
            }

            try self.transport.write(entry);
        }
    };
}

// ============================================================================
// Comptime Interface Tests
// ============================================================================

/// Valid transport implementation for testing
const MockTransport = struct {
    entries: std.ArrayList(lib.Entry),

    pub fn init(allocator: std.mem.Allocator) MockTransport {
        return .{
            .entries = std.ArrayList(lib.Entry).init(allocator),
        };
    }

    pub fn deinit(self: *MockTransport) void {
        self.entries.deinit();
    }

    pub fn write(self: *MockTransport, entry: lib.Entry) !void {
        try self.entries.append(entry);
    }
};

/// Invalid transport (missing write method) - should fail compile time check
const InvalidTransportMissingWrite = struct {
    pub fn someOtherMethod() void {}
};

/// Invalid transport (wrong signature) - should fail compile time check
const InvalidTransportWrongSignature = struct {
    pub fn write(self: *InvalidTransportWrongSignature, msg: []const u8) !void {
        _ = self;
        _ = msg;
    }
};

test "Transport.check accepts valid transport" {
    // This should compile successfully
    comptime Transport.check(MockTransport);
}

test "Transport.isValid returns true for valid transport" {
    comptime try std.testing.expect(Transport.isValid(MockTransport));
}

test "Transport.isValid returns false for invalid transport" {
    comptime try std.testing.expect(!Transport.isValid(InvalidTransportMissingWrite));
    comptime try std.testing.expect(!Transport.isValid(InvalidTransportWrongSignature));
}

test "GenericLogger works with valid transport" {
    const allocator = std.testing.allocator;

    var transport = MockTransport.init(allocator);
    defer transport.deinit();

    var logger = GenericLogger(MockTransport).init(.info, transport);

    try logger.log(.info, "test message", allocator);

    try std.testing.expectEqual(@as(usize, 1), transport.entries.items.len);
    try std.testing.expectEqualStrings("test message", transport.entries.items[0].message);
}

test "GenericLogger filters by level" {
    const allocator = std.testing.allocator;

    var transport = MockTransport.init(allocator);
    defer transport.deinit();

    var logger = GenericLogger(MockTransport).init(.warn, transport);

    // Debug should be filtered
    try logger.log(.debug, "debug msg", allocator);
    try std.testing.expectEqual(@as(usize, 0), transport.entries.items.len);

    // Error should pass through
    try logger.log(.err, "error msg", allocator);
    try std.testing.expectEqual(@as(usize, 1), transport.entries.items.len);
}

test "BDD: Given GenericLogger with MockTransport, When logging with fields, Then transport receives entry" {
    const allocator = std.testing.allocator;

    // Given
    var transport = MockTransport.init(allocator);
    defer transport.deinit();

    var logger = GenericLogger(MockTransport).init(.info, transport);

    // When
    try logger.logWithFields(.info, "request handled", .{
        .method = "GET",
        .path = "/api/users",
    }, allocator);

    // Then
    try std.testing.expectEqual(@as(usize, 1), transport.entries.items.len);
    try std.testing.expectEqualStrings("request handled", transport.entries.items[0].message);
}

// ============================================================================
// Compile-Time Error Tests
// ============================================================================
//
// The following tests demonstrate compile-time interface enforcement.
// Uncomment to verify compile errors are descriptive:
//
// test "Compile error: missing write method" {
//     // This should produce:
//     // "Transport interface violation: type 'InvalidTransportMissingWrite' missing 'write' method"
//     comptime Transport.check(InvalidTransportMissingWrite);
// }
//
// test "Compile error: wrong parameter type" {
//     // This should produce:
//     // "Transport interface violation: second parameter of 'write' must be 'Entry'"
//     comptime Transport.check(InvalidTransportWrongSignature);
// }
