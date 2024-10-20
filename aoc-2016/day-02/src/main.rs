use anyhow::Result;
use num_complex::Complex;
use std::fs;

// Represent the keypad as a disc K = {|s|^2 < r} in the complex plane chosen so that
// the integer points inside K correspond to the keys.
// A move on the keypad is valid as long as the destination remains in K.
//
// for part 1: 2 < r < 4
// for part 2: 4 < r < 5

type Position = Complex<i8>;

const UP: Complex<i8> = Complex::new(0, 1);
const DOWN: Complex<i8> = Complex::new(0, -1);
const LEFT: Complex<i8> = Complex::new(-1, 0);
const RIGHT: Complex<i8> = Complex::new(1, 0);

/// Take a single step starting from position `pos` in direction `dir`
/// on a keypad with squared radius `rad_sqr`.
fn step(rad_sqr: f32, pos: Position, dir: char) -> Position {
    let step = match dir {
        'U' => UP,
        'D' => DOWN,
        'L' => LEFT,
        'R' => RIGHT,
        _ => panic!("unexpected direction: {dir}"),
    };

    let dest = pos + step;

    // if the destination is outside the keypad, return the starting position
    if dest.norm_sqr() as f32 >= rad_sqr {
        return pos;
    }

    dest
}

fn walk(rad_sqr: f32, ins: &str) -> Position {
    let mut pos: Complex<i8> = Complex::new(0, 0);

    for dir in ins.chars() {
        pos = step(rad_sqr, pos, dir);
    }

    pos
}

/// Converts a position on the keypad from part 1 to the corresponding character
/// NOTE: did not need to shift, can start from center
fn convert_1(pos: Complex<i8>) -> char {
    let row = (1 - pos.im) as u32; // top-to-bottom, start at 0
    let col = 2 - (1 - pos.re) as u32; // left-to-right, start at 0

    let num = 1 + 3 * row + col;

    char::from_digit(num, 10).expect("failed to convert {num} to char")
}

/// Converts a position on the keypad from part 2 to the corresponding character
fn convert_2(pos: Complex<i8>) -> char {
    let row_start: [i32; 5] = [1, 2, 5, 10, 13];
    let row_shift: [i32; 5] = [0, 1, 2, 1, 0]; // length left and right of center

    let row = (2 - pos.im) as u32;

    // compute the column relative to the row
    let col = (pos.re as i32) + row_shift[row as usize];

    let num = row_start[row as usize] + col;

    char::from_digit(num as u32, 14).expect("failed to convert {num} to char")
}

fn main() -> Result<()> {
    let input: Vec<String> = fs::read_to_string("input/day-02.txt")?
        .lines()
        .map(String::from)
        .collect();

    let part_1: String = input
        .iter()
        .map(|ins| walk(3.0, ins))
        .map(|pos| convert_1(pos))
        .collect();

    dbg!(part_1);

    let part_2: String = input
        .iter()
        .map(|ins| walk(4.5, ins))
        .map(|pos| convert_2(pos))
        .collect();

    let part_2 = part_2.to_uppercase();

    dbg!(part_2);

    Ok(())
}
