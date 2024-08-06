const std = @import("std");

pub fn nextLine(reader: anytype, buffer: []u8) !?[]const u8 {
    const line = (try reader.readUntilDelimiterOrEof(
        buffer,
        '\n',
    )) orelse return null;

    // trim annoying windows-only carriage return character
    if (@import("builtin").os.tag == .windows) {
        return std.mem.trimRight(u8, line, "\r");
    } else {
        return line;
    }
}

pub fn openInputFile(day: usize) !std.fs.File {
    const input_dir = try std.fs.cwd().openDir("../../puzzle-inputs/", .{ .access_sub_paths = false });
    var buffer: [100]u8 = undefined;
    const file_name = try std.fmt.bufPrint(&buffer, "day-{s}.txt", .{std.fmt.digits2(day)});
    return try input_dir.openFile(file_name, .{ .mode = .read_only });
}
