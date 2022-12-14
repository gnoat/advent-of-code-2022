use itertools::iproduct;
use std::{
    collections::{HashMap, HashSet},
    include_str,
};

fn main() {
    // You can read the solutions by simply running this Rust crate, but to get the full effect
    // you should run `python3 view.py` from the root Day 9 Rust solution directory.  This is a 
    // simply Python script runs the Rust program, captures the output, and visualizes the path
    // taken.
    static DATA: &str = include_str!("data.txt");
    static VALS: &str = "SabcdefghijklmnopqrstuvwxyzE";
    let char_order: HashMap<char, i32> = VALS
        .chars()
        .enumerate()
        .map(|(x, y)| (y, x as i32))
        .collect();

    println!(
        "It takes {:?} moves to get to the top from the start.\n\n",
        part_one(DATA, char_order.clone()).unwrap_or(0)
    );
    println!(
        "It takes {:?} moves to get to the top from any 'a' position.",
        part_two(DATA, char_order).unwrap_or(0)
    );
}

fn part_one(data: &str, char_order: HashMap<char, i32>) -> Option<usize> {
    let (width, height, start, _, matrix) = read_string_to_vector(data, &char_order);
    let mut walker = Walk::new(
        matrix,
        start,
        width,
        height,
        char_order,
        27,
        Direction::Ascending,
    );
    walker.propagate()
}

fn part_two(data: &str, char_order: HashMap<char, i32>) -> Option<usize> {
    let (width, height, _, end, matrix) = read_string_to_vector(data, &char_order);
    let mut walker = Walk::new(
        matrix,
        end,
        width,
        height,
        char_order,
        1,
        Direction::Descending,
    );
    walker.propagate()
}

fn read_string_to_vector(
    data: &str,
    char_order: &HashMap<char, i32>,
) -> (i32, i32, [i32; 2], [i32; 2], Vec<Vec<i32>>) {
    let tree_array: Vec<Vec<i32>> = data
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().map(|c| char_order[&c]).collect())
        .collect();
    let width = tree_array[0].len() as i32;
    let height = tree_array.len() as i32;
    let start = iproduct!(0..height as usize, 0..width as usize)
        .find(|(y, x)| tree_array[*y][*x] == 0)
        .unwrap();
    let end = iproduct!(0..height as usize, 0..width as usize)
        .find(|(y, x)| tree_array[*y][*x] == 27)
        .unwrap();
    (
        width,
        height,
        [start.0 as i32, start.1 as i32],
        [end.0 as i32, end.1 as i32],
        tree_array,
    )
}

enum Direction {
    Ascending,
    Descending,
}

impl Direction {
    fn valid(&self, p_val: &i32, q_val: &i32) -> bool {
        match self {
            Direction::Ascending => p_val - q_val <= 1,
            Direction::Descending => p_val - q_val >= -1,
        }
    }
}

struct Walk {
    visited: HashSet<[i32; 2]>,
    current: HashSet<[i32; 2]>,
    data: Vec<Vec<i32>>,
    width: i32,
    height: i32,
    cycle: usize,
    char_order: HashMap<char, i32>,
    end: i32,
    direction: Direction,
}

impl Walk {
    fn new(
        data: Vec<Vec<i32>>,
        start: [i32; 2],
        width: i32,
        height: i32,
        char_order: HashMap<char, i32>,
        end: i32,
        direction: Direction,
    ) -> Self {
        Walk {
            visited: HashSet::from([start]),
            current: HashSet::from([start]),
            data: data,
            width: width,
            height: height,
            cycle: 0,
            char_order: char_order,
            end: end,
            direction: direction,
        }
    }

    fn next(&mut self) -> Option<bool> {
        let mut new_line = HashSet::new();
        self.current
            .clone()
            .into_iter()
            .map(|p| self.surrounding(&p))
            .for_each(|p| new_line.extend(p));
        if new_line.len() > 0 {
            self.visited.extend(new_line.clone());
            let last_visited = match self.direction {
                Direction::Ascending => self
                    .visited
                    .clone()
                    .into_iter()
                    .map(|p| self.d(&p))
                    .max()?,
                Direction::Descending => self
                    .visited
                    .clone()
                    .into_iter()
                    .map(|p| self.d(&p))
                    .min()?,
            };
            self.current = new_line;
            self.cycle = self.cycle + 1;
            self.draw_progress();
            Some(last_visited == self.end)
        } else {
            self.draw_progress();
            None
        }
    }

    fn draw_progress(&self) {
        for h in 0..self.height {
            let line: String = (0..self.width)
                .map(|w| {
                    if self.visited.contains(&[h, w]) {
                        '.'
                    } else {
                        self.inv_d(&[h, w])
                    }
                })
                .collect();
            println!("{}", line);
        }
        println!("\n");
    }

    fn inv_d(&self, p: &[i32; 2]) -> char {
        self.char_order
            .clone()
            .into_iter()
            .find(|(_, v)| *v == self.d(&p))
            .unwrap()
            .0
    }

    fn propagate(&mut self) -> Option<usize> {
        let mut contains_end = false;
        while !contains_end {
            let dce = self.next();
            match dce {
                Some(x) => {
                    contains_end = x;
                }
                None => return None,
            }
        }
        Some(self.cycle)
    }

    fn surrounding(&self, p: &[i32; 2]) -> HashSet<[i32; 2]> {
        [
            [p[0] + 1, p[1]],
            [p[0] - 1, p[1]],
            [p[0], p[1] + 1],
            [p[0], p[1] - 1],
        ]
        .into_iter()
        .filter(|q| (0 <= q[0]) && (q[0] < self.height) && (0 <= q[1]) && (q[1] < self.width))
        .filter(|q| self.direction.valid(&self.d(&q), &self.d(&p)))
        .filter(|q| !self.visited.contains(q))
        .collect()
    }

    fn d(&self, p: &[i32; 2]) -> i32 {
        self.data[p[0] as usize][p[1] as usize]
    }
}

#[test]
fn test_examples() {
    static TEST_DATA: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;
    static VALS: &str = "SabcdefghijklmnopqrstuvwxyzE";
    let char_order: HashMap<char, i32> = VALS
        .chars()
        .enumerate()
        .map(|(x, y)| (y, x as i32))
        .collect();

    assert_eq!(part_one(TEST_DATA, char_order.clone()), Some(31));
    assert_eq!(part_two(TEST_DATA, char_order), Some(29));
}
