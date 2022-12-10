use std::{collections::HashSet, include_str, mem::take};

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!("The tail visits {} distinct locations.", part_one(DATA));
    println!("The 9th tail visits {} distinct locations.", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let mut board = Board::new();
    data.lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.split(" "))
        .for_each(|mut l| {
            board = board.moves(
                &l.next().unwrap().chars().next().unwrap(),
                &l.last().unwrap().parse::<i32>().unwrap(),
            )
        });

    board.visits.len()
}

fn part_two(data: &str) -> usize {
    // This is super messy and inefficient.  I would've modeled the entire system completely
    // differently if I'd see what part two was going to ask me to do.  That not being what
    // happened, though, left me with trying to fit a square peg in a circular hole because I don't
    // have the time to rewrite everything from scratch.
    //
    // This would be much easier and cleaner to model with a struct that has a tails field that is are
    // Vec<[i32; 2]> type instead of just a [i32; 2] type.  This would allow us to simply iterate
    // over every tail of the rope and apply the `trail` method to each one in sequence.
    //
    // There are only 8 tail Boards because the tail of the 8th tail board is the 9th tail to the
    // chain.
    let mut head = Board::new();
    let mut tail_1 = Board::new();
    let mut tail_2 = Board::new();
    let mut tail_3 = Board::new();
    let mut tail_4 = Board::new();
    let mut tail_5 = Board::new();
    let mut tail_6 = Board::new();
    let mut tail_7 = Board::new();
    let mut tail_8 = Board::new();

    for mut cmd in data.lines().filter(|l| l.len() > 0).map(|l| l.split(" ")) {
        let dir = &cmd.next().unwrap().chars().next().unwrap();
        let dist = &cmd.last().unwrap().parse::<i32>().unwrap();
        for _ in (0..*dist).into_iter() {
            head = head.single_move(dir);
            tail_1 = tail_1.trail([head.t[0], head.t[1]]);
            tail_2 = tail_2.trail([tail_1.t[0], tail_1.t[1]]);
            tail_3 = tail_3.trail([tail_2.t[0], tail_2.t[1]]);
            tail_4 = tail_4.trail([tail_3.t[0], tail_3.t[1]]);
            tail_5 = tail_5.trail([tail_4.t[0], tail_4.t[1]]);
            tail_6 = tail_6.trail([tail_5.t[0], tail_5.t[1]]);
            tail_7 = tail_7.trail([tail_6.t[0], tail_6.t[1]]);
            tail_8 = tail_8.trail([tail_7.t[0], tail_7.t[1]]);
        }
    }
    tail_8.visits.len()
}

struct Board {
    h: [i32; 2],
    t: [i32; 2],
    visits: HashSet<[i32; 2]>,
}

impl Board {
    fn new() -> Self {
        Board {
            h: [0, 0],
            t: [0, 0],
            visits: HashSet::from([[0, 0]]),
        }
    }

    fn is_nearby(&self, next_position: &[i32; 2]) -> bool {
        let diff: [i32; 2] = [next_position[0] - self.t[0], next_position[1] - self.t[1]];
        let max_diff = diff.iter().map(|&v| v.abs()).max().unwrap();
        max_diff <= 1
    }

    fn moves(&mut self, direction: &char, distance: &i32) -> Self {
        (0..*distance)
            .into_iter()
            .for_each(|_| *self = self.single_move(&direction));
        Board {
            h: take(&mut self.h),
            t: take(&mut self.t),
            visits: take(&mut self.visits),
        }
    }

    fn single_move(&mut self, direction: &char) -> Self {
        let new_h_position: [i32; 2] = match direction {
            'U' => [self.h[0], self.h[1] + 1],
            'D' => [self.h[0], self.h[1] - 1],
            'L' => [self.h[0] - 1, self.h[1]],
            'R' => [self.h[0] + 1, self.h[1]],
            _ => [self.h[0], self.h[1]],
        };
        let new_t_position: [i32; 2] = if self.is_nearby(&new_h_position) {
            self.t
        } else {
            self.h
        };
        self.visits.insert(new_t_position);
        Board {
            h: new_h_position,
            t: new_t_position,
            visits: take(&mut self.visits),
        }
    }

    fn trail(&mut self, next_position: [i32; 2]) -> Self {
        let calc_dir = [next_position[0] - self.t[0], next_position[1] - self.t[1]];
        if calc_dir[0].abs() > 1 || calc_dir[1].abs() > 1 {
            let new_tail = [
                self.t[0] + calc_dir[0].signum(),
                self.t[1] + calc_dir[1].signum(),
            ];
            self.visits.insert(new_tail);
            Board {
                h: next_position,
                t: new_tail,
                visits: take(&mut self.visits),
            }
        } else {
            Board {
                h: next_position,
                t: self.t,
                visits: take(&mut self.visits),
            }
        }
    }
}

#[test]
fn test_examples() {
    static TEST_DATA: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    assert_eq!(part_one(TEST_DATA), 13);
    assert_eq!(part_two(TEST_DATA), 1);
}

#[test]
fn test_example_two() {
    static TEST_DATA: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    assert_eq!(part_two(TEST_DATA), 36);
}
