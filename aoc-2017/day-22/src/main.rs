use std::collections::HashMap;

use std::ops::{AddAssign, MulAssign};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Complex {
    x: i32,
    y: i32,
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        (self.x, self.y) = (
            self.x * rhs.x - self.y * rhs.y,
            self.y * rhs.x + self.x * rhs.y,
        );
    }
}

#[derive(PartialEq)]
enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Status {
    fn update(&self) -> Self {
        match *self {
            Status::Clean => Status::Weakened,
            Status::Weakened => Status::Infected,
            Status::Infected => Status::Flagged,
            Status::Flagged => Status::Clean,
        }
    }
}

struct Virus {
    /// Map containing the position and status of the non-clean nodes.
    /// All nodes without an entry in this map are clean.
    /// The positions are relative to the origin of the virus.
    nodes: HashMap<Complex, Status>,

    pos: Complex,
    dir: Complex,

    /// Number of steps which cause a node to become infected.
    new_infections: u32,
}

impl From<&str> for Virus {
    fn from(value: &str) -> Self {
        // parse the input into a two-dimensional grid indicating the infected cells
        let grid: Vec<Vec<_>> = value.lines().fold(Vec::new(), |mut grid, line| {
            grid.push(line.chars().map(|ch| ch == '#').collect());
            grid
        });

        // The grid is a square with sides of odd lengths, so the offset
        // is the value of the x and y coorinates of its center.
        let offset = grid.len() as i32 / 2;

        let mut nodes = HashMap::new();

        // iterate over the grid and insert the coordinates of infected nodes into the set
        for row in 0..grid.len() {
            for col in (0..grid.len()).filter(|&col| grid[row][col]) {
                // compute the coordinates of the infected node relative to the grid center
                let x = col as i32 - offset;
                let y = offset - row as i32;

                nodes.insert(Complex { x, y }, Status::Infected);
            }
        }

        Self {
            nodes,
            pos: Complex { x: 0, y: 0 },
            dir: Complex { x: 0, y: 1 },
            new_infections: 0,
        }
    }
}

impl Virus {
    /// Works one step following the rules of Part One.
    fn work_v1(&mut self) {
        // recover and clear the status of the current node
        // in Part One, only the Infected status is possible
        let is_infected = self.nodes.remove(&self.pos).is_some();

        self.dir *= match is_infected {
            true => Complex { x: 0, y: -1 },
            false => Complex { x: 0, y: 1 },
        };

        if !is_infected {
            self.nodes.insert(self.pos.clone(), Status::Infected);
            self.new_infections += 1;
        }

        self.pos += self.dir.clone();
    }

    /// Works one step following the rules of Part Two.
    fn work_v2(&mut self) {
        // recover and clear the status of the current node
        let status = self.nodes.remove(&self.pos).unwrap_or(Status::Clean);

        // update the direction depending on the status of the current node
        self.dir *= match status {
            Status::Clean => Complex { x: 0, y: 1 },
            Status::Weakened => Complex { x: 1, y: 0 },
            Status::Infected => Complex { x: 0, y: -1 },
            Status::Flagged => Complex { x: -1, y: 0 },
        };

        let new_status = status.update();

        if new_status == Status::Infected {
            self.new_infections += 1;
        }

        // insert the node in the map if it is not clean
        if new_status != Status::Clean {
            self.nodes.insert(self.pos.clone(), new_status);
        }

        // take a step in the current direction
        self.pos += self.dir.clone();
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    // --- Part One --- //

    let mut virus = Virus::from(&input[..]);

    for _ in 0..10_000 {
        virus.work_v1();
    }

    let part_one = virus.new_infections;

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let mut virus = Virus::from(&input[..]);

    for _ in 0..10_000_000 {
        virus.work_v2();
    }

    let part_two = virus.new_infections;

    println!("Part Two: {}", part_two);
}
