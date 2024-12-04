const std = @import("std");

const Position = struct {
    x: usize,
    y: usize,
};

const Direction = enum(u3) {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
};

fn searchDirection(board: [][]const u8, pos: Position, pattern: []const u8, dir: Direction) bool {
    if (pattern.len == 0) return true;

    const x = switch (dir) {
        .N, .NW, .NE => b: {
            if (pos.x < pattern.len) return false;
            break :b pos.x - 1;
        },
        .S, .SE, .SW => b: {
            if (pos.x + pattern.len > board.len - 1) return false;
            break :b pos.x + 1;
        },
        else => pos.x,
    };

    const y = switch (dir) {
        .W, .SW, .NW => b: {
            if (pos.y < pattern.len) return false;
            break :b pos.y - 1;
        },
        .E, .SE, .NE => b: {
            if (pos.y + pattern.len > board.len - 1) return false;
            break :b pos.y + 1;
        },
        else => pos.y,
    };

    if (board[y][x] == pattern[0]) return searchDirection(
        board,
        .{ .x = x, .y = y },
        pattern[1..],
        dir,
    );

    return false;
}

fn initBoard(input: []const u8, allocator: anytype) ![][]const u8 {
    const square_size = std.mem.indexOfScalar(u8, input, '\n').?;

    const mem = try allocator.alloc([]const u8, square_size);

    var line_it = std.mem.splitScalar(u8, input, '\n');
    var i: usize = 0;
    while (line_it.next()) |line| {
        mem[i] = line;
        i += 1;
    }

    return mem;
}

fn xmasSearch(input: []const u8, allocator: anytype) !usize {
    const board = try initBoard(input, allocator);
    defer allocator.free(board);

    var xmas_count: usize = 0;
    for (board, 0..) |row, y| {
        var offset: usize = 0;
        while (std.mem.indexOfScalarPos(u8, row, offset, 'X')) |x| {
            xmas_count += b: {
                var count: usize = 0;
                for (0..8) |d| {
                    if (searchDirection(
                        board,
                        .{ .x = x, .y = y },
                        "MAS",
                        @as(Direction, @enumFromInt(d)),
                    )) count += 1;
                }
                break :b count;
            };
            offset = x + 1;
        }
    }

    return xmas_count;
}

fn @"x-masSearch"(input: []const u8, allocator: anytype) !usize {
    const board = try initBoard(input, allocator);
    defer allocator.free(board);

    var xmas_count: usize = 0;
    for (board[1 .. board.len - 1], 1..) |row, y| {
        for (row[1 .. row.len - 1], 1..) |c, x| {
            if (c == 'A') {
                const a = [2]u8{ board[y - 1][x - 1], board[y + 1][x + 1] };
                const b = [2]u8{ board[y + 1][x - 1], board[y - 1][x + 1] };
                if (std.mem.eql(u8, &a, "SM") and std.mem.eql(u8, &b, "SM")) {
                    xmas_count += 1;
                } else if (std.mem.eql(u8, &a, "SM") and std.mem.eql(u8, &b, "MS")) {
                    xmas_count += 1;
                } else if (std.mem.eql(u8, &a, "MS") and std.mem.eql(u8, &b, "SM")) {
                    xmas_count += 1;
                } else if (std.mem.eql(u8, &a, "MS") and std.mem.eql(u8, &b, "MS")) {
                    xmas_count += 1;
                }
            }
        }
    }

    return xmas_count;
}

pub fn partOne(input: []const u8) !usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();
    return xmasSearch(input, allocator);
}

pub fn partTwo(input: []const u8) !usize {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();
    return @"x-masSearch"(input, allocator);
}

const sample_input =
    \\MMMSXXMASM
    \\MSAMXMSMSA
    \\AMXSXMAAMM
    \\MSAMASMSMX
    \\XMASAMXAMM
    \\XXAMMXXAMA
    \\SMSMSASXSS
    \\SAXAMASAAA
    \\MAMMMXMMMM
    \\MXMXAXMASX
;

const testing = @import("std").testing;
test "word search" {
    try std.testing.expectEqual(18, xmasSearch(sample_input, testing.allocator));
}

test "part two" {
    try std.testing.expectEqual(9, @"x-masSearch"(sample_input, testing.allocator));
}
