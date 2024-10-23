use anyhow::Result;
use std::collections::HashMap;

/// Compute the number of times that each character appears in each column.
/// Return a vector with one entry per column, containing the character count as hashmap.
fn column_char_count(input: &Vec<String>) -> Vec<HashMap<char, u32>> {
    let input: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();

    let rows = input.len();
    let cols = input[0].len();

    let mut maps: Vec<HashMap<char, u32>> = Vec::with_capacity(cols);

    for col in 0..cols {
        let mut map: HashMap<char, u32> = HashMap::new();

        for row in 0..rows {
            let count: &mut u32 = map.entry(input[row][col]).or_insert(0);
            *count += 1;
        }

        maps.push(map);
    }

    maps
}

/// Compute the message formed by taking the most common character in each column.
fn column_most_common(maps: &Vec<HashMap<char, u32>>) -> String {
    maps.into_iter()
        .map(|map| {
            map.into_iter()
                .max_by(|&x, &y| x.1.cmp(&y.1))
                .expect("max not found: {x}, {y}")
                .0
        })
        .collect()
}

/// Compute the message formed by taking the least common character in each column.
fn column_least_common(maps: &Vec<HashMap<char, u32>>) -> String {
    maps.into_iter()
        .map(|map| {
            map.into_iter()
                .min_by(|&x, &y| x.1.cmp(&y.1))
                .expect("min not found: {x}, {y}")
                .0
        })
        .collect()
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-06.txt")?
        .lines()
        .map(String::from)
        .collect();

    let maps = column_char_count(&input);

    let part_1 = column_most_common(&maps);

    dbg!(part_1);

    let part_2 = column_least_common(&maps);

    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\n\
                        nssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";

    #[test]
    fn part_1() {
        let input: Vec<String> = INPUT.lines().map(String::from).collect();
        let maps = column_char_count(&input);
        let msg = column_most_common(&maps);

        assert_eq!("easter", msg);
    }

    #[test]
    fn part_2() {
        let input: Vec<String> = INPUT.lines().map(String::from).collect();
        let maps = column_char_count(&input);
        let msg = column_least_common(&maps);

        assert_eq!("advent", msg);
    }
}
