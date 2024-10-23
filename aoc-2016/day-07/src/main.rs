use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug)]
struct IPAddress {
    supernet: Vec<String>, // outside brackets
    hypernet: Vec<String>, // inside brackets
}

impl From<&str> for IPAddress {
    fn from(string: &str) -> Self {
        let split: Vec<String> = string.split(&['[', ']']).map(String::from).collect();

        // supernet = outside of brackets = even indices
        let supernet: Vec<String> = split.clone().into_iter().step_by(2).collect();
        let hypernet: Vec<String> = split.into_iter().skip(1).step_by(2).collect();

        Self { supernet, hypernet }
    }
}

impl From<String> for IPAddress {
    fn from(string: String) -> Self {
        Self::from(&string[..])
    }
}

impl IPAddress {
    /// Check whether the address supports TLS.
    /// The address supports TLS if it has a supernet ABBA and no hypernet ABBA.
    fn supports_tls(&self) -> bool {
        let hypernet_abba_count = self.hypernet.iter().filter(|s| has_abba(s)).count();

        if hypernet_abba_count > 0 {
            return false;
        }

        let supernet_abba_count = self.supernet.iter().filter(|s| has_abba(s)).count();

        supernet_abba_count > 0
    }

    fn supports_ssl(&self) -> bool {
        // find all ABA patterns in supernet and hypernet
        let supernet_aba: HashSet<&str> = find_aba_vec(&self.supernet);
        let hypernet_aba: HashSet<&str> = find_aba_vec(&self.hypernet);

        // reverse the ABA patterns to find the BAB patterns in hypernet
        let hypernet_bab: HashSet<String> = hypernet_aba
            .into_iter()
            .map(|aba| aba.chars().rev().collect())
            .collect();

        // convert the HashSet<&str> to HashSet<String> to match the type of hypernet_bab
        let supernet_aba: HashSet<String> = supernet_aba.into_iter().map(String::from).collect();

        // count the number of supernet ABA with a matching hypernet BAB
        let intersection = supernet_aba.intersection(&hypernet_bab).count();

        intersection > 0
    }
}

/// Check whether a string contains the ABBA pattern.
fn has_abba(string: &str) -> bool {
    // slide a window of 4 characters across the string
    for idx in 0..=(string.len() - 4) {
        let arr: &[u8] = string[idx..(idx + 4)].as_bytes();

        if (arr[0] == arr[3]) & (arr[1] == arr[2]) & (arr[0] != arr[1]) {
            return true;
        }
    }

    false
}

// The lifetime of the string slices in the returned set is the lifetime of the input.
// Since we call this function on string slices owned by the `IPAddress` struct only,
// the resulting `HashSet` remains valid as long as the struct is valid.

/// Find all instances of the pattern ABA in a string.
/// Return a HashSet containing the first two letters (AB) of each match.
fn find_aba(string: &str) -> HashSet<&str> {
    let mut aba: HashSet<&str> = HashSet::new();

    // slide a 3-characters window across the string
    for idx in 0..=(string.len() - 3) {
        let slice: &str = &string[idx..(idx + 3)];
        let arr: &[u8] = slice.as_bytes();

        if (arr[0] == arr[2]) & (arr[0] != arr[1]) {
            aba.insert(&slice[..2]);
        }
    }

    aba
}

/// Find all instances of ABA in a set of strings.
fn find_aba_vec(vec: &Vec<String>) -> HashSet<&str> {
    vec.iter()
        .map(|s| find_aba(s))
        .fold(HashSet::new(), |mut set, s| {
            set = set.union(&s).map(|aba| *aba).collect();
            set
        })
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input/day-07.txt")?
        .lines()
        .map(String::from)
        .collect();

    let addrs: Vec<IPAddress> = input.into_iter().map(IPAddress::from).collect();

    let part_1 = addrs.iter().filter(|&addr| addr.supports_tls()).count();

    dbg!(part_1);

    let part_2 = addrs.iter().filter(|&addr| addr.supports_ssl()).count();

    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tls() {
        let addr_1 = IPAddress::from("abba[mnop]qrst");
        let addr_2 = IPAddress::from("abcd[bddb]xyyx");
        let addr_3 = IPAddress::from("aaaa[qwer]tyui");
        let addr_4 = IPAddress::from("ioxxoj[asdfgh]zxcvbn");

        assert!(addr_1.supports_tls());
        assert!(!addr_2.supports_tls());
        assert!(!addr_3.supports_tls());
        assert!(addr_4.supports_tls());
    }

    #[test]
    fn ssl() {
        let addr_1 = IPAddress::from("aba[bab]xyz");
        let addr_2 = IPAddress::from("xyx[xyx]xyx");
        let addr_3 = IPAddress::from("aaa[kek]eke");
        let addr_4 = IPAddress::from("zazbz[bzb]cdb");

        assert!(addr_1.supports_ssl());
        assert!(!addr_2.supports_ssl());
        assert!(addr_3.supports_ssl());
        assert!(addr_4.supports_ssl());
    }
}
