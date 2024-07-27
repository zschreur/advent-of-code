const std = @import("std");
const nextLine = @import("../file.zig").nextLine;

const ZERO: u8 = "0"[0];

fn calibrationValue(line: []const u8) ?u32 {
    var first: ?u8 = null;
    var last: ?u8 = null;
    var i: usize = 0;

    while (first == null and i < line.len) : (i += 1) {
        const char = line[i];
        if (std.ascii.isDigit(char)) {
            first = char;
            last = char;
        }
    }

    while (i < line.len) : (i += 1) {
        const char = line[i];
        if (std.ascii.isDigit(char)) {
            last = char;
        }
    }

    return (first.? - ZERO) * 10 + last.? - ZERO;
}

pub fn runPartOne(input_reader: anytype) !u32 {
    var res: u32 = 0;
    var buffer: [100]u8 = undefined;
    while (true) {
        if (try nextLine(input_reader, &buffer)) |value| {
            res += calibrationValue(value).?;
        } else {
            break;
        }
    }

    return res;
}

const Error = error{
    Unimplemented,
};

pub fn runPartTwo(_: anytype) !u32 {
    return Error.Unimplemented;
}
const expectEqual = std.testing.expectEqual;
test "part one" {
    const sample_input =
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;
    var stream = std.io.fixedBufferStream(sample_input);

    const res = try runPartOne(stream.reader());
    try expectEqual(142, res);
}

test "part two" {
    const sample_input =
        \\two1nine
        \\eightwothree
        \\abcone2threexyz
        \\xtwone3four
        \\4nineeightseven2
        \\zoneight234
        \\7pqrstsixteen
    ;
    var stream = std.io.fixedBufferStream(sample_input);

    const res = try runPartTwo(stream.reader());
    try expectEqual(281, res);
}
