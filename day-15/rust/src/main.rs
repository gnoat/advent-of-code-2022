use sscanf::sscanf;
use itertools::{Itertools, iproduct};
use std::{
    collections::{HashMap, HashSet},
    include_str,
    cmp,
};

fn main() {
    static DATA: &str = include_str!("data.txt");
    let net = Network::from(DATA);

    println!("There are {:?} places where a beacon is not possible.", part_one(net.clone(), 2_000_000));
    // println!("The found frequency is {:?}.", part_two(net, 4_000_000));
}

fn part_one(net: Network, row: i64) -> usize {
    net.count_empty_row(row)
}

fn part_two(net: Network, max_search: i64) -> i64 {
    net.full_search(max_search)
}

#[derive(Debug, Clone)]
struct Ranger {
    ranges: HashSet<[i64; 2]>
}

impl Ranger {
    fn new() -> Self {
        Ranger { ranges: HashSet::new() }
    }

    fn insert(&mut self, r: &[i64; 2]) {
        println!(">> {:?} ~ {:?}", self.ranges, r);
        let intersect = self.ranges.clone().into_iter()
            .find(|[min_p, max_p]| (min_p >= &r[0]) && (&r[0] <= max_p) ||
                (max_p >= &r[1]) && (min_p <= &r[1]));
        println!("{:?}", intersect);
        let new_range: [i64; 2] = match intersect {
            Some(i) => {
                self.ranges.remove(&i);
                if (i[0] >= r[0]) && (r[0] <= i[1]) {
                    [i[0], cmp::max(r[1], i[1])]
                } else if (i[1] >= r[1]) && (i[0] <= r[1]) {
                    [cmp::min(i[0], r[0]), i[1]]
                } else { *r }
            },
            None => *r
        };
        self.ranges.insert(new_range);
    }
}

#[derive(Debug, Clone)]
struct Network {
    sensors: HashMap<[i64; 2], i64>,
    width: [i64; 2],
    ranges: HashMap<i64, bool>,
}

impl Network {
    fn from(data: &str) -> Self {
        let raw: HashSet<(i64, i64, i64, i64)> = data
            .lines()
            .filter_map(|s| {
                sscanf!(
                    s,
                    "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}"
                )
                .ok()
            }).collect();
        let sensors: HashMap<[i64; 2], i64> = raw.clone().into_iter()
            .flat_map(|(x0, y0, x1, y1)| [([x0, y0], (x1 - x0).abs() + (y1 - y0).abs()), ([x1, y1], 0)])
            .collect();
        let mut ranges: HashMap<i64, HashSet<[i64; 2]>> = HashMap::new();
        for (k, v) in sensors.clone().into_iter().filter(|(_, v)| v != &0) {
            for (i, n) in (k[1] + v..k[1] + 1).enumerate() {
                ranges.entry(n).or_default().insert([k[0] - i as i64, k[0] + i as i64]);
            }
            for (i, n) in (k[1] - v..k[1] + 1).enumerate() {
                ranges.entry(n).or_default().insert([k[0] - i as i64, k[0] + i as i64]);
            }
        };
        let mut debu: Vec<(i64, HashSet<[i64; 2]>)> = ranges.clone().into_iter().collect();
        debu.sort_by_key(|(i, _)| i.clone());
        debu.into_iter().for_each(|x| println!("{:?}", x));
        let ranges: HashMap<i64, bool> = ranges.into_iter()
            .map(|(i, h)| (i, Self::fuckery(h)))
            .collect();
        println!("\n{:?}", ranges);
        let mut debu: Vec<i64> = ranges.clone().into_iter().filter(|(_, b)| *b).map(|(i, _)| i).collect();
        debu.sort();
        println!("\n{:?}", debu);
        let width: HashSet<i64> = sensors
            .clone()
            .into_iter()
            .flat_map(|(key, val)| [key[0] - val, key[0] + val])
            .collect();
        Network {
            sensors: sensors,
            width: [*width.iter().min().unwrap(), *width.iter().max().unwrap()],
            ranges: ranges,
        }
    }

    fn within_bounds(&self, pos: [&i64; 2]) -> bool {
        self.sensors
            .clone()
            .into_iter()
            .find(|(key, value)| (key[0] - pos[0]).abs() + (key[1] - pos[1]).abs() <= *value)
            .is_some()
    }

    fn count_empty_row(&self, row: i64) -> usize {
        (self.width[0]..self.width[1] + 1)
            .filter(|x| self.within_bounds([x, &row]) && self.sensors.get(&[x.clone(), row]).unwrap_or(&1) != &0)
            .count()
    }
    
    fn full_search(&self, max_search: i64) -> i64 {
        let (x_o, y_o) = iproduct!(0..max_search + 1, 0..max_search + 1)
            .find(|(x, y)| !self.within_bounds([x, y]))
            .unwrap();
        x_o * 4_000_000 + y_o
    }

    fn fuckery(s: HashSet<[i64; 2]>) -> bool {
        let mut v: Vec<[i64; 2]> = s.into_iter()
            .collect();
        v.sort_by_key(|[i, _]| i.clone());
        v.into_iter().tuple_windows::<(_, _)>()
            .find(|(x, y)| x[1] != y[0] + 1)
            .is_some()
    }
}

#[test]
fn test_examples() {
    static TEST_DATA: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    let net = Network::from(TEST_DATA);

    assert_eq!(part_one(net.clone(), 10), 26);
    // assert_eq!(part_two(net, 20), 56_000_011);
}
