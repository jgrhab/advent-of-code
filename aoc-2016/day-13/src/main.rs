use anyhow::Result;
use itertools::iproduct;
use num_complex::Complex;
use std::collections::{HashMap, HashSet};

type Node = Complex<i32>;

const ORIGIN: Node = Node::new(1, 1); // starting node (not (0, 0))
const TARGET: Node = Node::new(31, 39); // target node

const SHIFTS: [Node; 4] = [
    Complex::new(1, 0),  // right
    Complex::new(-1, 0), // left
    Complex::new(0, 1),  // up
    Complex::new(0, -1), // down
];

fn is_open_space(x: i32, y: i32, input: i32) -> bool {
    let mut val = x * x + 3 * x + 2 * x * y + y + y * y + input;
    let mut count = 0;

    while val > 0 {
        count += val & 1;
        val = val >> 1;
    }

    count % 2 == 0
}

// ----------------------------------------------

struct Graph {
    nodes: HashSet<Node>,
}

impl Graph {
    fn new(max_node: Node, input: i32) -> Self {
        let mut nodes = HashSet::new();

        for (x, y) in iproduct!(0..=max_node.re, 0..=max_node.im) {
            if is_open_space(x, y, input) {
                nodes.insert(Node::new(x, y));
            }
        }

        Self { nodes }
    }

    fn neighbors(&self, node: &Node) -> HashSet<&Node> {
        let mut neighbors = HashSet::new();

        for shift in SHIFTS {
            if let Some(other) = self.nodes.get(&(node + shift)) {
                neighbors.insert(other);
            }
        }

        neighbors
    }
}

// ----------------------------------------------

fn dijkstra(graph: &Graph) -> HashMap<&Node, u32> {
    let mut unvisited: HashSet<&Node> = graph.nodes.iter().collect();
    let mut distances: HashMap<&Node, u32> =
        graph.nodes.iter().map(|node| (node, u32::MAX)).collect();

    *distances.get_mut(&ORIGIN).unwrap() = 0; // ORIGIN should always be in the graph

    // while the univisited set is not empty, get the node with smallest distance
    while let Some((&node, &dist)) = distances
        .iter()
        .filter(|(&node, _)| unvisited.contains(node))
        .min_by_key(|(_, &dist)| dist)
    {
        if dist == u32::MAX {
            break;
        }

        for neighbor in graph.neighbors(node) {
            // the neighbors are in the graph by construction so unwrap is ok
            let prev_dist: u32 = *distances.get(neighbor).unwrap();
            *distances.get_mut(neighbor).unwrap() = u32::min(prev_dist, dist + 1);
        }

        let _ = unvisited.remove(node);
    }

    distances
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/day-13.txt")?
        .trim()
        .parse::<i32>()?;

    // Write TARGET = (Tx, Ty).
    // Define a graph with containing all nodes in a rectangle delimited by max_node = k * TARGET.
    // The minimal length of a path passing outside of the rectangle is min{ (2k-1)Tx + Ty, (2k-1)Ty + Tx }.
    // A shortest path inside the rectangle with length less than this it is the global shortest path,
    // and not just the shortest in the rectangle.

    // initialize a graph containing all nodes in a rectangle centered at TARGET
    let graph = Graph::new(2 * TARGET, input);

    let distances = dijkstra(&graph);

    let part_1 = *distances.get(&TARGET).unwrap();

    // check that the path found is the global shortest path
    assert!(part_1 < i32::min(3 * TARGET.re + TARGET.im, 3 * TARGET.im + TARGET.re) as u32);

    dbg!(part_1);

    let part_2 = distances.iter().filter(|(_, &dist)| dist <= 50).count();

    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijk() {
        let graph = Graph::new(Node::new(9, 6), 10);

        let dists = dijkstra(&graph);

        assert_eq!(Some(&11), dists.get(&Node::new(7, 4)));
    }
}
