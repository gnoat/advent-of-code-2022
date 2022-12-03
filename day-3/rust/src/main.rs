use std::collections::HashMap;
use std::include_str;

fn main() {
    // Create default strs
    static DATA: &str = include_str!("data.txt");
    static VALS: &str = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // Create a hash map to measure priority for a given character
    let priority: HashMap<char, usize> = VALS.chars().enumerate().map(|(x, y)| (y, x)).collect();

    // Basically just iterate over the characters of each entry, split in two,
    // check for a common character, the use priority HashMap to evaluate each string
    // and sum together.
    let total_priority = &DATA
        .split("\n")
        .map(|row| {
            row[..row.len() / 2]
                .chars()
                .into_iter()
                .filter(|c| row[row.len() / 2..].contains(*c))
                .next()
                .unwrap_or('0')
        })
        .map(|c| priority.get(&c).unwrap_or(&0))
        .sum::<usize>();

    println!(
        "The total priority of the mixed items is {}.",
        total_priority
    );

    // Basically just chunks the data into three line groups and do the same as above,
    // except now we have to calculate the common character between three strings
    // instead of two.
    let total_group_priority: usize = DATA
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .filter(|v| v.len() == 3)
        .map(|v| {
            v[0][..]
                .chars()
                .into_iter()
                .filter(|c| v[1][..].contains(*c))
                .into_iter()
                .filter(|c| v[2][..].contains(*c))
                .next()
                .unwrap_or('0')
        })
        .map(|c| priority.get(&c).unwrap_or(&0))
        .sum();

    println!(
        "The total priority of each group's badges is {}.",
        total_group_priority
    );
}
