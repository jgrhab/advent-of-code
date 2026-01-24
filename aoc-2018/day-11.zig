const std = @import("std");
const mem = std.mem;

const input = @embedFile("input/day-11.txt");

const side = 300; // length of the sides of the power grid

const PowerGrid = struct {
    // Store power level of each cell.
    // Index in the array corresponds to (x, y) coordiantes via: idx = x + side * y.
    // Cells are numbered from 1 to 300 (inclusive) so x = 0 and y = 0 are invalid cells.
    cells: [(side + 1) * (side + 1)]i32 = [_]i32{0} ** ((side + 1) * (side + 1)),

    const Self = @This();

    inline fn idx(x: usize, y: usize) usize {
        return x + side * y;
    }

    fn cellPtr(self: *Self, x: usize, y: usize) *i32 {
        return &self.cells[x + side * y];
    }

    fn setCellPowerLevel(self: *Self, x: usize, y: usize, serial_number: usize) void {
        const rack_id = x + 10;
        const power_level = (rack_id * y + serial_number) * rack_id;
        const hundreds_digit = (power_level / 100) - ((power_level / 1000) * 10);

        self.cells[idx(x, y)] = @as(i32, @intCast(hundreds_digit)) - 5;
    }

    fn init(serial_number: usize) Self {
        var grid: Self = .{};

        for (1..side + 1) |x| {
            for (1..side + 1) |y| {
                grid.setCellPowerLevel(x, y, serial_number);
            }
        }

        return grid;
    }
};

/// Returns the total power of the square whose top-left fuel cell is as position (x, y).
fn getSquarePower(grid: *const PowerGrid, x: usize, y: usize, square_side: usize) i32 {
    var power: i32 = 0;

    for (0..square_side) |col| {
        for (0..square_side) |row| {
            power += grid.cells[PowerGrid.idx(x + col, y + row)];
        }
    }

    return power;
}

const MaxPower = struct {
    x: usize = undefined,
    y: usize = undefined,
    power: i32 = 0,
};

fn getMaxSquarePower(grid: *const PowerGrid, square_side: usize) MaxPower {
    var max_power: MaxPower = .{};

    for (1..side - square_side + 1) |x| {
        for (1..side - square_side + 1) |y| {
            const square_power = getSquarePower(grid, x, y, square_side);

            if (square_power > max_power.power) {
                max_power = .{ .x = x, .y = y, .power = square_power };
            }
        }
    }

    return max_power;
}

pub fn main() !void {
    const serial_number: usize = try std.fmt.parseInt(u32, mem.trim(u8, input, "\n"), 10);
    const grid = PowerGrid.init(serial_number);

    // -- part one -- //

    const max_power_3 = getMaxSquarePower(&grid, 3);

    std.debug.print("part one: {}, {}\n", .{ max_power_3.x, max_power_3.y });

    // -- part two -- //

    var max_power: MaxPower = .{};
    var max_power_side: usize = undefined;

    for (1..side + 1) |square_side| {
        const res = getMaxSquarePower(&grid, square_side);

        if (res.power > max_power.power) {
            max_power = res;
            max_power_side = square_side;
        }
    }

    std.debug.print("part two: {}, {}, {}\n", .{ max_power.x, max_power.y, max_power_side });
}
