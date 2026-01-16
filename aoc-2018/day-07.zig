const std = @import("std");
const mem = std.mem;

const input = @embedFile("input/day-07.txt");

// parameters for part two
const worker_count = 5;
const task_time = 60;

const InstructionSet = struct {
    reqs: [26]u26, // requirements for each step, each bit correspond to a step
    todo: [26]bool, // steps that need to be done
    started: [26]bool = [_]bool{false} ** 26, // steps being worked on or already finished

    fn parseInput(str: []const u8) @This() {
        var reqs: [26]u26 = [_]u26{0} ** 26;
        var todo: [26]bool = [_]bool{false} ** 26;

        var iter = mem.tokenizeScalar(u8, str, '\n');

        while (iter.next()) |line| {
            const req: u5 = @intCast(line[5] - 'A');
            const step: usize = @intCast(line[36] - 'A');

            reqs[step] += @as(u26, 1) << req;

            // mark all steps appearing in the input as todo
            todo[req] = true;
            todo[step] = true;
        }

        return .{ .reqs = reqs, .todo = todo };
    }

    fn nextAvailableStep(self: *@This()) ?usize {
        for (self.reqs, self.todo, self.started, 0..) |req, todo, taken, idx| {
            if (todo and req == 0 and !taken) return idx;
        }

        return null;
    }

    fn executeStep(self: *@This(), idx: usize) void {
        std.debug.assert(self.todo[idx]);
        std.debug.assert(self.reqs[idx] == 0); // requirements are satisfied

        const bit = @as(u26, 1) << @as(u5, @intCast(idx));

        for (&self.reqs) |*req| req.* ^= bit & req.*; // XOR the bit to zero for all steps

        self.todo[idx] = false;
    }
};

pub fn main() !void {
    var instructions: InstructionSet = .parseInput(input);

    // -- part one -- //

    var steps_order: [26]u8 = undefined;
    var steps_done: u32 = 0;

    while (instructions.nextAvailableStep()) |idx| : (steps_done += 1) {
        instructions.executeStep(idx);

        steps_order[steps_done] = @as(u8, @intCast(idx)) + 'A';
    }

    std.debug.print("part one: {s}\n", .{steps_order});

    // -- part two -- //

    instructions = .parseInput(input); // reset instructions

    var time: usize = 0;
    var current_task: [worker_count]?usize = [_]?usize{null} ** worker_count; // current task of each worker
    var time_left: [worker_count]usize = undefined; // time remaining for each current task

    while (mem.countScalar(bool, &instructions.todo, true) > 0) {

        // assign available tasks to idle workers
        for (0..worker_count) |worker| {
            if (current_task[worker] != null) continue; // advance to the next available worker
            const task = instructions.nextAvailableStep() orelse break; // find next available task

            current_task[worker] = task;
            time_left[worker] = task_time + 1 + task; // task ID starts at 'A' = 0 so add +1
            instructions.started[task] = true;
        }

        // advance currently assigned tasks
        for (0..worker_count) |worker| {
            const task = current_task[worker] orelse continue; // get current task if busy, skip otherwise

            time_left[worker] -= 1; // advance task

            if (time_left[worker] == 0) {
                instructions.executeStep(task); // execute the step once enough time has passed
                current_task[worker] = null; // mark worker as idle
            }
        }

        time += 1;
    }

    std.debug.print("part two: {}\n", .{time});
}
