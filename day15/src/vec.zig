// Because the std.ArrayList was cause segmentation faults

const std = @import("std");

pub fn Vec(comptime T: type) type {
    return struct {
        const Self = @This();
        items: []T,
        len: usize,
        cap: usize,
        allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator) !Self {
            const size = 10;
            return Self{
                .items = try allocator.alloc(T, size),
                .len = 0,
                .cap = size,
                .allocator = allocator,
            };
        }

        pub fn push(self: *Self, val: T) !void {
            if (self.len == self.cap) {
                self.cap *= 2;
                self.items = try realloc(T, self.allocator, self.items, self.cap);
            }
            self.items[self.len] = val;
            self.len += 1;
        }

        pub fn pop(self: *Self) ?T {
            if (self.len == 0) return null;
            return self.items[--self.len];
        }

        pub fn get(self: *const Self, idx: usize) ?T {
            if (self.len <= idx) {
                return null;
            } else {
                return self.items[idx];
            }
        }

        pub fn free(self: Self) void {
            self.allocator.free(self.items);
        }
    };
}

// Zig has no realloc?
fn realloc(comptime T: type, allocator: std.mem.Allocator, items: []T, size: usize) ![]T {
    var new = try allocator.alloc(T, size);
    std.mem.copy(T, new, items);
    allocator.free(items);
    return new;
}
