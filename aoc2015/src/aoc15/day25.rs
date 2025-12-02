use std::sync::LazyLock;

use fancy_regex::Regex;
use itertools::Itertools;

static COORDS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

fn next_pos((row, col): (usize, usize)) -> (usize, usize) {
    if row == 1 {
        (col + 1, 1)
    } else {
        (row - 1, col + 1)
    }
}

fn next_value(value: u64) -> u64 {
    (value * 252533) % 33554393
}

pub fn part1(input: String) -> u64 {
    let goal = COORDS_REGEX
        .find_iter(&input)
        .map(|s| s.unwrap().as_str().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut pos = (6, 6);
    let mut curr = 27995004;
    while pos != goal {
        curr = next_value(curr);
        pos = next_pos(pos);
    }
    curr
}
pub fn part2(_: String) -> u64 {
    unimplemented!("day 25 part 2 is free")
}
