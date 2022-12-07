use std::{collections::HashMap, include_str};

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!(
        "The crate {} ends up on top in the first problem",
        solution_part_one(&DATA)
    );

    println!(
        "The crate {} ends up on top in the second problem",
        solution_part_two(&DATA)
    )
}

#[derive(Debug)]
struct Arrangement {
    stacks: HashMap<char, Vec<char>>,
    moves: Vec<(u32, char, char)>,
}

impl Arrangement {
    fn new(raw: &str) -> Self {
        let parts = raw.split("\n\n");
        let mut configuration: HashMap<char, Vec<char>> = HashMap::new();
        let starting: Vec<&str> = parts.clone().next().unwrap_or(&"").lines().collect();
        for c in starting[starting.len() - 1].chars().into_iter() {
            if c != ' ' {
                configuration.insert(c, Vec::new());
            }
        }
        for content in starting[..starting.len() - 1].iter().rev() {
            for (idx, c) in content.chars().enumerate() {
                if (c != ' ') && (c != '[') && (c != ']') {
                    let base_c = starting[starting.len() - 1].chars().nth(idx).unwrap();
                    configuration.get_mut(&base_c).unwrap().push(c);
                }
            }
        }

        let read_moves: Vec<(u32, char, char)> = parts
            .last()
            .unwrap_or(&"")
            .lines()
            .map(|r| {
                r.replace("move ", "")
                    .replace("from ", "")
                    .replace("to ", "")
                    .split(" ")
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
            })
            .filter(|v| v.len() == 3)
            .map(|v| {
                (
                    v[0].parse::<u32>().unwrap_or(0),
                    v[1].chars().next().unwrap_or('1'),
                    v[2].chars().next().unwrap_or('1'),
                )
            })
            .collect();

        Arrangement {
            stacks: configuration,
            moves: read_moves,
        }
    }

    fn move_crates(&mut self, num: u32, from: char, to: char) {
        for _ in 0..num {
            let removed = self.stacks.get_mut(&from).unwrap().pop();
            match removed {
                Some(x) => self.stacks.get_mut(&to).unwrap().push(x),
                None => {
                    println!("[error] missing move {} from {} to {}", num, from, to);
                }
            }
        }
    }

    fn move_crates_new_crane(&mut self, num: u32, from: char, to: char) {
        let current = self.stacks.get_mut(&from).unwrap().clone();
        let mut to_be_moved: Vec<char> = current
            .iter()
            .rev()
            .take(num as usize)
            .rev()
            .map(|r| r.to_owned())
            .collect();
        self.stacks
            .get_mut(&from)
            .unwrap()
            .drain(current.len() - to_be_moved.len()..);
        self.stacks.get_mut(&to).unwrap().append(&mut to_be_moved);
    }

    fn skim_top(&mut self) -> String {
        let mut top = "".to_string();
        for idx in 1..10 {
            let index = idx.to_string().chars().next().unwrap();
            if self.stacks.contains_key(&index) {
                top = format!(
                    "{}{}",
                    top,
                    &self.stacks.get_mut(&index).unwrap().pop().unwrap_or(' ')
                );
            }
        }
        top
    }

    fn make_moves(&mut self) {
        for step in self.moves.clone().into_iter() {
            let _ = &self.move_crates(step.0, step.1, step.2);
        }
    }

    fn make_moves_new_crane(&mut self) {
        for step in self.moves.clone().into_iter() {
            let _ = &self.move_crates_new_crane(step.0, step.1, step.2);
        }
    }
}

fn solution_part_one(data: &'static str) -> String {
    let mut boxes = Arrangement::new(data);
    boxes.make_moves();
    boxes.skim_top()
}

fn solution_part_two(data: &'static str) -> String {
    let mut boxes = Arrangement::new(data);
    boxes.make_moves_new_crane();
    boxes.skim_top()
}

#[test]
fn example_tests() {
    static TEST_DATA: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    assert_eq!(solution_part_one(TEST_DATA), "CMZ".to_string());
    assert_eq!(solution_part_two(TEST_DATA), "MCD".to_string());
}
