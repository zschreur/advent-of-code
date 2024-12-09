pub fn IndexedIterator(T: type) type {
    return struct {
        index: usize = 0,
        iter: T,

        const IndexAndValue = struct {
            value: []const u8,
            index: usize,
        };

        const Self = @This();

        pub fn next(self: *Self) ?IndexAndValue {
            if (self.iter.next()) |value| {
                const result = IndexAndValue{ .value = value, .index = self.index };
                self.index += 1;

                return result;
            }

            return null;
        }
    };
}
