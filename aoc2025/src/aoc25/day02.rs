use std::sync::LazyLock;

use fancy_regex::Regex;
use itertools::Itertools;

static INVALID_REGEX_P1: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([1-9][0-9]*)\1$").unwrap());

static INVALID_REGEX_P2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([1-9][0-9]*)\1+$").unwrap());

fn is_invalid_p1(input: u64) -> bool {
    INVALID_REGEX_P1.is_match(&input.to_string()).unwrap()
}

fn is_invalid_p2(input: u64) -> bool {
    INVALID_REGEX_P2.is_match(&input.to_string()).unwrap()
}

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = u64>> {
    input.split(',').map(|range| {
        let (a, b) = range
            .split('-')
            .map(|x| {
                x.parse()
                    .unwrap_or_else(|_| panic!("{x} should be a valid int literal"))
            })
            .collect_tuple()
            .unwrap();
        a..=b
    })
}

pub fn part1(input: &str) -> u64 {
    parse(input.trim())
        .flatten()
        .filter(|&x| is_invalid_p1(x))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse(input.trim())
        .flatten()
        .filter(|&x| is_invalid_p2(x))
        .sum()
}
