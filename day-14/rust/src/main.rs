use itertools::Itertools;
use std::{cmp, collections::HashMap, include_str};

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!(
        "There are {:?} grains of sand before infinity.",
        part_one(DATA)
    );
    println!(
        "There are {:?} grains of sand before it hits the spout.",
        part_two(DATA)
    );
}

fn part_one(data: &str) -> usize {
    let mut cracks = Cracks::from(data, false);
    cracks.drop_all();
    cracks.sand_grains
}

fn part_two(data: &str) -> usize {
    let mut cracks = Cracks::from(data, true);
    cracks.drop_all();
    cracks.sand_grains
}

struct Cracks {
    blocks: HashMap<[usize; 2], char>,
    max_depth: usize,
    sand_grains: usize,
    floor: bool,
}

impl Cracks {
    fn from(data: &str, floor: bool) -> Self {
        let mut blocks: HashMap<[usize; 2], char> = HashMap::new();
        let mut max_depth = 0;

        for row in data.lines().map(|s| s.split(" -> ")) {
            for (x, y) in row.tuple_windows::<(_, _)>() {
                let first: Vec<usize> = x.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
                let second: Vec<usize> =
                    y.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
                max_depth = cmp::max(max_depth, cmp::max(first[1], second[1]));
                if first[0] != second[0] {
                    for i in cmp::min(first[0], second[0])..cmp::max(first[0], second[0]) + 1 {
                        blocks.insert([i, first[1]], '#');
                    }
                } else {
                    for i in cmp::min(first[1], second[1])..cmp::max(first[1], second[1]) + 1 {
                        blocks.insert([first[0], i], '#');
                    }
                }
            }
        }
        Cracks {
            blocks: blocks,
            max_depth: max_depth,
            sand_grains: if floor { 1 } else { 0 },
            floor: floor,
        }
    }

    fn drop_one(&mut self) -> bool {
        let mut pos: Option<[usize; 2]> = Some([500, 0]);
        while self.next_position(pos.unwrap()).is_some() {
            pos = self.next_position(pos.unwrap());
            if (pos.unwrap()[1] >= self.max_depth) && !self.floor {
                return true;
            } else if (pos.unwrap()[1] >= self.max_depth + 1) && self.floor {
                self.blocks.insert(pos.unwrap(), 'o');
                return false;
            };
        }
        if pos.unwrap()[1] == 0 {
            return true;
        }
        self.blocks.insert(pos.unwrap(), 'o');
        false
    }

    fn drop_all(&mut self) {
        while !self.drop_one() {
            self.sand_grains += 1;
        }
    }

    fn next_position(&self, pos: [usize; 2]) -> Option<[usize; 2]> {
        if !self.blocks.contains_key(&[pos[0], pos[1] + 1]) {
            Some([pos[0], pos[1] + 1])
        } else if !self.blocks.contains_key(&[pos[0] - 1, pos[1] + 1]) {
            Some([pos[0] - 1, pos[1] + 1])
        } else if !self.blocks.contains_key(&[pos[0] + 1, pos[1] + 1]) {
            Some([pos[0] + 1, pos[1] + 1])
        } else {
            None
        }
    }
}

#[test]
fn test_examples() {
    static TEST_DATA: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    assert_eq!(part_one(TEST_DATA), 24);
    assert_eq!(part_two(TEST_DATA), 93);
}
