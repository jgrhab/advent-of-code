const std = @import("std");
const mem = std.mem;
const input = @embedFile("input/day-03.txt");

pub fn InputLineData(comptime T: type) type {
    return struct { row: T, col: T, wdt: T, hgt: T };
}

/// Parses an input line of the form "#ID @ COL,ROW: WDTxHGT".
fn parseInputLine(comptime T: type, line: []const u8) !InputLineData(T) {
    var iter = std.mem.tokenizeAny(u8, line, " ,:x");

    _ = iter.next(); // ignore ID
    _ = iter.next(); // ignore "@"

    const col = try std.fmt.parseInt(T, iter.next().?, 10);
    const row = try std.fmt.parseInt(T, iter.next().?, 10);
    const wdt = try std.fmt.parseInt(T, iter.next().?, 10);
    const hgt = try std.fmt.parseInt(T, iter.next().?, 10);

    return .{ .col = col, .row = row, .wdt = wdt, .hgt = hgt };
}

pub fn InputData(comptime T: type) type {
    return struct {
        items: []InputLineData(T), // not using data-oriented design for simplicity
        len: usize = undefined,
        pos: usize = 0,

        const Self = @This();

        pub fn init(allocator: mem.Allocator, len: usize) !Self {
            const items = try allocator.alloc(InputLineData(T), len);

            return .{ .items = items, .len = len };
        }

        pub fn appendLine(self: *Self, line: []const u8) !void {
            if (self.pos >= self.len) return error.OverCapacity;

            self.items[self.pos] = try parseInputLine(T, line);
            self.pos += 1;
        }

        pub fn dimensions(self: *Self) struct { wdt: T, hgt: T } {
            var max_wdt: T = 0;
            var max_hgt: T = 0;

            for (0..self.pos) |idx| {
                const curr_wdt = self.items[idx].col + self.items[idx].wdt;
                const curr_hgt = self.items[idx].row + self.items[idx].hgt;

                if (curr_wdt > max_wdt) max_wdt = curr_wdt;
                if (curr_hgt > max_hgt) max_hgt = curr_hgt;
            }

            return .{ .wdt = max_wdt, .hgt = max_hgt };
        }
    };
}

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    const line_count = mem.count(u8, input, "\n");

    var data: InputData(u32) = try .init(allocator, line_count);
    defer allocator.free(data.items);

    var lines = mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| try data.appendLine(line);
    std.debug.assert(data.pos == data.len); // ensure that all rows have been parsed

    const dims = data.dimensions();

    var claim_counts = try allocator.alloc(u16, dims.wdt * dims.hgt);
    defer allocator.free(claim_counts);

    @memset(claim_counts, 0);

    for (data.items) |item| {
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

    item: for (data.items, 1..) |item, id| {
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
