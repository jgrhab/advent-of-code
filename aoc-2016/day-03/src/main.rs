use anyhow::Result;

fn is_possible(triple: [u32; 3]) -> bool {
    triple[0] + triple[1] > triple[2]
        && triple[1] + triple[2] > triple[0]
        && triple[2] + triple[0] > triple[1]
}

fn parse_input(input: Vec<String>) -> Result<Vec<[u32; 3]>> {
    let mut matrix: Vec<[u32; 3]> = Vec::with_capacity(input.len());

    for line in input {
        let mut tr: [u32; 3] = [0, 0, 0];
        for (ind, s) in line.split_whitespace().enumerate() {
            tr[ind] = s.parse()?;
        }

        matrix.push(tr);
    }

    Ok(matrix)
}

fn count_vertical(matrix: &Vec<[u32; 3]>) -> u32 {
    let mut count: u32 = 0;

    for col in 0..3 {
        for row in (0..matrix.len()).step_by(3) {
            let tr = [matrix[row][col], matrix[row + 1][col], matrix[row + 2][col]];
            if is_possible(tr) {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-03.txt")?
        .lines()
        .map(String::from)
        .collect();

    let matrix = parse_input(input)?;

    let part_1 = matrix.iter().filter(|&&tr| is_possible(tr)).count();

    dbg!(part_1);

    let part_2 = count_vertical(&matrix);

    dbg!(part_2);

    Ok(())
}
