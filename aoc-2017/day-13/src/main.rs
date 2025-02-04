use std::collections::HashMap;

// The packet enters the layer [depth:range] at time t := depth + delay.
// The packet gets caught by the layer if the scanner is at position 0
// when the packet enters the layer.
// The scanner position is periodic with period 2 * (range - 1),
// so it is at position 0 when t = 0 mod 2(range-1).

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let layers: HashMap<u32, u32> = input.lines().fold(HashMap::new(), |mut map, line| {
        let mut iter = line.split(": ");

        let depth = iter.next().unwrap().parse().unwrap();
        let range = iter.next().unwrap().parse().unwrap();

        map.insert(depth, range);

        map
    });

    // --- Part One --- //

    // For Part One, the delay is zero.
    // The packet gets caught by layer [depth:range] iff
    // depth = 0 mod 2 * (range - 1).

    let part_one: u32 = layers
        .iter()
        .filter(|&(depth, range)| depth % (2 * (range - 1)) == 0)
        .map(|(depth, range)| depth * range)
        .sum();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    // The packet gets caught by layer [depth:range] iff
    // (depth + delay) % (2 * (range - 1) == 0.
    // We need to find the smallest delay such that
    // this equality is false for all layers.

    let mut delay = 1;

    'outer: loop {
        // check each layer
        for (&depth, &range) in layers.iter() {
            let time = delay + depth;
            let modulus = 2 * (range - 1);

            // skip to the next iteration as soon as one layer catches the packet
            if time % modulus == 0 {
                delay += 1;
                continue 'outer;
            }
        }

        break;
    }

    let part_two = delay;

    println!("Part Two: {}", part_two);
}
