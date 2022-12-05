use std::include_str;

fn main() {
    static DATA: &str = include_str!("data.txt");

    // Part One
    let fully_overlapping_count = &DATA
        .lines()
        .map(|row| row
             .split(",")
             .map(|elf| elf
                .split("-")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
              .collect::<Vec<Vec<i32>>>())
        .filter(|v| ((v[0][0]..v[0][1]+1).contains(&v[1][0]) &&
            (v[0][0]..v[0][1]+1).contains(&v[1][1])) ||
            ((v[1][0]..v[1][1]+1).contains(&v[0][0]) &&
            (v[1][0]..v[1][1]+1).contains(&v[0][1])))
        .count();
    println!("There are {} pairs that are entirely overlapping.", fully_overlapping_count);

    // Part Two
    let partially_overlapping_count = &DATA
        .lines()
        .map(|row| row
             .split(",")
             .map(|elf| elf
                .split("-")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
              .collect::<Vec<Vec<i32>>>())
        .filter(|v| (v[0][0]..v[0][1]+1).contains(&v[1][0]) ||
            (v[0][0]..v[0][1]+1).contains(&v[1][1]) ||
            (v[1][0]..v[1][1]+1).contains(&v[0][0]) ||
            (v[1][0]..v[1][1]+1).contains(&v[0][1]))
        .count();
    println!("There are {} pairs that are partially overlapping.", partially_overlapping_count);
}
