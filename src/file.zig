const std = @import("std");

pub fn openInputFile(day: usize) !std.fs.File {
    const input_dir = try std.fs.cwd().openDir("input", .{ .access_sub_paths = false });
    var buffer: [100]u8 = undefined;
    const file_name = try std.fmt.bufPrint(&buffer, "day-{s}.txt", .{std.fmt.digits2(day)});
    return try input_dir.openFile(file_name, .{ .mode = .read_only });
}
