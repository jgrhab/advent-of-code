use anyhow::Result;

/// Enum representing an argument to a command.
#[derive(Clone, Copy)]
enum Arg {
    Int(i32),   // integer
    Reg(usize), // register
}

impl From<&str> for Arg {
    fn from(string: &str) -> Self {
        if let Ok(int) = string.parse::<i32>() {
            Self::Int(int)
        } else {
            let reg = string.chars().next().unwrap();
            Self::Reg(reg as usize - 'a' as usize)
        }
    }
}

// ----------------------------------------------

/// Command enum reprensenting a command (= instruction).
#[derive(Clone)]
enum Cmd {
    Inc(Arg),      // [Reg]: increase arg0
    Dec(Arg),      // [Reg]: decrease arg0
    Tgl(Arg),      // [IoR]: toggle command arg0 away
    Cpy(Arg, Arg), // [IoR, Reg]: copy arg1 into arg
    Jnz(Arg, Arg), // [IoR, Int]: jump arg2 steps if arg1 is nonzero
}

impl From<&str> for Cmd {
    fn from(string: &str) -> Self {
        let split: Vec<&str> = string.split_whitespace().collect();
        let args: Vec<Arg> = split[1..].into_iter().map(|&s| Arg::from(s)).collect();

        match split[0] {
            "inc" => Cmd::Inc(args[0]),
            "dec" => Cmd::Dec(args[0]),
            "tgl" => Cmd::Tgl(args[0]),
            "cpy" => Cmd::Cpy(args[0], args[1]),
            "jnz" => Cmd::Jnz(args[0], args[1]),
            _ => panic!("invalid command: {string}"),
        }
    }
}

// ----------------------------------------------

struct Program {
    registers: [i32; 4], // values of the registers ['a', 'b', 'c', 'd']
    commands: Vec<Cmd>,
}

impl Program {
    /// Increment the value of `arg0` (register).
    fn increment(&mut self, arg0: &Arg) {
        if let Arg::Reg(reg) = arg0 {
            self.registers[*reg] += 1;
        }
    }

    /// Decrement the value of `arg0` (register).
    fn decrement(&mut self, arg0: &Arg) {
        if let Arg::Reg(reg) = arg0 {
            self.registers[*reg] -= 1;
        }
    }

    /// Toggle the instruction `arg0` (integer or register) steps away of position `ind`.
    fn toggle(&mut self, arg0: &Arg, ind: usize) {
        let x: i32 = match *arg0 {
            Arg::Int(int) => int,
            Arg::Reg(reg) => self.registers[reg],
        };

        let ind = (ind as i32 + x) as usize; // index of the instruction to toggle

        if ind >= self.commands.len() {
            return;
        }

        let new_command = match self.commands[ind] {
            Cmd::Inc(arg0) => Cmd::Dec(arg0),
            Cmd::Dec(arg0) => Cmd::Inc(arg0),
            Cmd::Tgl(arg0) => Cmd::Inc(arg0),
            Cmd::Cpy(arg0, arg1) => Cmd::Jnz(arg0, arg1),
            Cmd::Jnz(arg0, arg1) => Cmd::Cpy(arg0, arg1),
        };

        self.commands[ind] = new_command;
    }

    /// Copy the value of `arg0` (integer or register) into `arg1` (register).
    fn copy(&mut self, arg0: &Arg, arg1: &Arg) {
        let val = match *arg0 {
            Arg::Int(int) => int,
            Arg::Reg(reg) => self.registers[reg],
        };

        match *arg1 {
            Arg::Reg(reg) => self.registers[reg] = val,
            _ => return,
        };
    }

    /// Jump `arg1` (integer or register) steps if `arg0` (integer or register) is not zero.
    /// Return the number of commands to jump minus one (to offset the skip).
    fn jump(&self, arg0: &Arg, arg1: &Arg) -> i32 {
        let jmp = match *arg1 {
            Arg::Int(int) => int,
            Arg::Reg(reg) => self.registers[reg],
        };

        let val = match *arg0 {
            Arg::Int(int) => int,
            Arg::Reg(reg) => self.registers[reg],
        };

        match val {
            0 => 0, // return 0 to ignore the command
            _ => jmp - 1,
        }
    }

    /// Execute the command at index `ind` and return the new index.
    fn execute(&mut self, ind: usize) -> usize {
        let mut ind = ind;

        match self.commands[ind] {
            Cmd::Inc(arg0) => self.increment(&arg0),
            Cmd::Dec(arg0) => self.decrement(&arg0),
            Cmd::Tgl(arg0) => self.toggle(&arg0, ind),
            Cmd::Cpy(arg0, arg1) => self.copy(&arg0, &arg1),
            Cmd::Jnz(arg0, arg1) => {
                let jmp = self.jump(&arg0, &arg1);
                ind = (ind as i32 + jmp) as usize; // new index - 1
            }
        }

        ind + 1
    }

    /// Run the entire program.
    fn run(&mut self) {
        let mut ind = 0;

        while ind < self.commands.len() {
            ind = self.execute(ind);
        }
    }
}

// ----------------------------------------------

fn main() -> Result<()> {
    let commands: Vec<Cmd> = std::fs::read_to_string("input/day-23.txt")?
        .lines()
        .map(Cmd::from)
        .collect();

    // ----------------------------------------------

    let mut program = Program {
        registers: [7, 0, 0, 0],
        commands: commands.clone(),
    };

    program.run();

    let part_1 = program.registers[0];

    dbg!(part_1);

    // ----------------------------------------------

    let mut program = Program {
        registers: [12, 0, 0, 0],
        commands,
    };

    program.run();

    let part_2 = program.registers[0];

    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INP12: [&str; 6] = ["cpy 41 a", "inc a", "inc a", "dec a", "jnz a 2", "dec a"];
    const INP23: [&str; 7] = [
        "cpy 2 a", "tgl a", "tgl a", "tgl a", "cpy 1 a", "dec a", "dec a",
    ];

    #[test]
    fn test_input_day_12() {
        let mut prog = Program {
            registers: [0, 0, 0, 0],
            commands: INP12.into_iter().map(Cmd::from).collect(),
        };

        prog.run();

        assert_eq!(prog.registers[0], 42);
    }

    #[test]
    fn test_input_day_23() {
        let mut prog = Program {
            registers: [0, 0, 0, 0],
            commands: INP23.into_iter().map(Cmd::from).collect(),
        };

        prog.run();

        assert_eq!(prog.registers[0], 3);
    }
}
