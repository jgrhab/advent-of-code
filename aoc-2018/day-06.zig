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
/// This gives the minimal dimensions of a grid containing all points with one cell of padding.
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
    cols: usize,
    rows: usize,

    /// Coordinates flattening for internal representation as 1D-array.
    inline fn idx(self: *const @This(), col: usize, row: usize) usize {
        return row * self.cols + col;
    }

    fn init(allocator: mem.Allocator, points: []const Point(i32)) !@This() {
        const dims = maxCoordinates(points);
        const grid = try allocator.alloc(?usize, dims.col * dims.row);

        return .{ .items = grid, .cols = dims.col, .rows = dims.row };
    }

    /// Fills the grid so that each cell contains the index of its closest point.
    /// Cells equidistant from two or more points contain the value null.
    fn fillDistances(self: *@This(), points: []const Point(i32)) void {
        for (0..self.cols) |col| {
            for (0..self.rows) |row| {
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

        for (0..self.cols) |col| {
            if (self.items[self.idx(col, 0)]) |pt_idx| areas[pt_idx] = 0;
            if (self.items[self.idx(col, self.rows - 1)]) |pt_idx| areas[pt_idx] = 0;
        }

        for (0..self.rows) |row| {
            if (self.items[self.idx(0, row)]) |pt_idx| areas[pt_idx] = 0;
            if (self.items[self.idx(self.cols - 1, row)]) |pt_idx| areas[pt_idx] = 0;
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

    // A cell C in the set must satisfy min{d(C,P) : P point} < B/K, where B = 10_000 and K = #{points},
    // so it is enough to loop over [min_col - B/K, max_col + B/K] x [min_row - B/K, max_row + B/K].
    // Since min_row and min_col are close to zero, ignore those values for simplicity.

    const max_dist = 10_000; // = B
    const offset: usize = max_dist / points.len + 1; // = B/K with extra 1 just in case

    // store the total (sum) distance of each cell
    var total_distances = try allocator.alloc(u32, (grid.cols + 2 * offset) * (grid.rows + 2 * offset));
    defer allocator.free(total_distances);
    @memset(total_distances, 0);

    for (0..grid.cols + 2 * offset) |x| {
        const col = @as(i32, @intCast(x)) - @as(i32, @intCast(offset));

        for (0..grid.rows + 2 * offset) |y| {
            const row = @as(i32, @intCast(y)) - @as(i32, @intCast(offset));

            for (points) |pt| {
                const dist = @abs(row - pt.row) + @abs(col - pt.col);
                total_distances[x + y * (grid.cols + 2 * offset)] += dist;
            }
        }
    }

    // count points in the region
    var region_area: usize = 0;
    for (total_distances) |dist| {
        if (dist < max_dist) region_area += 1;
    }

    std.debug.print("part two: {}\n", .{region_area});
}
