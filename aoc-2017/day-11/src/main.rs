use crate::Direction::*;

/// Enum representing all directions on the hexagonal grid.
///
/// The discriminant corresponds to the angle on the circle, starting at `North`.
/// We have, e.g. `NorthWest = 1` as it is one step after `North`
/// on the circle, thus corresponds to an angle of pi/6.
enum Direction {
    North = 0,
    NorthWest = 1,
    SouthWest = 2,
    South = 3,
    SouthEast = 4,
    NorthEast = 5,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "n" => North,
            "s" => South,
            "ne" => NorthEast,
            "nw" => NorthWest,
            "se" => SouthEast,
            "sw" => SouthWest,
            _ => panic!("invalid direction: {}", value),
        }
    }
}

/// Structure representing a position on the hexagonal grid.
///
/// The position is stored as the sequence of steps leading to it.
/// This is well-defined as the order of steps on a grid is irrelevant.
/// This representation is not unique as multiple paths can lead to the
/// same position (which are represented, modulo loops, by different steps).
struct HexPosition {
    steps: [u32; 6],
}

impl From<&str> for HexPosition {
    fn from(value: &str) -> Self {
        let mut steps = [0; 6];

        for dir in value.split(',') {
            steps[Direction::from(dir) as usize] += 1;
        }

        Self { steps }
    }
}

impl HexPosition {
    fn new() -> Self {
        Self { steps: [0; 6] }
    }

    fn add(&mut self, direction: Direction) {
        self.steps[direction as usize] += 1;
    }

    /// Expresses the position using a basis of two Directions.
    ///
    /// The basis consists of the parameter `elt` and of the Direction
    /// two steps further on the circle (e.g. `NorthEast` and `NorthWest`).
    fn express_in_basis(&self, elt: Direction) -> (i32, i32) {
        let e1 = elt as usize;
        let e2 = (e1 + 1) % 6;
        let e3 = (e1 + 2) % 6;

        let e1_steps = self.steps[e1] as i32 - self.steps[(e1 + 3) % 6] as i32;
        let e2_steps = self.steps[e2] as i32 - self.steps[(e2 + 3) % 6] as i32;
        let e3_steps = self.steps[e3] as i32 - self.steps[(e3 + 3) % 6] as i32;

        // have e1 + e3 = e2
        (e1_steps + e2_steps, e3_steps + e2_steps)
    }

    /// Computes the norm of the position.
    ///
    /// The norm is the minimal number of steps necessary to reach the position.
    /// Viewing the hexagonal grid as a sequence of concentric circles,
    /// this is equal to the radius of the circle on which the position lies.
    fn norm(&self) -> u32 {
        // express in an arbitrary basis
        let (x1, x3) = self.express_in_basis(NorthEast);

        // if the signs are different, then the basis vectors
        // are consecutive on the circle and there is no reduction
        if x1.signum() != x3.signum() {
            return (x1.abs() + x3.abs()) as u32;
        }

        // if both coordinates have the same sign, we reduce
        // each pair (e1, e3) into a single e2 (as e1 + e3 = e2)

        // sign is irrelevant for the norm so make the coordinates nonnegative
        let x1 = x1.unsigned_abs();
        let x3 = x3.unsigned_abs();

        let x2 = u32::min(x1, x3); // number of pairs

        // remove the pairs and add x2 - one of x1 or x3 becomes zero
        (x1 - x2) + x2 + (x3 - x2)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_string();

    // --- Part One --- //

    let position = HexPosition::from(input.as_str());

    let part_one = position.norm();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let mut max_norm = 0;
    let mut position = HexPosition::new();

    for dir in input.split(',').map(Direction::from) {
        position.add(dir);

        max_norm = u32::max(max_norm, position.norm());
    }

    let part_two = max_norm;

    println!("Part Two: {}", part_two);
}
