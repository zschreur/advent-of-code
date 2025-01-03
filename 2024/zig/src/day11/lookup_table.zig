const std = @import("std");

pub const LookupTableEntry = struct {
    val: u64,
    counts_mem: []usize,
    counts: []usize,
    end: struct { usize, usize },
};

pub const BlinkLookupTable = struct {
    items: []LookupTableEntry,
    leaves: []u64,

    const Self = @This();

    pub fn get(self: Self, stone: u64) ?LookupTableEntry {
        if (std.sort.binarySearch(
            LookupTableEntry,
            self.items,
            stone,
            struct {
                fn cmp(key: u64, mid_item: LookupTableEntry) std.math.Order {
                    return std.math.order(key, mid_item.val);
                }
            }.cmp,
        )) |i| {
            return self.items[i];
        }

        return null;
    }

    pub fn deinit(self: Self, allocator: anytype) void {
        for (self.items) |item| {
            allocator.free(item.counts_mem);
        }
        allocator.free(self.items);
        allocator.free(self.leaves);
    }
};

