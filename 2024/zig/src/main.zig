const std = @import("std");
const day0 = @import("./day0.zig");
const day1 = @import("./day1.zig");
const day2 = @import("./day2.zig");
const day3 = @import("./day3.zig");
const day4 = @import("./day4.zig");

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
            4 => day4.partOne(puzzle_input),
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
            4 => day4.partTwo(puzzle_input),
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
    _ = @import("./day0.zig");
    _ = @import("./day1.zig");
    _ = @import("./day2.zig");
    _ = @import("./day3.zig");
    _ = @import("./day4.zig");
}
