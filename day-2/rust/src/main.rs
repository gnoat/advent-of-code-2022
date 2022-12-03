use std::include_str;
use day_two::{
    conceptual::main_conceptual_solution,
    speed::main_speed_solution,
};

fn main() {
    // I solved this problem in two separate ways, one way was the conceptual way
    // to build enums and methods that mimic the game.  I did this to build my Rust
    // skills.
    //
    // The second solution is the fast, efficient solution.  This one just manually
    // maps scores and applies it to the data.
    static DATA: &str = include_str!("data.txt");

    println!("+ Conceptual solution results:");
    main_conceptual_solution(DATA);

    println!("+ Speed solution results:");
    main_speed_solution(DATA);

}

