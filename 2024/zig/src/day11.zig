const std = @import("std");

pub fn partOne(input: []const u8, allocator: anytype) anyerror!u64 {
    var it = std.mem.splitScalar(u8, input, ' ');
    var stones = std.ArrayList(u64).init(allocator);
    defer stones.deinit();
    while (it.next()) |s| {
        const stone = try std.fmt.parseInt(u64, s, 10);
        try stones.append(stone);
    }
    return try blink(25, stones.items, allocator);
}

pub fn partTwo(input: []const u8, allocator: std.mem.Allocator) anyerror!u64 {
    var it = std.mem.splitScalar(u8, input, ' ');
    var stones = std.ArrayList(u64).init(allocator);
    defer stones.deinit();
    while (it.next()) |s| {
        const stone = try std.fmt.parseInt(u64, s, 10);
        try stones.append(stone);
    }
    return try blink(75, stones.items, allocator);
}

const sample_input = "125 17";

const testing = std.testing;
test "part one" {
    try testing.expectEqual(55312, partOne(sample_input, testing.allocator));
}

fn blink(count: usize, stones: []const u64, allocator: std.mem.Allocator) !usize {
    var current_level = try createLevel(stones, allocator);
    defer current_level.deinit();

    var next_level = Level.init(allocator);
    defer next_level.deinit();

    for (0..count) |_| {
        for (current_level.items) |item| {
            const next = splitStone(item.stone);
            try insertStone(next.left, item.multiplier, &next_level);
            if (next.right) |right| try insertStone(right, item.multiplier, &next_level);
        }
        std.mem.swap(Level, &current_level, &next_level);

        for (next_level.items) |*item| {
            item.*.multiplier = 0;
        }
    }

    var res: usize = 0;
    for (current_level.items) |item| res += item.multiplier;
    return res;
}

const Level = std.ArrayList(LevelItem);
fn insertStone(stone: u64, multiplier: usize, level: *Level) !void {
    const i = std.sort.lowerBound(LevelItem, level.items, stone, LevelItem.cmp);
    if (i < level.items.len and level.items[i].stone == stone) {
        level.items[i].multiplier += multiplier;
    } else {
        try level.insert(i, .{
            .stone = stone,
            .multiplier = multiplier,
        });
    }
}

pub fn createLevel(stones: []const u64, allocator: anytype) !Level {
    var level = Level.init(allocator);
    errdefer level.deinit();

    for (stones) |stone| {
        try insertStone(stone, 1, &level);
    }

    return level;
}

test "createLevel" {
    const level = try createLevel(&.{ 2, 0, 2, 4 }, testing.allocator);
    defer level.deinit();

    const items = level.items;
    try testing.expectEqual(0, items[0].stone);
    try testing.expectEqual(2, items[1].stone);
    try testing.expectEqual(4, items[2].stone);

    try testing.expectEqual(1, items[0].multiplier);
    try testing.expectEqual(2, items[1].multiplier);
    try testing.expectEqual(1, items[2].multiplier);
}

const LevelItem = struct {
    stone: u64,
    multiplier: u64,

    fn cmp(ctx: u64, item: @This()) std.math.Order {
        return std.math.order(ctx, item.stone);
    }
};

const Children = struct {
    left: u64,
    right: ?u64 = null,
};

pub fn splitStone(stone: u64) Children {
    if (stone == 0) return .{ .left = 1 };

    const width = std.math.log10_int(stone) + 1;
    if (width % 2 == 0) {
        const d = std.math.pow(u64, 10, width / 2);
        const left = stone / d;
        const right = stone % d;
        return .{ .left = left, .right = right };
    }

    return .{ .left = stone * 2024 };
}

test "splitStone" {
    try std.testing.expectEqual(1, splitStone(0).left);
    try std.testing.expectEqual(null, splitStone(0).right);
    try std.testing.expectEqual(12, splitStone(1234).left);
    try std.testing.expectEqual(34, splitStone(1234).right);
    try std.testing.expectEqual(253000, splitStone(125).left);
    try std.testing.expectEqual(null, splitStone(125).right);
}
