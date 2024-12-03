const std = @import("std");

fn checkTolerance(increasing: bool, prev: u64, next: u64) bool {
    const levels = if (increasing) [_]u64{ next, prev } else [_]u64{ prev, next };
    const a = levels[0];
    const b = levels[1];

    return a > b and a - b >= 1 and a - b <= 3;
}

pub fn partOne(puzzle_input: []const u8) !u64 {
    var safe_count: u64 = 0;

    var line_it = std.mem.splitScalar(u8, puzzle_input, '\n');
    line_loop: while (line_it.next()) |line| {
        var it = std.mem.splitScalar(u8, line, ' ');
        var prev: u64 = try std.fmt.parseInt(u64, it.first(), 10);

        const increasing = blk: {
            const next = try std.fmt.parseInt(u64, it.peek().?, 10);
            break :blk next > prev;
        };

        while (it.next()) |s| {
            const level = try std.fmt.parseInt(u64, s, 10);
            if (checkTolerance(increasing, prev, level)) {
                prev = level;
            } else {
                continue :line_loop;
            }
        }

        safe_count += 1;
    }

    return safe_count;
}

pub fn partTwo(puzzle_input: []const u8) !u64 {
    _ = &puzzle_input;
    return error.NotImplemented;
}

const sample_input =
    \\7 6 4 2 1
    \\1 2 7 8 9
    \\9 7 6 2 1
    \\1 3 2 4 5
    \\8 6 4 4 1
    \\1 3 6 7 9
    \\4 1 3 4 7
;

const testing = @import("std").testing;
test "part one" {
    try testing.expectEqual(2, partOne(sample_input));
}

test "part two" {
    try testing.expectEqual(5, partTwo(sample_input));
}
