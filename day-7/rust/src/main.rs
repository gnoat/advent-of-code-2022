use std::{collections::HashMap, include_str, mem::take};

fn main() {
    static DATA: &str = include_str!("data.txt");

    println!(
        "The sum of total sizes of directories with at most size 100,000 is {}.",
        sum_dirs_part_one(DATA)
    );

    println!(
        "The smallest file to delete that frees up needed space is {}.",
        smallest_to_delete_part_two(DATA)
    );
}

fn sum_dirs_part_one(data: &str) -> u64 {
    FileSystem::extract_fs(data)
        .contents
        .values()
        .filter(|&&size| size <= 100_000)
        .sum()
}

fn smallest_to_delete_part_two(data: &str) -> u64 {
    let fs = FileSystem::extract_fs(data);
    let needed_free = 30_000_000 - (70_000_000 - fs.contents.get(&"/".to_string()).unwrap_or(&0));
    println!("Need to free {}.", needed_free);
    *fs.contents
        .values()
        .filter(|&&size| (30_000_000 >= size) && (size >= needed_free))
        .min()
        .unwrap_or(&0)
}

#[derive(Debug)]
struct FileSystem {
    cd: String,
    contents: HashMap<String, u64>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            cd: "/".to_string(),
            contents: HashMap::<String, u64>::new(),
        }
    }

    fn extract_fs(data: &str) -> FileSystem {
        let mut fs = FileSystem::new();

        data.split("$ ")
            .filter_map(|block| block.split_once("\n"))
            .for_each(|(cmd, output)| fs = fs.exec(cmd, output));
        fs
    }

    fn exec(&mut self, cmd: &str, follow_block: &str) -> FileSystem {
        if cmd.contains("cd /") {
            FileSystem {
                cd: "/".to_string(),
                contents: take(&mut self.contents),
            }
        } else if cmd.contains("cd ..") {
            FileSystem {
                cd: self.cd.rsplit_once("/").unwrap_or(("/", "")).0.to_string(),
                contents: take(&mut self.contents),
            }
        } else if cmd.contains("cd") {
            let new_dir = cmd.split(" ").last().unwrap_or("/").to_string();
            FileSystem {
                cd: format!("{}{}/", self.cd, new_dir),
                contents: take(&mut self.contents),
            }
        } else if cmd.contains("ls") {
            let current_contents = Self::measure_block(follow_block);
            self.contents
                .entry(self.cd.clone())
                .or_insert(current_contents);
            self.backpropagate_content(self.cd.clone());
            FileSystem {
                cd: take(&mut self.cd),
                contents: take(&mut self.contents),
            }
        } else {
            FileSystem {
                cd: take(&mut self.cd),
                contents: take(&mut self.contents),
            }
        }
    }

    fn measure_block(follow_block: &str) -> u64 {
        follow_block
            .lines()
            .filter(|row| !row.contains("dir ") && !row.is_empty())
            .map(|row| row.split(" ").next().unwrap().parse::<u64>().unwrap_or(0))
            .sum()
    }

    fn backpropagate_content(&mut self, dir: String) {
        // This is ugly but works.  If I get time, I'll refactor with a more functional approach.
        let mut parent_dir = dir.clone();
        let backprop_value = self.contents.get(&parent_dir).unwrap().clone();
        while parent_dir != "/".to_string() {
            let uncorrected_parent_dir = parent_dir
                .rsplit_once("/")
                .unwrap_or(("/", ""))
                .0
                .rsplit_once("/")
                .unwrap_or(("/", ""))
                .0;
            parent_dir = if uncorrected_parent_dir != "/" {
                format!("{}/", uncorrected_parent_dir)
            } else {
                "/".to_string()
            };
            let current_contents = self.contents.get_mut(&parent_dir).unwrap().clone();
            let new_contents = current_contents + backprop_value;
            self.contents.insert(parent_dir.to_string(), new_contents);
        }
    }
}

#[test]
fn text_examples() {
    static TEST_DATA: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    assert_eq!(sum_dirs_part_one(TEST_DATA), 95437);
    assert_eq!(smallest_to_delete_part_two(TEST_DATA), 24933642);
}
