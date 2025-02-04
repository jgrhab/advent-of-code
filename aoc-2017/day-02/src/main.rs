/// Computes the diameter of a list of numbers.
///
/// The diameter of a set of numbers is the difference between
/// the largest and the smallest value in the set.
fn compute_diameter<T: AsRef<[u32]>>(nums: T) -> u32 {
    let nums = nums.as_ref();

    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    max - min
}

/// Find a pair of numbers with one evenly dividing the
/// other and return their quotient.
///
/// If there are more than one such pairs, the first
/// found pair is returned.
fn compute_quotient<T: AsRef<[u32]>>(nums: T) -> Option<u32> {
    let nums = nums.as_ref();

    for numerator in nums {
        for denominator in nums.iter().filter(|&num| num < numerator) {
            if numerator % denominator == 0 {
                return Some(numerator / denominator);
            }
        }
    }

    None
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let spreadsheet: Vec<Vec<_>> = input
        .lines()
        .map(|row| {
            row.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    // --- Part One --- //

    let part_one: u32 = spreadsheet.iter().map(compute_diameter).sum();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let part_two: u32 = spreadsheet
        .iter()
        .map(|row| compute_quotient(row).unwrap())
        .sum();

    println!("Part Two: {}", part_two);
}
