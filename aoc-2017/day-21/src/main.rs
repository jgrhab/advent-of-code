use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Image {
    pixels: Box<[Box<[bool]>]>,
}

impl From<&str> for Image {
    fn from(value: &str) -> Self {
        let pixels = value.split('/').fold(Vec::new(), |mut vec, row| {
            let row: Vec<_> = row.chars().map(|ch| ch == '#').collect();
            vec.push(row.into_boxed_slice());

            vec
        });

        let pixels = pixels.into_boxed_slice();

        Self { pixels }
    }
}

impl Image {
    /// Creates a new image with all pixels off.
    fn new(size: usize) -> Self {
        let pixels = (0..size)
            .fold(Vec::with_capacity(size), |mut vec, _| {
                vec.push(vec![false; size].into_boxed_slice());
                vec
            })
            .into_boxed_slice();

        Self { pixels }
    }

    /// Returns the size of the image.
    ///
    /// The size of an image is defined as the length (in pixels) of its edges.
    fn size(&self) -> usize {
        self.pixels.len()
    }

    /// Rotates the image clockwise by a quarter-turn.
    fn rotate(&self) -> Self {
        let mut image = self.clone();

        for row in 0..self.size() {
            for col in 0..self.size() {
                // image.pixels[col][self.size() - row - 1] = self.pixels[row][col];
                image.pixels[row][col] = self.pixels[self.size() - col - 1][row];
            }
        }

        image
    }

    /// Flips the image vertically.
    fn vflip(&self) -> Self {
        let mut image = self.clone();

        for row in 0..self.size() {
            image.pixels[row] = self.pixels[self.size() - row - 1].clone();
        }

        image
    }

    /// Computes the D4-orbit of the image.
    ///
    /// The dihedral group D4 has order 8 so the resulting set
    /// has between 1 and 8 (unique) elements.
    fn orbit(&self) -> HashSet<Self> {
        let mut set = HashSet::from([self.clone()]);

        let mut img = self.clone();

        for _ in 0..3 {
            img = img.rotate();

            set.insert(img.clone()); // a^k * x
            set.insert(img.vflip()); // a^kb * x
        }

        set
    }

    /// Extracts a sub-image from the image.
    ///
    /// The sub-image starts (top-left corner) at position `pos`
    /// and has size `size`.
    fn get_sub_image(&self, pos: (usize, usize), size: usize) -> Self {
        let mut image = Self::new(size);

        for row in 0..size {
            for col in 0..size {
                image.pixels[row][col] = self.pixels[pos.0 + row][pos.1 + col];
            }
        }

        image
    }

    /// Sets the values of a sub-image of the image.
    ///
    /// Modifies the image in-place so that the square of size `size` with
    /// top-left corner at position `pos` contains the values of `img`.
    fn set_sub_image(&mut self, pos: (usize, usize), img: &Self) {
        for row in 0..img.size() {
            for col in 0..img.size() {
                self.pixels[pos.0 + row][pos.1 + col] = img.pixels[row][col]
            }
        }
    }

    /// Splits the image into a grid of sub-images.
    ///
    /// The sub-images have size 2 if `self.size` is even, and size 3 otherwise.
    fn split(&self) -> Vec<Vec<Self>> {
        let sub_size = if self.size() & 1 == 0 { 2 } else { 3 };
        let grid_size = self.size() / sub_size;

        let mut grid = Vec::with_capacity(grid_size);

        for row in 0..grid_size {
            let mut grid_row = Vec::with_capacity(grid_size);

            for col in 0..grid_size {
                let pos = (row * sub_size, col * sub_size);

                // the sub-image at position pos gets put in grid[row][col]
                grid_row.push(self.get_sub_image(pos, sub_size));
            }

            grid.push(grid_row);
        }

        grid
    }

    /// Merges a grid of images into one image.
    ///
    /// The images of the grid are positioned in the resulting image
    /// according to their position in the grid.
    fn merge(grid: Vec<Vec<Self>>) -> Self {
        let sub_size = grid[0][0].size();
        let size = grid.len() * sub_size;

        let mut image = Image::new(size);

        for row in 0..grid.len() {
            for col in 0..grid.len() {
                let pos = (row * sub_size, col * sub_size);
                image.set_sub_image(pos, &grid[row][col]);
            }
        }

        image
    }

    /// Enhances the image once using a set of rules.
    ///
    /// The image is divided into sub-images of size 2 or 3,
    /// each of which is enhanced according to the rules.
    /// All enhanced images are merged to form the result.
    fn enhance(&self, rules: &HashMap<Image, Image>) -> Self {
        let mut grid = self.split();

        // enhance each image in the grid
        for row in 0..grid.len() {
            for col in 0..grid.len() {
                grid[row][col] = rules.get(&grid[row][col]).unwrap().clone();
            }
        }

        Self::merge(grid)
    }

    /// Counts the number of pixels that are on in the image.
    fn count_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|px| **px).count())
            .sum()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    // starting image
    let mut image = Image::from(".#./..#/###");

    // create a map containing all the rules (including equivalent patterns)
    let rules = input.lines().fold(HashMap::new(), |mut map, line| {
        let mut iter = line.split(" => ");

        let inp_pat: Image = iter.next().unwrap().into();
        let out_pat: Image = iter.next().unwrap().into();

        // add a rule for each pattern equivalent to the input pattern
        for pat in inp_pat.orbit() {
            map.insert(pat, out_pat.clone());
        }

        map
    });

    // --- Part One --- //

    for _ in 0..5 {
        image = image.enhance(&rules);
    }

    let part_one = image.count_pixels();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    for _ in 0..13 {
        image = image.enhance(&rules);
    }

    let part_two = image.count_pixels();

    println!("Part Two: {}", part_two);
}
