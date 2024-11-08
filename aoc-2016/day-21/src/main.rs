use anyhow::Result;
use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    SwapPosition(usize, usize), // swap position X with position Y
    SwapLetter(char, char),     // swap letter X with letter Y
    RotateLeft(usize),          // rotate left X steps
    RotateRight(usize),         // rotate right X steps
    RotatePosition(char),       // rotate based on position of letter X
    Reverse(usize, usize),      // reverse positions X through Y
    Move(usize, usize),         // move position X to position Y
}

impl From<&str> for Operation {
    fn from(string: &str) -> Self {
        let split: Vec<&str> = string.split_whitespace().collect();

        match &split[..=1] {
            &["swap", "position"] => {
                let x = split[2].parse::<usize>().unwrap();
                let y = split[5].parse::<usize>().unwrap();
                Self::SwapPosition(x, y)
            }
            &["swap", "letter"] => {
                let x = split[2].chars().next().unwrap();
                let y = split[5].chars().next().unwrap();
                Self::SwapLetter(x, y)
            }
            &["rotate", "left"] => {
                let x = split[2].parse::<usize>().unwrap();
                Self::RotateLeft(x)
            }
            &["rotate", "right"] => {
                let x = split[2].parse::<usize>().unwrap();
                Self::RotateRight(x)
            }
            &["rotate", "based"] => {
                let x = split[6].chars().next().unwrap();
                Self::RotatePosition(x)
            }
            &["reverse", "positions"] => {
                let x = split[2].parse::<usize>().unwrap();
                let y = split[4].parse::<usize>().unwrap();
                Self::Reverse(x, y)
            }
            &["move", "position"] => {
                let x = split[2].parse::<usize>().unwrap();
                let y = split[5].parse::<usize>().unwrap();
                Self::Move(x, y)
            }
            _ => panic!(),
        }
    }
}

struct Password(VecDeque<char>);

impl ToString for Password {
    fn to_string(&self) -> String {
        String::from_iter(self.0.iter())
    }
}

impl Password {
    fn new(string: &str) -> Self {
        Self(VecDeque::from_iter(string.chars()))
    }

    fn swap_position(&mut self, x: usize, y: usize) {
        (self.0[x], self.0[y]) = (self.0[y], self.0[x]);
    }

    fn swap_letter(&mut self, x: char, y: char) {
        let ind_x = self.0.iter().position(|ch| *ch == x).unwrap();
        let ind_y = self.0.iter().position(|ch| *ch == y).unwrap();
        self.swap_position(ind_x, ind_y);
    }

    fn rotate_left(&mut self, x: usize) {
        for _ in 0..x {
            let ch = self.0.pop_front().unwrap();
            self.0.push_back(ch);
        }
    }

    fn rotate_right(&mut self, x: usize) {
        for _ in 0..x {
            let ch = self.0.pop_back().unwrap();
            self.0.push_front(ch);
        }
    }

    fn rotate_position(&mut self, x: char) {
        let ind_x = self.0.iter().position(|ch| *ch == x).unwrap();
        let num = 1 + ind_x + if ind_x >= 4 { 1 } else { 0 };
        self.rotate_right(num);
    }

    fn reverse(&mut self, x: usize, y: usize) {
        let mut tail = self.0.drain((y + 1)..).collect();
        let mut middle = self.0.drain(x..=y).rev().collect();

        self.0.append(&mut middle);
        self.0.append(&mut tail);
    }

    fn move_position(&mut self, x: usize, y: usize) {
        let ch = self.0.remove(x).unwrap();
        self.0.insert(y, ch);
    }

    fn execute(&mut self, op: &Operation) {
        match *op {
            Operation::SwapPosition(x, y) => self.swap_position(x, y),
            Operation::SwapLetter(x, y) => self.swap_letter(x, y),
            Operation::RotateLeft(x) => self.rotate_left(x),
            Operation::RotateRight(x) => self.rotate_right(x),
            Operation::RotatePosition(x) => self.rotate_position(x),
            Operation::Reverse(x, y) => self.reverse(x, y),
            Operation::Move(x, y) => self.move_position(x, y),
        }
    }

    /// Reverse the operation.
    /// To reverse RotatePosition, we compute (by hand) the inverse function
    /// for the length of the input (8 characters).
    fn execute_inverse(&mut self, op: &Operation) {
        match *op {
            Operation::SwapPosition(x, y) => self.swap_position(x, y),
            Operation::SwapLetter(x, y) => self.swap_letter(x, y),
            Operation::RotateLeft(x) => self.rotate_right(x),
            Operation::RotateRight(x) => self.rotate_left(x),
            Operation::RotatePosition(x) => {
                let ind_x = self.0.iter().position(|ch| *ch == x).unwrap();
                let inverse_ind = [1, 1, 6, 2, 7, 3, 0, 4];
                self.rotate_left(inverse_ind[ind_x]);
            }
            Operation::Reverse(x, y) => self.reverse(x, y),
            Operation::Move(x, y) => self.move_position(y, x),
        }
    }
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-21.txt")?
        .lines()
        .map(String::from)
        .collect();

    // ------------------------------------------

    let operations: Vec<Operation> = input.into_iter().map(|s| Operation::from(&s[..])).collect();

    let mut password = Password::new("abcdefgh");

    for op in operations.iter() {
        password.execute(op);
    }

    let part_1 = password.to_string();

    dbg!(part_1);

    // ------------------------------------------

    let mut scrambled = Password::new("fbgdceah");

    for op in operations.iter().rev() {
        scrambled.execute_inverse(op);
    }

    let part_2 = scrambled.to_string();

    dbg!(part_2);

    Ok(())
}
