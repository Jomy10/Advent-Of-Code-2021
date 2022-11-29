const std = @import("std");
const stdout = std.io.getStdOut().writer();
// const ArrayList = std.ArrayList;
const Vec = @import("vec.zig").Vec;
const AutoHashMap = std.AutoHashMap;
const PriorityQueue = @import("priorityQueue.zig").PriorityQueue;
const __input = @import("input.zig");
const readInput = __input.readInput;
const deallocInput = __input.deallocInput;
const Grid = @import("grid.zig").Grid;

const Vec2 = struct { x: u32, y: u32 };
fn vec_eql(a: Vec2, b: Vec2) bool {
    const x_eql: bool = a.x == b.x;
    const y_eql: bool = a.y == b.y;
    return x_eql and y_eql;
}

pub fn main() !void {
    const cost = try part1("input.txt", Vec2{ .x = 100, .y = 100 });

    try stdout.print("Part 1 cost: {d}\n", .{cost});

    const cost2 = try part2("input.txt", Vec2{ .x = 100, .y = 100 });

    try stdout.print("Part 2 cost: {d}\n", .{cost2});
}

fn part1(input_file: []const u8, map_size: Vec2) !u32 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = try readInput(allocator, input_file);
    defer {
        deallocInput(input);
        allocator.destroy(input);
    }

    const grid = Grid.init(input, @as(usize, map_size.x), @as(usize, map_size.y));

    const end = Vec2{
        .x = map_size.x - 1,
        .y = map_size.y - 1,
    };

    const cost = try pathfind(allocator, &grid, end);

    return cost;
}

fn part2(input_file: []const u8, original_map_size: Vec2) !u32 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = try readInput(allocator, input_file);
    defer {
        deallocInput(input);
        allocator.destroy(input);
    }

    const grid = Grid.init(input, @as(u32, original_map_size.x), @as(u32, original_map_size.y));

    // var row: usize = 0;
    // while (row < 50) : (row += 1) {
    //     var col: usize = 0;
    //     while (col < 50) : (col += 1) {
    //         try stdout.print("{d}", .{grid.get(row, col)});
    //     }
    //     try stdout.print("\n", .{});
    // }

    const end = Vec2{
        .x = (original_map_size.x * 5) - 1,
        .y = (original_map_size.y * 5) - 1,
    };

    const cost = try pathfind(allocator, &grid, end);

    return cost;
}

/// A* pathfinding
fn pathfind(allocator: std.mem.Allocator, input: *const Grid, end: Vec2) !u32 {
    var frontier = PriorityQueue(Vec2).init(allocator);
    defer frontier.free();
    try frontier.push(Vec2{ .x = 0, .y = 0 }, 0);

    var came_from = AutoHashMap(Vec2, Vec2).init(allocator);
    defer came_from.deinit();

    var cost_so_far = AutoHashMap(Vec2, u32).init(allocator);
    defer cost_so_far.deinit();

    var _current = frontier.pop();
    while (_current != null) : (_current = frontier.pop()) {
        var current = _current.?;

        if (vec_eql(current, end)) {
            break;
        }

        for (neighbours(current, end)) |_next| {
            if (_next == null) continue;
            var next = _next.?;

            var cost_so_far_current: ?u32 = cost_so_far.get(current);
            if (cost_so_far_current == null) cost_so_far_current = 0;
            // const cost = input.get(next.y).?.get(next.x).?;
            const cost = input.get(next.y, next.x);
            const new_cost = cost_so_far_current.? + @as(u32, cost);

            if (!cost_so_far.contains(next) or new_cost < cost_so_far.get(next).?) {
                try cost_so_far.put(next, new_cost);
                const priority = new_cost;
                try frontier.push(next, priority);
                try came_from.put(next, current);
            }
        }
    }

    // Walk path and add costs
    var current_pos = end;
    var total_cost: u32 = 0;
    var path = try Vec(Vec2).init(allocator);
    defer path.free();
    var path_len: u32 = 1;

    // var iter = came_from.iterator();
    // var elem = iter.next();
    // while (elem != null) : (elem = iter.next()) {
    //     try stdout.print("{} -> {}\n", .{ elem.?.key_ptr.*, elem.?.value_ptr.* });
    // }

    try path.push(current_pos);
    // total_cost += input.items[current_pos.y].items[current_pos.x];
    total_cost += input.get(current_pos.y, current_pos.x);
    while (!vec_eql(current_pos, Vec2{ .x = 0, .y = 0 })) {
        const current_pos_tmp = came_from.get(current_pos);
        current_pos = if (current_pos_tmp == null) return Error.CameFromIsNull else current_pos_tmp.?;
        // total_cost += input.items[current_pos.y].items[current_pos.x];
        total_cost += input.get(current_pos.y, current_pos.x);
        // try stdout.print("{} - {}\n", .{ current_pos, input.items[current_pos.y].items[current_pos.x] });
        try path.push(current_pos);
        path_len += 1;
    }
    // total_cost -= input.items[0].items[0]; // Don't count starting position
    total_cost -= input.get(0, 0);

    // Print taken path
    var i: usize = path_len;
    while (i > 0) {
        i -= 1;

        const item = path.items[i];
        try stdout.print("{}, {}", item);
        if (i != 0) {
            try stdout.print(" -> ", .{});
        } else {
            try stdout.print("\n", .{});
        }
    }

    return total_cost;
}

const Error = error{CameFromIsNull};

fn neighbours(pos: Vec2, bounds: Vec2) [4]?Vec2 {
    var _neighbours: [4]?Vec2 = [_]?Vec2{ null, null, null, null };
    if (pos.x != 0) {
        _neighbours[0] = pos;
        _neighbours[0].?.x -= 1;
    }
    if (pos.y != 0) {
        _neighbours[1] = pos;
        _neighbours[1].?.y -= 1;
    }
    if (pos.x != bounds.x) {
        _neighbours[2] = pos;
        _neighbours[2].?.x += 1;
    }
    if (pos.y != bounds.y) {
        _neighbours[3] = pos;
        _neighbours[3].?.y += 1;
    }
    return _neighbours;
}

const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;

test "example_input1" {
    const cost: u32 = try part1("example.txt", Vec2{ .x = 10, .y = 10 });

    try expectEqual(@as(u32, 40), cost);
}

test "example_input2" {
    const cost: u32 = try part2("example.txt", Vec2{ .x = 10, .y = 10 });

    try expectEqual(@as(u32, 315), cost);
}
