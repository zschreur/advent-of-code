const std = @import("std");

fn checkTolerance(increasing: bool, prev: u8, next: u8) bool {
    const levels = if (increasing) [_]u8{ next, prev } else [_]u8{ prev, next };
    const a = levels[0];
    const b = levels[1];

    return a > b and a - b >= 1 and a - b <= 3;
}

pub fn partOne(puzzle_input: []const u8) !usize {
    var safe_count: usize = 0;

    var line_it = std.mem.splitScalar(u8, puzzle_input, '\n');
    line_loop: while (line_it.next()) |line| {
        var it = std.mem.splitScalar(u8, line, ' ');
        var prev = try std.fmt.parseInt(u8, it.first(), 10);

        const increasing = blk: {
            const next = try std.fmt.parseInt(u8, it.peek().?, 10);
            break :blk next > prev;
        };

        while (it.next()) |s| {
            const level = try std.fmt.parseInt(u8, s, 10);
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

fn isSafeDelta(d: i8) bool {
    return d >= 1 and d <= 3;
}

fn checkDeltas(V: type, deltas_vec: V) bool {
    const len = @typeInfo(V).vector.len;
    const checks = blk: {
        const a = deltas_vec >= @as(V, @splat(1)); // pred
        const b = deltas_vec <= @as(V, @splat(3)); // b
        break :blk @select(bool, a, b, a);
    };

    const buf = @as([len]bool, checks);
    if (std.mem.indexOfScalar(bool, buf[0..(len - 1)], false)) |index| {
        if (index == 0 and checks[index + 1]) {
            // try drop first
            return std.mem.indexOfScalar(bool, buf[1..(len)], false) == null;
        } else if (isSafeDelta(deltas_vec[index] + deltas_vec[index + 1])) {
            // try combine with next
            return std.mem.indexOfScalar(bool, buf[index + 2 .. len], false) == null;
        } else if (index > 0 and isSafeDelta(deltas_vec[index] + deltas_vec[index - 1])) {
            // try combine with previous
            return std.mem.indexOfScalar(bool, buf[index + 1 .. len], false) == null;
        }

        return false;
    }

    return true;
}

pub fn partTwo(puzzle_input: []const u8) !usize {
    var safe_count: usize = 0;

    var levels_buf: [8]i8 = .{0} ** 8;
    var line_it = std.mem.splitScalar(u8, puzzle_input, '\n');
    line_loop: while (line_it.next()) |line| {
        var it = std.mem.splitScalar(u8, line, ' ');
        var i: usize = 0;
        while (it.next()) |s| {
            levels_buf[i] = try std.fmt.parseInt(i8, s, 10);
            i += 1;
        }

        switch (i - 1) {
            inline 3...7 => |delta_size| {
                const V = @Vector(delta_size, i8);
                const d =
                    @as(V, levels_buf[1 .. delta_size + 1].*) - @as(V, levels_buf[0..delta_size].*);

                // Multiply deltas by -1 or 1 depending on if it is increasing or not
                const x: V = blk_2: {
                    const a = d[0];
                    const b = d[1];
                    const c = d[2];

                    if (a != 0 and ((b != 0 and a ^ b >= 0) or (c != 0 and a ^ c >= 0))) {
                        break :blk_2 if (a > 0) @as(V, @splat(1)) else @as(V, @splat(-1));
                    } else if (b != 0 and c != 0) {
                        break :blk_2 if (b > 0) @as(V, @splat(1)) else @as(V, @splat(-1));
                    } else {
                        continue :line_loop;
                    }
                };

                if (checkDeltas(V, d * x)) safe_count += 1;
            },
            else => unreachable,
        }
    }

    return safe_count;
}

const sample_input =
    \\7 6 4 2 1
    \\1 2 7 8 9
    \\9 7 6 2 1
    \\1 3 2 4 5
    \\8 6 4 4 1
    \\1 3 6 7 9
    \\4 1 3 4 7
    \\12 10 7 8 6 4
;

const testing = @import("std").testing;
test "part one" {
    try testing.expectEqual(2, partOne(sample_input));
}

test "part two" {
    try testing.expectEqual(6, partTwo(sample_input));
}
