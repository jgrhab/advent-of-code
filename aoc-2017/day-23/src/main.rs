enum Arg {
    Int(i32),
    Reg(usize),
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        match value.parse() {
            Ok(int) => Self::Int(int),
            Err(_) => Self::Reg(value.chars().next().unwrap() as usize - 'a' as usize),
        }
    }
}

enum CmdKind {
    Set,
    Sub,
    Mul,
    Jnz,
}

impl From<&str> for CmdKind {
    fn from(value: &str) -> Self {
        match value {
            "set" => Self::Set,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "jnz" => Self::Jnz,
            _ => panic!("invalid command: {}", value),
        }
    }
}

struct Cmd {
    kind: CmdKind,
    arg0: Arg,
    arg1: Arg,
}

impl From<&str> for Cmd {
    fn from(value: &str) -> Self {
        let mut iter = value.split_whitespace();

        let kind = iter.next().unwrap().into();
        let arg0 = iter.next().unwrap().into();
        let arg1 = iter.next().unwrap().into();

        Self { kind, arg0, arg1 }
    }
}

struct Coprocessor {
    registers: [i32; 8],
    offset: i32,
    mul_count: u32,
}

impl Coprocessor {
    fn new() -> Self {
        Self {
            registers: [0; 8],
            offset: 0,
            mul_count: 0,
        }
    }

    fn get(&self, arg: &Arg) -> i32 {
        match *arg {
            Arg::Int(int) => int,
            Arg::Reg(reg) => self.registers[reg],
        }
    }

    fn set(&mut self, arg0: &Arg, arg1: &Arg) {
        if let Arg::Reg(reg) = *arg0 {
            self.registers[reg] = self.get(arg1);
        }
    }

    fn sub(&mut self, arg0: &Arg, arg1: &Arg) {
        if let Arg::Reg(reg) = *arg0 {
            self.registers[reg] -= self.get(arg1);
        }
    }

    fn mul(&mut self, arg0: &Arg, arg1: &Arg) {
        if let Arg::Reg(reg) = *arg0 {
            self.registers[reg] *= self.get(arg1);
            self.mul_count += 1;
        }
    }

    fn jnz(&mut self, arg0: &Arg, arg1: &Arg) {
        if self.get(arg0) != 0 {
            self.offset = self.get(arg1);
        }
    }

    fn execute(&mut self, cmd: &Cmd) -> i32 {
        match cmd.kind {
            CmdKind::Set => self.set(&cmd.arg0, &cmd.arg1),
            CmdKind::Sub => self.sub(&cmd.arg0, &cmd.arg1),
            CmdKind::Mul => self.mul(&cmd.arg0, &cmd.arg1),
            CmdKind::Jnz => self.jnz(&cmd.arg0, &cmd.arg1),
        }

        let jump = if self.offset != 0 { self.offset } else { 1 };
        self.offset = 0;

        jump
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let cmds: Vec<_> = input.lines().map(Cmd::from).collect();

    // --- Part One --- //

    let mut proc = Coprocessor::new();

    let mut idx: i32 = 0;

    while (0 <= idx) && (idx < cmds.len() as i32) {
        idx += proc.execute(&cmds[idx as usize]);
    }

    let part_one = proc.mul_count;

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    // We read the assembly code and implement the resulting program,
    // adding some optimizations.

    // get the value that gets put in register b at the start
    let x = match cmds[0].arg1 {
        Arg::Int(int) => int as u64,
        Arg::Reg(_) => unreachable!(),
    };

    // initialize registers b and c
    let mut b = 100_000 + 100 * x;
    let c = b + 17_000;

    // The program counts the composite numbers of the form n = b + 17k,
    // with b <= n <= c.

    let mut h = 0;

    while b <= c {
        let mut f = 1;

        // Check whether b is prime.
        // Here, we optimize the code, as the original code uses two loops:
        // for d in 2..b {
        //     for e in 2..b {
        //         // check if d * e == b
        //     }
        // }
        for n in 2..b {
            if b % n == 0 {
                f = 0;
                break;
            }
        }

        if f == 0 {
            h += 1;
        }

        b += 17;
    }

    // We could optimize the colde further, but there is no point.
    // Instead, we keep it as close to the input as possible.

    let part_two = h;

    println!("Part Two: {}", part_two);
}
