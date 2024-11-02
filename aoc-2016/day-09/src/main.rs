use anyhow::Result;

struct MarkerSplit<'a> {
    head: &'a str, // part before first marker
    tail: &'a str, // part after first marker
    length: usize, // 1st number of marker
    repeat: usize, // 2nd number of marker
}

/// Split the input at the first marker and return the result as a `MarkerSplit`.
///
/// This function panics if an opening parenthese has no matching closing parenthese,
/// or if parsing the integers in the marker fail.
/// Neither of these should happen with the problem input.
fn split_at_first_marker(input: &str) -> Option<MarkerSplit> {
    // find start index of the first marker
    let idx_l = match input.find('(') {
        Some(idx) => idx,
        None => return None,
    };

    // find end index of the first marker - panic if none found
    let idx_r = idx_l + input[idx_l..].find(')').unwrap();

    // split the input into head(mark)tail
    let head = &input[..idx_l];
    let mark = &input[(idx_l + 1)..idx_r];
    let tail = &input[(idx_r + 1)..];

    // parse the marker - panics if parsing fails
    let mut iter = mark.split('x');
    let length = iter.next().unwrap().parse::<usize>().unwrap();
    let repeat = iter.last().unwrap().parse::<usize>().unwrap();

    Some(MarkerSplit {
        head,
        tail,
        length,
        repeat,
    })
}

fn decompress_v1(input: &str) -> String {
    // split the input at the first marker
    let ms: MarkerSplit = match split_at_first_marker(input) {
        Some(split) => split,
        None => return String::from(input),
    };

    // extract the part to be copied from the tail
    let copy = &ms.tail[..ms.length];
    let tail = &ms.tail[ms.length..];

    // build the decompressed part for the current marker
    let mut result = String::from(ms.head);
    for _ in 0..ms.repeat {
        result.push_str(copy);
    }

    // recursively decompress the tail and append to the result
    let result_tail = decompress_v1(tail);
    result.push_str(&result_tail);

    result
}

/// Decompress at the first marker of the input.
/// Return the length of the input head and the input tail (as `Option`).
/// When the input contains a marker, the tail returned is decompressed accordingly.
fn decompress_at_first_marker(input: &str) -> (usize, Option<String>) {
    // split the input at the first marker
    let ms: MarkerSplit = match split_at_first_marker(input) {
        Some(split) => split,
        None => return (input.len(), None),
    };

    // extract the part to be copied from the tail
    let copy = &ms.tail[..ms.length];
    let tail = &ms.tail[ms.length..];

    // build the decompressed part following the marker
    let mut result = String::with_capacity(copy.len() * ms.repeat + tail.len());
    for _ in 0..ms.repeat {
        result.push_str(copy);
    }
    result.push_str(tail);

    (ms.head.len(), Some(result))
}

/// Compute the length of the input decompressed using v2 of the algorithm.
fn decompress_v2_len(input: &str) -> usize {
    let mut input = String::from(input);
    let mut total_length = 0;

    // for every marker, decompress the tail and increase the total length
    while let (head_length, Some(tail)) = decompress_at_first_marker(&input) {
        input = tail;
        total_length += head_length;

        dbg!(total_length);
    }

    // add length of the tail once there are no markers left
    total_length += input.len();

    total_length
}

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("input/day-09.txt")?
        .trim_end()
        .to_string();

    let part_1 = decompress_v1(&input).len();
    dbg!(part_1);

    // NOTE this takes a while as the result is > 10 billions
    let part_2 = decompress_v2_len(&input);
    dbg!(part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_v1_1() {
        let res = decompress_v1("ADVENT");
        let exp = String::from("ADVENT");

        assert_eq!(res, exp);
    }

    #[test]
    fn decompress_v1_2() {
        let res = decompress_v1("A(1x5)BC");
        let exp = String::from("ABBBBBC");

        assert_eq!(res, exp);
    }

    #[test]
    fn decompress_v1_3() {
        let res = decompress_v1("(3x3)XYZ");
        let exp = String::from("XYZXYZXYZ");

        assert_eq!(res, exp);
    }

    #[test]
    fn decompress_v1_4() {
        let res = decompress_v1("A(2x2)BCD(2x2)EFG");
        let exp = String::from("ABCBCDEFEFG");

        assert_eq!(res, exp);
    }

    #[test]
    fn decompress_v1_5() {
        let res = decompress_v1("(6x1)(1x3)A");
        let exp = String::from("(1x3)A");

        assert_eq!(res, exp);
    }

    #[test]
    fn decompress_v1_6() {
        let res = decompress_v1("X(8x2)(3x3)ABCY");
        let exp = String::from("X(3x3)ABC(3x3)ABCY");

        assert_eq!(res, exp);
    }

    #[test]
    fn decompress_v2_1() {
        let res = decompress_v2_len("(3x3)XYZ");
        let exp = String::from("XYZXYZXYZ").len();

        assert_eq!(res, exp);
    }

    #[test]
    fn decompress_v2_2() {
        let res = decompress_v2_len("X(8x2)(3x3)ABCY");
        let exp = String::from("XABCABCABCABCABCABCY").len();

        assert_eq!(res, exp);
    }

    #[test]
    fn decompress_v2_3() {
        let res = decompress_v2_len("(27x12)(20x12)(13x14)(7x10)(1x12)A");

        assert_eq!(res, 241920);
    }

    #[test]
    fn decompress_v2_4() {
        let res = decompress_v2_len("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");

        assert_eq!(res, 445);
    }
}
