use anyhow::Result;
use std::collections::VecDeque;

const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];

#[derive(Eq, PartialEq, Hash, Clone)]
struct Node {
    row: u8,
    col: u8,
    path: String, // steps taken so far
}

impl Node {
    fn neighbors(&self, passcode: &str) -> Vec<Node> {
        let hash = md5::compute(format!("{}{}", passcode, self.path));

        // compute the available neighbors (open door + not an edge)
        // the hash is [u8; 16] so one value is two hex characters
        let is_available = [
            (hash[0] >> 4 >= 0xb) & (self.row > 0),   // up
            (hash[0] & 0x0f >= 0xb) & (self.row < 3), // down
            (hash[1] >> 4 >= 0xb) & (self.col > 0),   // left
            (hash[1] & 0xf >= 0xb) & (self.col < 3),  // right
        ];

        let mut neighbors: Vec<Node> = Vec::new();

        for (ind, dir) in DIRS.into_iter().enumerate() {
            if !is_available[ind] {
                continue;
            }

            let (row, col) = match ind {
                0 => (self.row - 1, self.col),
                1 => (self.row + 1, self.col),
                2 => (self.row, self.col - 1),
                3 => (self.row, self.col + 1),
                _ => unreachable!(),
            };

            let path = format!("{}{dir}", self.path);

            neighbors.push(Node { row, col, path });
        }

        neighbors
    }
}

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("input/day-17.txt")?
        .trim()
        .to_string();

    // create the starting node
    let start = Node {
        row: 0,
        col: 0,
        path: String::new(),
    };

    // initialize to something longer than the shortest path (guess)
    let mut min_path: String = ['x'; 1_000].into_iter().collect();

    let mut max_path_len = 0;

    let mut queue: VecDeque<Node> = VecDeque::from([start]);

    while let Some(node) = queue.pop_front() {
        if node.row == 3 && node.col == 3 {
            // update the shortest path if applicable
            if node.path.len() < min_path.len() {
                min_path = node.path.clone();
            }

            // update the length of the longest path if applicable
            max_path_len = usize::max(max_path_len, node.path.len());
        } else {
            // if node is not the target, add its neighbors to the queue
            queue.extend(node.neighbors(&input));
        }
    }

    let part_1 = min_path;
    dbg!(part_1);

    let part_2 = max_path_len;
    dbg!(part_2);

    Ok(())
}
