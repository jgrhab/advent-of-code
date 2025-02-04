use std::cmp::{max, min};

#[derive(Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn rotate_left(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Returns the sup-norm (l0-norm) of the vector,
    fn sup_norm(&self) -> u32 {
        max(self.x.abs(), self.y.abs()) as u32
    }
}

/// An iterator which walks the grid in a spiral pattern.
///
/// The grid is composed of a chain of concentric squares, each included in the next.
/// The walker starts at the origin (position (0, 0), value 1).
///
/// It enters a square one tile above the square's bottom right corner and visits
/// all tiles following the edges of the square counterclockwise.
/// Once all tiles have been visited (when reaching the bottom right corner),
/// it takes a step to the right to enter the next square.
struct GridWalker {
    /// Current position on the grid
    position: Vector,

    /// Vector to add to the position to take the next step
    step_delta: Vector,
}

impl GridWalker {
    fn new() -> Self {
        Self {
            position: Vector { x: 0, y: 0 },
            step_delta: Vector { x: 1, y: 0 },
        }
    }
}

impl Iterator for GridWalker {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        // Compute the sup-norm of the current position.
        // Note that all tiles in a square have the same sup-norm.
        let sup_norm = self.position.sup_norm();

        // take a step
        self.position = self.position.add(&self.step_delta);

        // Check whether we have reached a corner of the current square.
        // The corners are at the four positions (x, y) with |x| = |y| = sup_norm.
        let corner_reached = min(self.position.x.abs(), self.position.y.abs()) == sup_norm as i32;

        // check whether we are in the bottom right quadrant
        let in_bottom_right = (self.position.x > 0) && (self.position.y < 0);

        // check whether we have stepped into the next square
        let entered_next_square = self.position.sup_norm() > sup_norm;

        // turn left at any but the bottom right corner or after entering a new square
        if (corner_reached && !in_bottom_right) || entered_next_square {
            self.step_delta = self.step_delta.rotate_left();
        }

        Some(self.position.clone())
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input: u32 = input.trim().parse().unwrap();

    // --- Part One --- //

    let mut walker = GridWalker::new();

    // take input - 1 steps starting at the origin (value = 1)
    // to get to value = input
    for _ in 1..input {
        let _ = walker.next();
    }

    let part_one = walker.position.x.abs() + walker.position.y.abs();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    // Create a grid large enough to contain the target tile
    // by taking the grid containing the target tile from part one.
    // Note that this grid is much larger than it needs to be.
    let origin_idx = walker.position.sup_norm() as usize;
    let side_length = 2 * origin_idx + 1;
    let mut grid = vec![vec![0; side_length]; side_length];

    grid[origin_idx][origin_idx] = 1; // set origin value

    let mut walker = GridWalker::new();

    let part_two = loop {
        let position = walker.next().unwrap();

        // get the indices for the current tile in the grid
        let row_idx = (position.x + origin_idx as i32) as usize;
        let col_idx = (position.y + origin_idx as i32) as usize;

        // comput the value of the current tile by summing all neighbors
        // all unvisited tiles have value 0 (including the current tile)
        let mut value = 0;
        for row_off in 0..3 {
            for col_off in 0..3 {
                value += grid[row_idx + row_off - 1][col_idx + col_off - 1];
            }
        }

        grid[row_idx][col_idx] = value;

        if value > input {
            break value;
        }
    };

    println!("Part Two: {}", part_two);
}
