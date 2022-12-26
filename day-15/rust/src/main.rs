use itertools::{iproduct, Itertools};
use sscanf::sscanf;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    include_str,
};

fn main() {
    static DATA: &str = include_str!("data.txt");
    let net = Network::from(DATA);

    println!(
        "There are {:?} places where a beacon is not possible.",
        part_one(net.clone(), 2_000_000)
    );
    println!("The found frequency is {:?}.", part_two(net, 4_000_000));
}

fn part_one(net: Network, row: i64) -> usize {
    net.count_empty_row(row)
}

fn part_two(net: Network, max_search: i64) -> i64 {
    net.ranges
        .into_iter()
        .filter(|(idx, b)| (b > &0) && (&0 < idx) && (idx <= &max_search))
        .map(|(idx, b)| b * 4_000_000 + idx)
        .next()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Network {
    sensors: HashMap<[i64; 2], i64>,
    width: [i64; 2],
    ranges: HashMap<i64, i64>,
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
            })
            .collect();
        let sensors: HashMap<[i64; 2], i64> = raw
            .clone()
            .into_iter()
            .flat_map(|(x0, y0, x1, y1)| {
                [([x0, y0], (x1 - x0).abs() + (y1 - y0).abs()), ([x1, y1], 0)]
            })
            .collect();
        let mut ranges: HashMap<i64, HashSet<[i64; 2]>> = HashMap::new();
        for (k, v) in sensors.clone().into_iter() {
            for (i, n) in (k[1]..k[1] + v + 1).enumerate() {
                ranges
                    .entry(n)
                    .or_default()
                    .insert([k[0] - (v - i as i64), k[0] + (v - i as i64)]);
            }
            for (i, n) in (k[1] - v..k[1] + 1).enumerate() {
                ranges
                    .entry(n)
                    .or_default()
                    .insert([k[0] - i as i64, k[0] + i as i64]);
            }
        }
        let ranges: HashMap<i64, i64> = ranges
            .into_iter()
            .map(|(i, h)| (i, Self::evaluate(h)))
            .collect();
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
            .filter(|x| {
                self.within_bounds([x, &row])
                    && self.sensors.get(&[x.clone(), row]).unwrap_or(&1) != &0
            })
            .count()
    }

    fn combine(v0: Vec<[i64; 2]>) -> Vec<[i64; 2]> {
        let mut v = v0.clone();
        v.sort_by_key(|[i, _]| i.clone());
        let mut init: [i64; 2] = v[0];
        let mut com = vec![init];
        for idx in 1..v.len() {
            if v[idx][0] <= init[1] + 1 {
                init = [init[0], cmp::max(v[idx][1], init[1])];
                com = vec![init];
            } else {
                let rest = v.split_off(idx);
                com.extend(Self::combine(rest));
                break;
            }
        }
        com
    }

    fn evaluate(h: HashSet<[i64; 2]>) -> i64 {
        let mut v: Vec<[i64; 2]> = h.into_iter().collect();
        v.sort_by_key(|[i, _]| i.clone());
        let out = Self::combine(v);
        if out.len() > 1 {
            out[0][1] + 1
        } else {
            0
        }
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
    assert_eq!(part_two(net, 20), 56_000_011);
}
