const std = @import("std");
const mem = std.mem;

const input = @embedFile("input/day-10.txt");

const SkyMap = struct {
    pxs: []i32, // position x
    pys: []i32, // position y
    vxs: []i32, // velocity x
    vys: []i32, // velocity y

    const Self = @This();

    /// Parses lines of the form "position=<X, Y> velocity=<x, y>".
    fn fromInput(allocator: mem.Allocator, str: []const u8) !Self {
        const line_count = mem.countScalar(u8, str, '\n');

        var pxs = try allocator.alloc(i32, line_count);
        var pys = try allocator.alloc(i32, line_count);
        var vxs = try allocator.alloc(i32, line_count);
        var vys = try allocator.alloc(i32, line_count);

        var lines = mem.splitScalar(u8, str, '\n');

        for (0..line_count) |idx| {
            var it = mem.tokenizeAny(u8, lines.next().?, "<, >");

            _ = it.next(); // skip "position="
            pxs[idx] = try std.fmt.parseInt(i32, it.next().?, 10);
            pys[idx] = try std.fmt.parseInt(i32, it.next().?, 10);

            _ = it.next(); // skip "velocity="
            vxs[idx] = try std.fmt.parseInt(i32, it.next().?, 10);
            vys[idx] = try std.fmt.parseInt(i32, it.next().?, 10);
        }

        return .{ .pxs = pxs, .pys = pys, .vxs = vxs, .vys = vys };
    }

    fn deinit(self: *const Self, allocator: mem.Allocator) void {
        allocator.free(self.pxs);
        allocator.free(self.pys);
        allocator.free(self.vxs);
        allocator.free(self.vys);
    }

    fn advance(self: *Self, seconds: i32) void {
        for (0..self.pxs.len) |idx| {
            self.pxs[idx] += self.vxs[idx] * seconds;
            self.pys[idx] += self.vys[idx] * seconds;
        }
    }

    fn print(self: *const Self, allocator: mem.Allocator) !void {
        const min_px = mem.min(i32, self.pxs);
        const max_px = mem.max(i32, self.pxs);
        const min_py = mem.min(i32, self.pys);
        const max_py = mem.max(i32, self.pys);

        const min = @min(min_px, min_py);

        const cols: usize = @intCast(max_px - min + 1);
        const rows: usize = @intCast(max_py - min + 1);

        var rep = try allocator.alloc(u8, @as(usize, @intCast(cols * rows))); // x + y * width
        defer allocator.free(rep);
        @memset(rep, '.');

        for (0..self.pxs.len) |idx| {
            const col: usize = @intCast(self.pxs[idx] - min);
            const row: usize = @intCast(self.pys[idx] - min);
            rep[col + row * cols] = '#';
        }

        for (0..rows) |row| std.debug.print("{s}\n", .{rep[row * cols .. (row + 1) * cols]});
    }
};

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer std.debug.assert(gpa.deinit() == .ok);

    const allocator = gpa.allocator();

    var map: SkyMap = try .fromInput(allocator, input);
    defer map.deinit(allocator);

    // NOTE Find the number of steps at which the dimensions of the map are minimal
    // by iterating until size of the smallest edge of the map starts increasing.
    map.advance(10_240);

    try map.print(allocator); // displays the message

}
