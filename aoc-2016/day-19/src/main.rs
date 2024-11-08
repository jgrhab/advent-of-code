use anyhow::Result;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/day-19.txt")?
        .trim()
        .parse::<u32>()?;

    // We only need to know the id of the last elf in the circle.

    // ------------------------------------------

    // Represent the circle of elves by a queue of ids.
    // Ensure that the current elf is always the first element.

    let mut elves: VecDeque<u32> = VecDeque::from_iter(1..=input);

    while elves.len() >= 2 {
        let elf = elves.pop_front().unwrap(); // current elf

        elves.pop_front(); // remove next elf
        elves.push_back(elf); // put current elf at back of queue
    }

    let part_1 = elves[0]; // only one elf remains

    dbg!(part_1);

    // ------------------------------------------

    // Split the circle into two ranges: left and right.
    // Ensure that the current elf is always the first entry in `left`
    // and that we always have left.len >= right.len.
    //
    // The elf across the current elf (left[0]) is:
    // - if left.len > right.len -> the last elf in left
    // - if left.len = right.len -> the first elf in right
    //
    // At each step, we remove the elf across and shift the queues to maintain
    // the current elf at the start of left and maintain the length condition.
    // To do this, we move the current elf to the back of right (the very end of the circle)
    // and move the first elf of right to the end of left (to keep the queues balanced).
    //
    // The process terminates when the right queue is empty; the only element in the
    // left queue is then the last remaining elf.

    let num = (input + 1) / 2 + 1;
    let mut left: VecDeque<u32> = VecDeque::from_iter(1..num);
    let mut right: VecDeque<u32> = VecDeque::from_iter(num..=input);

    while let Some(elf) = right.pop_front() {
        // right.len has been reduced by 1 when popping
        if left.len() > right.len() + 1 {
            left.pop_back(); // the last elf in left is the elf across

            left.push_back(elf);
            right.push_back(left.pop_front().unwrap());
        } else {
            // `elf` is the elf across (first elf in right)
            right.push_back(left.pop_front().unwrap());
            left.push_back(right.pop_front().unwrap());
        }
    }

    let part_2 = left[0];

    dbg!(part_2);

    Ok(())
}
