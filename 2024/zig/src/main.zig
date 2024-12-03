const std = @import("std");
const day0 = @import("./day0.zig");
const day1 = @import("./day1.zig");
const day2 = @import("./day2.zig");

const puzzle_input = std.mem.trim(
    u8,
    @embedFile("puzzle_input"),
    &std.ascii.whitespace,
);
const day = @import("config").day;

pub fn main() !void {
    {
        var timer = try std.time.Timer.start();
        const result = try switch (day) {
            0 => day0.partOne(puzzle_input),
            1 => day1.partOne(puzzle_input),
            2 => day2.partOne(puzzle_input),
            else => @compileError("Day is not implemented"),
        };
        const lap = timer.lap();
        std.debug.print("{d}\n", .{result});
        std.debug.print("{s}\n", .{std.fmt.fmtDuration(lap)});
    }

    {
        var timer = try std.time.Timer.start();
        const result = try switch (day) {
            0 => day0.partTwo(puzzle_input),
            1 => day1.partTwo(puzzle_input),
            2 => day2.partTwo(puzzle_input),
            else => @compileError("Day is not implemented"),
        };
        const lap = timer.lap();
        std.debug.print("{d}\n", .{result});
        std.debug.print("{s}\n", .{std.fmt.fmtDuration(lap)});
    }
}

test {
    _ = @import("./day0.zig");
    _ = @import("./day1.zig");
    _ = @import("./day2.zig");
}
