use std::collections::HashMap;

/// Finds the index of the largest element in a slice.
/// In case of tie, the smallest index is returned.
///
/// (We assume that the list is not empty)
fn max_index<T: AsRef<[u32]>>(list: T) -> usize {
    let list = list.as_ref();
    let mut max_idx = 0;

    for idx in 0..list.len() {
        if list[idx] > list[max_idx] {
            max_idx = idx;
        }
    }

    max_idx
}

/// Distribute the blocks of a chosen bank into all banks.
fn distribute(banks: &[u32], idx: usize) -> Vec<u32> {
    let mut banks = banks.to_owned();
    let blocks_to_distribute = banks[idx];
    banks[idx] = 0;

    let mut idx = idx;

    for _ in 0..blocks_to_distribute {
        idx = (idx + 1) % banks.len();
        banks[idx] += 1;
    }

    banks
}

/// Detects the cycle in the reallocation routine.
///
/// Returns the indices `(start, end)` of the start and end of the cycle.
/// The routine state after `end` steps is the same as after `start` steps.
fn detect_cycle<T: AsRef<[u32]>>(banks: T) -> (usize, usize) {
    let mut banks = banks.as_ref().to_vec();

    // store the seen configurations and the corresponding step count
    let mut map: HashMap<Vec<u32>, usize> = HashMap::new();

    let mut steps = 0;

    while !map.contains_key(&banks) {
        map.insert(banks.clone(), steps);

        let max_idx = max_index(&banks);
        banks = distribute(&banks, max_idx);

        steps += 1;
    }

    let cycle_start = *map.get(&banks).unwrap();

    (cycle_start, steps)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let banks: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (cycle_start, cycle_end) = detect_cycle(&banks);

    // --- Part One --- //

    let part_one = cycle_end;

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let part_two = cycle_end - cycle_start;

    println!("Part Two: {}", part_two);
}
