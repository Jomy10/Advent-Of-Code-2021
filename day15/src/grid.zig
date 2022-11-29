const Vec = @import("vec.zig").Vec;

const Size = struct {
    rows: usize,
    cols: usize,
};

pub const Grid = struct {
    original: *const Vec(Vec(u8)),
    /// Size of the original map
    map_size: Size,

    pub fn init(grid: *const Vec(Vec(u8)), row_len: usize, col_len: usize) Grid {
        return Grid{ .original = grid, .map_size = Size{
            .rows = row_len,
            .cols = col_len,
        } };
    }

    pub fn get(self: Grid, r: usize, c: usize) u32 {
        const row: usize = r % self.map_size.rows;
        const col: usize = c % self.map_size.cols;

        // const std = @import("std");
        // std.log.warn("row - {}, col - {}, row size - {}, col size {}", .{ row, col, self.map_size.rows, self.map_size.cols });

        const val: u8 = self.original.items[row].items[col];

        const shift_right: usize = r / self.map_size.rows;
        const shift_down: usize = c / self.map_size.cols;
        const total_shift: usize = shift_right + shift_down;

        const item_val: u32 = @as(u32, val) + @truncate(u32, total_shift);
        var final_val: u32 = item_val;
        // const final_val: u32 = item_val % 10; // wrap back if > 9
        while (final_val > 9) {
            final_val -= 9;
        }

        // std.log.warn("r {}, c {}, row {}, col {}, val {}, shift {}, final {}", .{ r, c, row, col, val, total_shift, final_val });

        // std.log.warn("shift: {} - item_val {} - final_val {} - original val - {}", .{ total_shift, item_val, final_val, val });

        return final_val;
    }
};

const testing = @import("std").testing;
const expectEqual = testing.expectEqual;
const test_allocator = testing.allocator;

test "grid" {
    var vec = try Vec(Vec(u8)).init(test_allocator);
    var x: usize = 0;
    while (x < 10) : (x += 1) {
        var y: u8 = 0;
        try vec.push(try Vec(u8).init(test_allocator));
        while (y < 10) : (y += 1) {
            try vec.items[x].push(y);
        }
    }

    defer {
        for (vec.items) |_vec| {
            _vec.free();
        }
        vec.free();
    }

    const grid = Grid.init(&vec, 10, 10);

    x = 0;
    while (x < 10) : (x += 1) {
        var y: usize = 0;
        while (y < 10) : (y += 1) {
            try expectEqual(
                @as(u32, vec.get(x).?.get(y).?),
                grid.get(x, y),
            );
        }
    }

    try expectEqual(
        Size{ .rows = 10, .cols = 10 },
        grid.map_size,
    );

    try expectEqual(
        @as(u32, vec.get(9).?.get(9).?),
        grid.get(grid.map_size.rows - 1, grid.map_size.cols - 1),
    );

    try expectEqual(
        @as(u32, vec.get(0).?.get(0).? + 1),
        grid.get(grid.map_size.rows, 0),
    );

    try expectEqual(
        @as(u32, vec.get(0).?.get(0).? + 2),
        grid.get(grid.map_size.rows, grid.map_size.cols),
    );

    try expectEqual(
        @as(u32, vec.get(0).?.get(0).? + 1),
        grid.get(0, grid.map_size.cols),
    );
}
