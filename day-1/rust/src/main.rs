use std::include_str;

fn main() {
    static DATA: &str = include_str!("data.txt");

    let max_cal_count = &DATA
        .split("\n\n")
        .map(|r|
             r
             .split("\n")
             .map(|e|
                  e
                  .parse()
                  .unwrap_or(0))
             .sum::<i32>())
        .max()
        .unwrap();

    println!("The top Elf is carrying the most calories at {}", max_cal_count);

    let sorted_max_cal_elf = &mut DATA
        .split("\n\n")
        .map(|r|
             r
             .split("\n")
             .map(|e|
                  e
                  .parse()
                  .unwrap_or(0))
             .sum::<i32>())
        .collect::<Vec<i32>>();

    sorted_max_cal_elf.sort_by(|x, y| y.cmp(&x));
    let top_three_combined_cals: i32 = sorted_max_cal_elf[..3].iter().sum();
    println!("The top three biggest calorie carrying elves have a combined {} cals", top_three_combined_cals);
}
