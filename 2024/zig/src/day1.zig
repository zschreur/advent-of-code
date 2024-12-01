const std = @import("std");

const Lists = struct {
    left_ids: []u32,
    right_ids: []u32,

    const Self = @This();

    fn parseInput(input_reader: anytype, left_buf: []u32, right_buf: []u32) !Self {
        var count: usize = 0;
        var buf: [16]u8 = undefined;
        while (try input_reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
            var it = std.mem.splitSequence(u8, line, "   ");
            const left = try std.fmt.parseInt(u32, it.first(), 10);
            const right = try std.fmt.parseInt(u32, it.next().?, 10);
            left_buf[count] = left;
            right_buf[count] = right;
            count += 1;
        }

        const left_ids = left_buf[0..count];
        const right_ids = right_buf[0..count];

        return Self{ .left_ids = left_ids, .right_ids = right_ids };
    }
};

pub fn partOne(input_reader: anytype) !u64 {
    var left_buf: [2048]u32 = undefined;
    var right_buf: [2048]u32 = undefined;

    const lists = try Lists.parseInput(input_reader, &left_buf, &right_buf);
    const left_ids = lists.left_ids;
    const right_ids = lists.right_ids;

    std.mem.sortUnstable(u32, left_ids, {}, std.sort.asc(u32));
    std.mem.sortUnstable(u32, right_ids, {}, std.sort.asc(u32));

    var total_distance: u32 = 0;
    for (left_ids, right_ids) |left, right| {
        total_distance += if (left > right) left - right else right - left;
    }

    return @intCast(total_distance);
}

pub fn partTwo(input_reader: anytype) !u64 {
    var left_buf: [2048]u32 = undefined;
    var right_buf: [2048]u32 = undefined;

    const lists = try Lists.parseInput(input_reader, &left_buf, &right_buf);
    const left_ids = lists.left_ids;
    const right_ids = lists.right_ids;

    var seen_ids: [2048]u32 = undefined;
    var id_counts: [2048]usize = undefined;
    var i: usize = 0;

    var similarity_score: u64 = 0;
    for (left_ids) |id| {
        if (std.mem.indexOfScalar(u32, seen_ids[0..i], id)) |seen_index| {
            similarity_score += @as(u64, @intCast(id)) * id_counts[seen_index];
        } else {
            id_counts[i] = std.mem.count(u32, right_ids, &[1]u32{id});
            seen_ids[i] = id;
            similarity_score += @as(u64, @intCast(id)) * id_counts[i];
            i += 1;
        }
    }

    return similarity_score;
}

const sample_input =
    \\3   4
    \\4   3
    \\2   5
    \\1   3
    \\3   9
    \\3   3
;

const testing = @import("std").testing;
test "part one" {
    var stream = std.io.fixedBufferStream(sample_input);
    const reader = stream.reader();
    const result = try partOne(reader);

    try testing.expectEqual(11, result);
}

test "part two" {
    var stream = std.io.fixedBufferStream(sample_input);
    const reader = stream.reader();
    const result = try partTwo(reader);

    try testing.expectEqual(31, result);
}
