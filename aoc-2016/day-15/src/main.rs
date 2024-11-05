use anyhow::Result;

// Let f_k(t) be the position of disc #k at time t.
// If disc #k has m_k positions, then this is given by f_k(t) = f_k(0) + t mod m_k.
// For the capsule to pass through each disc successively starting at time t_0, we need
// f_k(t_0 + k) = 0 for all k, which is equivalent to t_0 = -f_k(0) - 1 mod m_k for all k.
//
// Since the m_k are all prime (in my input), there is a unique solution mod m = prod{m_k}.
// (If the coprimality condition is not met, we need to check that the conditions are compatible.)
// We can find the solution by iterating over all integers mod m (the naive way).

/// Return the number of positions m_k and starting position f_k(0)
fn parse_line(line: String) -> (u32, u32) {
    let split: Vec<&str> = line.split_whitespace().collect();

    let count = split[3].parse::<u32>().unwrap();
    let start = split[11].trim_end_matches('.').parse::<u32>().unwrap();

    (count, start)
}

/// Find solution to the congruence system using systematic search
/// see: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Systematic_search
fn systematic_search(discs: &Vec<(u32, u32)>) -> Option<u32> {
    let modulus: u32 = discs.iter().map(|(count, _)| *count).product();

    'x: for x in 0..modulus {
        for (idx, (count, start)) in discs.iter().enumerate() {
            let k = idx as u32 + 1;
            if (x + start + k) % count != 0 {
                continue 'x;
            }
        }

        return Some(x);
    }

    None
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-15.txt")?
        .lines()
        .map(String::from)
        .collect();

    let mut discs: Vec<_> = input.into_iter().map(parse_line).collect();

    // ------------------------------------------

    let part_1 = systematic_search(&discs).unwrap();

    dbg!(part_1);

    // ------------------------------------------

    discs.push((11, 0));

    let part_2 = systematic_search(&discs).unwrap();

    dbg!(part_2);

    Ok(())
}
