fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input: usize = input.trim().parse().unwrap();

    // --- Part One --- //

    let mut buffer: Vec<usize> = Vec::with_capacity(2018);
    buffer.push(0);

    let mut idx = 0;

    for val in 1..=2017 {
        let insert_idx = (idx + input) % buffer.len() + 1;

        buffer.insert(insert_idx, val);
        idx = insert_idx;
    }

    let part_one = buffer[idx + 1];

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    // For Part Two, we do not need to fill the buffer.
    // We only need to determine the last value which gets
    // inserted at position 1.

    let mut values = Vec::new();

    let mut len = 1;
    let mut idx = 0;

    for val in 1..=50_000_000 {
        idx = (idx + input) % len + 1;
        len += 1;

        if idx == 1 {
            values.push(val);
        }
    }

    let part_two = values.into_iter().last().unwrap();

    println!("Part Two: {}", part_two);
}
