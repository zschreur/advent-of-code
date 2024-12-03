//! Build for 2024 Advent of Code

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const day = b.option(u8, "day", "Day of puzzle to run") orelse 0;

    const options = b.addOptions();
    options.addOption(u8, "day", day);

    const exe = b.addExecutable(.{
        .name = "day" ++ std.fmt.digits2(day),
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    exe.root_module.addOptions("config", options);
    exe.root_module.addAnonymousImport(
        "puzzle_input",
        .{ .root_source_file = b.path(
            "../input/day-" ++ std.fmt.digits2(day) ++ ".txt",
        ) },
    );

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const exe_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_exe_unit_tests.step);
}
