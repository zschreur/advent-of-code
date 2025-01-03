const std = @import("std");
const BlinkLookupTable = @import("./day11/lookup_table.zig").BlinkLookupTable;
const ComputeRunner = @import("./day11/compute_runner.zig").ComputeRunner;
const createLevel = @import("./day11/compute_runner.zig").createLevel;

pub fn partOne(input: []const u8, allocator: anytype) anyerror!u64 {
    var it = std.mem.splitScalar(u8, input, ' ');
    const table = BlinkLookupTable{ .items = &.{}, .leaves = &.{} };
    var initial_level = std.ArrayList(u64).init(allocator);
    defer initial_level.deinit();
    while (it.next()) |s| {
        const stone = try std.fmt.parseInt(u64, s, 10);
        try initial_level.append(stone);
    }
    var runner = ComputeRunner(25).init(try createLevel(initial_level.items, allocator), .{
        .lookup_table = table,
        .allocator = allocator,
    });
    defer runner.deinit();

    const res = try runner.run();
    return res;
}

pub fn partTwo(input: []const u8, allocator: std.mem.Allocator) anyerror!u64 {
    var it = std.mem.splitScalar(u8, input, ' ');
    const table = BlinkLookupTable{ .items = &.{}, .leaves = &.{} };
    var initial_level = std.ArrayList(u64).init(allocator);
    defer initial_level.deinit();
    while (it.next()) |s| {
        const stone = try std.fmt.parseInt(u64, s, 10);
        try initial_level.append(stone);
    }
    var runner = ComputeRunner(75).init(try createLevel(initial_level.items, allocator), .{
        .lookup_table = table,
        .allocator = allocator,
    });
    defer runner.deinit();

    const res = try runner.run();
    return res;
}

const sample_input = "125 17";

const testing = std.testing;
test "part one" {
    try testing.expectEqual(55312, partOne(sample_input, testing.allocator));
}

test {}
