const std = @import("std");
// const ArrayList = std.ArrayList;
const fs = std.fs;
const Vec = @import("vec.zig").Vec;
const stdout = std.io.getStdOut().writer();

pub fn readInput(allocator: std.mem.Allocator, input_file: []const u8) !*Vec(Vec(u8)) {
    const file = try fs.cwd().openFile(input_file, .{});
    defer file.close();

    // const file_size = try file.getEndPos();

    var buffer: [100 * 100 + 100]u8 = undefined;
    const bytes_read = try file.read(buffer[0..buffer.len]);

    const input_string = buffer[0..bytes_read];

    // var input_arr: col_len][row_len]u8 = undefined;
    // var input_arr = try Vec(Vec(u8)).init(allocator);
    var input_arr = try allocator.create(Vec(Vec(u8)));
    input_arr.* = try Vec(Vec(u8)).init(allocator);
    var col: u32 = 0;
    var row: u32 = 0;
    // try input_arr.push(Vec(u8).init(allocator));
    try input_arr.push(try Vec(u8).init((allocator)));
    for (input_string) |char| {
        if (char == '\n') {
            col = 0;
            row += 1;
            try input_arr.push(try Vec(u8).init(allocator));
            continue;
        }

        // input_arr[row][col] = char;
        try input_arr.items[row].push(char);

        col += 1;
    }

    // for (input_arr.items) |_row| {
    //     for (_row.items) |item| {
    //         try stdout.print("{c}", .{item});
    //     }
    //     try stdout.print("\n", .{});
    // }

    // input as char to integer
    var r: usize = 0;
    while (r < input_arr.len) : (r += 1) {
        var c: usize = 0;
        while (c < input_arr.items[0].len) : (c += 1) {
            input_arr.items[r].items[c] -= 48;
            // try stdout.print("{}", .{input_arr.items[r].items[c]});
        }
        // try stdout.print("\n", .{});
    }
    // for (input_arr.items) |*_row, row_n| {
    //     try stdout.print("{}\n", .{row_n});
    //     for (_row.items) |*item| {
    //         item.* -= 48; // ASCII value of 0
    //     }
    // }

    return input_arr;
}

pub fn deallocInput(input: *Vec(Vec(u8))) void {
    // for (input.items) |row, i| {
    //     stdout.print("Dealloc {}\n", .{i}) catch {};
    //     row.free();
    // }
    var i: usize = 0;
    // stdout.print("len: {}\n", .{inputlen}) catch {};
    while (i < input.len) : (i += 1) {
        input.items[i].free();
    }
    input.free();
}
