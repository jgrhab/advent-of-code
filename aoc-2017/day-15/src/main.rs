fn parse_input(input: &str) -> (u64, u64) {
    let nums: Vec<_> = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
        .collect();

    (nums[0], nums[1])
}

const RATIO_A: u64 = 16807;
const RATIO_B: u64 = 48271;
const MODULUS: u64 = 2147483647;

const BITS: u64 = 65535; // = 2^16 - 1 = 0x11...1 (16 1s)

struct Judge;

impl Judge {
    /// Compares the last (lowest) 16 bits of two numbers.
    fn compare(x: u64, y: u64) -> bool {
        (x & BITS) == (y & BITS)
    }
}

struct Generator {
    value: u64,
    ratio: u64,
    modulus: u64,
    condition: fn(u64) -> bool,
}

impl Generator {
    fn new(value: u64, ratio: u64, condition: fn(u64) -> bool) -> Self {
        Self {
            value,
            ratio,
            modulus: MODULUS,
            condition,
        }
    }

    fn update_value(&mut self) {
        self.value = (self.value * self.ratio) % self.modulus;
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.update_value();

        while !(self.condition)(self.value) {
            self.update_value();
        }

        Some(self.value)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (input_a, input_b) = parse_input(&input);

    // --- Part One --- //

    let mut generator_a = Generator::new(input_a, RATIO_A, |_| true);
    let mut generator_b = Generator::new(input_b, RATIO_B, |_| true);

    let mut part_one = 0;

    for _ in 0..40_000_000 {
        let val_a = generator_a.next().unwrap();
        let val_b = generator_b.next().unwrap();

        if Judge::compare(val_a, val_b) {
            part_one += 1;
        }
    }

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let mut generator_a = Generator::new(input_a, RATIO_A, |val| val % 4 == 0);
    let mut generator_b = Generator::new(input_b, RATIO_B, |val| val % 8 == 0);

    let mut part_two = 0;

    for _ in 0..5_000_000 {
        let val_a = generator_a.next().unwrap();
        let val_b = generator_b.next().unwrap();

        if Judge::compare(val_a, val_b) {
            part_two += 1;
        }
    }

    println!("Part Two: {}", part_two);
}
