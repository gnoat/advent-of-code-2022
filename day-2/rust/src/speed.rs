pub fn main_speed_solution(data: &str) -> (u32, u32) {
    let first_strat_total = &data
        .split("\n")
        .map(|row| fast_score_old_strat(row.trim()))
        .sum::<u32>();
    println!("    - The total score of this strategy is {}", first_strat_total);

    let second_strat_total = &data
        .split("\n")
        .map(|row| fast_score_new_strat(row.trim()))
        .sum::<u32>();
    println!("    - The total score with the new strategy is {}", second_strat_total);

    (*first_strat_total, *second_strat_total)
}

fn fast_score_old_strat(row: &str) -> u32 {
    match row.trim() {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

fn fast_score_new_strat(row: &str) -> u32 {
    match row.trim() {
        "A X" => 3 + 0,
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 2 + 0,
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => 0,
    }
}
