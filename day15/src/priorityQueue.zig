const std = @import("std");
const ArrayList = std.ArrayList;

pub fn PriorityQueue(comptime T: type) type {
    return struct {
        const Self = @This();

        priorities: ArrayList(u32),
        items: ArrayList(T),
        allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator) Self {
            const pr = ArrayList(u32).init(allocator);
            const it = ArrayList(T).init(allocator);
            return Self{
                .priorities = pr,
                .items = it,
                .allocator = allocator,
            };
        }

        pub fn pop(self: *Self) ?T {
            if (self.priorities.items.len == 0) return null;

            var lowest: u32 = 9999999;
            var lowest_idx: usize = 0;
            for (self.priorities.items) |priority, idx| {
                if (priority < lowest) {
                    lowest = priority;
                    lowest_idx = idx;
                }
            }

            if (lowest_idx == -1) return null;

            _ = self.priorities.swapRemove(lowest_idx);
            const item = self.items.swapRemove(lowest_idx);

            return item;
        }

        pub fn push(self: *Self, item: T, priority: u32) !void {
            try self.priorities.append(priority);
            try self.items.append(item);
        }

        pub fn free(self: Self) void {
            // self.allocator.free(self.priorities);
            // self.allocator.free(self.items);
            self.priorities.deinit();
            self.items.deinit();
        }
    };
}
