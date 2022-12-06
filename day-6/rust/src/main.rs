use std::{
    include_str,
    collections::HashSet
};

fn main() {
    static DATA: &str = include_str!("data.txt");

    // Part One
    println!("For part one, the signal starts at index {}", find_signal_index(&DATA, 4));

    // Part two
    println!("For part two, the signal starts at index {}", find_signal_index(&DATA, 14));
}

fn find_signal_index(data: &str, window_size: usize) -> usize {
    data.as_bytes()
        .windows(window_size)
        .position(|s| HashSet::<&u8>::from_iter(s.into_iter()).len() == window_size)
        .unwrap_or(0) + window_size // must add window_size at the end because iterating over windows is
}

// Adding problem examples as unit tests
#[test]
fn test_part_one() {
    assert_eq!(find_signal_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    assert_eq!(find_signal_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(find_signal_index("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(find_signal_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(find_signal_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
}

#[test]
fn test_part_two() {
    assert_eq!(find_signal_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(find_signal_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(find_signal_index("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(find_signal_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(find_signal_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}
