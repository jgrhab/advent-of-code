use std::collections::HashMap;
use std::ops::{Add, Sub};

#[derive(PartialEq, Debug)]
enum Tile {
    Line,
    Turn,
    Letter(char),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Line,
            '-' => Tile::Line,
            '+' => Tile::Turn,
            'A'..='Z' => Tile::Letter(value),
            _ => panic!("invalid character: {}", value),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vector {
    row: i32,
    col: i32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

/// TODO
///
struct GridWalker {
    /// Store the grid as a map: coordinates -> tile.
    map: HashMap<Vector, Tile>,

    /// Current position on the grid.
    pos: Vector,

    /// Current direction of movement.
    dir: Vector,

    /// Characters (letters) encountered during the walk.
    chs: Vec<char>,
}

impl From<&str> for GridWalker {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();

        for (row, line) in value.lines().enumerate() {
            for (col, ch) in line.char_indices().filter(|&(_, ch)| ch != ' ') {
                let vec = Vector {
                    row: row as i32,
                    col: col as i32,
                };

                map.insert(vec, ch.into());
            }
        }

        // Find the starting position.
        // There is only one non-empty tile in the first row.
        let pos = *map.keys().find(|vec| vec.row == 0).unwrap();

        // Set the initial direction to 'down'
        let dir = Vector { row: 1, col: 0 };

        let chs = Vec::new();

        Self { map, pos, dir, chs }
    }
}

impl Iterator for GridWalker {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        match self.map.get(&self.pos) {
            Some(Tile::Line) => (),
            Some(Tile::Turn) => self.dir = self.turn(),
            Some(Tile::Letter(ch)) => self.chs.push(*ch),
            None => return None,
        }

        self.pos = self.pos + self.dir;

        Some(self.pos)
    }
}

impl GridWalker {
    fn turn(&self) -> Vector {
        // this method should only be called on a Turn tile
        assert_eq!(self.map.get(&self.pos), Some(&Tile::Turn));

        // Compute the previous position by reversing the last step.
        let prev = self.pos - self.dir;

        // Compute the position of the next tile.
        // Each turn should be connected to two tiles (previous and next).
        // If there is no next, we have reached the end.
        let next = [
            Vector { row: 1, col: 0 },
            Vector { row: -1, col: 0 },
            Vector { row: 0, col: 1 },
            Vector { row: 0, col: -1 },
        ]
        .into_iter()
        .map(|dir| self.pos + dir)
        .filter(|vec| self.map.contains_key(vec))
        .find(|vec| *vec != prev);

        // Return the next direction if there is a next tile.
        if let Some(next_pos) = next {
            return next_pos - self.pos;
        }

        // If there is no next tile, return the current direction.
        // This ensures that the walker lands on an empty tile after taking a step.
        self.dir
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut walker = GridWalker::from(&input[..]);

    let mut step_count = 0;

    for _ in walker.by_ref() {
        step_count += 1;
    }

    // --- Part One --- //

    let part_one: String = walker.chs.iter().collect();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let part_two = step_count;

    println!("Part Two: {}", part_two);
}
