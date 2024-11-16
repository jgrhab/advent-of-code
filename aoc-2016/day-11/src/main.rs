use anyhow::Result;
use std::collections::{BTreeSet, HashSet, VecDeque};

use crate::Kind::*;

// If we have a solution for a given input, adding a pair adds the following
// number of steps:
// - a pair on floor 1 adds 12 steps
// - a pair on floor 2 adds 8 steps
// - a pair on floor 3 adds 4 steps
// This is easy to see starting from the solution (all items on floor 4)
// and counting the number of steps it takes to go get the extra pair.
//
// We can therefore remove all pairs from the input and solve the rest.

// store the element names in a static vector and use the index as element id
static mut ELEMENTS: Vec<String> = Vec::new();

/// Get the id of an element from the static id vector.
/// Add the element to the vector if it is not already in it.
fn element_id(element: &str) -> u8 {
    let id = unsafe {
        if let Some(id) = ELEMENTS.iter().position(|elt| elt == element) {
            id
        } else {
            ELEMENTS.push(String::from(element));
            ELEMENTS.len() - 1
        }
    };

    id as u8
}

// ----------------------------------------------

#[derive(Eq, PartialEq, Hash, Clone, PartialOrd, Ord, Debug)]
enum Kind {
    Microchip,
    Generator,
}

#[derive(Eq, PartialEq, Hash, Clone, PartialOrd, Ord, Debug)]
struct Item {
    kind: Kind,
    id: u8,
}

// ----------------------------------------------

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    items: [BTreeSet<Item>; 4], // items on each floor (0 to 3)
    floor: usize,               // position of the elevator
    steps: u32,                 // steps taken since the start
}

impl State {
    /// Read one line from the input and add the data to `self`.
    fn add_floor_data(&mut self, line: &str, floor: usize) {
        let split: Vec<&str> = line.split_whitespace().collect();

        for generator in split
            .iter()
            .enumerate()
            .filter(|(_, word)| word.trim_end_matches(&[',', '.']) == "generator")
            .map(|(ind, _)| split[ind - 1])
        {
            let item = Item {
                kind: Generator,
                id: element_id(generator),
            };
            self.items[floor].insert(item);
        }

        for microchip in split
            .iter()
            .enumerate()
            .filter(|(_, word)| word.trim_end_matches(&[',', '.']) == "microchip")
            .map(|(ind, _)| split[ind - 1].split('-').next().unwrap())
        {
            let item = Item {
                kind: Microchip,
                id: element_id(microchip),
            };
            self.items[floor].insert(item);
        }
    }

    /// Remove the pairs where both items are on the same floor and add
    /// the corresponding number of steps to the total.
    /// (floor 3 -> 4 steps, floor 2 -> 8 steps, floor 1 -> 12 steps)
    fn remove_pairs(&mut self) {
        for floor in 0..4 {
            let floor_set = &self.items[floor];

            let mut pairs: BTreeSet<u8> = BTreeSet::new();

            for chip in floor_set.iter().filter(|&it| it.kind == Microchip) {
                for gen in floor_set.iter().filter(|&it| it.kind == Generator) {
                    if chip.id == gen.id {
                        pairs.insert(chip.id);
                    }
                }
            }

            for pair in pairs {
                self.items[floor].remove(&Item {
                    kind: Microchip,
                    id: pair,
                });

                self.items[floor].remove(&Item {
                    kind: Generator,
                    id: pair,
                });

                self.steps += 4 * (3 - floor as u32);
            }
        }
    }
}

impl From<&str> for State {
    fn from(string: &str) -> Self {
        let mut state = Self {
            items: std::array::from_fn(|_| BTreeSet::new()),
            floor: 0,
            steps: 0,
        };

        for (floor, line) in string.lines().enumerate() {
            state.add_floor_data(line, floor);
        }

        state
    }
}

// ----------------------------------------------

impl State {
    /// Check whether the items on a given floor are part of a valid state.
    /// A floor is valid unless it contains a chip without the corresponding
    /// generator, and a different generator.
    /// A state is valid if all its floors are valid.
    /// Checking on a per-floor basis allows us to check only the initial and
    /// and destination floors when making a move, instead of all four floors.
    fn is_valid(&self, floor: usize) -> bool {
        let (chips, gens): (HashSet<&Item>, HashSet<&Item>) = self.items[floor]
            .iter()
            .partition(|&it| it.kind == Microchip);

        if gens.is_empty() {
            return true;
        }

        let chip_ids: HashSet<u8> = chips.iter().map(|&it| it.id).collect();
        let gen_ids: HashSet<u8> = gens.iter().map(|&it| it.id).collect();

        chip_ids.difference(&gen_ids).next().is_none()
    }

    /// Move an item from `init_floor` to `dest_floor`.
    /// Panic if the item is not present in `init_floor`.
    fn move_item(&mut self, item: &Item, init_floor: usize, dest_floor: usize) {
        let item: Item = self.items[init_floor].take(item).expect("item not present");
        self.items[dest_floor].insert(item);
    }

    /// Compute all valid states resulting from moving one item.
    fn move_one_item(&self, init_floor: usize, dest_floor: usize) -> Vec<State> {
        let mut next_states = Vec::new();

        for item in self.items[init_floor].iter() {
            let mut next: State = self.clone();

            next.move_item(item, init_floor, dest_floor);

            if next.is_valid(init_floor) & next.is_valid(dest_floor) {
                next.floor = dest_floor;
                next.steps += 1;

                next_states.push(next);
            }
        }

        next_states
    }

    /// Compute all valid states resulting from moving two items.
    fn move_two_items(&self, init_floor: usize, dest_floor: usize) -> Vec<State> {
        let mut next_states = Vec::new();

        for item1 in self.items[init_floor].iter() {
            for item2 in self.items[init_floor].iter().filter(|&it| it != item1) {
                let mut next: State = self.clone();

                next.move_item(item1, init_floor, dest_floor);
                next.move_item(item2, init_floor, dest_floor);

                if next.is_valid(init_floor) & next.is_valid(dest_floor) {
                    next.floor = dest_floor;
                    next.steps += 1;

                    next_states.push(next);
                }
            }
        }

        next_states
    }

    /// Compute all valid states resulting from moving up with the elevator.
    /// When possible, move two items.
    fn moves_up(&self) -> Vec<State> {
        if self.floor == 3 {
            return Vec::new();
        }

        let mut next_states = self.move_two_items(self.floor, self.floor + 1);
        if next_states.is_empty() {
            next_states.extend(self.move_one_item(self.floor, self.floor + 1));
        }

        next_states
    }

    /// Compute all valid states resulting from moving down with the elevator.
    /// When possible, move only one item.
    fn moves_down(&self) -> Vec<State> {
        if self.floor == 0 {
            return Vec::new();
        }

        let mut next_states = self.move_one_item(self.floor, self.floor - 1);
        if next_states.is_empty() {
            next_states.extend(self.move_two_items(self.floor, self.floor - 1));
        }

        next_states
    }

    /// Compute the next valid states for the current state.
    fn next_states(&self) -> Vec<State> {
        let mut next = self.moves_up();
        next.extend(self.moves_down());

        next
    }
}

/// Breadth-first search starting from state `state`.
/// Return the minimal number of steps.
fn bfs(state: State) -> Option<u32> {
    // total number of items (microchips + generators)
    let total_items: usize = state.items.iter().map(|set| set.len()).sum();

    let mut queue: VecDeque<State> = VecDeque::from([state]);
    let mut visited: HashSet<State> = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if state.items[3].len() == total_items {
            return Some(state.steps);
        }

        if visited
            .iter()
            .filter(|&other| other.items == state.items)
            .filter(|&other| other.floor == state.floor)
            .filter(|&other| other.steps <= state.steps)
            .next()
            .is_some()
        {
            continue;
        }

        queue.extend(state.next_states());

        visited.insert(state);
    }

    None
}

// ----------------------------------------------

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("input/day-11.txt")?;

    // read the state from the puzzle input
    let mut state = State::from(&input[..]);

    // remove the pairs and add the corresponding number of steps
    state.remove_pairs();

    let part_1 = bfs(state).unwrap();

    dbg!(part_1);

    let part_2 = part_1 + 24;

    dbg!(part_2);

    Ok(())
}
