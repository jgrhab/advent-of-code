const std = @import("std");
const mem = std.mem;

const input = @embedFile("input/day-09.txt");

/// Parses the input, which has the form "X players; last marble is worth Y points".
fn parseInput(str: []const u8) !struct { players: u32, last_marble: u32 } {
    var iter = mem.tokenizeScalar(u8, str, ' ');

    const players = try std.fmt.parseInt(u32, iter.next().?, 10);
    for (0..5) |_| _ = iter.next();
    const last_marble = try std.fmt.parseInt(u32, iter.next().?, 10);

    return .{ .players = players, .last_marble = last_marble };
}

const GameState = struct {
    marbles: std.ArrayList(u32),
    current: usize, // current marble index
    turn: u32, // current turn = current marble

    allocator: mem.Allocator,

    const Self = @This();

    fn init(allocator: mem.Allocator, marble_count: u32) !Self {
        var marbles: std.ArrayList(u32) = try .initCapacity(allocator, marble_count);
        try marbles.append(allocator, 0);
        try marbles.append(allocator, 1);

        return .{ .marbles = marbles, .current = 1, .turn = 1, .allocator = allocator };
    }

    fn insertRegularMarble(self: *Self) !void {
        self.turn += 1;

        var pos = self.current + 2;

        if (pos == self.marbles.items.len) {
            try self.marbles.append(self.allocator, self.turn);
        } else {
            pos = if (pos > self.marbles.items.len) pos - self.marbles.items.len else pos;
            try self.marbles.insert(self.allocator, pos, self.turn);
        }

        self.current = pos;
    }

    fn insertScoreMarble(self: *Self) u32 {
        self.turn += 1;

        const pos = if (self.current < 7) self.marbles.items.len + self.current - 7 else self.current - 7;
        self.current = pos; // all marbles right of pos shift to the left when removing

        return self.turn + self.marbles.orderedRemove(pos);
    }
};

fn computeHighScore(allocator: mem.Allocator, players: u32, last_marble: u32) !u32 {
    const scoring_turns: u32 = (last_marble + 1) / 23; // number of turns where the score changes

    var game_state: GameState = try .init(allocator, last_marble + 1);
    defer game_state.marbles.deinit(allocator);

    // array containing the score of each player (players are numbered starting at 1)
    var score = try allocator.alloc(u32, players + 1);
    defer allocator.free(score);
    @memset(score, 0);

    // play blocks of turns until a turn that is a multiple of 23
    for (1..scoring_turns + 1) |block| {

        // play until last turn before multiple of 23
        while (game_state.turn < 23 * block - 1) try game_state.insertRegularMarble();

        // play turn multiple of 23 and update the score
        score[game_state.turn % players] += game_state.insertScoreMarble();
    }

    // NOTE no need to play the remaining turns since they do not affect the score

    return mem.max(u32, score);
}

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer std.debug.assert(gpa.deinit() == .ok);

    const allocator = gpa.allocator();

    const data = try parseInput(input);

    // -- part one -- //

    const high_score_one = try computeHighScore(allocator, data.players, data.last_marble);
    std.debug.print("part one: {}\n", .{high_score_one});

    // -- part two -- //

    // not the way this is intended to be done -- use linked list?
    const high_score_two = try computeHighScore(allocator, data.players, data.last_marble * 100);
    std.debug.print("part two: {}\n", .{high_score_two});
}
