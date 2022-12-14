pub fn main_conceptual_solution(data: &str)  -> (u32, u32) {
    let first_strat_total_score: &u32 = &data
        .split("\n")
        .map(|row| score_from_row(row).unwrap_or(0))
        .sum();

    println!("    - The total score of this strategy is {}", first_strat_total_score);

    let new_strat_total_score: &u32 = &data
        .split("\n")
        .map(|row| score_from_row_new_strat(row).unwrap_or(0))
        .sum();

    println!("    - The total score with the new strategy is {}", new_strat_total_score);
    (*first_strat_total_score, *new_strat_total_score)
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

fn score_from_row_new_strat(row: &str) -> Option<u32> {
    // After all the work for the first question, I got lazy af so you just get this
    // hacky function instead.  It works, it's just not beautiful.
    let row = row.trim();
    let their_char = row.chars().next()?;
    let my_char = row.chars().last()?;
    let their_hand = Hand::new(their_char)?;
    let their_value = their_hand.value();

    // Don't pay any attention to the sketchy ass logic below, it works
    match my_char {
        'X' => Some(if their_value > 1 { their_value - 1 } else { 3 }),
        'Y' => Some(their_value + 3),
        'Z' => Some(if their_value == 3 { 1 } else { their_value + 1 } + 6),
        _ => None
    }

}


