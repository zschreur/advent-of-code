const std = @import("std");
const PuzzleArgs = @import("./aoc_args.zig").PuzzleArgs;

const day0 = @import("./day0.zig");

fn runPuzzle(args: PuzzleArgs) !void {
    const day = args.day;

    {
        var timer = try std.time.Timer.start();
        const input_file = try std.fs.cwd().openFile(args.input_file_path, .{});
        var buffered_reader = std.io.bufferedReader(input_file.reader());
        const reader = buffered_reader.reader();
        const result = try switch (day) {
            0 => day0.partOne(reader),
            else => unreachable(),
        };
        const lap = timer.lap();
        std.debug.print("{d}\n", .{result});
        std.debug.print("{s}\n", .{std.fmt.fmtDuration(lap)});
    }

    {
        var timer = try std.time.Timer.start();
        const input_file = try std.fs.cwd().openFile(args.input_file_path, .{});
        var buffered_reader = std.io.bufferedReader(input_file.reader());
        const reader = buffered_reader.reader();
        const result = try switch (day) {
            0 => day0.partTwo(reader),
            else => unreachable(),
        };
        const lap = timer.lap();
        std.debug.print("{d}\n", .{result});
        std.debug.print("{s}\n", .{std.fmt.fmtDuration(lap)});
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const args = try PuzzleArgs.init(allocator);
    defer args.deinit();

    try runPuzzle(args);
}

test {
    _ = @import("./day0.zig");
}
