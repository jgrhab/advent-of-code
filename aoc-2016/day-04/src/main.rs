use anyhow::{anyhow, Result};
use std::collections::HashMap;

struct Room {
    name: Vec<String>,
    sector_id: u32,
    checksum: String,
}

impl Room {
    /// Read a line of input and returns the corresponding `Room`.
    /// NOTE: Could implement the `From` trait instead.
    pub fn from_line(line: &str) -> Result<Self> {
        // split the line into a body and a checksum (line = body[checksum])
        let line: Vec<&str> = line.trim_end_matches(']').split('[').collect();

        let checksum = String::from(line[1]);

        let body: Vec<&str> = line[0].split('-').collect();

        let sector_id = body[body.len() - 1]; // the sector ID is the last element of the body
        let sector_id: u32 = match sector_id.parse() {
            Ok(num) => num,
            Err(_) => return Err(anyhow!("invalid sector ID: {sector_id}")), // can also use bail!
        };

        let name: Vec<String> = body[..(body.len() - 1)]
            .iter()
            .map(|&s| String::from(s))
            .collect();

        Ok(Self {
            name,
            sector_id,
            checksum,
        })
    }

    pub fn is_real(&self) -> bool {
        // fold the name Vector into a String
        let name: String = self.name.iter().fold(String::new(), |mut name, s| {
            name.push_str(s);
            name
        });

        // populate a HashMap with the character count for the name (could also use a for-loop)
        let char_count: HashMap<char, i32> = name.chars().fold(HashMap::new(), |mut map, ch| {
            *map.entry(ch).or_insert(0) += 1;
            map
        });

        // map the tuples: (char, count) -> (-count, char)
        // sorting the resulting vector (in ascending lexicographic order) puts the most
        // frequent characters first and ties are ordered alphabetically (asc. order for char)
        let mut char_count: Vec<_> = char_count
            .into_iter()
            .map(|(ch, count)| (-count, ch))
            .collect();

        char_count.sort(); // sorting gives the correct ordering

        // collect the 5 most common characters into a String
        let top_5: String = char_count.into_iter().take(5).map(|(_, ch)| ch).collect();

        top_5 == self.checksum
    }

    /// Decrypts a single word of the room name (a vector of words).
    fn decrypt_word(&self, word: &str) -> String {
        let shift: u8 = (self.sector_id % 26) as u8;

        word.chars()
            .map(|ch| (((ch as u8) - ('a' as u8) + shift) % 26 + ('a' as u8)) as char)
            .collect::<String>()
    }

    /// Decrypts the room name
    pub fn decrypt(&self) -> String {
        let name: Vec<String> = self
            .name
            .iter()
            .map(|word| self.decrypt_word(word))
            .collect();

        name.join(" ")
    }
}

/// Filter the input to retain only the real rooms.
fn filter_real_rooms(input: &Vec<String>) -> Result<Vec<Room>> {
    let mut rooms: Vec<Room> = Vec::with_capacity(input.len());

    for line in input {
        let room = Room::from_line(&line)?;
        rooms.push(room);
    }

    Ok(rooms.into_iter().filter(|room| room.is_real()).collect())
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-04.txt")?
        .lines()
        .map(String::from)
        .collect();

    let rooms = filter_real_rooms(&input)?;
    let part_1: u32 = rooms.iter().map(|room| room.sector_id).sum();

    dbg!(part_1);

    let target: Room = rooms
        .into_iter()
        .filter(|room| room.decrypt().contains("northpole"))
        .next()
        .ok_or(anyhow!("no target found"))?;

    let part_2 = target.sector_id;

    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 4] = [
        "aaaaa-bbb-z-y-x-123[abxyz]",
        "a-b-c-d-e-f-g-h-987[abcde]",
        "not-a-real-room-404[oarel]",
        "totally-real-room-200[decoy]",
    ];

    #[test]
    fn part_1() {
        let input: Vec<String> = INPUT.into_iter().map(String::from).collect();
        let rooms = filter_real_rooms(&input);

        assert!(rooms.is_ok()); // check that the rooms were read successfully

        let rooms = rooms.unwrap();
        let result: u32 = rooms.iter().map(|room| room.sector_id).sum();

        assert_eq!(1514, result);
    }

    #[test]
    fn decrypt() {
        let line = "qzmt-zixmtkozy-ivhz-343[abcd]"; // added a checksum
        let room = Room::from_line(line).unwrap();

        assert_eq!("very encrypted name", room.decrypt());
    }
}
