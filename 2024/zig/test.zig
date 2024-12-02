const std = @import("std");

test {
    const a: i8 = -1;
    const b: i8 = 1;
    const c: i8 = -3;
    const d: i8 = 3;

    std.debug.print("{any}\n", .{a ^ b}); // bad
    std.debug.print("{any}\n", .{a ^ c}); // good
    std.debug.print("{any}\n", .{b ^ c}); // bad
    std.debug.print("{any}\n", .{b ^ d}); // good
}
