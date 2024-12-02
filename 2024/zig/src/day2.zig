const std = @import("std");

fn checkTolerance(increasing: bool, prev: u64, next: u64) bool {
    const levels = if (increasing) [_]u64{ next, prev } else [_]u64{ prev, next };
    const a = levels[0];
    const b = levels[1];

    return a > b and a - b >= 1 and a - b <= 3;
}

pub fn partOne(input_reader: anytype) !u64 {
    var safe_count: u64 = 0;

    var buf: [24]u8 = undefined;
    line_loop: while (try input_reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
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

pub fn partTwo(input_reader: anytype) !u64 {
    _ = &input_reader;
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
    var stream = std.io.fixedBufferStream(sample_input);
    const reader = stream.reader();
    const result = try partOne(reader);

    try testing.expectEqual(2, result);
}

test "part two" {
    var stream = std.io.fixedBufferStream(sample_input);
    const reader = stream.reader();
    const result = try partTwo(reader);

    try testing.expectEqual(5, result);
}
