const std = @import("std");
const Grid = @import("./grid.zig").Grid;
const Position = @import("./grid.zig").Position;
const GridDirection = @import("./grid.zig").GridDirection;

const DSU = struct {
    mem: []usize,
    size: usize,
    allocator: std.mem.Allocator,

    const Self = @This();

    fn init(size: usize, allocator: std.mem.Allocator) !Self {
        const mem = try allocator.alloc(usize, size * 2);
        const self: Self = .{ .mem = mem, .size = size, .allocator = allocator };
        self.reset();

        return self;
    }

    fn reset(self: Self) void {
        @memset(self.mem[self.size..], 0);

        // initialize so that every item is its own set
        for (0..self.size) |i| {
            self.mem[i] = i;
        }
    }

    fn parent(self: Self) []usize {
        return self.mem[0..self.size];
    }

    fn rating(self: Self) []usize {
        return self.mem[self.size..];
    }

    fn deinit(self: *Self) void {
        self.allocator.free(self.mem);
    }

    fn makeSet(self: Self, v: usize) void {
        self.parent()[v] = v;
    }

    fn findSet(self: Self, v: usize) usize {
        if (v == self.parent()[v]) {
            return v;
        }

        return self.findSet(self.parent()[v]);
    }

    fn unionSet(self: Self, a: usize, b: usize) void {
        const a_set = self.findSet(a);
        const b_set = self.findSet(b);

        if (a_set != b_set) {
            self.parent()[b_set] = a_set;
        }
    }
};

fn findRating(base_set: usize, pos: Position, pos_value: u8, grid: anytype, dsu: DSU) void {
    const i = pos.y * grid.height + pos.x;
    if (pos_value == 9) {
        dsu.unionSet(base_set, i);
        dsu.rating()[i] = 1;
        return;
    }

    inline for (@typeInfo(GridDirection).@"enum".fields) |field| {
        const dir = @as(GridDirection, @enumFromInt(field.value));
        const position = Position{ .x = i % grid.height, .y = i / grid.height };

        if (position.move(dir, grid)) |next_pos| {
            const next_value = (grid.get(next_pos).?) - '0';
            const next_i = next_pos.y * grid.height + next_pos.x;

            // at this point next is on the grid and we know its position
            if (pos_value + 1 == next_value) {
                // combine sets and adjust rating

                if (dsu.findSet(next_i) != base_set and dsu.rating()[next_i] == 0) {
                    findRating(base_set, next_pos, next_value, grid, dsu);
                }
                dsu.rating()[i] += dsu.rating()[next_i];
                dsu.unionSet(i, next_i);
            }
        }
    }
}

pub fn partOne(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var grid = try Grid(.{ .T = u8 }).initFromInputString(input, allocator);
    defer grid.deinit();

    var total_score: usize = 0;
    var offset: usize = 0;

    var peak_indexes = std.ArrayList(usize).init(allocator);
    defer peak_indexes.deinit();
    var nine_offset: usize = 0;
    while (std.mem.indexOfScalarPos(u8, grid.buf, nine_offset, '9')) |i| {
        try peak_indexes.append(i);
        nine_offset = i + 1;
    }

    var dsu = try DSU.init(grid.buf.len, allocator);
    defer dsu.deinit();

    while (std.mem.indexOfScalarPos(u8, grid.buf, offset, '0')) |i| {
        offset = i + 1;

        const pos = Position{ .x = i % grid.height, .y = i / grid.height };
        findRating(i, pos, 0, grid, dsu);

        for (peak_indexes.items) |j| {
            total_score += dsu.rating()[j];
        }

        dsu.reset();
    }

    return total_score;
}

pub fn partTwo(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var grid = try Grid(.{ .T = u8 }).initFromInputString(input, allocator);
    defer grid.deinit();

    var total_score: usize = 0;
    var offset: usize = 0;

    var dsu = try DSU.init(grid.buf.len, allocator);
    defer dsu.deinit();

    while (std.mem.indexOfScalarPos(u8, grid.buf, offset, '0')) |i| {
        offset = i + 1;

        const pos = Position{ .x = i % grid.height, .y = i / grid.height };
        findRating(i, pos, 0, grid, dsu);
        total_score += dsu.rating()[i];
    }

    return total_score;
}

const sample_input =
    \\89010123
    \\78121874
    \\87430965
    \\96549874
    \\45678903
    \\32019012
    \\01329801
    \\10456732
;

const testing = @import("std").testing;
test "part one" {
    try std.testing.expectEqual(36, partOne(sample_input, testing.allocator));
}

test "part two" {
    try std.testing.expectEqual(81, partTwo(sample_input, testing.allocator));
}
