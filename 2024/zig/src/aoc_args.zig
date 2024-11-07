const std = @import("std");

const usage =
    \\Usage: ./advent-of-code [options]
    \\
    \\Options:
    \\  --day-number (-d) DAY
    \\  --input-file (-i) INPUT_JSON_FILE
    \\  --help (-h)
    \\
;

pub const PuzzleArgs = struct {
    day: u8,
    input_file_path: []const u8,
    allocator: std.mem.Allocator,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) !PuzzleArgs {
        const args = try std.process.argsAlloc(allocator);
        defer std.process.argsFree(allocator, args);

        var opt_input_file_path: ?[]const u8 = null;
        var opt_day_number: ?u8 = null;
        {
            var i: usize = 1;
            while (i < args.len) : (i += 1) {
                const arg = args[i];
                if (std.mem.eql(u8, "--help", arg) or std.mem.eql(u8, "-h", arg)) {
                    Self.exitHelp();
                } else if (std.mem.eql(u8, "--day-number", arg) or std.mem.eql(u8, "-d", arg)) {
                    i += 1;
                    if (i > args.len) Self.fatal("expected arg after '{s}'", .{arg});
                    if (opt_day_number != null) Self.fatal("duplicated {s} argument", .{arg});
                    opt_day_number = try std.fmt.parseInt(u8, args[i], 10);
                } else if (std.mem.eql(u8, "--input-file", arg) or std.mem.eql(u8, "-i", arg)) {
                    i += 1;
                    if (i > args.len) Self.fatal("expected arg after '{s}'", .{arg});
                    if (opt_input_file_path != null) Self.fatal("duplicated {s} argument", .{arg});
                    opt_input_file_path = args[i];
                } else {
                    Self.fatal("unrecognized arg: '{s}'", .{arg});
                }
            }
        }

        const input_file_path = opt_input_file_path orelse Self.fatal("missing --input-file", .{});
        const day = opt_day_number orelse Self.fatal("missing --day-number", .{});

        const buf = try allocator.alloc(u8, input_file_path.len);
        @memcpy(buf, input_file_path);

        return .{
            .day = day,
            .input_file_path = buf,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *const Self) void {
        self.allocator.free(self.input_file_path);
    }

    fn fatal(comptime format: []const u8, args: anytype) noreturn {
        std.debug.print(format ++ "\n", args);
        std.process.exit(1);
    }

    fn exitHelp() noreturn {
        std.debug.print(usage, .{});
        std.process.exit(0);
    }
};
