const std = @import("std");

fn containsDuplicates(str: []const u8) struct { double: bool, triple: bool } {
    var counts = [_]u8{0} ** 26;

    for (str) |char| counts[char - 'a'] += 1;

    var double = false;
    var triple = false;

    for (counts) |count| {
        if (count == 2) double = true;
        if (count == 3) triple = true;
    }

    return .{ .double = double, .triple = triple };
}

fn computeChecksum(input: []const u8) u32 {
    var doubles: u32 = 0;
    var triples: u32 = 0;

    var lines = std.mem.tokenizeAny(u8, input, "\n");

    while (lines.next()) |line| {
        const res = containsDuplicates(line);

        if (res.double) doubles += 1;
        if (res.triple) triples += 1;
    }

    return doubles * triples;
}

fn areMatchingIds(str1: []const u8, str2: []const u8) bool {
    var dist: u8 = 0;

    for (str1, str2) |char1, char2| {
        if (char1 != char2) dist += 1;
    }

    return dist == 1;
}

fn findMatchingIds(input: []const u8) ?struct { []const u8, []const u8 } {
    var lines = std.mem.tokenizeAny(u8, input, "\n");

    while (lines.next()) |line1| {
        var rem_lines = std.mem.tokenizeAny(u8, lines.rest(), "\n");

        while (rem_lines.next()) |line2| {
            if (areMatchingIds(line1, line2)) return .{ line1, line2 };
        }
    }

    return null;
}

fn printCommonLetters(str1: []const u8, str2: []const u8) void {
    for (str1, str2) |char1, char2| {
        if (char1 == char2) std.debug.print("{c}", .{char1});
    }

    std.debug.print("\n", .{});
}

pub fn main() !void {
    const input = @embedFile("input/day-02.txt");

    // -- part one -- //

    const checksum = computeChecksum(input);
    std.debug.print("part one: {}\n", .{checksum});

    // -- part two -- //

    const id_pair = findMatchingIds(input).?;
    printCommonLetters(id_pair.@"0", id_pair.@"1");
}
