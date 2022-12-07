use std::include_str;

fn main() {
    static DATA: &str = include_str!("data.txt");

    let max_cal_count = cal_counter(&DATA);
    println!("The top Elf is carrying the most calories at {}", max_cal_count);

    let sorted_max_cal_elf = combined_top_three(DATA);
    println!("The top three biggest calorie carrying elves have a combined {} cals", sorted_max_cal_elf);
}

fn cal_counter(data: &str) -> i32 {
    data
    .split("\n\n")
        .map(|r|
             r
             .lines()
             .map(|e|
                  e
                  .parse()
                  .unwrap_or(0))
             .sum::<i32>())
        .max()
        .unwrap()
}

fn combined_top_three(data: &str) -> i32 {
    let out = &mut data
        .split("\n\n")
        .map(|r|
             r
             .lines()
             .map(|e|
                  e
                  .parse()
                  .unwrap_or(0))
             .sum::<i32>())
        .collect::<Vec<i32>>();
    
    out.sort_by(|x, y| y.cmp(&x));

    out[..3].iter().sum()
}

#[test]
fn example_tests() {
    static TEST_DATA: &str =
r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    assert_eq!(cal_counter(&TEST_DATA), 24000);
    assert_eq!(combined_top_three(TEST_DATA), 45000);
}
