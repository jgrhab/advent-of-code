const std = @import("std");

pub fn main() !void {
    const input = @embedFile("input/day-01.txt");

    var freq: i32 = 0;
    var iter = std.mem.tokenizeScalar(u8, input, '\n');

    // -- part one -- //

    while (iter.next()) |str| freq += try std.fmt.parseInt(i32, str, 10);

    std.debug.print("part one: {}\n", .{freq});

    // -- part two -- //

    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    var seen: std.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen.clearAndFree(allocator);

    iter.reset();
    freq = 0;

    outer: while (true) {
        while (iter.next()) |str| {
            freq += try std.fmt.parseInt(i32, str, 10);
            if (seen.contains(freq)) break :outer;
            try seen.put(allocator, freq, void{});
        }
        iter.reset();
    }

    std.debug.print("part two: {}\n", .{freq});
}
