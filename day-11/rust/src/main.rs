use sscanf::sscanf;
use std::{collections::HashMap, include_str};

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!(
        "The level of monkey business after 20 rounds is {}.\n",
        part_one(DATA)
    );
    println!(
        "The level of monkey business after 1000 rounds is {}.\n",
        part_two(DATA)
    );
}

fn part_one(data: &str) -> u64 {
    let mut troop = Troop::new(3);
    troop.load(data);
    troop.monkey_around_too_much(20);
    let mut vals: Vec<u64> = troop.held_count.values().map(|n| *n as u64).collect();
    vals.sort();
    vals.reverse();
    vals[0] * vals[1]
}

fn part_two(data: &str) -> u64 {
    let mut troop = Troop::new(1);
    troop.load(data);
    troop.monkey_around_too_much(10000);
    let mut vals: Vec<&u64> = troop.held_count.values().collect();
    vals.sort();
    vals.reverse();
    vals[0] * vals[1]
}

enum Operation {
    Mult(u64),
    Add(u64),
    Square,
    Double,
}

impl Operation {
    fn eval(&self, val: &u64) -> u64 {
        match self {
            Operation::Mult(n) => n * val,
            Operation::Add(n) => n + val,
            Operation::Square => val * val,
            Operation::Double => val + val,
        }
    }
}

struct Monkey {
    monkey: usize,
    held_items: Vec<u64>,
    operation: Operation,
    test: u64,
    true_toss: usize,
    false_toss: usize,
}

impl Monkey {
    fn from(block: &str) -> Self {
        let mut lines = block.lines();
        let monkey_n = sscanf!(lines.next().unwrap().trim(), "Monkey {usize}:").unwrap();
        let starting: Vec<u64> = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .into_iter()
            .rev()
            .collect();
        let op_tupe = sscanf!(
            lines.next().unwrap().trim(),
            "Operation: new = old {char} {str}"
        )
        .unwrap();
        let op = match op_tupe.0 {
            '*' => match op_tupe.1 {
                "old" => Operation::Square,
                _ => Operation::Mult(op_tupe.1.parse::<u64>().unwrap()),
            },
            '+' => match op_tupe.1 {
                "old" => Operation::Double,
                _ => Operation::Add(op_tupe.1.parse::<u64>().unwrap()),
            },
            _ => panic!("Operation not found"),
        };
        let test_n = sscanf!(lines.next().unwrap().trim(), "Test: divisible by {u64}").unwrap();
        let true_toss_n = sscanf!(
            lines.next().unwrap().trim(),
            "If true: throw to monkey {usize}"
        )
        .unwrap();
        let false_toss_n = sscanf!(
            lines.next().unwrap().trim(),
            "If false: throw to monkey {usize}"
        )
        .unwrap();
        Monkey {
            monkey: monkey_n,
            held_items: starting,
            operation: op,
            test: test_n,
            true_toss: true_toss_n,
            false_toss: false_toss_n,
        }
    }
}

struct Troop {
    monkeys: HashMap<usize, Monkey>,
    held_count: HashMap<usize, u64>,
    div: u64,
    gcd: u64,
}

impl Troop {
    fn new(d: u64) -> Self {
        Troop {
            monkeys: HashMap::new(),
            held_count: HashMap::new(),
            div: d,
            gcd: 1,
        }
    }

    fn load(&mut self, data: &str) {
        for block in data.split("\n\n") {
            let monkey = Monkey::from(block);
            self.held_count.insert(monkey.monkey.clone(), 0);
            self.monkeys.insert(monkey.monkey.clone(), monkey);
        }

        self.gcd = self.monkeys.values().map(|m| m.test).product();
    }

    fn monkey_around_once(&mut self) {
        for monkey_idx in 0..self.monkeys.len() {
            while self.monkeys[&monkey_idx].held_items.len() > 0 {
                let initial_item_worry = self
                    .monkeys
                    .get_mut(&monkey_idx)
                    .unwrap()
                    .held_items
                    .pop()
                    .unwrap();
                let mut item_worry = self.monkeys[&monkey_idx]
                    .operation
                    .eval(&initial_item_worry)
                    / self.div;
                item_worry %= self.gcd;
                self.held_count
                    .insert(monkey_idx.clone(), self.held_count[&monkey_idx] + 1);
                if item_worry % self.monkeys[&monkey_idx].test == 0 {
                    self.monkeys
                        .get_mut(&self.monkeys[&monkey_idx].true_toss.clone())
                        .unwrap()
                        .held_items
                        .insert(0, item_worry);
                } else {
                    self.monkeys
                        .get_mut(&self.monkeys[&monkey_idx].false_toss.clone())
                        .unwrap()
                        .held_items
                        .insert(0, item_worry);
                };
            }
        }
    }

    fn monkey_around_too_much(&mut self, iterations: u64) {
        (0..iterations).for_each(|_| self.monkey_around_once());
    }
}

#[test]
fn test_examples() {
    static TEST_DATA: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    assert_eq!(part_one(TEST_DATA), 10605);
    assert_eq!(part_two(TEST_DATA), 2713310158);
}
