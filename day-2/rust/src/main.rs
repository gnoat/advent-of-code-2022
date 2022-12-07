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

#[test]
fn example_tests() {
    static TEST_DATA:&str =
r#"A Y
B X
C Z
"#;

    // Test conceptual solution
    let (first_ans_concept, second_ans_concept) = main_conceptual_solution(TEST_DATA);
    assert_eq!(first_ans_concept, 15);
    assert_eq!(second_ans_concept, 12);

    // Test fast solution
    let (first_ans_fast, second_ans_fast) = main_speed_solution(TEST_DATA);
    assert_eq!(first_ans_fast, 15);
    assert_eq!(second_ans_fast, 12);
}
