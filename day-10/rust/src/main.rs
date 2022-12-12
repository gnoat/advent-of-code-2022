use std::include_str;

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!("The sum of signal strengths is {}.\n", part_one(DATA));
    println!("Image for part two:\n");
    println!("{}", part_two(DATA));
}

struct Signal {
    x: i32,
    history: Vec<i32>,
    image: Vec<char>
}

impl Signal {
    fn new() -> Self {
        Signal { x: 1 , history: vec![1], image: vec!['#'] }
    }

    fn instr(&mut self, cmd: &str)  {
        if cmd.starts_with("noop") {
            self.noop();
        } else if cmd.starts_with("addx") {
            self.addx(cmd.split(" ").last().unwrap_or("0").parse::<i32>().unwrap_or(0));
        }
    }

    fn noop(&mut self) {
        self.update_image();
        self.history.push(self.x.clone());
    }

    fn addx(&mut self, val: i32) {
        self.update_image();
        self.history.push(self.x.clone());
        self.x = self.x + val;
        self.update_image();
        self.history.push(self.x.clone());
    }

    fn from_instructions(data: &str) -> Self {
        let mut signal = Signal::new();
        data.lines()
            .for_each(|instruction| signal.instr(instruction));

        signal
    }

    fn update_image(&mut self) {
        if self.history.len() < 240 {
            let cycle_position = (self.history.len()) as i32 % 40;
            if cycle_position == 0 {
                self.image.push('\n');
            }
            if (self.x - 1 <= cycle_position) && (cycle_position <= self.x + 1) {
                self.image.push('#');
            } else {
                self.image.push('.');
            }
        }
    }
}

fn part_one(data: &str) -> i32 {
    let signal = Signal::from_instructions(data);
    (0..6).map(|n| 20 + n * 40)
        .map(|n| (n as i32) * signal.history[n - 1])
        .sum()
}

fn part_two(data: &str) -> String {
    let signal = Signal::from_instructions(data);
    signal.image.iter().collect()
}

#[test]
fn test_examples() {
    static TEST_DATA: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    static EXPECTED_IMAGE: &str = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

    assert_eq!(part_one(TEST_DATA), 13140);
    assert_eq!(part_two(TEST_DATA), EXPECTED_IMAGE.to_string());
}

