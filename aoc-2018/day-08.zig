const std = @import("std");
const mem = std.mem;

const input = @embedFile("input/day-08.txt");

fn parseInput(allocator: mem.Allocator, str: []const u8) ![]u32 {
    var numbers: std.ArrayList(u32) = .empty;

    var iter = mem.tokenizeAny(u8, str, " \n");

    while (iter.next()) |num| {
        const val = try std.fmt.parseInt(u32, num, 10);
        try numbers.append(allocator, val);
    }

    return numbers.toOwnedSlice(allocator);
}

const Node = struct {
    children: []Node,
    metadata: []const u32,

    fn init(allocator: mem.Allocator, children_len: u32, metadata_len: u32) !@This() {
        const children = try allocator.alloc(Node, children_len);
        const metadata = try allocator.alloc(u32, metadata_len);

        return .{ .children = children, .metadata = metadata };
    }
};

fn buildTree(allocator: mem.Allocator, data: []const u32) !Node {
    var root: Node = try .init(allocator, data[0], data[1]);
    var tail = data[2..];

    // store the nodes and the number of children currently attached to them
    var queue: std.MultiArrayList(struct { node: *Node, attached: usize }) = .empty;
    defer queue.deinit(allocator);

    try queue.append(allocator, .{ .node = &root, .attached = 0 });

    while (tail.len > 0) {
        const elt = queue.pop().?;

        // once all children are attached, finalize the node with metadata
        if (elt.attached == elt.node.children.len) {
            elt.node.metadata = tail[0..elt.node.metadata.len];
            tail = tail[elt.node.metadata.len..];
            continue; // leave finalized node out of queue
        }

        // attach next child
        elt.node.children[elt.attached] = try .init(allocator, tail[0], tail[1]);
        tail = tail[2..];

        // insert both the incomplete node and the child onto the queue
        try queue.append(allocator, .{ .node = elt.node, .attached = elt.attached + 1 });
        try queue.append(allocator, .{ .node = &elt.node.children[elt.attached], .attached = 0 });
    }

    return root;
}

fn sumMetadata(allocator: mem.Allocator, root: *const Node) !u32 {
    var queue: std.Deque(*const Node) = .empty;
    defer queue.deinit(allocator);

    try queue.pushBack(allocator, root);

    var sum_metadata: u32 = 0;

    // BFS to traverse the tree
    while (queue.popFront()) |node| {
        for (node.metadata) |val| sum_metadata += val;
        for (node.children) |*child| try queue.pushBack(allocator, child);
    }

    return sum_metadata;
}

fn computeRootValue(allocator: mem.Allocator, root: *const Node) !u32 {
    var queue: std.ArrayList(*const Node) = .empty; // queue for nodes to be evaluated
    defer queue.deinit(allocator);

    var node_values: std.AutoHashMap(*const Node, u32) = .init(allocator); // contains value of evaluated nodes
    defer node_values.deinit();

    try queue.append(allocator, root);

    while (queue.pop()) |node| {
        std.debug.assert(!node_values.contains(node)); // the queue should only contain nodes not yet evaluated

        // nodes without children can be evaluated directly
        if (node.children.len == 0) {
            var node_value: u32 = 0;
            for (node.metadata) |x| node_value += x;
            try node_values.put(node, node_value);
            continue;
        }

        // count how many children have a value
        var children_with_value: u32 = 0;
        for (node.children) |*child| {
            if (node_values.contains(child)) children_with_value += 1;
        }

        // if all children have been evaluated, compute node value from the children values
        if (children_with_value == node.children.len) {
            var node_value: u32 = 0;

            for (node.metadata) |idx| {
                if (idx == 0 or idx > node.children.len) continue;
                node_value += node_values.get(&node.children[idx - 1]).?;
            }

            try node_values.put(node, node_value);

            continue;
        }

        try queue.append(allocator, node);
        for (node.children) |*child| if (!node_values.contains(child)) try queue.append(allocator, child);
    }

    return node_values.get(root).?;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    const numbers = try parseInput(allocator, input);
    defer allocator.free(numbers);

    const root = try buildTree(allocator, numbers);

    // -- part one -- //

    const sum_metadata = try sumMetadata(allocator, &root);

    std.debug.print("part one: {}\n", .{sum_metadata});

    // -- part two -- //

    const root_value = try computeRootValue(allocator, &root);

    std.debug.print("part two: {}\n", .{root_value});
}
