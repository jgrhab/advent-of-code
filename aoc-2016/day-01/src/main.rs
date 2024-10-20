use anyhow::{bail, Result};
use num_complex::Complex;
use std::collections::HashSet;
use std::fs;

struct Instruction {
    dir: Complex<i32>, // better to use enum?
    len: i32,
}

fn parse_instructions(input: Vec<String>) -> Result<Vec<Instruction>> {
    let mut ins: Vec<Instruction> = Vec::with_capacity(input.len());

    for string in input {
        let len: i32 = string[1..].parse()?;
        let dir: Complex<i32> = match string.chars().next() {
            Some('L') => Complex::new(0, 1),
            Some('R') => Complex::new(0, -1),
            Some(ch) => bail!("invalid direction: {ch}"),
            None => bail!("empty instruction String"),
        };

        ins.push(Instruction { dir, len });
    }

    Ok(ins)
}

fn compute_destination(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut pos = Complex::new(0, 0); // starting position
    let mut dir = Complex::new(0, 1); // starting direction

    for ins in instructions {
        dir = dir * ins.dir;
        pos = pos + ins.len * dir;
    }

    (pos.re.abs(), pos.im.abs())
}

fn first_visited_twice(instructions: &Vec<Instruction>) -> Option<(i32, i32)> {
    let mut pos = Complex::new(0, 0); // starting position
    let mut dir = Complex::new(0, 1); // starting direction

    // build a set to contain all visited positions
    let mut set: HashSet<Complex<i32>> = HashSet::new();
    set.insert(pos); // insert starting position

    for ins in instructions {
        dir = dir * ins.dir;

        // walk one step at the time and add all visited positions to the set
        for _ in 1..=ins.len {
            pos = pos + dir;

            if set.contains(&pos) {
                return Some((pos.re.abs(), pos.im.abs()));
            }

            set.insert(pos);
        }
    }

    None
}

fn main() -> Result<()> {
    let input: Vec<String> = fs::read_to_string("input/day-01.txt")?
        .trim()
        .split(", ")
        .map(String::from)
        .collect();

    let ins = parse_instructions(input)?;

    let dest = compute_destination(&ins);
    let part_1 = dest.0 + dest.1;

    dbg!(part_1);

    let fwt = first_visited_twice(&ins).expect("no position visited twice");
    let part_2 = fwt.0 + fwt.1;

    dbg!(part_2);

    Ok(())
}
