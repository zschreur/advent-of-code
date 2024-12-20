const std = @import("std");

const Pos = struct { x: usize, y: usize };
const Direction = enum(u2) { N, S, E, W };

const Map = struct {
    /// each array list has the index of obstacles for that row
    east_west: []std.ArrayList(usize),
    /// each array list has the index of obstacles for that column
    north_south: []std.ArrayList(usize),
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
    }

    fn init(input: []const u8, allocator: std.mem.Allocator) !Self {
        var line_it = std.mem.splitScalar(u8, input, '\n');

        // assume square
        const size = line_it.peek().?.len;

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
        };
    }
};

const History = enum(u2) {
    None,
    Dec,
    Inc,
    Both,
};

const Range = struct {
    start: usize,
    end: usize,
    history: History = .None,

    fn updateHistory(self: *@This(), h: History) void {
        if (self.history == .Both) {
            return;
        } else if (self.history == .None) {
            self.history = h;
        } else if (self.history != h) {
            self.history = .Both;
        }
    }
};

const Record = struct {
    visited: []bool,
    obstacles: []bool,
    east_west: []std.ArrayList(Range),
    north_south: []std.ArrayList(Range),
    allocator: std.mem.Allocator,
    map: Map,
    count: usize = 0,
    obstacle_count: usize = 0,

    const Self = @This();

    fn init(allocator: anytype, map: Map) !Self {
        const map_size = map.east_west.len;
        const visited = try allocator.alloc(bool, map_size * map_size);
        const obstacles = try allocator.alloc(bool, map_size * map_size);
        @memset(visited, false);
        @memset(obstacles, false);

        const east_west = try allocator.alloc(std.ArrayList(Range), map.east_west.len);
        const north_south = try allocator.alloc(std.ArrayList(Range), map.north_south.len);

        for (map.east_west, 0..) |row, y| {
            var ary = std.ArrayList(Range).init(allocator);

            var prev: ?usize = null;
            for (row.items) |n| {
                if (prev) |p| {
                    if (n - p > 1) {
                        try ary.append(.{ .start = p + 1, .end = n - 1 });
                    }
                } else if (n > 0) {
                    try ary.append(.{ .start = 0, .end = n - 1 });
                }
                prev = n;
            }
            if (prev != null and prev.? < map_size - 1) {
                try ary.append(.{ .start = prev.? + 1, .end = map_size - 1 });
            } else if (prev == null) {
                try ary.append(.{ .start = 0, .end = map_size - 1 });
            }

            east_west[y] = ary;
        }

        for (map.north_south, 0..) |col, x| {
            var ary = std.ArrayList(Range).init(allocator);

            var prev: ?usize = null;
            for (col.items) |n| {
                if (prev) |p| {
                    if (n - p > 1) {
                        try ary.append(.{ .start = p + 1, .end = n - 1 });
                    }
                } else if (n > 0) {
                    try ary.append(.{ .start = 0, .end = n - 1 });
                }
                prev = n;
            }
            if (prev != null and prev.? < map_size - 1) {
                try ary.append(.{ .start = prev.? + 1, .end = map_size - 1 });
            } else if (prev == null) {
                try ary.append(.{ .start = 0, .end = map_size - 1 });
            }

            north_south[x] = ary;
        }

        return .{
            .visited = visited,
            .obstacles = obstacles,
            .allocator = allocator,
            .map = map,
            .east_west = east_west,
            .north_south = north_south,
        };
    }

    fn deinit(self: *Self) void {
        for (self.east_west) |a| {
            a.deinit();
        }
        self.allocator.free(self.east_west);
        for (self.north_south) |a| {
            a.deinit();
        }
        self.allocator.free(self.north_south);

        self.allocator.free(self.visited);
        self.allocator.free(self.obstacles);
    }

    fn recordWalk(self: *Self, comptime dir: Direction, start: Pos, count: usize) void {
        for (0..count) |i| {
            const pos: Pos = switch (dir) {
                .N => .{ .y = start.y - i, .x = start.x },
                .S => .{ .y = start.y + i, .x = start.x },
                .E => .{ .y = start.y, .x = start.x + i },
                .W => .{ .y = start.y, .x = start.x - i },
            };
            self.markVisited(pos);
            self.checkForObstaclePlacement(dir, pos);

            const lane = switch (dir) {
                .N, .S => self.north_south[pos.x].items,
                .E, .W => self.east_west[pos.y].items,
            };

            const ctx = switch (dir) {
                .N, .W => History.Dec,
                .S, .E => History.Inc,
            };

            const n = switch (dir) {
                .N, .S => pos.y,
                .E, .W => pos.x,
            };
            blk: {
                for (0..lane.len) |j| {
                    if (lane[j].start <= n and lane[j].end >= n) {
                        lane[j].updateHistory(ctx);
                        break :blk;
                    }
                }
                unreachable;
            }
        }
    }

    fn checkForObstaclePlacement(self: *Self, comptime dir: Direction, pos: Pos) void {
        // for each point along the route, if the range that occupies
        // in its column or row has been walked along in the 90 deg turn
        // direction, then placing an obstacle at the next spot would cause
        // a loop
        const map_size = self.map.east_west.len;

        const intersection = switch (dir) {
            .N, .S => self.east_west[pos.y].items,
            .E, .W => self.north_south[pos.x].items,
        };

        const range = b: {
            for (intersection) |range| {
                const i = switch (dir) {
                    .N, .S => pos.x,
                    .E, .W => pos.y,
                };
                if (range.start <= i and range.end >= i) break :b range;
            }

            unreachable;
        };

        switch (dir) {
            .N => {
                if (pos.y == 0) return;

                const cross = range.history == .Inc or range.history == .Both;

                const i = (pos.y - 1) * map_size + pos.x;
                if (cross and !self.obstacles[i]) {
                    self.obstacles[i] = true;
                    self.obstacle_count += 1;
                }
            },
            .S => {
                if (pos.y == map_size - 1) return;

                const cross = range.history == .Dec or range.history == .Both;

                const i = (pos.y + 1) * map_size + pos.x;
                if (cross and !self.obstacles[i]) {
                    self.obstacles[i] = true;
                    self.obstacle_count += 1;
                }
            },
            .E => {
                if (pos.x == map_size - 1) return;

                const cross = range.history == .Inc or range.history == .Both;

                const i = pos.y * map_size + (pos.x + 1);
                if (cross and !self.obstacles[i]) {
                    self.obstacles[i] = true;
                    self.obstacle_count += 1;
                }
            },
            .W => {
                if (pos.x == 0) return;

                const cross = range.history == .Dec or range.history == .Both;

                const i = pos.y * map_size + (pos.x - 1);
                if (cross and !self.obstacles[i]) {
                    self.obstacles[i] = true;
                    self.obstacle_count += 1;
                }
            },
        }
    }

    fn markVisited(self: *Self, pos: Pos) void {
        const size = self.map.east_west.len;
        const i = pos.y * size + pos.x;

        if (!self.visited[i]) {
            self.visited[i] = true;
            self.count += 1;
        }
    }
};

fn predictRoute(map: Map, allocator: std.mem.Allocator, b: bool) !usize {
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

        // lane is a slice of indexes where
        // each index is the location of an obstacle
        const lane = switch (dir) {
            .N, .S => map.north_south[pos.x].items,
            else => map.east_west[pos.y].items,
        };

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

    if (b) {
        return record.count;
    } else {
        return record.obstacle_count;
    }
}

pub fn partOne(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var map = try Map.init(input, allocator);
    defer map.deinit();

    return predictRoute(map, allocator, true);
}

pub fn partTwo(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var map = try Map.init(input, allocator);
    defer map.deinit();

    return predictRoute(map, allocator, false);
}

const sample_input =
    //0123456789
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

const loops =
    //0123456789
    \\..........
    \\..........
    \\..........
    \\..........
    \\....#.....
    \\..........
    \\...#^.....
    \\.....#....
    \\..........
    \\..........
;

const testing = @import("std").testing;
test "part one" {
    try testing.expectEqual(41, partOne(sample_input, std.testing.allocator));
    try testing.expectEqual(7, partOne(loops, std.testing.allocator));
}

test "part two" {
    try testing.expectEqual(6, partTwo(sample_input, std.testing.allocator));
    try testing.expectEqual(1, partTwo(loops, std.testing.allocator));
}
