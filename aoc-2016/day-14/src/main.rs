use anyhow::Result;
use fancy_regex::Regex; // regex crate does not support backreferences
use std::collections::{HashSet, VecDeque};

const KEY_COUNT: usize = 64; // number of keys to find

/// Store the data of a three-of-a-kind pattern
struct TOAK {
    index: usize,
    character: char,
}

/// Find each character part of a five-of-a-kind pattern in the hash
fn find_foak(hash: &str, pat: &Regex) -> HashSet<char> {
    pat.find_iter(&hash)
        .map(|m| m.unwrap().as_str().chars().next().unwrap())
        .collect()
}

/// Hash function for part 1 (MD5 hash)
fn hash_md5(input: &str, index: usize) -> String {
    format!("{:?}", md5::compute(format!("{input}{index}")))
}

/// Hash function for part 2 (stretched MD5 hash)
fn hash_md5_stretched(input: &str, index: usize) -> String {
    let mut hash = hash_md5(input, index);

    for _ in 0..2016 {
        hash = format!("{:?}", md5::compute(hash));
    }

    hash
}

fn find_key_indices(salt: &str, hash: &dyn Fn(&str, usize) -> String) -> Vec<usize> {
    let re3 = Regex::new(r"(\h)\1{2}").unwrap(); // three-of-a-kind pattern
    let re5 = Regex::new(r"(\h)\1{4}").unwrap(); // five-of-a-kind pattern

    // ------------------------------------------

    // store the indices producing valid keys (at least KEY_COUNT, generally more)
    // use a set to avoid storing the same index twice, if it matches two FOAKs
    let mut key_indices: HashSet<usize> = HashSet::with_capacity(KEY_COUNT);

    // NOTE We could also make a buffer of fixed size 1000 (containing Options)
    // but we would then have to loop through the entire 1000-elements buffer
    // to find a match. Since there should only be ~125 TOAK in 1000 hashes,
    // storing only those reduces the number of iterations when looking for a match
    // at the cost of a few checks (index of oldest buffer element before popping, ...).

    // buffer with the three-of-a-kind patterns from the last 1000 hashes
    let mut toak_buffer: VecDeque<TOAK> = VecDeque::with_capacity(1000);

    let mut index = 0;

    while key_indices.len() < KEY_COUNT {
        let hash = hash(&salt, index);

        // remove oldest buffer entry if the index gap is larger than 1000
        if !toak_buffer.is_empty() && toak_buffer[0].index + 1000 < index {
            toak_buffer.pop_front();
        }

        // find each character part of a five-of-a-kind pattern in the hash
        let foak = find_foak(&hash, &re5);

        // look for hashes in the TOAK buffer matching one of the FOAK
        for toak in &toak_buffer {
            if foak.contains(&toak.character) {
                key_indices.insert(toak.index);
            }
        }

        // find and store the first triple in the hash (if any)
        if let Some(m) = re3.find(&hash).unwrap() {
            let character = m.as_str().chars().next().unwrap();
            toak_buffer.push_back(TOAK { index, character });
        }

        index += 1;
    }

    // After finding 64 keys, we need to continue until the buffer is clear,
    // otherwise we might not have found the first 64 keys.
    // E.g. if there are keys at indices 1 and 2, matched by FOAK patterns at
    // indices 1000 and 3 respectively, we find key 2 before key 1.
    // Perform the same loop as above but without adding any new TOAK to the buffer.
    while !toak_buffer.is_empty() {
        let hash = hash(&salt, index);

        if toak_buffer[0].index + 1000 < index {
            toak_buffer.pop_front();
        }

        let foak = find_foak(&hash, &re5);

        for toak in &toak_buffer {
            if foak.contains(&toak.character) {
                key_indices.insert(toak.index);
            }
        }

        index += 1;
    }

    // convert to a Vec and sort in ascending order
    let mut key_indices: Vec<usize> = key_indices.into_iter().collect();
    key_indices.sort();

    key_indices
}

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("input/day-14.txt")?
        .trim()
        .to_string();

    // ------------------------------------------

    let key_indices = find_key_indices(&input, &hash_md5);
    let part_1 = key_indices[KEY_COUNT - 1];
    dbg!(part_1);

    // ------------------------------------------

    let key_indices = find_key_indices(&input, &hash_md5_stretched);
    let part_2 = key_indices[KEY_COUNT - 1];
    dbg!(part_2);

    Ok(())
}
