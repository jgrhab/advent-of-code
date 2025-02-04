/// Reads the list of connected nodes from an input line.
///
/// Only reads the connected nodes (after "<->"), not the node itself.
fn parse_input_line(line: &str) -> Vec<usize> {
    let mut iter = line.split(" <-> ");

    iter.next(); // skip the node id

    let links = iter
        .next()
        .unwrap()
        .split(", ")
        .map(|num| num.parse().unwrap())
        .collect();

    links
}

struct Graph {
    links: Vec<Vec<usize>>,
}

impl Graph {
    /// Computes the connected component containing a node.
    ///
    /// Returns the result as a partition of the set of all nodes,
    /// where the nodes contained in the component are indicated as such.
    fn connected_component(&self, node: usize) -> Vec<bool> {
        let mut visited = vec![false; self.links.len()];

        let mut stack = Vec::from([node]);

        while let Some(current) = stack.pop() {
            if visited[current] {
                continue;
            }

            stack.extend_from_slice(&self.links[current]);

            visited[current] = true;
        }

        visited
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let links = input.lines().fold(Vec::new(), |mut vec, line| {
        let links = parse_input_line(line);
        vec.push(links);

        vec
    });

    let graph = Graph { links };

    // --- Part One --- //

    let zero_component = graph.connected_component(0);

    let part_one = zero_component.iter().filter(|&&b| b).count();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    // For Part Two, we partition the graph into connected components.

    // mark all the nodes already in the partition as visited
    let mut visited = zero_component.clone();

    // store the components to form the graph partition
    let mut partition = Vec::from([zero_component]);

    // find the next node not yet in any component
    while let Some((node, _)) = visited.iter().enumerate().find(|(_, &visited)| !visited) {
        // compute the connected component containing the node
        let node_component = graph.connected_component(node);

        // mark all nodes from the connected component as visited
        for idx in 0..visited.len() {
            visited[idx] |= node_component[idx];
        }

        partition.push(node_component);
    }

    let part_two = partition.len();

    println!("Part Two: {}", part_two);
}
