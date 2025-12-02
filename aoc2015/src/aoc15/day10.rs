use std::sync::LazyLock;

use fancy_regex::Regex;
use itertools::Itertools;

static DIGIT_SEQUENCES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d)\1*").unwrap());

fn look_and_say(n: &str) -> String {
    DIGIT_SEQUENCES
        .find_iter(n)
        .flat_map(|s| {
            let s = s.unwrap().as_str();
            [s.len().to_string(), s[..1].to_string()]
        })
        .join("")
}

pub fn part1(input: String) -> u64 {
    let mut n = input;
    for _ in 0..40 {
        n = look_and_say(&n);
    }
    n.len() as u64
}
pub fn part2(input: String) -> u64 {
    let mut n = input;
    for _ in 0..50 {
        n = look_and_say(&n);
    }
    n.len() as u64
}
