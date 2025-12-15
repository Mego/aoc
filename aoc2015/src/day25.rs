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

pub fn part1(input: &str) -> impl ToString {
    let goal = COORDS_REGEX
        .find_iter(input)
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

pub fn part2(_input: &str) -> impl ToString {
    "free"
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 25;
    static INPUT: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string(util::input_path(YEAR, DAY)).unwrap());

    #[test]
    fn test_part1() {
        assert!(check(YEAR, DAY, 1, &part1(&INPUT).to_string()).unwrap());
    }
}
