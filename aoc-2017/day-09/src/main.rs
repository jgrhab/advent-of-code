/// Computes the score of the input and counts the characters within the garbage.
fn count(stream: &str) -> (u32, u32) {
    let chars: Vec<_> = stream.chars().collect();

    let mut total_score = 0;
    let mut garbage_char_count = 0;

    let mut current_score = 0;
    let mut is_garbage = false;

    let mut idx = 0;

    while idx < chars.len() {
        if is_garbage {
            match chars[idx] {
                '!' => idx += 1,
                '>' => is_garbage = false,
                _ => garbage_char_count += 1,
            }
        } else {
            match chars[idx] {
                '<' => is_garbage = true,
                '{' => current_score += 1,
                '}' => {
                    total_score += current_score;
                    current_score -= 1;
                }
                _ => (),
            }
        }

        idx += 1;
    }

    (total_score, garbage_char_count)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (part_one, part_two) = count(&input);

    println!("Part One: {}", part_one);

    println!("Part Two: {}", part_two);
}
