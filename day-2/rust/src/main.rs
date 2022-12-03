use std::include_str;

fn main() {
    static DATA: &str = include_str!("data.txt");

    let first_strat_total_score: &u32 = &DATA
        .split("\n")
        .map(|row| score_from_row(row).unwrap_or(0))
        .sum();

    println!("The total score of this strategy is {}", first_strat_total_score);

    let new_strat_total_score: &u32 = &DATA
        .split("\n")
        .map(|row| score_from_row_new_strat(row).unwrap_or(0))
        .sum();

    println!("The total score with the new strategy is {}", new_strat_total_score)
}

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {

    fn new(character: char) -> Option<Hand> {
        match character {
            'X' => Some(Hand::Rock),
            'Y' => Some(Hand::Paper),
            'Z' => Some(Hand::Scissors),
            'A' => Some(Hand::Rock),
            'B' => Some(Hand::Paper),
            'C' => Some(Hand::Scissors),
            _ => None,
        }
    }

    fn value(&self) -> u32 {
        // MAPPING:
        // A, X = rock -> 1
        // B, Y = paper -> 2
        // C, Z = scissors -> 3
        match &*self {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
        }
    }

   fn cmp(&self, other: &Hand) -> u32 {
        // ORDER:
        // rock > scissors > paper > rock ...
       match &*self {
            Hand::Rock => match other {
                Hand::Rock => 3,
                Hand::Paper => 0,
                Hand::Scissors => 6,
            },
            Hand::Paper => match other {
                Hand::Rock => 6,
                Hand::Paper => 3,
                Hand::Scissors => 0,
            },
            Hand::Scissors => match other {
                Hand::Rock => 0,
                Hand::Paper => 6,
                Hand::Scissors => 3,
            }
        }
    }

    fn score(&self, other: &Hand) -> u32 {
        self.cmp(other) + self.value()
    }
}

fn score_from_row(row: &str) -> Option<u32> {
    let row = row.trim();
    let their_hand = Hand::new(row.chars().next()?)?;
    let my_hand = Hand::new(row.chars().last()?)?;
    Some(my_hand.score(&their_hand))
}

fn cheap_permute(n: u32, forward: bool) -> u32 {
    if forward {
        match &n {
            1 => 2,
            2 => 3,
            _ => 1,
        }
    } else {
        match &n {
            2 => 1,
            3 => 2,
            _ => 3,
        }
    }
}
fn score_from_row_new_strat(row: &str) -> Option<u32> {
    let row = row.trim();
    let their_char = row.chars().next()?;
    let my_char = row.chars().last()?;
    let their_hand = Hand::new(their_char)?;
    let their_value = their_hand.value();
    match my_char {
        'X' => Some(cheap_permute(their_value, false)),
        'Y' => Some(their_value + 3),
        'Z' => Some(cheap_permute(their_value, true) + 6),
        _ => None
    }
}
