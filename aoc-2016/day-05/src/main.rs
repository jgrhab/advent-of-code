use anyhow::Result;

// The digest is a struct Digest([u8; 16])

/// Check whether a sting is admissible.
/// A string is admissible if the corresponding MD5 hash starts with five zeros.
/// If `string` is admissible, return the sixth and seventh characters of the hash.
fn is_admissible(string: &str) -> Option<(u8, u8)> {
    let d = md5::compute(string); // compute the digest

    // extract the first 3 bytes (6 digits) of the digest
    let head: u32 = ((d[0] as u32) << 16) + ((d[1] as u32) << 8) + (d[2] as u32);

    // return None if any of the first 5 digits are nonzero
    if (head & 0xff_ff_f0) != 0 {
        return None;
    }

    let ch_6 = d[2] & 0x0f;
    let ch_7 = (d[3] & 0xf0) >> 4;

    Some((ch_6, ch_7))
}

/// Find the password following the pattern of part 1.
fn find_password_1(door_id: &str) -> String {
    let mut password = String::with_capacity(8);
    let mut num: u32 = 0;

    for _ in 0..8 {
        let mut opt: Option<(u8, u8)> = None;

        while opt == None {
            opt = is_admissible(&format!("{door_id}{num}"));
            num += 1;
        }

        let (ch, _) = opt.unwrap(); // know that opt is Ok
        let ch = char::from_digit(ch as u32, 16).expect("failed to convert {ch} to char");

        password.push(ch);
    }

    password
}

/// Find the password following the pattern of part 2.
fn find_password_2(door_id: &str) -> String {
    let mut filled: u8 = 0; // use 8 bits to mark the filled positions
    let mut pwd: [u8; 8] = [99; 8]; // initialize to invalid value 99

    let mut num: u32 = 0;

    while filled != 0xff {
        let mut opt: Option<(u8, u8)> = None;

        while opt == None {
            opt = is_admissible(&format!("{door_id}{num}"));
            num += 1;
        }

        let (pos, ch) = opt.unwrap();

        // check that the position is valid and not already filled
        if (pos > 7) || ((1 << pos) & filled != 0) {
            continue;
        }

        pwd[pos as usize] = ch; // store the character at the correct position
        filled += 1 << pos; // mark the position as filled
    }

    // convert the password to a String and return
    pwd.into_iter()
        .map(|num| char::from_digit(num as u32, 16).expect("failed to convert {num} to char"))
        .fold(String::new(), |mut string, ch| {
            string.push(ch);
            string
        })
}

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("input/day-05.txt")?;
    let door_id: String = input.trim().to_string();

    let part_1 = find_password_1(&door_id);
    dbg!(part_1);

    let part_2 = find_password_2(&door_id);
    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixth() {
        assert_eq!(None, is_admissible("abc0"));
        assert_eq!(Some((1, 5)), is_admissible("abc3231929"));
        assert_eq!(Some((8, 0xf)), is_admissible("abc5017308"));
    }

    #[test]
    fn password_1() {
        assert_eq!("18f47a30", find_password_1("abc"));
    }

    #[test]
    fn password_2() {
        assert_eq!("05ace8e3", find_password_2("abc"));
    }
}
