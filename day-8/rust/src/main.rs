use itertools::iproduct;
use std::include_str;

fn main() {
    static DATA: &str = include_str!("data.txt");
    let (width, height, trees, treepose) = read_string_to_vector(DATA);

    println!(
        "There are {} trees visible from outside the grid.",
        part_one(&trees, &width, &height)
    );
    println!(
        "The highest scenic score possible is {}.",
        part_two(&trees, &treepose, &width, &height)
    );
}

fn part_one(trees: &Vec<Vec<usize>>, width: &usize, height: &usize) -> usize {
    iproduct!(1..width - 1, 1..height - 1)
        .filter(|&(x, y)| {
            ((0..y).map(|i| trees[x][i]).max().unwrap_or(0) < trees[x][y])
                || ((y + 1..*height).map(|i| trees[x][i]).max().unwrap_or(0) < trees[x][y])
                || ((0..x).map(|i| trees[i][y]).max().unwrap_or(0) < trees[x][y])
                || ((x + 1..*width).map(|i| trees[i][y]).max().unwrap_or(0) < trees[x][y])
        })
        .count()
        + 2 * width
        + 2 * height
        - 4
}

fn part_two(
    trees: &Vec<Vec<usize>>,
    treepose: &Vec<Vec<usize>>,
    width: &usize,
    height: &usize,
) -> usize {
    iproduct!(1..width - 1, 1..height - 1)
        .map(|(x, y)| {
            [treewalker(&trees, &x, &y), treewalker(&treepose, &y, &x)]
                .into_iter()
                .flatten()
                .product::<usize>()
        })
        .max()
        .unwrap()
}

fn treewalker(data: &Vec<Vec<usize>>, x: &usize, y: &usize) -> [usize; 2] {
    let (left_side, right_side_raw) = data[*y].split_at(*x);
    let current_height = right_side_raw[0];
    let right_side = &right_side_raw[1..];
    let left_option = left_side.iter().rev().position(|h| h >= &current_height);
    let left_index = match left_option {
        Some(x) => x + 1,
        None => left_side.len(),
    };
    let right_option = right_side.iter().position(|h| h >= &current_height);
    let right_index = match right_option {
        Some(x) => x + 1,
        None => right_side.len(),
    };
    [left_index, right_index]
}

fn read_string_to_vector(data: &str) -> (usize, usize, Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let tree_array: Vec<Vec<usize>> = data
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.chars()
                .into_iter()
                .map(|n| n.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let transpose: Vec<Vec<usize>> = (0..tree_array.len())
        .map(|y| {
            (0..tree_array[0].len())
                .map(|x| tree_array[x][y])
                .collect::<Vec<usize>>()
        })
        .collect();
    (tree_array[0].len(), tree_array.len(), tree_array, transpose)
}

#[test]
fn test_examples() {
    static TEST_DATA: &str = r#"
30373
25512
65332
33549
35390
"#;

    let (width, height, trees, treepose) = read_string_to_vector(TEST_DATA);
    assert_eq!(part_one(&trees, &width, &height), 21);
    assert_eq!(part_two(&trees, &treepose, &width, &height), 8);
}
