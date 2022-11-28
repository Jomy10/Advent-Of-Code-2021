const std = @import("std");
const stdout = std.io.getStdOut().writer();
// const ArrayList = std.ArrayList;
const Vec = @import("vec.zig").Vec;
const AutoHashMap = std.AutoHashMap;
const PriorityQueue = @import("priorityQueue.zig").PriorityQueue;
const __input = @import("input.zig");
const readInput = __input.readInput;
const deallocInput = __input.deallocInput;

const Vec2 = struct { x: u32, y: u32 };
fn vec_eql(a: Vec2, b: Vec2) bool {
    const x_eql: bool = a.x == b.x;
    const y_eql: bool = a.y == b.y;
    return x_eql and y_eql;
}

pub fn main() !void {
    const cost = try part1("input.txt");

    try stdout.print("Part 1 cost: {d}\n", .{cost});

    const cost2 = try part2("input.txt");

    try stdout.print("Part 2 cost: {d}\n", .{cost2});
}

fn part1(input_file: []const u8) !u32 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = try readInput(allocator, input_file);
    defer {
        deallocInput(input);
        allocator.destroy(input);
    }

    const cost = try pathfind(allocator, input);

    return cost;
}

fn part2(input_file: []const u8) !u32 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input = try readInput(allocator, input_file);
    defer {
        deallocInput(input);
        allocator.destroy(input);
    }

    const new_input = processInput(input);

    const cost = try pathfind(allocator, new_input);

    return cost;
}

/// A* pathfinding
fn pathfind(allocator: std.mem.Allocator, input: *const Vec(Vec(u8))) !u32 {
    var frontier = PriorityQueue(Vec2).init(allocator);
    defer frontier.free();
    try frontier.push(Vec2{ .x = 0, .y = 0 }, 0);

    var came_from = AutoHashMap(Vec2, Vec2).init(allocator);
    defer came_from.deinit();

    var cost_so_far = AutoHashMap(Vec2, u32).init(allocator);
    defer cost_so_far.deinit();

    const row_len = input.items[0].len;
    // const bound = Vec2{
    //     .x = @truncate(u32, row_len),
    //     .y = @truncate(u32, input.len),
    // };

    const end = Vec2{
        .x = @truncate(u32, row_len - 1),
        .y = @truncate(u32, input.len - 1),
    };

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
            const cost = input.get(next.y).?.get(next.x).?;
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
    total_cost += input.items[current_pos.y].items[current_pos.x];
    while (!vec_eql(current_pos, Vec2{ .x = 0, .y = 0 })) {
        const current_pos_tmp = came_from.get(current_pos);
        current_pos = if (current_pos_tmp == null) return Error.CameFromIsNull else current_pos_tmp.?;
        total_cost += input.items[current_pos.y].items[current_pos.x];
        // try stdout.print("{} - {}\n", .{ current_pos, input.items[current_pos.y].items[current_pos.x] });
        try path.push(current_pos);
        path_len += 1;
    }
    total_cost -= input.items[0].items[0]; // Don't count starting position

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

/// Process input for part 2
fn processInput(input: *const Vec(Vec(u8))) *const Vec(Vec(u8)) {
    return input;
}

const expect = std.testing.expect;

test "example_input" {
    const cost = try part1("example.txt");

    try expect(cost == 40);
}
