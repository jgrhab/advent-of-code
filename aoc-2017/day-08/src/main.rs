use std::cmp::Ordering;
use std::collections::HashMap;

/// Represents a comparison of the form `register cmp value`.
struct Comparison {
    register: String,
    value: i32,
    ord: Ordering,
    invert_ord: bool,
}

/// Structure representing an increment instruction.
/// The value in `register` is incremented by `value`
/// provided that `condition` holds.
struct Instruction {
    register: String,
    value: i32,
    condition: Comparison,
}

impl From<&str> for Comparison {
    fn from(value: &str) -> Self {
        let mut iter = value.split_whitespace();

        let register = iter.next().unwrap().to_string();

        let (ord, invert_ord) = match iter.next().unwrap() {
            ">" => (Ordering::Greater, false),
            "<" => (Ordering::Less, false),
            "==" => (Ordering::Equal, false),
            "<=" => (Ordering::Greater, true),
            ">=" => (Ordering::Less, true),
            "!=" => (Ordering::Equal, true),
            _ => unreachable!(),
        };

        let value = iter.next().unwrap().parse().unwrap();

        Self {
            register,
            value,
            ord,
            invert_ord,
        }
    }
}

impl Comparison {
    fn holds(&self, registers: &HashMap<String, i32>) -> bool {
        let register_value = *registers.get(&self.register).unwrap_or(&0);

        let result = register_value.cmp(&self.value) == self.ord;

        // invert the result iff invert_ord is true (using XOR)
        self.invert_ord ^ result
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut iter = value.split(" if ");

        let mut head = iter.next().unwrap().split_whitespace();

        let register = head.next().unwrap().to_string();
        let mult = if head.next().unwrap() == "inc" { 1 } else { -1 };
        let value = mult * head.next().unwrap().parse::<i32>().unwrap();

        let condition = Comparison::from(iter.next().unwrap());

        Self {
            register,
            value,
            condition,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let instructions: Vec<_> = input.lines().map(Instruction::from).collect();

    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut max_value = 0; // highest value attained during the process

    // execute the instructions and keep track of the highest value attained
    for ins in instructions {
        if ins.condition.holds(&registers) {
            let entry = registers.entry(ins.register).or_insert(0);
            *entry += ins.value;

            max_value = i32::max(max_value, *entry);
        }
    }

    // --- Part One --- //

    let part_one = registers.values().map(|val| *val).max().unwrap();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let part_two = max_value;

    println!("Part Two: {}", part_two);
}
