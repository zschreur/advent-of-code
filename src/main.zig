const std = @import("std");
const openInputFile = @import("./file.zig").openInputFile;

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();
    const d = try std.fmt.parseInt(usize, args.next().?, 10);
    const file = try openInputFile(d);
    defer file.close();
}
