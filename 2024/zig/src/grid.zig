const std = @import("std");
const indexedIterator = @import("./itertools.zig").indexedIterator;

pub const GridOptions = struct {
    T: type,
    comptime square_grid: bool = true,
};

pub const GridDirection = enum {
    N,
    S,
    E,
    W,
};

pub const Position = struct {
    x: usize,
    y: usize,

    pub fn move(self: @This(), comptime dir: GridDirection, grid: anytype) ?@This() {
        switch (dir) {
            .N => if (self.y > 0) return .{ .x = self.x, .y = self.y - 1 } else return null,
            .S => if (self.y < grid.height - 1) return .{
                .x = self.x,
                .y = self.y + 1,
            } else return null,
            .E => if (self.x < grid.width - 1) return .{
                .x = self.x + 1,
                .y = self.y,
            } else return null,
            .W => if (self.x > 0) return .{ .x = self.x - 1, .y = self.y } else return null,
        }
    }
};

pub fn Grid(options: GridOptions) type {
    return struct {
        /// Holds the entire grid. Index i = (height * y + x)
        buf: []options.T,
        width: usize,
        height: usize,
        allocator: std.mem.Allocator,

        const Self = @This();
        pub fn initFromInputString(s: []const u8, allocator: std.mem.Allocator) !Self {
            const width = std.mem.indexOfScalar(u8, s, '\n').?;
            const height = blk: {
                if (options.square_grid) break :blk width;
                var it = std.mem.splitScalar(u8, s, '\n');
                var count: usize = 0;
                while (it.next()) |_| count += 1;

                break :blk count;
            };

            const buf = try allocator.alloc(u8, width * height);

            var it = blk: {
                const it = std.mem.splitScalar(u8, s, '\n');
                break :blk indexedIterator(it);
            };
            while (it.next()) |vi| {
                const line = vi.value;
                @memcpy(buf[vi.index * height ..][0..line.len], line);
            }

            return .{
                .buf = buf,
                .width = width,
                .height = height,
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {
            self.allocator.free(self.buf);
        }

        pub fn get(self: Self, position: Position) ?options.T {
            if (position.x > self.width or position.y > self.height) return null;
            return self.buf[position.y * self.height + position.x];
        }
    };
}
