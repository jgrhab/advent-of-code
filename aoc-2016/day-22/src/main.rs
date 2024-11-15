use anyhow::Result;
use std::collections::HashMap;

#[derive(PartialEq)]
struct Node {
    x: i8,
    y: i8,
    size: u16,
    used: u16,
}

impl From<&str> for Node {
    fn from(string: &str) -> Self {
        let split: Vec<&str> = string.split_whitespace().collect();

        let path_split: Vec<&str> = split[0].split('-').collect();
        let x = path_split[1][1..].parse::<i8>().unwrap();
        let y = path_split[2][1..].parse::<i8>().unwrap();

        let size = split[1].trim_end_matches('T').parse::<u16>().unwrap();
        let used = split[2].trim_end_matches('T').parse::<u16>().unwrap();

        Node { x, y, size, used }
    }
}

#[derive(Debug)]
enum NodeType {
    Empty,
    Dot,
    Full,
    Goal,
}

fn find_node_types(
    sizes: &HashMap<(i8, i8), u16>,
    usage: &HashMap<(i8, i8), u16>,
) -> HashMap<(i8, i8), NodeType> {
    let x_max: i8 = *sizes.keys().map(|(x, _)| x).max().unwrap();

    let mut types: HashMap<(i8, i8), NodeType> = HashMap::with_capacity(sizes.len());

    // Find the Empty and Full nodes.
    // A node is full if its data cannot fit in any of its neighbors.
    for (&(x, y), &used) in usage.iter() {
        if used == 0 {
            types.insert((x, y), NodeType::Empty);
            continue;
        }

        if used > 100 {
            types.insert((x, y), NodeType::Full);
        }
    }

    // insert the Goal node at its starting position (x_max, 0)
    types.insert((x_max, 0), NodeType::Goal);

    // insert the remaining nodes with Dot type
    for (&pos, _) in usage.iter() {
        if !types.contains_key(&pos) {
            types.insert(pos, NodeType::Dot);
        }
    }

    types
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-22.txt")?
        .lines()
        .skip(2)
        .map(String::from)
        .collect();

    let nodes: Vec<Node> = input.into_iter().map(|s| Node::from(&s[..])).collect();

    // ------------------------------------------

    let mut part_1 = 0;

    for node_a in nodes.iter().filter(|node| node.used > 0) {
        for node_b in nodes.iter().filter(|node| *node != node_a) {
            if node_a.used <= (node_b.size - node_b.used) {
                part_1 += 1;
            }
        }
    }

    dbg!(part_1);

    // ------------------------------------------

    // read the node sizes and usage into two hashmaps
    let mut sizes: HashMap<(i8, i8), u16> = HashMap::with_capacity(nodes.len());
    let mut usage: HashMap<(i8, i8), u16> = HashMap::with_capacity(nodes.len());

    for node in nodes.iter() {
        sizes.insert((node.x, node.y), node.size);
        usage.insert((node.x, node.y), node.used);
    }

    let x_max: i8 = *sizes.keys().map(|(x, _)| x).max().unwrap();
    let y_max: i8 = *sizes.keys().map(|(_, y)| y).max().unwrap();

    // ------------------------------------------

    // identify the type of each node
    let types = find_node_types(&sizes, &usage);

    // store the node types in a 2D array for printing
    // the array has coordinates (y, x) to make printing easier
    let mut type_array = vec![vec!['?'; x_max as usize + 1]; y_max as usize + 1];
    for (&(x, y), node_type) in types.iter() {
        type_array[y as usize][x as usize] = match node_type {
            NodeType::Empty => '_',
            NodeType::Full => '#',
            NodeType::Goal => 'G',
            NodeType::Dot => '.',
        }
    }

    // Solve part 2 by hand, counting the steps on the plot, following the example.
    // 1. Bring the empty node to the left of the goal (count steps on plot)
    // 2. Switch Goal and Empty (1 step)
    // 3. Move Empty to left of Goal (4 steps)
    // 4. Switch Goal and Empty (1 step)
    // 5. Repeat 3-4 until Goal is in the leftmost column (x_max - 1 times)

    for row in type_array {
        let row: String = row.into_iter().collect();
        println!("{}", row);
    }

    Ok(())
}
