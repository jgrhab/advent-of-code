use anyhow::Result;
use std::collections::HashMap;

enum IntOrReg {
    Int(i32),
    Reg(char),
}

impl From<&str> for IntOrReg {
    fn from(string: &str) -> Self {
        match string.parse::<i32>() {
            Ok(int) => Self::Int(int),
            Err(_) => Self::Reg(string.chars().next().unwrap()),
        }
    }
}

enum Instruction {
    Cpy { x: IntOrReg, y: char },
    Inc(char),
    Dec(char),
    Jnz { x: IntOrReg, y: i32 },
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let split: Vec<&str> = string.split_whitespace().collect();

        match split[0] {
            "cpy" => {
                let x = IntOrReg::from(split[1]);
                let y = split[2].chars().next().unwrap();
                Self::Cpy { x, y }
            }
            "inc" => Self::Inc(split[1].chars().next().unwrap()),
            "dec" => Self::Dec(split[1].chars().next().unwrap()),
            "jnz" => {
                let x = IntOrReg::from(split[1]);
                let y = split[2].parse::<i32>().unwrap();
                Self::Jnz { x, y }
            }
            _ => panic!("invalid instruction: {}", split[0]),
        }
    }
}

fn execute_instructions(regs: &mut HashMap<char, i32>, instructions: &Vec<Instruction>) {
    let mut ind = 0;

    while ind < instructions.len() {
        match &instructions[ind] {
            Instruction::Inc(reg) => *regs.get_mut(&reg).unwrap() += 1,
            Instruction::Dec(reg) => *regs.get_mut(&reg).unwrap() -= 1,
            Instruction::Cpy { x, y } => match x {
                IntOrReg::Int(val) => *regs.get_mut(y).unwrap() = *val,
                IntOrReg::Reg(reg) => *regs.get_mut(y).unwrap() = *regs.get(reg).unwrap(),
            },
            Instruction::Jnz { x, y } => {
                let val = match x {
                    IntOrReg::Int(val) => *val,
                    IntOrReg::Reg(reg) => *regs.get(reg).unwrap(),
                };

                if val != 0 {
                    // cannot cast y as usize as it can be negative
                    ind = ((ind as i32) + y - 1) as usize; // -1 to offset the +1 after the loop
                }
            }
        };

        ind += 1;
    }
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("input/day-12.txt")?
        .lines()
        .map(Instruction::from)
        .collect();

    let mut regs: HashMap<char, i32> = HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)]);
    execute_instructions(&mut regs, &instructions);

    let part_1 = regs[&'a'];

    dbg!(part_1);

    let mut regs: HashMap<char, i32> = HashMap::from([('a', 0), ('b', 0), ('c', 1), ('d', 0)]);
    execute_instructions(&mut regs, &instructions);

    let part_2 = regs[&'a'];

    dbg!(part_2);

    Ok(())
}
