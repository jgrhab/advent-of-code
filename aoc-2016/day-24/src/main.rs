use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

// The puzzle input is a grid containing three types of tiles:
// - wall: '#'
// - empty: '.'
// - digit: '0', '1', ..., '8' (not necessarly up to 8, but never including 9)
// We represent the grid as a HashMap mapping the position of a non-wall tile
// to its content. The position is comprised of the row and column numbers,
// while the content is an Option containing Some(digit) for the digit tiles and
// None for the empty tiles.
// As the outermost tiles of the grid are all walls, the row and column numbers are
// positive for all tiles included in the map.

/// Position of a (non-wall) tile in the grid.
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    row: u8,
    col: u8,
}

impl Position {
    /// Compute the l1-distance between two positions.
    fn distance(&self, other: &Position) -> u32 {
        self.row.abs_diff(other.row) as u32 + self.col.abs_diff(other.col) as u32
    }
}

/// Represent the grid as a map: Position -> Option<u8> containing the non-wall tiles.
struct Grid {
    tiles: HashMap<Position, Option<u8>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut tiles = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.chars().enumerate().filter(|(_, ch)| *ch != '#') {
                let position = Position {
                    row: row as u8,
                    col: col as u8,
                };

                let tile = ch.to_digit(10).map(|digit| digit as u8);

                tiles.insert(position, tile);
            }
        }

        Self { tiles }
    }
}

impl Grid {
    /// Find the position of the digit-tile with digit `digit` in the grid.
    fn find(&self, digit: u8) -> Option<Position> {
        self.tiles
            .iter()
            .filter(|(_, &tile)| tile == Some(digit))
            .map(|(&pos, _)| pos)
            .next()
    }

    /// Encode the digit-tiles as an 8-bit integer.
    /// The k-th bit is one if and only if the grid contains a tile with digit k.
    fn encode_digit_tiles(&self) -> u8 {
        self.tiles
            .iter()
            .filter_map(|(_, &tile)| tile)
            .fold(0, |mut enc, digit| {
                enc += 1 << digit;
                enc
            })
    }

    /// Find the positions one step away from `pos`.
    fn neighbors(&self, pos: Position) -> Vec<Position> {
        self.tiles
            .iter()
            .filter(|(other, _)| pos.distance(other) == 1)
            .map(|(&pos, _)| pos)
            .collect()
    }
}

// ----------------------------------------------

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    steps: u32, // number of steps taken so far
    seen: u8,   // digit-nodes seen so far, encoded by bit position
}

// ----------------------------------------------

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("input/day-24.txt")?;

    let grid = Grid::from(&input[..]);
    let all_digits = grid.encode_digit_tiles();

    // ----------------------------------------------

    // minimal number of steps for parts 1 and 2
    let mut min_steps_1: Option<u32> = None;
    let mut min_steps_2: Option<u32> = None;

    // keep track of the states in which each position was visited
    let mut visited: HashMap<Position, HashSet<State>> = HashMap::new();

    let position = grid.find(0).unwrap(); // starting position = position of tile 0
    let state = State { steps: 0, seen: 0 }; // starting state

    // initialize the queue with the starting (position, state) pair
    let mut queue: VecDeque<(Position, State)> = VecDeque::from([(position, state)]);

    // ----------------------------------------------

    // Use depth-first search to find the shortest paths through all digit tiles
    // and back to the start.
    // Because DFS visits the states ordered by ascending step count, the first
    // path we find through all digits (and through all digits and back to 0) is
    // necessarly the shortest.
    while let Some((position, mut state)) = queue.pop_front() {
        // update the seen digits for the current state
        if let Some(digit) = grid.tiles.get(&position).unwrap() {
            state.seen = state.seen | (1 << digit); // set digit bit to one
        }

        // check if the current position has already been visited with a better state,
        // i.e. a state with (at least) the same digits seen and a lower step count
        if let Some(set) = visited.get(&position) {
            let better_states = set
                .iter()
                .filter(|&other| other.steps <= state.steps)
                .filter(|&other| (other.seen & state.seen) == state.seen);

            // drop the branch if we have visited the position with a better state
            if better_states.count() > 0 {
                continue;
            }
        }

        // add the current state to the set of visited states for the current position
        visited
            .entry(position)
            .or_insert(HashSet::new())
            .insert(state.clone());

        if state.seen == all_digits {
            // set the value for part 1 if it has not yet been found
            if min_steps_1.is_none() {
                min_steps_1 = Some(state.steps);
            }

            // if we are on tile 0, exit the loop
            if let Some(0) = grid.tiles.get(&position).unwrap() {
                min_steps_2 = Some(state.steps);
                break;
            }
        }

        // add neighbors to the back of the queue
        for next_position in grid.neighbors(position) {
            let mut next_state = state.clone();
            next_state.steps += 1;

            queue.push_back((next_position, next_state));
        }
    }

    // ----------------------------------------------

    let part_1 = min_steps_1.unwrap();
    let part_2 = min_steps_2.unwrap();

    dbg!(part_1);
    dbg!(part_2);

    Ok(())
}
