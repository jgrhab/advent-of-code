use anyhow::Result;

#[derive(PartialEq, Clone)]
enum Tile {
    Trap,
    Safe,
}

impl Tile {
    /// Determine the type of a tile from the [Left, Center, Right] tiles above it.
    /// A tile is a trap if exactly one of [Left, Right] is a trap.
    fn derive(above: &[Tile]) -> Self {
        if above[0] != above[2] {
            Tile::Trap
        } else {
            Tile::Safe
        }
    }
}

/// Store a row as a Vec of Tiles, padded at both ends by a safe tile.
#[derive(Clone)]
struct Row {
    tiles: Vec<Tile>,
}

impl From<&str> for Row {
    fn from(string: &str) -> Self {
        let string = format!(".{string}."); // add padding

        let tiles: Vec<Tile> = string
            .chars()
            .map(|ch| if ch == '^' { Tile::Trap } else { Tile::Safe })
            .collect();

        Self { tiles }
    }
}

// Only used for testing
impl ToString for Row {
    fn to_string(&self) -> String {
        self.tiles[1..(self.tiles.len() - 1)] // remove padding
            .iter()
            .map(|tile| if *tile == Tile::Trap { '^' } else { '.' })
            .collect()
    }
}

impl Row {
    /// Generate the next row according to the rules.
    fn next(&self) -> Self {
        let mut tiles: Vec<Tile> = Vec::with_capacity(self.tiles.len());
        tiles.push(Tile::Safe); // left padding

        for ind in 1..(self.tiles.len() - 1) {
            let above = &self.tiles[(ind - 1)..=(ind + 1)];
            tiles.push(Tile::derive(above));
        }

        tiles.push(Tile::Safe); // right padding

        Row { tiles }
    }

    /// Count the number of save tiles in a row.
    fn safe_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| **tile == Tile::Safe)
            .count()
            - 2 // remove padding tiles
    }
}

fn count_safe_tiles(start: &Row, row_count: usize) -> usize {
    let mut row: Row = start.clone();
    let mut count = row.safe_count();

    for _ in 1..row_count {
        row = row.next();
        count += row.safe_count();
    }

    count
}

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("input/day-18.txt")?
        .trim()
        .to_string();

    // ------------------------------------------

    let start = Row::from(&input[..]);

    let part_1 = count_safe_tiles(&start, 40);

    dbg!(part_1);

    let part_2 = count_safe_tiles(&start, 400_000);

    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_conversion() {
        let string = String::from("..^^.");

        let row = Row::from(&string[..]);
        let res = row.to_string();

        assert_eq!(string, res);
    }

    #[test]
    fn next() {
        let first = Row::from("..^^.");
        let second = String::from(".^^^^");
        let third = String::from("^^..^");

        let res = first.next();
        assert_eq!(res.to_string(), second);

        let res = res.next();
        assert_eq!(res.to_string(), third);
    }
}
