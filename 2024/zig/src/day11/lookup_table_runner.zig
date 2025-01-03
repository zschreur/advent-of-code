const std = @import("std");
const LookupTableEntry = @import("./lookup_table.zig").LookupTableEntry;
const BlinkLookupTable = @import("./lookup_table.zig").BlinkLookupTable;
const splitStone = @import("./split_stone.zig").splitStone;

pub fn CreateLookupTableRunner(
    comptime initial_stone: u64,
    comptime blink_count: usize,
) type {
    return struct {
        leaves: std.ArrayList(u64),
        stack: BlinkStack(StackItem, blink_count),
        results: std.ArrayList(LookupTableEntry),
        allocator: std.mem.Allocator,
        depth_counts: DepthCounts,

        const StackItem = struct {
            current: u64,
            next: ?u64,
            leaves_offset: usize,
        };

        const DepthCounts = struct {
            mem: []usize,

            fn init(allocator: anytype) !@This() {
                const mem = try allocator.alloc(usize, (blink_count * (blink_count + 1)) / 2);
                @memset(mem, 0);
                return .{ .mem = mem };
            }

            fn deinit(self: @This(), allocator: anytype) void {
                allocator.free(self.mem);
            }

            fn get(self: @This(), depth: usize) []usize {
                const len = blink_count - depth;
                const offset = self.mem.len - (len * (len + 1) / 2);

                return self.mem[offset..][0..len];
            }

            fn update(self: *@This(), depth: usize, amount: usize) void {
                for (0..depth) |i| {
                    const j = depth - i - 1;
                    self.get(i)[j] += amount;
                }
            }
        };

        pub fn init(allocator: anytype) !@This() {
            return .{
                .leaves = std.ArrayList(u64).init(allocator),
                .results = std.ArrayList(LookupTableEntry).init(allocator),
                .stack = BlinkStack(StackItem, blink_count).init(),
                .allocator = allocator,
                .depth_counts = try DepthCounts.init(allocator),
            };
        }

        pub fn deinit(self: @This()) void {
            self.depth_counts.deinit(self.allocator);
        }

        const Self = @This();
        fn blink(self: *Self, stone: u64) !void {
            const split_stone = splitStone(stone);
            try self.stack.push(.{
                .current = split_stone.left,
                .next = split_stone.right,
                .leaves_offset = self.leaves.items.len,
            });

            self.depth_counts.update(
                self.stack.items.len,
                if (split_stone.right != null) 2 else 1,
            );

            if (self.stack.items.len == blink_count) {
                if (self.stack.end()) |end| {
                    try self.leaves.append(end.current);
                    if (end.next) |next| try self.leaves.append(next);
                }
            }
        }

        fn updateItem(self: *Self, value: u64, leaves_offset: usize) !void {
            const cmp = struct {
                fn cmp(key: u64, mid_item: LookupTableEntry) std.math.Order {
                    return std.math.order(key, mid_item.val);
                }
            }.cmp;
            var item = if (std.sort.binarySearch(
                LookupTableEntry,
                self.results.items,
                value,
                cmp,
            )) |i| &self.results.items[i] else blk: {
                const i = std.sort.lowerBound(
                    LookupTableEntry,
                    self.results.items,
                    value,
                    cmp,
                );

                const mem = try self.allocator.alloc(usize, blink_count);
                try self.results.insert(i, .{
                    .val = value,
                    .counts_mem = mem,
                    .counts = mem[0..0],
                    .end = .{ 0, 0 },
                });

                break :blk &self.results.items[i];
            };

            const counts_for_item = self.depth_counts.get(self.stack.items.len);
            if (counts_for_item.len > item.counts.len) {
                const offset = item.counts.len;
                const new_item_count = counts_for_item.len - offset;
                @memcpy(item.*.counts_mem[offset..][0..new_item_count], counts_for_item[offset..]);
                item.*.counts = item.counts_mem[0..counts_for_item.len];
                item.*.end = .{ leaves_offset, self.leaves.items.len };
            }
            @memset(counts_for_item, 0);
        }

        const Visit = enum {
            empty,
            next,
            visited,
        };

        fn visitEnd(self: *Self) !Visit {
            if (self.stack.end()) |end| {
                try self.updateItem(end.current, end.leaves_offset);

                if (end.next) |next| {
                    end.*.current = next;
                    end.*.next = null;
                    return .next;
                }

                return .visited;
            }

            return .empty;
        }

        pub fn run(self: *Self) !BlinkLookupTable {
            try self.blink(initial_stone);

            outer: while (self.stack.items.len > 0) {
                if (self.stack.items.len == blink_count) {
                    while (self.stack.pop()) |_| {
                        switch (try self.visitEnd()) {
                            .empty => break :outer,
                            .next => continue :outer,
                            .visited => {},
                        }
                    }
                } else {
                    try self.blink(self.stack.end().?.current);
                }
            }

            try self.updateItem(initial_stone, 0);
            const items = try self.results.toOwnedSlice();
            const leaves = try self.leaves.toOwnedSlice();

            return .{ .items = items, .leaves = leaves };
        }
    };
}

fn BlinkStack(T: type, blink_count: usize) type {
    const StackItem = T;

    return struct {
        mem: [blink_count]StackItem = undefined,
        items: []StackItem,

        const Self = @This();

        const StackError = error{
            OutOfMemory,
        };

        fn init() Self {
            var result = Self{ .items = undefined };
            result.items = result.mem[0..0];
            return result;
        }

        fn push(self: *Self, item: StackItem) StackError!void {
            if (self.items.len < self.mem.len) {
                self.items = self.mem[0 .. self.items.len + 1];
                self.items[self.items.len - 1] = item;
            } else {
                return StackError.OutOfMemory;
            }
        }

        fn pop(self: *Self) ?StackItem {
            if (self.items.len > 0) {
                const result = self.items[self.items.len - 1];
                self.items = self.items[0 .. self.items.len - 1];
                return result;
            }

            return null;
        }

        fn end(self: Self) ?*StackItem {
            return if (self.items.len > 0) &self.items[self.items.len - 1] else null;
        }
    };
}

const testing = std.testing;
test "create lookup table" {
    const lookup_table = blk: {
        var runner = try CreateLookupTableRunner(0, 4).init(testing.allocator);
        defer runner.deinit();
        break :blk try runner.run();
    };
    defer lookup_table.deinit(testing.allocator);
    // 0
    // 1
    // 2024
    // 20 24
    // 2 0 2 4

    // 0 1 20 24 2024

    try testing.expectEqual(4, lookup_table.leaves.len);
    try testing.expectEqual(5, lookup_table.items.len);

    const zero = lookup_table.get(0).?;
    try testing.expectEqual(4, zero.counts.len);
    try testing.expectEqual(1, zero.counts[0]);
    try testing.expectEqual(1, zero.counts[1]);
    try testing.expectEqual(2, zero.counts[2]);
    try testing.expectEqual(4, zero.counts[3]);
}
