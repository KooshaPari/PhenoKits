const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Core library module
    const root = b.createModule(.{
        .root_source_file = b.path("src/lib.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Adapter modules
    const file_adapter = b.createModule(.{
        .root_source_file = b.path("src/adapters/file.zig"),
        .target = target,
        .optimize = optimize,
    });
    file_adapter.addImport("lib", root);

    const stderr_adapter = b.createModule(.{
        .root_source_file = b.path("src/adapters/stderr.zig"),
        .target = target,
        .optimize = optimize,
    });
    stderr_adapter.addImport("lib", root);

    // Interface module
    const interface_mod = b.createModule(.{
        .root_source_file = b.path("src/interface.zig"),
        .target = target,
        .optimize = optimize,
    });
    interface_mod.addImport("lib", root);

    // Core library tests
    const core_tests = b.addTest(.{
        .name = "core-test",
        .root_module = root,
    });

    // File adapter tests
    const file_tests = b.addTest(.{
        .name = "file-adapter-test",
        .root_module = file_adapter,
    });

    // Stderr adapter tests
    const stderr_tests = b.addTest(.{
        .name = "stderr-adapter-test",
        .root_module = stderr_adapter,
    });

    // Interface tests
    const interface_tests = b.addTest(.{
        .name = "interface-test",
        .root_module = interface_mod,
    });

    // Run all tests
    const run_core = b.addRunArtifact(core_tests);
    const run_file = b.addRunArtifact(file_tests);
    const run_stderr = b.addRunArtifact(stderr_tests);
    const run_interface = b.addRunArtifact(interface_tests);

    const test_step = b.step("test", "Run all unit tests");
    test_step.dependOn(&run_core.step);
    test_step.dependOn(&run_file.step);
    test_step.dependOn(&run_stderr.step);
    test_step.dependOn(&run_interface.step);

    // Install step for library consumers
    b.installArtifact(core_tests);
}
