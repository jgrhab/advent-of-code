const std = @import("std");
const mem = std.mem;

const input = @embedFile("input/day-06.txt");

fn Point(comptime T: type) type {
    return struct { col: T, row: T };
}

// All coordinates are non-negative; parse as i32 to avoid subtraction issues when computing distances.
fn parseInput(allocator: mem.Allocator, str: []const u8) ![]Point(i32) {
    var points = try allocator.alloc(Point(i32), mem.count(u8, str, "\n"));

    var min: i32 = std.math.maxInt(i32);

    var iter = mem.tokenizeAny(u8, str, " ,\n");

    for (0..points.len) |idx| {
        const col = try std.fmt.parseInt(i32, iter.next().?, 10);
        const row = try std.fmt.parseInt(i32, iter.next().?, 10);

        if (col < min) min = col;
        if (row < min) min = row;

        points[idx] = .{ .col = col, .row = row };
    }

    std.debug.assert(mem.eql(u8, iter.rest(), ""));

    // shift all points so that the overall smallest coordinate is one
    for (points) |*point| {
        point.col -= min - 1;
        point.row -= min - 1;
    }

    return points;
}

/// Finds the maximal col and row amongst all points.
/// This gives the miniaml height and width of a grid containing all points with one cell of padding.
fn maxCoordinates(points: []const Point(i32)) Point(usize) {
    var max_col: i32 = 0;
    var max_row: i32 = 0;

    for (points) |point| {
        if (point.col > max_col) max_col = point.col;
        if (point.row > max_row) max_row = point.row;
    }

    return .{ .col = @intCast(max_col + 1), .row = @intCast(max_row + 1) };
}

const Grid = struct {
    items: []?usize,
    dims: Point(usize),

    /// Coordinates flattening for internal representation as 1D-array.
    inline fn idx(self: *const @This(), col: usize, row: usize) usize {
        return row * self.dims.col + col;
    }

    fn init(allocator: mem.Allocator, points: []const Point(i32)) !@This() {
        const dims = maxCoordinates(points);
        const grid = try allocator.alloc(?usize, dims.col * dims.row);

        return .{ .items = grid, .dims = dims };
    }

    /// Fills the grid so that each cell contains the index of its closest point.
    /// Cells equidistant from two or more points contain the value null.
    fn fillDistances(self: *@This(), points: []const Point(i32)) void {
        for (0..self.dims.col) |col| {
            for (0..self.dims.row) |row| {
                var min_dist: u32 = std.math.maxInt(u32);
                var closest_idx: ?usize = undefined;

                for (points, 0..) |pt, pt_idx| {
                    const dist = @abs(pt.col - @as(i32, @intCast(col))) + @abs(pt.row - @as(i32, @intCast(row)));

                    if (dist < min_dist) {
                        min_dist = dist;
                        closest_idx = pt_idx;
                    } else if (dist == min_dist) closest_idx = null;
                }

                self.items[self.idx(col, row)] = closest_idx;
            }
        }
    }

    fn computeFiniteAreas(self: *const @This(), allocator: mem.Allocator, point_count: usize) !usize {
        var areas = try allocator.alloc(usize, point_count);
        defer allocator.free(areas);
        @memset(areas, 0);

        // count occurences of each point
        for (0..self.items.len) |cell| {
            if (self.items[cell]) |pt_idx| areas[pt_idx] += 1;
        }

        // set area of points on the edge of the grid (infinite ares) to zero

        for (0..self.dims.col) |col| {
            if (self.items[self.idx(col, 0)]) |pt_idx| areas[pt_idx] = 0;
            if (self.items[self.idx(col, self.dims.row - 1)]) |pt_idx| areas[pt_idx] = 0;
        }

        for (0..self.dims.row) |row| {
            if (self.items[self.idx(0, row)]) |pt_idx| areas[pt_idx] = 0;
            if (self.items[self.idx(self.dims.col - 1, row)]) |pt_idx| areas[pt_idx] = 0;
        }

        return mem.max(usize, areas);
    }
};

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer std.debug.assert(gpa.deinit() == .ok);

    const allocator = gpa.allocator();

    const points = try parseInput(allocator, input);
    defer allocator.free(points);

    // -- part one -- //

    var grid = try Grid.init(allocator, points);
    defer allocator.free(grid.items);

    grid.fillDistances(points);

    const max_finite_area = try grid.computeFiniteAreas(allocator, points.len);

    std.debug.print("part one: {}\n", .{max_finite_area});

    // -- part two -- //

    const max_dist = 10_000;

    var sum_dist = try allocator.alloc(u32, (grid.dims.col + 2 * max_dist) * (grid.dims.row + 2 * max_dist));
    defer allocator.free(sum_dist);
    @memset(sum_dist, 0);

    // loop over [min_col - 10_000, max_col + 10_000] x [min_row - 10_000, max_row + 10_000]
    // TODO the bounds can be tightened by a factor of points.len since the sum of distances must be < 10_000
    for (0..grid.dims.col + 2 * max_dist) |col| {
        const _col: i32 = @as(i32, @intCast(col)) - max_dist;
        for (0..grid.dims.row + 2 * max_dist) |row| {
            const _row: i32 = @as(i32, @intCast(row)) - max_dist;

            for (points) |pt| {
                const dist = @abs(_row - pt.row) + @abs(_col - pt.col);
                sum_dist[col + row * (grid.dims.col + 2 * max_dist)] += dist;
            }
        }
    }

    // count points in the region
    var region_area: usize = 0;
    for (sum_dist) |dist| {
        if (dist < max_dist) region_area += 1;
    }

    std.debug.print("part two: {}\n", .{region_area});
}
