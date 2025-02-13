use std::collections::{HashMap, VecDeque};

type Queue = VecDeque<i64>;

/// Enum representing an argument to a command.
///
/// The arguments can be either integers or registers.
#[derive(Clone, Copy)]
enum Arg {
    Int(i64),
    Reg(char),
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        match value.parse() {
            Ok(int) => Self::Int(int),
            Err(_) => Self::Reg(value.chars().next().unwrap()),
        }
    }
}

/// Enum representing the possible commands (= instructions).
///
/// Each command takes one or two arguments of type `Arg`.
#[derive(Clone)]
enum Cmd {
    Snd(Arg),      // (IoR)
    Rcv(Arg),      // (IoR)
    Set(Arg, Arg), // (Reg, IoR)
    Add(Arg, Arg), // (Reg, IoR)
    Mul(Arg, Arg), // (Reg, IoR)
    Mod(Arg, Arg), // (Reg, IoR)
    Jgz(Arg, Arg), // (IoR, IoR)
}

impl From<&str> for Cmd {
    fn from(value: &str) -> Self {
        let split: Vec<_> = value.split_whitespace().collect();

        let args: Vec<Arg> = split[1..].iter().map(|&s| Arg::from(s)).collect();

        match split[0] {
            "snd" => Cmd::Snd(args[0]),
            "set" => Cmd::Set(args[0], args[1]),
            "add" => Cmd::Add(args[0], args[1]),
            "mul" => Cmd::Mul(args[0], args[1]),
            "mod" => Cmd::Mod(args[0], args[1]),
            "rcv" => Cmd::Rcv(args[0]),
            "jgz" => Cmd::Jgz(args[0], args[1]),
            _ => panic!("invalid instruction: {}", value),
        }
    }
}

/// Status of a program as it exits the run loop.
///
/// The program exits with the `Wait` status when it
/// attempts to recover a value that is not present.
/// In this case, the index of the 'rcv' command is returned.
///
/// The program exits with the `Term` status when the index goes
/// out of bounds, which means that the program has terminated.
#[derive(PartialEq, Debug)]
enum Status {
    Wait(i64),
    Term,
}

impl Status {
    /// Returns the index stored in the status.
    ///
    /// Returns -1 for `Term` so that calling Program::run() with
    /// this value does nothing and returns `Term` once more.
    fn index(&self) -> i64 {
        match self {
            Status::Wait(idx) => *idx,
            Status::Term => -1,
        }
    }
}

struct Program {
    registers: HashMap<char, i64>,
    send_count: u32,
}

impl Program {
    fn new() -> Self {
        Self {
            registers: HashMap::new(),
            send_count: 0,
        }
    }

    /// Returns the value of the argument.
    ///
    /// If the argument is an `Int`, returns its value.
    /// If the argument is a `Reg`, returns the value in the register.
    /// If the register is not initialized, returns the starting value 0.
    fn get_value(&self, arg: &Arg) -> i64 {
        match *arg {
            Arg::Int(int) => int,
            Arg::Reg(reg) => *self.registers.get(&reg).unwrap_or(&0),
        }
    }

    /// Executes the 'snd' command.
    ///
    /// Sends the value of the argurment to the back of the queue.
    fn cmd_snd(&mut self, arg: &Arg, queue: &mut Queue) {
        queue.push_back(self.get_value(arg));
        self.send_count += 1;
    }

    /// Executes the 'rcv' command.
    ///
    /// Recovers the value at the front of the queue and stores it in
    /// the register determined by the argument.
    fn cmd_rcv(&mut self, arg: &Arg, queue: &mut Queue) -> Result<(), ()> {
        let val = queue.pop_front().ok_or(())?;

        self.cmd_set(arg, &Arg::Int(val));

        Ok(())
    }

    /// Executes the 'set' command.
    ///
    /// Sets the value of the register `arg0` to `arg1`.
    /// Note: Does nothing if `arg0` is not a register.
    fn cmd_set(&mut self, arg0: &Arg, arg1: &Arg) {
        if let Arg::Reg(reg) = *arg0 {
            self.registers.insert(reg, self.get_value(arg1));
        }
    }

    /// Executes the 'add' command.
    fn cmd_add(&mut self, arg0: &Arg, arg1: &Arg) {
        if let Arg::Reg(reg) = *arg0 {
            *self.registers.entry(reg).or_insert(0) += self.get_value(arg1);
        }
    }

    /// Executes the 'mul' command.
    fn cmd_mul(&mut self, arg0: &Arg, arg1: &Arg) {
        if let Arg::Reg(reg) = *arg0 {
            *self.registers.entry(reg).or_insert(0) *= self.get_value(arg1);
        }
    }

    /// Executes the 'mod' command.
    fn cmd_mod(&mut self, arg0: &Arg, arg1: &Arg) {
        if let Arg::Reg(reg) = *arg0 {
            *self.registers.entry(reg).or_insert(0) %= self.get_value(arg1);
        }
    }

    /// Executes the 'jnz' command.
    /// Returns the offset (relative position) to the next index.
    fn cmd_jgz(&self, arg0: &Arg, arg1: &Arg) -> i64 {
        match self.get_value(arg0) {
            1.. => self.get_value(arg1),
            _ => 1,
        }
    }

    /// Executes a command.
    ///
    /// Values sent by 'snd' commands are added to the back of the `snd_queue`.
    /// Values recovered by 'rcv' commands are taken from the front of the `rcv_queue`.
    ///
    /// Returns a `Result` whose `Ok` variant contains the offset (relative position)
    /// of the next instruction. (This is 1 for all but the 'jgz' command).
    /// Returns an `Err` when trying to recover a value from an empty `rcv_queue`.
    fn execute(
        &mut self,
        cmd: &Cmd,
        snd_queue: &mut Queue,
        rcv_queue: &mut Queue,
    ) -> Result<i64, ()> {
        match cmd {
            Cmd::Snd(arg) => self.cmd_snd(arg, snd_queue),
            Cmd::Rcv(arg) => self.cmd_rcv(arg, rcv_queue)?,
            Cmd::Set(arg0, arg1) => self.cmd_set(arg0, arg1),
            Cmd::Add(arg0, arg1) => self.cmd_add(arg0, arg1),
            Cmd::Mul(arg0, arg1) => self.cmd_mul(arg0, arg1),
            Cmd::Mod(arg0, arg1) => self.cmd_mod(arg0, arg1),
            Cmd::Jgz(arg0, arg1) => return Ok(self.cmd_jgz(arg0, arg1)),
        }

        Ok(1)
    }

    /// Runs the program until it terminates or waits for a value.
    ///
    /// Runs the set of commands starting at index `idx`.
    /// Sends values to `snd_queue` and recovers values from `rcv_queue`.
    ///
    /// The function returns when either
    /// * it terminates, that is the index goes out of bounds,
    /// * it tries to recover a value from an empty `rcv_queue`.
    ///
    /// In the second case, the returned `Status` contains the index of
    /// the 'rcv' command that could not be executed.
    fn run(
        &mut self,
        cmds: &[Cmd],
        start_idx: i64,
        snd_queue: &mut Queue,
        rcv_queue: &mut Queue,
    ) -> Status {
        let mut idx = start_idx;

        while 0 <= idx && idx < cmds.len() as i64 {
            match self.execute(&cmds[idx as usize], snd_queue, rcv_queue) {
                Ok(val) => idx += val,
                Err(_) => return Status::Wait(idx),
            };
        }

        Status::Term
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let cmds: Vec<_> = input.lines().map(Cmd::from).collect();

    // --- Part One --- //

    let mut program = Program::new();

    // only store the snd_queue as we never read any value
    let mut queue = Queue::new();

    let mut idx = 0;

    while let Status::Wait(out_idx) = program.run(&cmds, idx, &mut queue, &mut Queue::new()) {
        // Recover the argument from the command which caused the program to exit.
        // The Waiting status is only returned on a 'rcv' command.
        let arg = match &cmds[out_idx as usize] {
            Cmd::Rcv(arg) => arg,
            _ => unreachable!(),
        };

        // exit the loop when first trying to read a value
        if program.get_value(arg) != 0 {
            break;
        }

        // ignore the 'rcv' command when the arg value is 0
        idx += 1;
    }

    // The program has terminated when trying to recover a value for the first time.
    // The value to recover is the last value added to the out_queue.
    let part_one = queue.into_iter().last().unwrap();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let mut programs = (Program::new(), Program::new());

    programs.0.cmd_set(&Arg::Reg('p'), &Arg::Int(0));
    programs.1.cmd_set(&Arg::Reg('p'), &Arg::Int(1));

    // snd_queues for each program (and also rec_queue for the other program)
    let mut queues = (Queue::new(), Queue::new());

    let mut idx = (0, 0); // starting indices
    let mut status = (Status::Wait(0), Status::Wait(0)); // dummy starting values

    while status.0 != Status::Term || status.1 != Status::Term {
        // run both programs until lock or termination
        status.0 = programs.0.run(&cmds, idx.0, &mut queues.0, &mut queues.1);
        status.1 = programs.1.run(&cmds, idx.1, &mut queues.1, &mut queues.0);

        // recover the exit index of each program
        let exit_idx = (status.0.index(), status.1.index());

        // we reach a deadlock if both queues are empty
        if queues.0.is_empty() && queues.1.is_empty() {
            break;
        }

        idx = exit_idx;
    }

    let part_two = programs.1.send_count;

    println!("Part Two: {}", part_two);
}
