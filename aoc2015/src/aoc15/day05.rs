use std::sync::LazyLock;

use fancy_regex::Regex;

pub fn part1(input: String) -> u64 {
    static NAUGHTY_STRINGS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"ab|cd|pq|xy").unwrap());
    static DOUBLE_LETTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".{2,}").unwrap());
    input
        .lines()
        .filter(|&line| {
            !NAUGHTY_STRINGS.is_match(line).unwrap()
                && line.matches(['a', 'e', 'i', 'o', 'u']).count() >= 3
                && DOUBLE_LETTER.is_match(line).unwrap()
        })
        .count() as u64
}
pub fn part2(input: String) -> u64 {
    static DOUBLE_DOUBLE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(..).*(?=\1)").unwrap());
    static ABA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.).\1").unwrap());
    input
        .lines()
        .filter(|&line| DOUBLE_DOUBLE.is_match(line).unwrap() && ABA.is_match(line).unwrap())
        .count() as u64
}
