use std::include_str;

fn main() {
    static DATA: &str = include_str!("data.txt");

    // Part One
    let fully_overlapping_count = count_overlapping(DATA);
    println!(
        "There are {} pairs that are entirely overlapping.",
        fully_overlapping_count
    );

    // Part Two
    let partially_overlapping_count = count_partially_overlapping(DATA);
    println!(
        "There are {} pairs that are partially overlapping.",
        partially_overlapping_count
    );
}

fn count_overlapping(DATA: &str) -> usize {
    DATA.lines()
        .map(|row| {
            row.split(",")
                .map(|elf| {
                    elf.split("-")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .filter(|v| {
            ((v[0][0]..v[0][1] + 1).contains(&v[1][0]) && (v[0][0]..v[0][1] + 1).contains(&v[1][1]))
                || ((v[1][0]..v[1][1] + 1).contains(&v[0][0])
                    && (v[1][0]..v[1][1] + 1).contains(&v[0][1]))
        })
        .count()
}

fn count_partially_overlapping(DATA: &str) -> usize {
    DATA.lines()
        .map(|row| {
            row.split(",")
                .map(|elf| {
                    elf.split("-")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .filter(|v| {
            (v[0][0]..v[0][1] + 1).contains(&v[1][0])
                || (v[0][0]..v[0][1] + 1).contains(&v[1][1])
                || (v[1][0]..v[1][1] + 1).contains(&v[0][0])
                || (v[1][0]..v[1][1] + 1).contains(&v[0][1])
        })
        .count()
}

#[test]
fn examples_tests() {
    static TEST_DATA: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    assert_eq!(count_overlapping(TEST_DATA), 2);
    assert_eq!(count_partially_overlapping(TEST_DATA), 4);
}
