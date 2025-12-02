use std::sync::LazyLock;

use itertools::Itertools;
use regex_lite::Regex;

static ROOM_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^((?:[a-z]+-?)+)-(\d+)\[([a-z]{5})\]$").unwrap());

fn parse(input: &str) -> (&str, u64, &str) {
    let caps = ROOM_REGEX.captures(input).unwrap();
    (
        caps.get(1).unwrap().as_str(),
        caps.get(2).unwrap().as_str().parse().unwrap(),
        caps.get(3).unwrap().as_str(),
    )
}

fn validate(room: &(&str, u64, &str)) -> bool {
    room.0
        .chars()
        .filter(|&c| c != '-')
        .counts()
        .into_iter()
        .sorted_by_key(|&(c, n)| (-(n as isize), c))
        .map(|(c, _)| c)
        .take(5)
        .join("")
        == room.2
}

fn decrypt(room: (&str, u64, &str)) -> (String, u64) {
    (
        String::from_utf8(
            room.0
                .bytes()
                .map(|c| match c {
                    b'-' => b' ',
                    _ => b'a' + (((c - b'a') as u64 + room.1) % 26) as u8,
                })
                .collect(),
        )
        .unwrap(),
        room.1,
    )
}

pub fn part1(input: String) -> u64 {
    input
        .lines()
        .map(parse)
        .filter(validate)
        .fold(0, |a, (_, x, _)| a + x)
}
pub fn part2(input: String) -> u64 {
    input
        .lines()
        .map(parse)
        .filter(validate)
        .map(decrypt)
        .find(|(s, _)| s.starts_with("northpole"))
        .unwrap()
        .1
}
