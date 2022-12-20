use std::{
    include_str,
    cmp::Ordering,
};

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!("The index sum for all pairs in proper order is {:?}.", part_one(DATA));
    // println!("{:?}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    data.split("\n\n")
        .filter_map(|pair| pair.split_once("\n"))
        .map(|pair| Seq::from(pair.0).is_ordered(&mut Seq::from(pair.1)))
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(idx, _)| idx + 1)
        .for_each(|idx| println!("FOUND {}\n", idx));
        //.sum()
    0
}

fn part_two(data: &str) {
    todo!()
}

#[derive(Debug)]
struct Seq {
    stack: Vec<char>
}

impl Seq {
    fn from(s: &str) -> Self {
        let prep_string = s.replace("10", ":")
            .replace(",", "");
        println!("{:?}", prep_string);
        Seq { stack: prep_string.chars().rev().collect() }
    }

    fn is_ordered(&mut self, other: &mut Seq) -> bool {
        let curr_self = match self.stack.pop() {
            Some(x) => x,
            None => '!'
        };
        let curr_other = match other.stack.pop() {
            Some(x) => x,
            None => '!'
        };
        match (curr_self, curr_other) {
            ('!', '!') => {
                self.deb(&curr_self, &other, &curr_other, true);
                true
            },
            ('!', _) => {
                self.deb(&curr_self, &other, &curr_other, true);
                true
            },
            (_, '!') => {
                self.deb(&curr_self, &other, &curr_other, false);
                false
            },
            (_, _) => {
                match (curr_self, curr_other) {
                    (a, b) if a == b => self.is_ordered(other),
                    (_, ']') => {
                        self.deb(&curr_self, &other, &curr_other, false);
                        false
                    },
                    (']', _) => {
                        self.deb(&curr_self, &other, &curr_other, true);
                        true
                    }
                    ('[', x) => {
                        match other.stack.pop() {
                            Some(x) => {
                                other.stack.push(']');
                                other.stack.push(x);
                                println!("SHOWING --[_-- {:?}", other.stack);
                                self.is_ordered(other)
                            },
                            None => {
                                self.deb(&curr_self, &other, &curr_other, true);
                                true
                            }
                        }
                    }
                    (_, '[') => {
                        match self.stack.pop() {
                            Some(x) => {
                                self.stack.push(']');
                                self.stack.push(x);
                                println!("SHOWING --_[-- {:?}", self.stack);
                                self.is_ordered(other)
                            },
                            None => {
                                self.deb(&curr_self, &other, &curr_other, false);
                                false
                            }
                        }
                    }
                    (_, _) => {
                        let out = curr_self < curr_other;
                        if !out {
                            self.deb(&curr_self, &other, &curr_other, false);
                        } else {
                            self.deb(&curr_self, &other, &curr_other, true);
                        };
                        out
                    }
                }
            }
        }
    }

    fn deb(&self, curr_self: &char, other: &Seq, curr_other: &char, b: bool) {
        println!("{:?} {} + {:?} *VS* {} + {:?}", b, curr_self, self.stack.clone().into_iter().rev().collect::<Vec<char>>(), curr_other, other.stack.clone().into_iter().rev().collect::<Vec<char>>());
    }
}

#[test]
fn test_examples() {
    // true - r
    // true - w
    // false - w
    // true - r
    // false - r
    // true - r
    // false - r
    static TEST_DATA: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    assert_eq!(part_one(TEST_DATA), 13);
    // assert_eq!(part_one(TEST_DATA), ());
}
