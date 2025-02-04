/// Follows the instructions and update the offsets.
fn follow<T: AsRef<[i32]>>(list: T, f: impl Fn(i32) -> i32) -> u32 {
    let mut list = Vec::from(list.as_ref());

    let mut steps = 0;
    let mut idx: i32 = 0;

    while (0 <= idx) && ((idx as usize) < list.len()) {
        let next = idx + list[idx as usize];

        list[idx as usize] = f(list[idx as usize]);

        idx = next;
        steps += 1;
    }

    steps
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let list: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();

    // --- Part One --- //

    let part_one = follow(&list, |x| x + 1);

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let part_two = follow(&list, |x| if x >= 3 { x - 1 } else { x + 1 });

    println!("Part Two: {}", part_two);
}
