use anyhow::Result;
use std::collections::HashMap;

// indices for accessing low and high values
const LO: usize = 0;
const HI: usize = 1;

enum Recipient {
    Bot(u32),
    Output(u32),
}

impl From<(&str, &str)> for Recipient {
    fn from(data: (&str, &str)) -> Self {
        let id = data.1.parse::<u32>().unwrap();

        match data.0 {
            "bot" => Self::Bot(id),
            "output" => Self::Output(id),
            _ => panic!("invalid data: {}", data.0),
        }
    }
}

fn parse_send_commands(cmds: &Vec<String>) -> HashMap<u32, [Recipient; 2]> {
    let mut send: HashMap<u32, [Recipient; 2]> = HashMap::with_capacity(cmds.len());

    for cmd in cmds {
        let split: Vec<&str> = cmd.split_whitespace().collect();

        let id = split[1].parse::<u32>().unwrap();
        let lo = Recipient::from((split[5], split[6]));
        let hi = Recipient::from((split[10], split[11]));

        send.insert(id, [lo, hi]); // insert in the [LO, HI] order
    }

    send
}

fn parse_init_commands(cmds: &Vec<String>) -> HashMap<u32, Vec<u32>> {
    let mut bots: HashMap<u32, Vec<u32>> = HashMap::with_capacity(cmds.len());

    for string in cmds {
        let split: Vec<&str> = string.split_whitespace().collect();
        let value = split[1].parse::<u32>().unwrap();
        let bot_id = split[5].parse::<u32>().unwrap();

        let entry = bots.entry(bot_id).or_insert(Vec::with_capacity(2));
        entry.push(value);
    }

    bots
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-10.txt")?
        .lines()
        .map(String::from)
        .collect();

    // split the 'init' and the 'send' commands
    let (init, send): (Vec<_>, Vec<_>) = input.into_iter().partition(|cmd| &cmd[..3] != "bot");

    let send_cmds: HashMap<u32, [Recipient; 2]> = parse_send_commands(&send);
    let mut bots: HashMap<u32, Vec<u32>> = parse_init_commands(&init);
    let mut outputs: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut comparisons: HashMap<u32, [u32; 2]> = HashMap::new();

    // build a stack containing the ids of the bots holding two chips
    let mut stack: Vec<u32> = bots
        .iter()
        .filter(|(_, chips)| chips.len() == 2)
        .map(|(id, _)| *id)
        .collect();

    // pass the chips from bots on the stack and add recipients to the stack if needed
    while let Some(sender_id) = stack.pop() {
        // extract the chips from the sender bot
        let mut chips: Vec<u32> = bots.get_mut(&sender_id).unwrap().drain(..).collect();
        chips.sort(); // sort in [lo, hi] order

        // send chips to recipients and add bot recipients holding two chips to the stack
        for pos in [LO, HI] {
            let chip = chips[pos];

            match send_cmds[&sender_id][pos] {
                Recipient::Bot(id) => {
                    let bot = bots.entry(id).or_insert(Vec::with_capacity(2));
                    bot.push(chip);

                    if bot.len() == 2 {
                        stack.push(id); // add bot holding two chips to the stack
                    }
                }
                Recipient::Output(id) => {
                    let output = outputs.entry(id).or_insert(Vec::new());
                    output.push(chip);
                }
            }

            // track the comparison done by the sender bot
            // NOTE this assumes that every bot does a single comparison, which seems true
            comparisons.insert(sender_id, [chips[LO], chips[HI]]);
        }
    }

    let part_1 = comparisons
        .iter()
        .filter(|(_, chips)| **chips == [17, 61])
        .next()
        .unwrap()
        .0;

    dbg!(part_1);

    let part_2 = outputs[&0][0] * outputs[&1][0] * outputs[&2][0];

    dbg!(part_2);

    Ok(())
}
