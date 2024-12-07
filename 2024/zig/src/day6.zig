const std = @import("std");

const Pos = struct { x: usize, y: usize };
const Direction = enum(u2) { N, S, E, W };

const Map = struct {
    east_west: []std.ArrayList(usize),
    north_south: []std.ArrayList(usize),
    map: [][]bool,
    gaurd_position: struct { pos: Pos, dir: Direction },
    allocator: std.mem.Allocator,

    const Self = @This();

    fn deinit(self: *Self) void {
        for (self.east_west) |a| {
            a.deinit();
        }
        self.allocator.free(self.east_west);

        for (self.north_south) |a| {
            a.deinit();
        }
        self.allocator.free(self.north_south);

        for (self.map) |row| {
            self.allocator.free(row);
        }
        self.allocator.free(self.map);
    }

    fn init(input: []const u8, allocator: std.mem.Allocator) !Self {
        var line_it = std.mem.splitScalar(u8, input, '\n');

        // assume square
        const size = line_it.peek().?.len;

        const map = try allocator.alloc([]bool, size);
        for (0..size) |y| {
            map[y] = try allocator.alloc(bool, size);
            @memset(map[y], false);
        }
        var east_west = try allocator.alloc(std.ArrayList(usize), size);
        var north_south = try allocator.alloc(std.ArrayList(usize), size);
        for (0..size) |i| {
            east_west[i] = std.ArrayList(usize).init(allocator);
            north_south[i] = std.ArrayList(usize).init(allocator);
        }

        var gaurd_pos: ?@FieldType(Self, "gaurd_position") = null;
        var y: usize = 0;
        while (line_it.next()) |line| {
            defer y += 1;

            for (line, 0..) |c, x| {
                const pos: Pos = .{ .x = x, .y = y };
                switch (c) {
                    '.' => continue,
                    '#' => {
                        try east_west[y].append(x);
                        try north_south[x].append(y);
                        map[y][x] = true;
                    },
                    '^' => {
                        gaurd_pos = .{ .pos = pos, .dir = .N };
                    },
                    '<' => {
                        gaurd_pos = .{ .pos = pos, .dir = .W };
                    },
                    '>' => {
                        gaurd_pos = .{ .pos = pos, .dir = .E };
                    },
                    'v' => {
                        gaurd_pos = .{ .pos = pos, .dir = .S };
                    },
                    else => unreachable,
                }
            }
        }

        return .{
            .east_west = east_west,
            .north_south = north_south,
            .allocator = allocator,
            .gaurd_position = gaurd_pos.?,
            .map = map,
        };
    }
};

const Record = struct {
    visited: []u4,
    allocator: std.mem.Allocator,
    map: Map,
    count: usize = 0,
    obstacle_location_count: usize = 0,

    const Self = @This();

    fn init(a: anytype, m: Map) !Self {
        const visited = try a.alloc(u4, m.east_west.len * m.north_south.len);
        @memset(visited, 0);

        return .{ .visited = visited, .allocator = a, .map = m };
    }

    fn deinit(self: *Self) void {
        self.allocator.free(self.visited);
    }

    fn recordWalk(self: *Self, dir: Direction, start: Pos, count: usize) void {
        switch (dir) {
            .N => for (0..count) |i| self.markVisited(.{ .x = start.x - i - 1, .y = start.y }),
            .S => for (0..count) |i| self.markVisited(.{ .x = start.x + i + 1, .y = start.y }),
            .E => for (0..count) |i| self.markVisited(.{ .x = start.x, .y = start.y + i + 1 }),
            .E => for (0..count) |i| self.markVisited(.{ .x = start.x, .y = start.y - i - 1 }),
        }
    }

    fn markVisited(self: *Self, comptime dir: Direction, pos: Pos) void {
        // if I am marking a spot as visited, can I check that a surrounding spot would
        // be a place to set a block for a loop?
        //
        // If a 90deg turn would make a loop and there is no wall in front, then mark spot
        //
        const size = self.map.east_west.len;
        const i = pos.y * size + pos.x;
        const v = self.visited[i];
        if (v == 0) self.count += 1;

        self.checkForLoop(dir, pos);
        self.visited[i] = switch (dir) {
            .N => v | 0b1000,
            .S => v | 0b0100,
            .E => v | 0b0010,
            .W => v | 0b0001,
        };
    }

    fn checkForLoop(self: *Self, comptime dir: Direction, pos: Pos) void {
        const size = self.map.east_west.len;
        const i = pos.y * size + pos.x;
        const v = self.visited[i];

        if (switch (dir) {
            .N => (v & 0b0010) != 0 and pos.y > 0 and !self.map.map[pos.y - 1][pos.x],
            .S => (v & 0b0001) != 0 and pos.y < size - 1 and !self.map.map[pos.y + 1][pos.x],
            .E => (v & 0b0100) != 0 and pos.x < size - 1 and !self.map.map[pos.y][pos.x + 1],
            .W => (v & 0b1000) != 0 and pos.x > 0 and !self.map.map[pos.y][pos.x - 1],
        }) {
            std.debug.print("{any} {any}\n", .{ dir, pos });
            self.obstacle_location_count += 1;
        }
    }
};

fn predictRoute(map: Map, allocator: std.mem.Allocator) !usize {
    var record = try Record.init(allocator, map);
    defer record.deinit();

    const map_width = map.east_west.len;
    var gaurd_pos: @FieldType(Map, "gaurd_position") = map.gaurd_position;
    while (true) {
        const pos = gaurd_pos.pos;
        const dir = gaurd_pos.dir;

        const S = struct {
            fn compare(context: usize, item: usize) std.math.Order {
                return std.math.order(context, item);
            }
        };

        const lane = switch (dir) {
            .N, .S => map.north_south[pos.x].items,
            else => map.east_west[pos.y].items,
        };

        if (lane.len == 0) {
            break;
        }

        const p = switch (dir) {
            .N, .S => std.sort.upperBound(usize, lane, pos.y, S.compare),
            .E, .W => std.sort.upperBound(usize, lane, pos.x, S.compare),
        };

        switch (dir) {
            .N => {
                const y = if (p == 0) 0 else lane[p - 1] + 1;
                record.recordWalk(.N, pos, pos.y + 1 - y);

                if (p == 0) {
                    break;
                } else {
                    gaurd_pos = .{ .dir = .E, .pos = .{ .y = y, .x = pos.x } };
                }
            },
            .S => {
                const y = if (p == lane.len) map_width - 1 else lane[p] - 1;
                record.recordWalk(.S, pos, y + 1 - pos.y);

                if (p == lane.len) {
                    break;
                } else {
                    gaurd_pos = .{ .dir = .W, .pos = .{ .y = y, .x = pos.x } };
                }
            },
            .E => {
                const x = if (p == lane.len) map_width - 1 else lane[p] - 1;
                record.recordWalk(.E, pos, x + 1 - pos.x);

                if (p == lane.len) {
                    break;
                } else {
                    gaurd_pos = .{ .dir = .S, .pos = .{ .y = pos.y, .x = x } };
                }
            },
            .W => {
                const x = if (p == 0) 0 else lane[p - 1] + 1;
                record.recordWalk(.W, pos, pos.x + 1 - x);

                if (p == 0) {
                    break;
                } else {
                    gaurd_pos = .{ .dir = .N, .pos = .{ .y = pos.y, .x = x } };
                }
            },
        }
    }

    // std.debug.print("\n", .{});
    // for (0..map_width) |y| {
    // for (0..map_width) |x| {
    // const i = y * map_width + x;
    // std.debug.print("{x}", .{record.visited[i]});
    // }
    // std.debug.print("\n", .{});
    // }

    return record.count;
}

pub fn partOne(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var map = try Map.init(input, allocator);
    defer map.deinit();

    return predictRoute(map, allocator);
}

pub fn partTwo(input: []const u8, allocator: std.mem.Allocator) !u64 {
    _ = &input;
    _ = &allocator;
    return error.NotImplemented;
}

const sample_input =
    \\....#.....
    \\.........#
    \\..........
    \\..#.......
    \\.......#..
    \\..........
    \\.#..^.....
    \\........#.
    \\#.........
    \\......#...
;

const testing = @import("std").testing;
test "part one" {
    try testing.expectEqual(41, partOne(sample_input, std.testing.allocator));
}

test "part two" {
    return error.SkipZigTest;
}
