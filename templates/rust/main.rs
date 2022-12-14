use std::include_str;

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!("{:?}", part_one(DATA));
    println!("{:?}", part_two(DATA));
}

fn part_one(data: &str) {
    todo!()
}

fn part_two(data: &str) {
    todo!()
}

#[test]
fn test_examples() {
    static TEST_DATA: &str = r#""#;

    assert_eq!(part_one(TEST_DATA), ());
    assert_eq!(part_one(TEST_DATA), ());
}
