const std = @import("std");

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
