use std::collections::{HashMap, HashSet};

/// Checks whether a phrase contains a duplicate word.
fn contains_duplicate_word(phrase: &str) -> bool {
    let mut set = HashSet::new();

    for word in phrase.split_whitespace() {
        if set.contains(word) {
            return true;
        }

        set.insert(word);
    }

    false
}

/// Checks whethers a phrase contains two words which are anagrams of each other.
fn contains_anagrams(phrase: &str) -> bool {
    let mut maps: Vec<HashMap<char, u8>> = Vec::new();

    // count the occurences of each character in each word of the phrase
    for word in phrase.split_whitespace() {
        let mut map = HashMap::new();

        for ch in word.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }
        maps.push(map);
    }

    // compare the character counts for each pair of words
    for ind1 in 0..maps.len() {
        for ind2 in (ind1 + 1)..maps.len() {
            if maps[ind1] == maps[ind2] {
                return true;
            }
        }
    }

    false
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let phrases: Vec<&str> = input.lines().collect();

    // --- Part One --- //

    let part_one = phrases
        .iter()
        .filter(|phrase| !contains_duplicate_word(phrase))
        .count();

    println!("Part One: {}", part_one);

    // --- Part Two --- //

    let part_two = phrases
        .iter()
        .filter(|phrase| !contains_anagrams(phrase))
        .count();

    println!("Part Two: {}", part_two);
}
