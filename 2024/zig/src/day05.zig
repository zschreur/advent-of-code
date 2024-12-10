const std = @import("std");

const Puzzle = struct {
    ordering: std.AutoHashMap(u8, std.ArrayList(u8)),
    updates: std.ArrayList(std.ArrayList(u8)),
    allocator: std.mem.Allocator,

    const Self = @This();

    fn deinit(self: *Self) void {
        {
            var it = self.ordering.valueIterator();
            while (it.next()) |v| {
                v.deinit();
            }
            self.ordering.deinit();
        }

        for (self.updates.items) |update| {
            update.deinit();
        }
        self.updates.deinit();
    }

    fn init(input: []const u8, allocator: std.mem.Allocator) !Self {
        var it = std.mem.splitScalar(u8, input, '\n');

        var ordering = std.AutoHashMap(u8, std.ArrayList(u8)).init(allocator);
        while (it.next()) |line| {
            if (line.len == 0) {
                break;
            }

            var mid_it = std.mem.splitScalar(u8, line, '|');
            const l = try std.fmt.parseInt(u8, mid_it.first(), 10);
            const r = try std.fmt.parseInt(u8, mid_it.rest(), 10);

            var res = try ordering.getOrPut(l);
            if (!res.found_existing) {
                res.value_ptr.* = std.ArrayList(u8).init(allocator);
            }
            try res.value_ptr.append(r);
        }

        var updates = std.ArrayList(std.ArrayList(u8)).init(allocator);
        while (it.next()) |line| {
            var update = std.ArrayList(u8).init(allocator);
            var num_it = std.mem.splitScalar(u8, line, ',');
            while (num_it.next()) |num_s| {
                const num = try std.fmt.parseInt(u8, num_s, 10);
                try update.append(num);
            }

            try updates.append(update);
        }

        return .{
            .ordering = ordering,
            .updates = updates,
            .allocator = allocator,
        };
    }

    // part 1
    fn checkUpdates(self: *Self) !usize {
        var score: u64 = 0;
        update_blk: for (self.updates.items) |update| {
            for (update.items, 0..) |page_number, i| {
                if (self.ordering.get(page_number)) |rules| {
                    for (rules.items) |r| {
                        if (std.mem.indexOfScalar(u8, update.items[0..i], r) != null) {
                            continue :update_blk;
                        }
                    }
                }
            }

            score += update.items[update.items.len / 2];
        }

        return score;
    }

    // part 2
    fn sortIncorrect(self: *Self) !usize {
        var score: u64 = 0;

        for (0..self.updates.items.len) |update_index| {
            var update = self.updates.items[update_index];
            var did_fix: bool = false;
            var i: usize = 0;

            blk: while (i < update.items.len) {
                const page_number = update.items[i];
                if (self.ordering.get(page_number)) |rules| {
                    for (rules.items) |r| {
                        if (std.mem.indexOfScalar(u8, update.items[0..i], r)) |j| {
                            did_fix = true;
                            update.insertAssumeCapacity(j, update.orderedRemove(i));
                            i = j;
                            continue :blk;
                        }
                    }
                }

                i += 1;
            }

            if (did_fix) score += update.items[update.items.len / 2];
        }

        return score;
    }
};

pub fn partOne(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var puzzle = try Puzzle.init(input, allocator);
    defer puzzle.deinit();
    return try puzzle.checkUpdates();
}

pub fn partTwo(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var puzzle = try Puzzle.init(input, allocator);
    defer puzzle.deinit();
    return try puzzle.sortIncorrect();
}

const sample_input =
    \\47|53
    \\97|13
    \\97|61
    \\97|47
    \\75|29
    \\61|13
    \\75|53
    \\29|13
    \\97|29
    \\53|29
    \\61|53
    \\97|53
    \\61|29
    \\47|13
    \\75|47
    \\97|75
    \\47|61
    \\75|61
    \\47|29
    \\75|13
    \\53|13
    \\
    \\75,47,61,53,29
    \\97,61,53,29,13
    \\75,29,13
    \\75,97,47,61,53
    \\61,13,29
    \\97,13,75,29,47
;

const testing = std.testing;
test "part one" {
    try testing.expectEqual(143, partOne(sample_input, std.testing.allocator));
}

test "part two" {
    try testing.expectEqual(123, partTwo(sample_input, std.testing.allocator));
}
