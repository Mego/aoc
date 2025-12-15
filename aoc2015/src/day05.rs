use std::sync::LazyLock;

use fancy_regex::Regex;
use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
    static NAUGHTY_STRINGS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"ab|cd|pq|xy").unwrap());
    input
        .lines()
        .filter(|&line| {
            !NAUGHTY_STRINGS.is_match(line).unwrap()
                && line.matches(['a', 'e', 'i', 'o', 'u']).count() >= 3
                && line.chars().tuple_windows().any(|(a, b)| a == b)
        })
        .count()
}

pub fn part2(input: &str) -> impl ToString {
    static DOUBLE_DOUBLE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(..).*(?=\1)").unwrap());
    static ABA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.).\1").unwrap());
    input
        .lines()
        .filter(|&line| DOUBLE_DOUBLE.is_match(line).unwrap() && ABA.is_match(line).unwrap())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 5;
    static INPUT: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string(util::input_path(YEAR, DAY)).unwrap());

    #[test]
    fn test_part1() {
        assert!(check(YEAR, DAY, 1, &part1(&INPUT).to_string()).unwrap());
    }

    #[test]
    fn test_part2() {
        assert!(check(YEAR, DAY, 2, &part2(&INPUT).to_string()).unwrap());
    }
}
