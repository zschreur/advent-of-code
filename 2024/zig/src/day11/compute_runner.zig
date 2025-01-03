const std = @import("std");
const BlinkLookupTable = @import("./lookup_table.zig").BlinkLookupTable;
const splitStone = @import("./split_stone.zig").splitStone;
const testing = std.testing;

pub fn ComputeRunner(comptime blink_count: usize) type {
    return struct {
        stone_count: usize = 0,
        lookup_table: BlinkLookupTable,
        allocator: std.mem.Allocator,
        levels: [blink_count + 1]Level,

        const Self = @This();

        pub fn init(initial_level: Level, opts: anytype) Self {
            var levels: [blink_count + 1]Level = undefined;
            levels[0] = initial_level;
            for (1..levels.len) |i| {
                levels[i] = Level.init(opts.allocator);
            }

            return .{
                .lookup_table = opts.lookup_table,
                .allocator = opts.allocator,
                .levels = levels,
            };
        }

        pub fn deinit(self: Self) void {
            for (self.levels) |level| {
                level.deinit();
            }
        }

        pub fn run(self: *Self) !usize {
            for (0..self.levels.len - 1) |i| {
                const level = self.levels[i];
                // split stone and update
                // TODO use lookup table

                for (level.items) |item| {
                    const next = splitStone(item.stone);
                    const next_level = &self.levels[i + 1];
                    try insertStone(next.left, item.multiplier, next_level);
                    if (next.right) |right| {
                        try insertStone(right, item.multiplier, next_level);
                    }
                }
            }

            var res: usize = 0;
            for (self.levels[self.levels.len - 1].items) |item| {
                res += item.multiplier;
            }
            return res;
        }
    };
}

test "compute" {
    var runner = ComputeRunner(25).init(try createLevel(&.{ 125, 17 }, testing.allocator), .{
        .lookup_table = BlinkLookupTable{
            .leaves = &.{},
            .items = &.{},
        },
        .allocator = testing.allocator,
    });
    defer runner.deinit();

    try testing.expectEqual(55312, try runner.run());
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

test "foo" {
    const level = try createLevel(&.{ 125,17 }, testing.allocator);
    defer level.deinit();
}

const LevelItem = struct {
    stone: u64,
    multiplier: u64,

    fn cmp(ctx: u64, item: @This()) std.math.Order {
        return std.math.order(ctx, item.stone);
    }
};
