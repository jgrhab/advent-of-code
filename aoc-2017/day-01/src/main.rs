/// Sum the values of all digits matching the corresponding offset digit.
///
/// The offset of index `ind` is the index `ind + offset`,
/// modulo the length of the list.
fn sum_matching_offset(digits: &[u32], offset: usize) -> u32 {
    let mut sum = 0;

    for ind in 0..digits.len() {
        if digits[ind] == digits[(ind + offset) % digits.len()] {
            sum += digits[ind];
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let digits: Vec<u32> = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();

    // --- Part One --- //

    let part_one = sum_matching_offset(&digits, 1);

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let part_two = sum_matching_offset(&digits, digits.len() / 2);

    println!("Part Two: {}", part_two);
}
