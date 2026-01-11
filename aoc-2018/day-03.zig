const std = @import("std");
const mem = std.mem;
const input = @embedFile("input/day-03.txt");

const InputLine = struct {
    row: u32,
    col: u32,
    wdt: u32,
    hgt: u32,

    /// Parses an input line of the form "#ID @ COL,ROW: WDTxHGT".
    fn parse(line: []const u8) !@This() {
        var iter = std.mem.tokenizeAny(u8, line, " ,:x");

        _ = iter.next(); // ignore ID
        _ = iter.next(); // ignore "@"

        const col = try std.fmt.parseInt(u32, iter.next().?, 10);
        const row = try std.fmt.parseInt(u32, iter.next().?, 10);
        const wdt = try std.fmt.parseInt(u32, iter.next().?, 10);
        const hgt = try std.fmt.parseInt(u32, iter.next().?, 10);

        return .{ .col = col, .row = row, .wdt = wdt, .hgt = hgt };
    }
};

fn getMaxDimensions(data: []const InputLine) struct { wdt: u32, hgt: u32 } {
    var max_wdt: u32 = 0;
    var max_hgt: u32 = 0;

    for (0..data.len) |idx| {
        const curr_wdt = data[idx].col + data[idx].wdt;
        const curr_hgt = data[idx].row + data[idx].hgt;

        if (curr_wdt > max_wdt) max_wdt = curr_wdt;
        if (curr_hgt > max_hgt) max_hgt = curr_hgt;
    }

    return .{ .wdt = max_wdt, .hgt = max_hgt };
}

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    const line_count = mem.count(u8, input, "\n");

    var data = try allocator.alloc(InputLine, line_count);
    defer allocator.free(data);

    var iter = mem.tokenizeScalar(u8, input, '\n');
    for (0..line_count) |idx| data[idx] = try InputLine.parse(iter.next().?);
    std.debug.assert(mem.eql(u8, iter.rest(), "")); // ensure all rows have been parsed

    const dims = getMaxDimensions(data);

    var claim_counts = try allocator.alloc(u16, dims.wdt * dims.hgt);
    defer allocator.free(claim_counts);

    @memset(claim_counts, 0);

    for (data) |item| {
        for (item.col..item.col + item.wdt) |col| {
            for (item.row..item.row + item.hgt) |row| {
                claim_counts[col * dims.hgt + row] += 1;
            }
        }
    }

    // -- part one -- //

    var multiple_claims_count: u32 = 0;

    for (claim_counts) |count| {
        if (count > 1) multiple_claims_count += 1;
    }

    std.debug.print("part one: {}\n", .{multiple_claims_count});

    // -- part two -- //

    var target_claim_id: usize = 0;

    item: for (data, 1..) |item, id| {
        for (item.col..item.col + item.wdt) |col| {
            for (item.row..item.row + item.hgt) |row| {
                if (claim_counts[col * dims.hgt + row] > 1) continue :item;
            }
        }

        target_claim_id = id;
        break :item;
    }

    std.debug.print("part two: {}\n", .{target_claim_id});
}
