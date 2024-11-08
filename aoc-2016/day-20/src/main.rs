use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-20.txt")?
        .lines()
        .map(String::from)
        .collect();

    // ------------------------------------------

    // parse the ranges into a vector of (start, end) and sort in increasing order

    let mut ranges: Vec<(u32, u32)> = input
        .into_iter()
        .map(|s| {
            let split: Vec<&str> = s.split('-').collect();
            let lower = split[0].parse::<u32>().unwrap();
            let upper = split[1].parse::<u32>().unwrap();

            (lower, upper)
        })
        .collect();

    ranges.sort();

    // ------------------------------------------

    // merge the ranges that overlap to obtain a Vec of disjoint ranges
    let mut merged: Vec<(u32, u32)> = Vec::new();

    let mut curr = 0;
    while curr < ranges.len() - 1 {
        let mut end = ranges[curr].1;

        let mut next = curr + 1;

        while next < ranges.len() && ranges[next].0 - 1 <= end {
            end = u32::max(end, ranges[next].1);
            next += 1;
        }

        merged.push((ranges[curr].0, end));

        curr = next;
    }

    let part_1 = if merged[0].0 > 0 { 0 } else { merged[0].1 + 1 };

    dbg!(part_1);

    // ------------------------------------------

    let mut part_2 = 0;

    for ind in 0..(merged.len() - 1) {
        part_2 += merged[ind + 1].0 - merged[ind].1 - 1;
    }

    dbg!(part_2);

    Ok(())
}
