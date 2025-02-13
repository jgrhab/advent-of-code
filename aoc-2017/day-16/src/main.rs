enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl From<&str> for Move {
    fn from(value: &str) -> Move {
        let mut iter = value.chars();

        let move_type = iter.next().unwrap();
        let remainder: String = iter.collect();

        let mut iter = remainder.split('/');

        match move_type {
            's' => Move::Spin(remainder.parse().unwrap()),
            'x' => {
                let first = iter.next().unwrap().parse().unwrap();
                let second = iter.next().unwrap().parse().unwrap();

                Move::Exchange(first, second)
            }
            'p' => {
                let first = iter.next().unwrap().chars().next().unwrap();
                let second = iter.next().unwrap().chars().next().unwrap();

                Move::Partner(first, second)
            }
            _ => panic!("invalid move: {}", value),
        }
    }
}

struct Dance {
    values: [char; 16],
}

impl Dance {
    fn new() -> Self {
        let programs = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        ];

        Self { values: programs }
    }

    /// Applies a single move.
    fn apply_move(&mut self, mv: &Move) {
        match mv {
            Move::Spin(x) => self.spin(*x),
            Move::Exchange(x, y) => self.exchange(*x, *y),
            Move::Partner(x, y) => self.partner(*x, *y),
        }
    }

    /// Applies a sequence of moves.
    fn apply_moves(&mut self, moves: &[Move]) {
        for mv in moves {
            self.apply_move(mv);
        }
    }

    /// Finds the period of a sequence of moves.
    ///
    /// We define the period as the smallest positive number of times
    /// that the sequence needs to be repeated before returning to the initial state.
    fn period(moves: &[Move]) -> usize {
        let mut dance = Self::new();
        let initial_values = dance.values;

        dance.apply_moves(moves);

        let mut count = 1;

        while dance.values != initial_values {
            dance.apply_moves(moves);
            count += 1;
        }

        count
    }

    /// Applies a Spin move.
    fn spin(&mut self, val: usize) {
        let split_idx = self.values.len() - val; // first index of tail

        let (head, tail) = self.values.split_at(split_idx);

        let head = head.to_owned();
        let tail = tail.to_owned();

        self.values[..tail.len()].copy_from_slice(&tail);
        self.values[tail.len()..].copy_from_slice(&head);
    }

    /// Applies an Exchange move.
    fn exchange(&mut self, first: usize, second: usize) {
        (self.values[first], self.values[second]) = (self.values[second], self.values[first]);
    }

    /// Applies a Partner move.
    fn partner(&mut self, first: char, second: char) {
        let first = self
            .values
            .iter()
            .enumerate()
            .find(|&(_, &elt)| elt == first)
            .unwrap();

        let second = self
            .values
            .iter()
            .enumerate()
            .find(|&(_, &elt)| elt == second)
            .unwrap();

        self.exchange(first.0, second.0);
    }
}

// implement Display to get the to_string() method
impl std::fmt::Display for Dance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.values.iter().collect();
        write!(f, "{}", s)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let moves: Vec<Move> = input.trim().split(',').map(Move::from).collect();

    // --- Part One --- //

    let mut dance = Dance::new();

    dance.apply_moves(&moves);

    let part_one = dance.to_string();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    // find the number of iterations after which the
    // dance returns to the initial state
    let period = Dance::period(&moves);

    let mut dance = Dance::new();

    for _ in 0..(1_000_000_000 % period) {
        dance.apply_moves(&moves);
    }

    let part_two = dance.to_string();

    println!("Part Two: {}", part_two);
}
