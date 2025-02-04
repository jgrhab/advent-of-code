use std::collections::HashSet;

// The knot_hash module contains the Knot Hash struct from day-10.
pub mod knot_hash;

use knot_hash::KnotHash;

/// Converts a string of hexadecimal digits to a string of binary digits.
///
/// Each hex digit is represented as four bits.
fn hex_to_binary(hex_str: &str) -> String {
    hex_str
        .chars()
        .map(|ch| ch.to_digit(16).unwrap())
        .map(|d| format!("{d:04b}"))
        .collect::<Vec<_>>()
        .concat()
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .to_string();

    // build a set to contain the coordinates of the used squares (as (row, col)).
    let mut set: HashSet<(i32, i32)> = HashSet::new();

    // fill the set by hashing the input
    for row in 0..128 {
        // compute the KnotHash digest and convert it to a binary string
        let hex_digest = KnotHash::hash(&format!("{input}-{row}"));
        let bin_digest = hex_to_binary(&hex_digest);

        // filters the pairs (row, col) where the value is 1
        let pairs = bin_digest
            .chars()
            .enumerate()
            .filter_map(|(col, ch)| match ch {
                '1' => Some((row, col as i32)),
                _ => None,
            });

        set.extend(pairs);
    }

    // --- Part One --- //

    let part_one = set.len();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let mut component_count = 0;

    // find an element in the set that has not yet been visited
    while let Some(&elt) = set.iter().next() {
        // remove all elements connected to it from the set

        let mut stack = Vec::from([elt]);

        while let Some(current) = stack.pop() {
            let neighbors = [
                (current.0 - 1, current.1),
                (current.0 + 1, current.1),
                (current.0, current.1 - 1),
                (current.0, current.1 + 1),
            ]
            .into_iter()
            .filter(|nbh| set.contains(nbh));

            stack.extend(neighbors);

            set.remove(&current);
        }

        // the component containing the element has been removed from the set
        component_count += 1;
    }

    let part_two = component_count;

    println!("Part Two: {}", part_two);
}
