use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    dbg!(find_marker(input, 4));
    dbg!(find_marker(input, 14));
}

fn find_marker(input: &str, seq_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(seq_size)
        .position(|pos| pos.iter().collect::<HashSet<_>>().len() == seq_size)
        .map(|pos| pos + seq_size)
}

#[cfg(test)]
mod tests {
    use crate::find_marker;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_marker_4(index: usize, input: &str) {
        assert_eq!(Some(index), find_marker(input, 4));;
    }

    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_marker_14(index: usize, input: &str) {
        assert_eq!(Some(index), find_marker(input, 14));;
    }
}
