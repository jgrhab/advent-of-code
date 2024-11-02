use anyhow::Result;
use num_complex::Complex;
use std::collections::HashSet;

const HEIGHT: usize = 6;
const WIDTH: usize = 50;

type Pixel = Complex<i32>;

enum Operation {
    Add(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl Operation {
    /// Parse a line of input into an Operation.
    /// Returns `None` if it fails for any reason.
    fn from_str(string: &str) -> Option<Self> {
        let mut iter = string.split_whitespace();

        match iter.next() {
            Some("rect") => {
                // parse the two numbers and replace the Err variant with None
                let mut num_iter = iter.next()?.split('x').map(|s| s.parse::<usize>().ok());
                let width: usize = num_iter.next()??;
                let height: usize = num_iter.next()??;

                Some(Self::Add(width, height))
            }
            Some("rotate") => {
                let kind: &str = iter.next()?; // "row" or "column"

                // parse numbers from the remainder, which is of the form ["y=0", "by", "5"]
                let index: usize = iter.next()?.split('=').nth(1)?.parse::<usize>().ok()?;
                let shift: usize = iter.nth(1)?.parse::<usize>().ok()?;

                match kind {
                    "row" => Some(Self::RotateRow(index, shift)),
                    "column" => Some(Self::RotateCol(index, shift)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

/// Represent the screen by a set of active pixels.
/// The pixel at position (x,y) is represented by the complex number x+iy.
/// The screen is the rectangle {0 <= x < width} x {0 <= -y < height} (the top row is y = 0).
struct Screen {
    pixels: HashSet<Pixel>, // active pixels
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        let pixels = HashSet::with_capacity(height * width);

        Self {
            pixels,
            width,
            height,
        }
    }

    fn add_rectangle(&mut self, width: usize, height: usize) {
        let mut new_pixels: HashSet<Pixel> = HashSet::with_capacity(width * height);

        for x in 0..width {
            for y in 0..height {
                new_pixels.insert(Pixel::new(x as i32, -(y as i32)));
            }
        }

        self.pixels = self.pixels.union(&new_pixels).map(|px| *px).collect();
    }

    /// Reduce the coordinates of all pixels in the set to remain in the range of the screen,
    /// i.e. x + iy -> (x mod width) + i(y mod width).
    fn reduce(&mut self) {
        let (width, height) = (self.width as i32, self.height as i32);

        self.pixels = self
            .pixels
            .iter()
            .map(|px| Complex::new(px.re % width, px.im % height))
            .collect();
    }

    /// Rotate the row with index `idx` to the right by `shift` pixels.
    fn rotate_row(&mut self, idx: usize, shift: usize) {
        let y_row = -(idx as i32); // y-coordinate of the row
        let shift: Complex<i32> = Complex::new(shift as i32, 0);

        // Extract the pixels on the row and shift them
        // NOTE the `HashSet::extract_if` method (in nightly) would be better
        let mut row = self.pixels.clone();
        row.retain(|px| px.im == y_row);
        row = row.into_iter().map(|px| px + shift).collect();

        // Remove the initial row and add the shifted row
        self.pixels.retain(|px| px.im != y_row);
        self.pixels = self.pixels.union(&row).map(|px| *px).collect();
    }

    /// Rotate the column with index `idx` down by `shift` pixels.
    fn rotate_col(&mut self, idx: usize, shift: usize) {
        let x_col = idx as i32; // x-coordinate of the row
        let shift: Complex<i32> = Complex::new(0, -(shift as i32));

        let mut col = self.pixels.clone();
        col.retain(|px| px.re == x_col);
        col = col.into_iter().map(|px| px + shift).collect();

        self.pixels.retain(|px| px.re != x_col);
        self.pixels = self.pixels.union(&col).map(|px| *px).collect();
    }

    fn apply(&mut self, operation: &Operation) {
        match *operation {
            Operation::Add(width, height) => self.add_rectangle(width, height),
            Operation::RotateRow(index, shift) => self.rotate_row(index, shift),
            Operation::RotateCol(index, shift) => self.rotate_col(index, shift),
        }

        self.reduce(); // unnecessary for `Add`
    }

    fn display(&self) {
        for row_idx in 0..self.height {
            // find all pixels that are on row with index row_idx
            // here again, `extract_if()` would be better than `clone()` and `retain()`
            let mut row = self.pixels.clone();
            row.retain(|px| px.im == -(row_idx as i32));

            let mut vec = vec![' '; self.width];
            for px in row {
                vec[px.re as usize] = '#';
            }

            let s: String = vec.into_iter().collect();
            println!("{}", s);
        }
    }
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-08.txt")?
        .lines()
        .map(String::from)
        .collect();

    let operations: Vec<Operation> = input
        .into_iter()
        .map(|line| Operation::from_str(&line).unwrap())
        .collect();

    let mut screen = Screen::new(WIDTH, HEIGHT);
    for op in operations {
        screen.apply(&op);
    }

    let part_1 = screen.pixels.len();
    dbg!(part_1);

    // for part 2: read the output
    screen.display();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let ops = vec![
            Operation::Add(3, 2),
            Operation::RotateCol(1, 1),
            Operation::RotateRow(0, 4),
            Operation::RotateCol(1, 1),
        ];

        let mut screen = Screen::new(7, 3);

        for op in ops {
            screen.apply(&op);
        }

        let mut expected = HashSet::new();
        expected.insert(Pixel::new(1, 0));
        expected.insert(Pixel::new(4, 0));
        expected.insert(Pixel::new(6, 0));
        expected.insert(Pixel::new(0, -1));
        expected.insert(Pixel::new(2, -1));
        expected.insert(Pixel::new(1, -2));

        assert_eq!(expected, screen.pixels);
    }
}
