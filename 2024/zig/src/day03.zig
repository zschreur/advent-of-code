const std = @import("std");

fn checkMul(seq: []const u8) ?u64 {
    var it = std.mem.splitScalar(u8, seq, ',');

    const x = std.fmt.parseInt(
        u32,
        std.mem.trim(u8, it.first(), &std.ascii.whitespace),
        10,
    ) catch return null;
    const y =
        std.fmt.parseInt(
        u32,
        std.mem.trim(u8, it.rest(), &std.ascii.whitespace),
        10,
    ) catch return null;

    return x * y;
}

fn tryMulInstruction(seq: []const u8) ?u64 {
    if (std.mem.indexOfScalarPos(u8, seq, 4, ')')) |maybe_end| {
        return checkMul(seq[4..maybe_end]);
    }

    return null;
}

pub fn partOne(input: []const u8) !u64 {
    var sum: u64 = 0;

    var pos: usize = 0;
    while (std.mem.indexOfPos(u8, input, pos, "mul(")) |i| {
        if (tryMulInstruction(input[i..])) |r| sum += r;
        pos = i + 1;
    }

    return sum;
}

pub fn partTwo(input: []const u8) !u64 {
    var sum: u64 = 0;

    var enable = true;
    var pos: usize = 0;
    while (std.mem.indexOfPos(u8, input, pos, "mul(")) |i| {
        const slice = input[pos..i];
        const do_opt = std.mem.lastIndexOf(u8, slice, "do()");
        const dont_opt = std.mem.lastIndexOf(u8, slice, "don't()");

        if (dont_opt) |dont| {
            if (do_opt) |do| {
                enable = do > dont;
            }
            enable = false;
        } else if (do_opt != null) {
            enable = true;
        }

        if (enable) {
            if (tryMulInstruction(input[i..])) |r| sum += r;
        }

        pos = i + 1;
    }

    return sum;
}

const testing = @import("std").testing;
test "part one" {
    try testing.expectEqual(161, partOne(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    ));
}

test "part two" {
    try testing.expectEqual(48, partTwo(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    ));
}
