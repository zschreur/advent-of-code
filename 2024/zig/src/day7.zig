const std = @import("std");

fn Calibration(concat: bool) type {
    return struct {
        test_value: u64,
        nums: []u64,
        allocator: std.mem.Allocator,

        const Self = @This();

        fn init(line: []const u8, allocator: anytype) !Self {
            const split = std.mem.indexOfScalar(u8, line, ':').?;
            const test_value = try std.fmt.parseInt(u64, line[0..split], 10);

            const nums_len = std.mem.count(u8, line[split + 2 ..], " ") + 1;
            const nums = try allocator.alloc(u64, nums_len);
            errdefer allocator.free(nums);

            var it = std.mem.splitScalar(u8, line[split + 2 ..], ' ');
            var i: usize = 0;
            while (it.next()) |s| {
                nums[i] = try std.fmt.parseInt(u64, s, 10);
                i += 1;
            }

            return .{ .test_value = test_value, .nums = nums, .allocator = allocator };
        }

        fn deinit(self: *Self) void {
            self.allocator.free(self.nums);
        }

        fn check(self: *Self) bool {
            if (concat) {
                return testEquationConcat(self.test_value, self.nums);
            } else {
                return testEquation(self.test_value, self.nums);
            }
        }

        fn testEquation(val: u64, nums: []u64) bool {
            if (nums.len == 1 and val == nums[0]) return true;
            if (nums.len == 0) return false;

            const end = nums[nums.len - 1];
            if (val % end == 0 and testEquation(val / end, nums[0 .. nums.len - 1])) return true;
            if (val > end) return testEquation(val - end, nums[0 .. nums.len - 1]);

            return false;
        }

        fn testEquationConcat(val: u64, nums: []u64) bool {
            if (nums.len == 2 and val == concatNums(nums[0], nums[1])) return true;
            if (nums.len == 1 and val == nums[0]) return true;
            if (nums.len == 0) return false;

            const end = nums[nums.len - 1];
            if (val % end == 0) {
                if (testEquationConcat(val / end, nums[0 .. nums.len - 1])) return true;
            }
            if (val > end) {
                if (testEquationConcat(val - end, nums[0 .. nums.len - 1])) return true;
            }
            if (inverseConcat(val, end)) |n| {
                if (testEquationConcat(n, nums[0..nums.len-1])) return true;
            }

            return false;
        }
    };
}

fn concatNums(a: u64, b: u64) u64 {
    const digits = std.math.log10(b) + 1;
    return std.math.pow(u64, 10, digits) * a + b;
}

fn inverseConcat(a: u64, b: u64) ?u64 {
    const digits = std.math.log10(b) + 1;
    const pow = std.math.pow(u64, 10, digits);
    if (a % pow != b) return null;
    return (a - b) / pow;
}

pub fn partOne(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var line_it = std.mem.splitScalar(u8, input, '\n');

    var sum: u64 = 0;
    while (line_it.next()) |line| {
        if (line.len == 0) continue;
        var cal = try Calibration(false).init(line, allocator);
        defer cal.deinit();

        if (cal.check()) {
            sum += cal.test_value;
        }
    }

    return sum;
}

pub fn partTwo(input: []const u8, allocator: std.mem.Allocator) !u64 {
    var line_it = std.mem.splitScalar(u8, input, '\n');

    var sum: u64 = 0;
    while (line_it.next()) |line| {
        if (line.len == 0) continue;
        var cal = try Calibration(true).init(line, allocator);
        defer cal.deinit();

        if (cal.check()) sum += cal.test_value;
    }

    return sum;
}

const sample_input =
    \\190: 10 19
    \\3267: 81 40 27
    \\83: 17 5
    \\156: 15 6
    \\7290: 6 8 6 15
    \\161011: 16 10 13
    \\192: 17 8 14
    \\21037: 9 7 18 13
    \\292: 11 6 16 20
;

const testing = @import("std").testing;
test "part one" {
    try std.testing.expectEqual(3749, partOne(sample_input, std.testing.allocator));
}

test "concatNums" {
    try std.testing.expectEqual(12, concatNums(1, 2));
    try std.testing.expectEqual(123, concatNums(1, 23));
    try std.testing.expectEqual(123, concatNums(12, 3));
    try std.testing.expectEqual(1234, concatNums(12, 34));
    try std.testing.expectEqual(1234, concatNums(1, 234));
    try std.testing.expectEqual(1234, concatNums(123, 4));
}

test "part two" {
    try std.testing.expectEqual(11387, partTwo(sample_input, std.testing.allocator));
}
