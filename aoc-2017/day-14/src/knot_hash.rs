use std::fmt::Write;

pub struct KnotHash {
    list: [u8; 256],
    position: usize,
    skip_size: usize,
}

impl KnotHash {
    fn new() -> Self {
        Self {
            list: core::array::from_fn(|x| x as u8),
            position: 0,
            skip_size: 0,
        }
    }

    /// Hashes the input using the knot hash algorithm.
    ///
    /// Returns the dense hash as a hex string.
    pub fn hash(input: &str) -> String {
        // convert the input characters to ascii codes and add the end sequence
        let lengths: Vec<_> = input.chars().map(|ch| ch as usize).collect();
        let lengths = [lengths, vec![17, 31, 73, 47, 23]].concat();

        let mut hash = KnotHash::new();

        for _ in 0..64 {
            for length in lengths.iter() {
                hash.reverse(*length);
            }
        }

        hash.dense_hash_hex()
    }

    /// Reverses a range of values and update the position and skip size.
    ///
    /// The range starts at `self.position` and has length `length`,
    /// and wraps to the begining of the list if needed.
    fn reverse(&mut self, length: usize) {
        // Split the range to reverse into head + tail, where
        // the head goes until the end of the list (if needed) and
        // the tail is the part of the range that wraps at the start of the list.
        // If the range is small enough, the tail is empty (no wrapping).

        let head_end = usize::min(self.position + length, self.list.len());
        let head_len = head_end - self.position;
        let tail_len = length - head_len; // always ok as head_len <= length

        // get the indices and the values of the range to invert
        let range_idx: Vec<_> = (self.position..head_end).chain(0..tail_len).collect();
        let range_val: Vec<_> = range_idx.iter().map(|&idx| self.list[idx]).collect();

        // replace the value at each index by the value at the same index
        // starting from the end of the range of values
        for step in 0..range_idx.len() {
            self.list[range_idx[step]] = range_val[range_val.len() - step - 1];
        }

        // update the position and skip size
        self.position = (self.position + length + self.skip_size) % self.list.len();
        self.skip_size += 1;
    }

    /// Computes the dense hash.
    ///
    /// Uses the current value of `self.list` as sparse hash.
    fn dense_hash(&self) -> [u8; 16] {
        let mut hash = [0; 16];

        for block_idx in 0..16 {
            let block = &self.list[(block_idx * 16)..((block_idx + 1) * 16)];

            let block_hash = block[1..].iter().fold(block[0], |hash, val| hash ^ val);

            hash[block_idx] = block_hash;
        }

        hash
    }

    /// Returns the dense hash as a hex string.
    ///
    /// Each of the 16 numbers in the dense hash is represented by two hex digits.
    fn dense_hash_hex(&self) -> String {
        self.dense_hash()
            .into_iter()
            .fold(String::new(), |mut output, num| {
                let _ = write!(output, "{num:02x}");
                output
            })
    }
}
