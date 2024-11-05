use anyhow::Result;

fn dragon_curve(length: usize, input: &Vec<bool>) -> Vec<bool> {
    let mut vec: Vec<bool> = input.clone();

    while vec.len() < length {
        let mut new: Vec<bool> = vec.clone().into_iter().rev().map(|b| !b).collect();
        vec.push(false);
        vec.append(&mut new);
    }

    let _ = vec.drain(length..);

    vec
}

fn checksum(curve: &Vec<bool>) -> Vec<bool> {
    // NOTE the length of `curve` needs to be even

    let mut checksum: Vec<bool> = Vec::new();

    // reduce the input curve to obtain the first checksum
    for ind in 0..(curve.len() / 2) {
        checksum.push(curve[2 * ind] == curve[2 * ind + 1]);
    }

    // repeat the process while the length is even
    while checksum.len() % 2 == 0 {
        let mut reduced: Vec<bool> = Vec::new();

        for ind in 0..(checksum.len() / 2) {
            reduced.push(checksum[2 * ind] == checksum[2 * ind + 1]);
        }

        checksum = reduced;
    }

    checksum
}

fn to_string(curve: &Vec<bool>) -> String {
    curve
        .into_iter()
        .map(|b| if *b { '1' } else { '0' })
        .collect()
}

fn main() -> Result<()> {
    let input: Vec<bool> = std::fs::read_to_string("input/day-16.txt")?
        .trim()
        .chars()
        .map(|ch| ch == '1')
        .collect();

    // ------------------------------------------

    let curve = dragon_curve(272, &input);

    let part_1 = to_string(&checksum(&curve));

    dbg!(part_1);

    // ------------------------------------------

    let curve = dragon_curve(35651584, &input);

    let part_2 = to_string(&checksum(&curve));

    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compute_dragon_curve() {
        let input: Vec<bool> = "10000".chars().map(|ch| ch == '1').collect();
        let expect: Vec<bool> = "10000011110010000111".chars().map(|ch| ch == '1').collect();

        let result = dragon_curve(20, &input);

        assert_eq!(result, expect);
    }

    #[test]
    fn compute_checksum() {
        let curve: Vec<bool> = "110010110100".chars().map(|ch| ch == '1').collect();
        let expect = vec![true, false, false];

        let result = checksum(&curve);

        assert_eq!(result, expect);
    }
}
