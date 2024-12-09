const std = @import("std");
const testing = @import("std").testing;

fn fileHash(file_index: usize, block_start: usize, block_count: usize) u64 {
    // n * (a + l) / 2
    const sum_of_block_indexes = block_count * (block_start + block_count + block_start - 1) / 2;
    return file_index * sum_of_block_indexes;
}

test "fileHash" {
    try testing.expectEqual(6, fileHash(1, 1, 3));
    try testing.expectEqual(28, fileHash(2, 2, 4));
}

pub fn partOne(input: []const u8) !u64 {
    if (input.len % 2 == 0) {
        return error.ExpectedOddLengthInput;
    }

    var hash: u64 = 0;

    // Index of file in input (will be increasing by 2)
    var front_file_index: usize = 0;

    // Index of last file (will be decreasing by 2)
    var end_file_index: usize = input.len - 1;

    var position: usize = 0;

    // As we move parts of end file to block - sub from this number
    // When it hits 0 we can move the end_file_index back and reset this
    var remaining_end_file_blocks: usize = @intCast(input[end_file_index] - '0');
    if (remaining_end_file_blocks == 0) {
        return error.ZeroSizeFile;
    }

    while (front_file_index < end_file_index) {
        const front_file_size: usize = @intCast(input[front_file_index] - '0');

        hash += fileHash(front_file_index / 2, position, front_file_size);
        position += front_file_size;

        var empty_space_to_fill = input[front_file_index + 1] - '0';

        front_file_index += 2;
        while (empty_space_to_fill > 0 and end_file_index > front_file_index) {
            const fill_len = @min(remaining_end_file_blocks, empty_space_to_fill);
            hash += fileHash(end_file_index / 2, position, fill_len);

            position += fill_len;
            empty_space_to_fill -= fill_len;
            remaining_end_file_blocks -= fill_len;

            if (remaining_end_file_blocks == 0) {
                end_file_index -= 2;
                remaining_end_file_blocks = @intCast(input[end_file_index] - '0');
            }
        }
    }

    if (remaining_end_file_blocks > 0) {
        hash += fileHash(end_file_index / 2, position, remaining_end_file_blocks);
    }

    return hash;
}

pub fn partTwo(input: []const u8) !u64 {
    _ = &input;
    return error.NotImplemented;
}

const sample_input = "2333133121414131402";

test "part one" {
    try std.testing.expectEqual(1928, partOne(sample_input));
}

test "part two" {
    return error.SkipZigTest;
}
