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

    const grid = Grid.init(&vec);

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

test "example_input" {
    const std = @import("std");

    const total_grid = "11637517422274862853338597396444961841755517295286\n13813736722492484783351359589446246169155735727126\n21365113283247622439435873354154698446526571955763\n36949315694715142671582625378269373648937148475914\n74634171118574528222968563933317967414442817852555\n13191281372421239248353234135946434524615754563572\n13599124212461123532357223464346833457545794456865\n31254216394236532741534764385264587549637569865174\n12931385212314249632342535174345364628545647573965\n23119445813422155692453326671356443778246755488935\n22748628533385973964449618417555172952866628316397\n24924847833513595894462461691557357271266846838237\n32476224394358733541546984465265719557637682166874\n47151426715826253782693736489371484759148259586125\n85745282229685639333179674144428178525553928963666\n24212392483532341359464345246157545635726865674683\n24611235323572234643468334575457944568656815567976\n42365327415347643852645875496375698651748671976285\n23142496323425351743453646285456475739656758684176\n34221556924533266713564437782467554889357866599146\n33859739644496184175551729528666283163977739427418\n35135958944624616915573572712668468382377957949348\n43587335415469844652657195576376821668748793277985\n58262537826937364893714847591482595861259361697236\n96856393331796741444281785255539289636664139174777\n35323413594643452461575456357268656746837976785794\n35722346434683345754579445686568155679767926678187\n53476438526458754963756986517486719762859782187396\n34253517434536462854564757396567586841767869795287\n45332667135644377824675548893578665991468977611257\n44961841755517295286662831639777394274188841538529\n46246169155735727126684683823779579493488168151459\n54698446526571955763768216687487932779859814388196\n69373648937148475914825958612593616972361472718347\n17967414442817852555392896366641391747775241285888\n46434524615754563572686567468379767857948187896815\n46833457545794456865681556797679266781878137789298\n64587549637569865174867197628597821873961893298417\n45364628545647573965675868417678697952878971816398\n56443778246755488935786659914689776112579188722368\n55172952866628316397773942741888415385299952649631\n57357271266846838237795794934881681514599279262561\n65719557637682166874879327798598143881961925499217\n71484759148259586125936169723614727183472583829458\n28178525553928963666413917477752412858886352396999\n57545635726865674683797678579481878968159298917926\n57944568656815567976792667818781377892989248891319\n75698651748671976285978218739618932984172914319528\n56475739656758684176786979528789718163989182927419\n67554889357866599146897761125791887223681299833479";
    const grid_str = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";

    var input_arr = try Vec(Vec(u8)).init(test_allocator);
    var row: u32 = 0;
    try input_arr.push(try Vec(u8).init(test_allocator));
    for (grid_str) |char| {
        if (char == '\n') {
            row += 1;
            try input_arr.push(try Vec(u8).init(test_allocator));
            continue;
        }

        try input_arr.items[row].push(char - '0');
    }

    defer {
        for (input_arr.items) |item| {
            item.free();
        }
        input_arr.free();
    }

    const grid = Grid.init(&input_arr);

    var col: u32 = 0;
    row = 0;
    for (total_grid) |_char| {
        if (_char == '\n') {
            col = 0;
            row += 1;
            continue;
        }
        const char = _char - '0';

        const val = grid.get(row, col);

        std.log.warn("Char: {d} - ({}, {})\n", .{ char, row, col });

        try expectEqual(@as(u32, char), val);

        col += 1;
    }
}
