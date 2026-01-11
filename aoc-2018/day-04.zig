const std = @import("std");
const mem = std.mem;
const input = @embedFile("input/day-04.txt");

/// Parses a date-time string in the form "MM-DD HH:MM".
/// Ignores the year as it is contstant over all entries.
/// Returns the number of minutes since 01-01 00:00.
fn parseTime(str: []const u8) !u32 {
    var iter = mem.splitAny(u8, str, "- :");

    var elapsed: u32 = 0;

    // add number of days in complete months
    for (1..try std.fmt.parseInt(u32, iter.next().?, 10)) |month|
        elapsed += std.time.epoch.getDaysInMonth(1518, @enumFromInt(month));

    // add number of days elapsed in current month (days start at 1)
    elapsed += try std.fmt.parseInt(u32, iter.next().?, 10) - 1;
    elapsed *= 24; // convert to hours

    // add number of hours elapsed in current day (hours start at 0)
    elapsed += try std.fmt.parseInt(u32, iter.next().?, 10);
    elapsed *= 60; // convert to minutes

    // add number of minutes elapsed in current day (minutes start at 0)
    elapsed += try std.fmt.parseInt(u32, iter.next().?, 10);

    return elapsed;
}

const Event = union(enum) {
    begin_shift: u32, // contains guard ID
    falls_asleep: void,
    wakes_up: void,

    fn parse(str: []const u8) !@This() {
        if (str[0] == 'w') return .wakes_up;
        if (str[0] == 'f') return .falls_asleep;

        const tail = str[7..]; // drop "Guard #"
        const guard_id = try std.fmt.parseInt(u32, tail[0..mem.find(u8, tail, " ").?], 10);

        return .{ .begin_shift = guard_id };
    }
};

const InputLine = struct {
    time: u32,
    event: Event,

    fn parse(line: []const u8) !@This() {
        const head = line[6..17]; // "MM-DD HH:MM"
        const tail = line[19..];

        return .{ .time = try parseTime(head), .event = try Event.parse(tail) };
    }

    fn lessThanFn(_: void, lhs: @This(), rhs: @This()) bool {
        return lhs.time < rhs.time;
    }
};

// start <= x < start + len.
const Interval = struct {
    start: u32,
    len: u32,

    /// Counts the number of times that a minute (0 <= minute < 60) appears in the interval.
    fn countMinuteAppearances(self: @This(), minute: u32) u32 {
        const int_hours = self.len / 60;
        const int_mins = self.len % 60;
        const int_start = self.start % 60;

        if (int_start <= minute and minute < int_start + int_mins) return 1 + int_hours;

        return int_hours;
    }
};

/// Finds the entry (guard) with the most minutes asleep.
fn findMostSleepId(map: *std.AutoHashMapUnmanaged(u32, std.ArrayList(Interval))) u32 {
    var id_max: u32 = undefined;
    var max_minutes: u32 = 0;

    var iter = map.iterator();
    while (iter.next()) |entry| {
        var sum: u32 = 0;
        for (entry.value_ptr.items) |interval| sum += interval.len;

        if (sum > max_minutes) {
            max_minutes = sum;
            id_max = entry.key_ptr.*;
        }
    }

    return id_max;
}

fn findMostSleptMinute(intervals: *std.ArrayList(Interval)) struct { min: u32, apps: u32 } {
    var max_apps: u32 = 0;
    var max_min: u32 = undefined;

    for (0..60) |minute| {
        var appearances: u32 = 0;
        for (intervals.items) |i| appearances += i.countMinuteAppearances(@intCast(minute));

        if (appearances > max_apps) {
            max_apps = appearances;
            max_min = @intCast(minute);
        }
    }

    return .{ .min = max_min, .apps = max_apps };
}

fn fillMap(allocator: mem.Allocator, map: *std.AutoHashMapUnmanaged(u32, std.ArrayList(Interval)), lines: []const InputLine) !void {
    var shift_id: u32 = undefined; // guard ID for current shift
    var sleep_start: u32 = undefined;

    for (lines) |line| {
        switch (line.event) {
            .falls_asleep => sleep_start = line.time,
            .wakes_up => {
                const entry = map.getEntry(shift_id).?; // entry was created on shift start
                try entry.value_ptr.append(allocator, .{ .start = sleep_start, .len = line.time - sleep_start });
            },
            .begin_shift => |id| {
                // NOTE Assume that guards are always awake at the end of their shift (checked on input)
                // so there is no need to insert an interval at shift start.
                _ = try map.getOrPutValue(allocator, id, .empty); // create map entry for next guard if needed
                shift_id = id;
            },
        }
    }
}

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer std.debug.assert(gpa.deinit() == .ok);

    var arena: std.heap.ArenaAllocator = .init(gpa.allocator());
    defer arena.deinit();

    const allocator = arena.allocator();

    var lines = try allocator.alloc(InputLine, mem.count(u8, input, "\n"));
    defer allocator.free(lines);

    var iter = mem.tokenizeScalar(u8, input, '\n');
    for (0..lines.len) |idx| lines[idx] = try InputLine.parse(iter.next().?);
    std.debug.assert(mem.eql(u8, iter.rest(), "")); // ensure all lines were read

    std.sort.heap(InputLine, lines, {}, InputLine.lessThanFn); // sort input lines chronologically

    // create and fill map containing the intervals of each guard
    var interval_map: std.AutoHashMapUnmanaged(u32, std.ArrayList(Interval)) = .empty;
    try fillMap(allocator, &interval_map, lines);

    // -- part one -- //

    const part_one_id = findMostSleepId(&interval_map); // ID of guard with most sleep minutes
    const part_one_min = findMostSleptMinute(interval_map.getEntry(part_one_id).?.value_ptr).min; // most slept minute

    std.debug.print("part one: {}\n", .{part_one_id * part_one_min});

    // -- part two -- //

    var part_two_id: u32 = undefined;
    var part_two_min: u32 = undefined;
    var max_apps: u32 = 0;

    var map_iter = interval_map.iterator();

    while (map_iter.next()) |entry| {
        const res = findMostSleptMinute(entry.value_ptr);

        if (res.apps > max_apps) {
            part_two_id = entry.key_ptr.*;
            part_two_min = res.min;
            max_apps = res.apps;
        }
    }

    std.debug.print("part two : {}\n", .{part_two_id * part_two_min});
}
