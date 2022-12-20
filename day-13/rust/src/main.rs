use std::{cmp::Ordering, include_str};

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!(
        "The index sum for all pairs in proper order is {:?}.",
        part_one(DATA)
    );
    println!("The divider packet index product is {:?}.", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    data.split("\n\n")
        .filter_map(|pair| pair.split_once("\n"))
        .map(|pair| Seq::from(pair.0) <= Seq::from(pair.1))
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn part_two(data: &str) -> usize {
    let data_extended = format!("{}\n[[2]]\n[[6]]", data.replace("\n\n", "\n"));
    let mut seq_vec: Vec<Seq> = data_extended
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| Seq::from(s))
        .collect();
    seq_vec.sort();
    let first_idx = seq_vec
        .iter()
        .position(|seq| seq == &Seq::from("[[2]]"))
        .unwrap()
        + 1;
    let second_idx = seq_vec
        .iter()
        .position(|seq| seq == &Seq::from("[[6]]"))
        .unwrap()
        + 1;

    first_idx * second_idx
}

#[derive(Debug, Clone, Eq)]
struct Seq {
    stack: Vec<char>,
}

impl Seq {
    fn from<T: AsRef<str>>(s: T) -> Self {
        let prep_string = s.as_ref().replace("10", ":").replace(",", "");
        Seq {
            stack: prep_string.chars().rev().collect(),
        }
    }

    fn compare(&mut self, other: &mut Seq) -> Ordering {
        let curr_self = self.stack.pop().unwrap_or('!');
        let curr_other = other.stack.pop().unwrap_or('!');
        match (curr_self, curr_other) {
            ('!', '!') => Ordering::Equal,
            ('!', _) => Ordering::Less,
            (_, '!') => Ordering::Greater,
            (a, b) if a == b => self.compare(other),
            (_, ']') => Ordering::Greater,
            (']', _) => Ordering::Less,
            ('[', b) => {
                other.stack.push(']');
                other.stack.push(b);
                self.compare(other)
            }
            (a, '[') => {
                self.stack.push(']');
                self.stack.push(a);
                self.compare(other)
            }
            (a, b) => a.cmp(&b),
        }
    }
}

impl PartialOrd for Seq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.clone().compare(&mut other.clone()))
    }
}

impl Ord for Seq {
    fn cmp(&self, other: &Self) -> Ordering {
        self.clone().compare(&mut other.clone())
    }
}

impl PartialEq for Seq {
    fn eq(&self, other: &Self) -> bool {
        self.clone().compare(&mut other.clone()) == Ordering::Equal
    }
}

#[test]
fn test_examples() {
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
    assert_eq!(part_two(TEST_DATA), 140);
}
