const std = @import("std");
const day0 = @import("./day00.zig");
const day1 = @import("./day01.zig");
const day2 = @import("./day02.zig");
const day3 = @import("./day03.zig");
const day4 = @import("./day04.zig");
const day5 = @import("./day05.zig");
const day6 = @import("./day06.zig");
const day7 = @import("./day07.zig");
const day8 = @import("./day08.zig");
const day9 = @import("./day09.zig");
const day10 = @import("./day10.zig");

const label_color = "\x1b[38;5;250m";
const time_color = "\x1b[38;5;38m";
const answer_color = "\x1b[38;5;220m";
const reset = "\x1b[0m";

const puzzle_input = std.mem.trim(
    u8,
    @embedFile("puzzle_input"),
    &std.ascii.whitespace,
);
const day = @import("config").day;

fn ReturnType(comptime f: anytype) type {
    return @typeInfo(@TypeOf(f)).@"fn".return_type.?;
}

fn runWithAllocator(comptime func: anytype) ReturnType(func) {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();
    return try @call(.auto, func, .{ puzzle_input, allocator });
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    var buffer = std.io.bufferedWriter(stdout);
    var bufOut = buffer.writer();

    {
        var timer = try std.time.Timer.start();
        const result = try switch (day) {
            0 => day0.partOne(puzzle_input),
            1 => day1.partOne(puzzle_input),
            2 => day2.partOne(puzzle_input),
            3 => day3.partOne(puzzle_input),
            4 => runWithAllocator(day4.partOne),
            5 => runWithAllocator(day5.partOne),
            6 => runWithAllocator(day6.partOne),
            7 => runWithAllocator(day7.partOne),
            8 => runWithAllocator(day8.partOne),
            9 => day9.partOne(puzzle_input),
            10 => runWithAllocator(day10.partOne),
            else => @compileError("Day is not implemented"),
        };
        const lap = timer.lap();
        try bufOut.print("{s}Part 1: {s}{d: >10}{s}  [{s}]{s}\n", .{
            label_color,
            answer_color,
            result,
            time_color,
            std.fmt.fmtDuration(lap),
            reset,
        });
        try buffer.flush();
    }

    {
        var timer = try std.time.Timer.start();
        const result = try switch (day) {
            0 => day0.partTwo(puzzle_input),
            1 => day1.partTwo(puzzle_input),
            2 => day2.partTwo(puzzle_input),
            3 => day3.partTwo(puzzle_input),
            4 => runWithAllocator(day4.partTwo),
            5 => runWithAllocator(day5.partTwo),
            6 => runWithAllocator(day6.partTwo),
            7 => runWithAllocator(day7.partTwo),
            8 => runWithAllocator(day8.partTwo),
            9 => day9.partTwo(puzzle_input),
            10 => runWithAllocator(day10.partTwo),
            else => @compileError("Day is not implemented"),
        };
        const lap = timer.lap();
        try bufOut.print("{s}Part 2: {s}{d: >10}{s}  [{s}]{s}\n", .{
            label_color,
            answer_color,
            result,
            time_color,
            std.fmt.fmtDuration(lap),
            reset,
        });
        try buffer.flush();
    }
}

test {
    _ = @import("./day00.zig");
    _ = @import("./day01.zig");
    _ = @import("./day02.zig");
    _ = @import("./day03.zig");
    _ = @import("./day04.zig");
    _ = @import("./day05.zig");
    _ = @import("./day06.zig");
    _ = @import("./day07.zig");
    _ = @import("./day08.zig");
    _ = @import("./day09.zig");
    _ = @import("./day10.zig");
}
