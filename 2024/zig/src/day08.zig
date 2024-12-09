const std = @import("std");
const IndexedIterator = @import("./itertools.zig").IndexedIterator;

const Point = struct {
    x: usize,
    y: usize,

    fn index(self: @This(), width: usize) usize {
        return self.y * width + self.x;
    }

    const PointDelta = struct { dx: i32, dy: i32 };
    fn sub(self: @This(), other: @This()) PointDelta {
        return .{
            .dx = @as(i32, @intCast(self.x)) - @as(i32, @intCast(other.x)),
            .dy = @as(i32, @intCast(self.y)) - @as(i32, @intCast(other.y)),
        };
    }

    fn addDelta(self: @This(), dxy: PointDelta) ?@This() {
        const x = @as(i32, @intCast(self.x)) + dxy.dx;
        const y = @as(i32, @intCast(self.y)) + dxy.dy;
        if (x < 0 or y < 0) return null;
        return .{ .x = @intCast(x), .y = @intCast(y) };
    }

    fn subDelta(self: @This(), dxy: PointDelta) ?@This() {
        const x = @as(i32, @intCast(self.x)) - dxy.dx;
        const y = @as(i32, @intCast(self.y)) - dxy.dy;
        if (x < 0 or y < 0) return null;
        return .{ .x = @intCast(x), .y = @intCast(y) };
    }
};

const Grid = struct {
    allocator: std.mem.Allocator,
    frequencies: std.AutoHashMap(u8, std.MultiArrayList(Point)),
    map_size: usize,

    const Self = @This();

    fn init(input: []const u8, allocator: std.mem.Allocator) !Self {
        var it = std.mem.splitScalar(u8, input, '\n');
        const map_size = it.peek().?.len;

        const frequencies = std.AutoHashMap(u8, std.MultiArrayList(Point))
            .init(allocator);

        var self = Self{
            .frequencies = frequencies,
            .map_size = map_size,
            .allocator = allocator,
        };
        errdefer self.deinit();

        var indexed_iter = IndexedIterator(@TypeOf(it)){ .iter = it };
        while (indexed_iter.next()) |next| {
            const line = next.value;
            const y = next.index;
            for (line, 0..) |c, x| {
                if (c == '.') continue;

                var res = try self.frequencies.getOrPut(c);
                if (!res.found_existing) {
                    res.value_ptr.* = std.MultiArrayList(Point){};
                }
                try res.value_ptr.append(allocator, .{ .x = x, .y = y });
            }
        }

        return self;
    }

    fn deinit(self: *Self) void {
        var it = self.frequencies.valueIterator();
        while (it.next()) |v| {
            v.deinit(self.allocator);
        }

        self.frequencies.deinit();
    }
};

pub fn partOne(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var grid = try Grid.init(input, allocator);
    defer grid.deinit();

    const antinodes = try allocator.alloc(bool, grid.map_size * grid.map_size);
    defer allocator.free(antinodes);

    var freq_iter = grid.frequencies.iterator();
    while (freq_iter.next()) |kv| {
        const val = kv.value_ptr.*;

        const slice = val.slice();
        for (0..slice.len) |i| {
            var a = slice.get(i);
            for ((i + 1)..slice.len) |j| {
                const b = slice.get(j);

                const delta = b.sub(a);

                if (a.subDelta(delta)) |p| {
                    if (p.x < grid.map_size and p.y < grid.map_size) {
                        antinodes[p.index(grid.map_size)] = true;
                    }
                }

                if (b.addDelta(delta)) |p| {
                    if (p.x < grid.map_size and p.y < grid.map_size) {
                        antinodes[p.index(grid.map_size)] = true;
                    }
                }
            }
        }
    }

    return std.mem.count(bool, antinodes, &[_]bool{true});
}

pub fn partTwo(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var grid = try Grid.init(input, allocator);
    defer grid.deinit();

    const antinodes = try allocator.alloc(bool, grid.map_size * grid.map_size);
    defer allocator.free(antinodes);

    var freq_iter = grid.frequencies.iterator();
    while (freq_iter.next()) |kv| {
        const val = kv.value_ptr.*;

        const slice = val.slice();
        for (0..slice.len) |i| {
            const a = slice.get(i);
            antinodes[a.index(grid.map_size)] = true;
            for ((i + 1)..slice.len) |j| {
                const b = slice.get(j);
                antinodes[b.index(grid.map_size)] = true;

                const delta = b.sub(a);

                var prev = a;
                while (prev.subDelta(delta)) |p| {
                    if (p.x < grid.map_size and p.y < grid.map_size) {
                        antinodes[p.index(grid.map_size)] = true;
                    } else {
                        break;
                    }

                    prev = p;
                }

                prev = b;
                while (prev.addDelta(delta)) |p| {
                    if (p.x < grid.map_size and p.y < grid.map_size) {
                        antinodes[p.index(grid.map_size)] = true;
                    } else {
                        break;
                    }

                    prev = p;
                }
            }
        }
    }

    return std.mem.count(bool, antinodes, &[_]bool{true});
}

const sample_input =
    \\............
    \\........0...
    \\.....0......
    \\.......0....
    \\....0.......
    \\......A.....
    \\............
    \\............
    \\........A...
    \\.........A..
    \\............
    \\............
;

const testing = @import("std").testing;
test "part one" {
    try std.testing.expectEqual(14, partOne(sample_input, testing.allocator));
}

test "part two" {
    try std.testing.expectEqual(34, partTwo(sample_input, testing.allocator));
}
