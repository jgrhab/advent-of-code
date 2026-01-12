const std = @import("std");
const mem = std.mem;
const input = @embedFile("input/day-05.txt");

const Polymer = struct {
    chars: []const u8,
    upper: []const bool,

    fn init(allocator: mem.Allocator, str: []const u8) !@This() {
        const _str = mem.trimEnd(u8, str, "\n");

        var chars = try allocator.alloc(u8, _str.len);
        var upper = try allocator.alloc(bool, _str.len);

        for (_str, 0..) |char, idx| {
            switch (char) {
                'A'...'Z' => {
                    chars[idx] = char;
                    upper[idx] = true;
                },
                'a'...'z' => {
                    chars[idx] = char + 'A' - 'a';
                    upper[idx] = false;
                },
                else => return error.InvalidCharacter,
            }
        }

        return .{ .chars = chars, .upper = upper };
    }

    fn deinit(self: *const @This(), allocator: mem.Allocator) void {
        allocator.free(self.chars);
        allocator.free(self.upper);
    }

    fn initWithoutUnit(allocator: mem.Allocator, str: []const u8, unit: u8) !@This() {
        std.debug.assert('A' <= unit and unit <= 'Z');

        const _str = mem.trimEnd(u8, str, "\n");

        var chars: std.ArrayList(u8) = try .initCapacity(allocator, _str.len);
        var upper: std.ArrayList(bool) = try .initCapacity(allocator, _str.len);

        for (_str) |char| {
            if (char == unit or char == unit - 'A' + 'a') continue;

            switch (char) {
                'A'...'Z' => {
                    try chars.append(allocator, char);
                    try upper.append(allocator, true);
                },
                'a'...'z' => {
                    try chars.append(allocator, char - 'a' + 'A');
                    try upper.append(allocator, false);
                },
                else => return error.InvalidCharacter,
            }
        }

        return .{
            .chars = try chars.toOwnedSlice(allocator),
            .upper = try upper.toOwnedSlice(allocator),
        };
    }
};

fn reduce(allocator: mem.Allocator, polymer: *const Polymer) !u32 {
    // mask indicating which units remain in the polymer
    // add two guards at the end to avoid going out of bound
    var mask = try allocator.alloc(bool, polymer.chars.len + 2);
    defer allocator.free(mask);

    @memset(mask[0..polymer.chars.len], true); // all units start in the polymer
    @memset(mask[polymer.chars.len..mask.len], false); // guards set to false

    // find first two units still in the polymer (from the start)
    var fst = mem.findScalarPos(bool, mask, 0, true) orelse return 0;
    var snd = mem.findScalarPos(bool, mask, fst + 1, true) orelse return 1;

    while (snd < mask.len) {
        if (polymer.chars[fst] == polymer.chars[snd] and polymer.upper[fst] != polymer.upper[snd]) {
            mask[fst] = false; // delete units from polymer
            mask[snd] = false;

            // move fst to previous unit in polymer if any, next otherwise
            fst = mem.findLast(bool, mask[0..fst], &.{true}) orelse
                mem.findScalarPos(bool, mask, fst + 1, true) orelse break;
            snd = mem.findScalarPos(bool, mask, fst + 1, true) orelse break; // unit after fst
        } else {
            fst = snd;
            snd = mem.findScalarPos(bool, mask, fst + 1, true) orelse break;
        }
    }

    var count: u32 = 0;
    for (mask) |is_in| {
        if (is_in) count += 1;
    }

    return count;
}

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer std.debug.assert(gpa.deinit() == .ok);
    const allocator = gpa.allocator();

    const polymer = try Polymer.init(allocator, input);
    defer polymer.deinit(allocator);

    // -- part one -- //

    const units_remaining = try reduce(allocator, &polymer);

    std.debug.print("part one: {}\n", .{units_remaining});

    // -- part two -- //

    var min_len: usize = input.len;

    for ('A'..'Z' + 1) |unit| {
        const poly = try Polymer.initWithoutUnit(allocator, input, @intCast(unit));
        defer poly.deinit(allocator);

        const reduced_len = try reduce(allocator, &poly);
        if (reduced_len < min_len) min_len = reduced_len;
    }

    std.debug.print("part two: {}\n", .{min_len});
}
