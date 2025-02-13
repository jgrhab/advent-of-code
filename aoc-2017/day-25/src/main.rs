use std::collections::HashSet;

struct InputData {
    start_state: usize,
    steps: u32,
    states: Vec<State>,
}

impl InputData {
    /// Returns the (index of) the starting state from the first input line.
    fn parse_first_line(line: &str) -> usize {
        let state = line.split_whitespace().last().unwrap()[0..1]
            .chars()
            .next()
            .unwrap();

        state as usize - 'A' as usize
    }

    /// Returns the number of steps from the second input line.
    fn parse_second_line(line: &str) -> u32 {
        line.split_whitespace()
            .nth_back(1)
            .unwrap()
            .parse()
            .unwrap()
    }

    /// Parse the three lines defining an action.
    fn parse_action_block(block: &[&str]) -> Action {
        // keep only the last word of each line
        let data: Vec<_> = block
            .iter()
            .map(|line| line.split_whitespace().last().unwrap())
            .collect();

        Action {
            write_value: data[0] == "1.",
            move_right: data[1] == "right.",
            next_state: data[2].chars().next().unwrap() as usize - 'A' as usize,
        }
    }

    /// Parse the nine lines of a state block.
    fn parse_state_block(block: &str) -> State {
        let lines: Vec<_> = block.lines().collect();

        let action0 = Self::parse_action_block(&lines[2..=4]);
        let action1 = Self::parse_action_block(&lines[6..=8]);

        let actions = [action0, action1];

        State { actions }
    }
}

impl From<&str> for InputData {
    fn from(value: &str) -> Self {
        // iterate over each "block" of the input
        let mut blocks = value.split("\n\n");

        // the first block is the header
        let mut header = blocks.next().unwrap().lines();

        let start_state = Self::parse_first_line(header.next().unwrap());
        let steps = Self::parse_second_line(header.next().unwrap());

        let states = blocks.fold(Vec::new(), |mut vec, block| {
            vec.push(Self::parse_state_block(block));
            vec
        });

        Self {
            start_state,
            steps,
            states,
        }
    }
}

/// Action to take in a given state when the cursor is on a given value.
///
/// An action consists of three steps:
/// * write a value (0 or 1) under the cursor
/// * move the cursor one step (left or right)
/// * update the state of the Turing Machine.
struct Action {
    /// Value to write under the cursor.
    write_value: bool,

    /// Whether to move right after writing (false = move left).
    move_right: bool,

    /// Index of the state to continue to after moving.
    next_state: usize,
}

/// State of the Turing Machine.
///
/// A state correspond to two possible actions,
/// which get executed depending on the value under the cursor.
struct State {
    actions: [Action; 2],
}

struct TuringMachine {
    /// List of states of the machine.
    /// States are identified by their index in the list.
    states: Box<[State]>,

    /// Current state that the machine is in.
    state: usize,

    /// Set containing the positions which have the value 1 on the tape.
    ones: HashSet<i64>,

    /// Position of the cursor on the tape.
    /// Moving right increases the position, moving left decreases it.
    /// The cursor starts at position zero.
    cursor: i64,
}

impl TuringMachine {
    fn new(states: Vec<State>, start_state: usize) -> Self {
        Self {
            states: states.into_boxed_slice(),
            state: start_state,
            ones: HashSet::new(),
            cursor: 0,
        }
    }

    fn step(&mut self) {
        let current_value = self.ones.contains(&self.cursor) as usize;
        let action = &self.states[self.state].actions[current_value];

        match action.write_value {
            true => self.ones.insert(self.cursor),
            false => self.ones.remove(&self.cursor),
        };

        self.cursor += if action.move_right { 1 } else { -1 };
        self.state = action.next_state;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let input_data = InputData::from(&input[..]);

    let total_steps = input_data.steps;

    let mut machine = TuringMachine::new(input_data.states, input_data.start_state);

    // --- Part One --- //

    for _ in 0..total_steps {
        machine.step();
    }

    let part_one = machine.ones.len();

    println!("Part One: {}", part_one);
}
