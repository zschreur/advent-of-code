const std = @import("std");
const openInputFile = @import("./file.zig").openInputFile;
const day_one = @import("./days/one.zig");

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();
    const d = try std.fmt.parseInt(usize, args.next().?, 10);
    const file = try openInputFile(d);
    defer file.close();

    const res: [2]u32 = switch (d) {
        1 => .{ try day_one.runPartOne(file.reader()), try day_one.runPartTwo(file.reader()) },
        else => unreachable,
    };
    std.debug.print("Part 1: {d}\nPart 2: {d}\n", .{ res[0], res[1] });
}

test {
    _ = @import("./days/one.zig");
}
