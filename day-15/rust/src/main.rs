use sscanf::sscanf;
use std::{
    collections::{HashMap, HashSet},
    include_str,
    ops::Range,
};

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!("There are {:?} places where a beacon is not possible.", part_one(DATA, 2_000_000));
    // println!("{:?}", part_two(DATA));
}

fn part_one(data: &str, row: i32) -> usize {
    let net = Network::from(data);
    net.count_empty_row(row)
}

fn part_two(data: &str) {
    todo!()
}

#[derive(Debug)]
struct Network {
    sensors: HashMap<[i32; 2], i32>,
    beacons: HashSet<[i32; 2]>,
    width: [i32; 2],
    height: [i32; 2],
}

impl Network {
    fn from(data: &str) -> Self {
        let raw: HashSet<(i32, i32, i32, i32)> = data
            .lines()
            .filter_map(|s| {
                sscanf!(
                    s,
                    "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}"
                )
                .ok()
            }).collect();
        let sensors: HashMap<[i32; 2], i32> = raw.clone().into_iter()
            .map(|(x0, y0, x1, y1)| ([x0, y0], (x1 - x0).abs() + (y1 - y0).abs()))
            .collect();
        let beacons: HashSet<[i32; 2]> =  raw.into_iter()
            .map(|(_, _, y0, y1)| [y0, y1])
            .collect();
        let width: HashSet<i32> = sensors
            .clone()
            .into_iter()
            .flat_map(|(key, val)| [key[0] - val, key[0] + val])
            .collect();
        let height: HashSet<i32> = sensors
            .clone()
            .into_iter()
            .flat_map(|(key, val)| [key[1] - val, key[1] + val])
            .collect();
        Network {
            sensors: sensors,
            beacons: beacons,
            width: [*width.iter().min().unwrap(), *width.iter().max().unwrap()],
            height: [*height.iter().min().unwrap(), *height.iter().max().unwrap()],
        }
    }

    fn within_bounds(&self, pos: [&i32; 2]) -> bool {
        self.sensors
            .clone()
            .into_iter()
            .find(|(key, value)| (key[0] - pos[0]).abs() + (key[1] - pos[1]).abs() <= *value)
            .is_some()
    }

    fn count_empty_row(&self, row: i32) -> usize {
        (self.width[0]..self.width[1] + 1)
            .filter(|x| self.within_bounds([x, &row]) && !self.beacons.contains(&[x.clone(), row.clone()]))
            .count()
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

    assert_eq!(part_one(TEST_DATA, 10), 26);
    // assert_eq!(part_two(TEST_DATA), ());
}
