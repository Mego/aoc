use std::sync::LazyLock;

use fancy_regex::Regex;
use itertools::Itertools;
use rustc_hash::FxHashSet;

const ABBA: &str = r"([a-z])(?!\1)([a-z])\2\1";
static TLS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(ABBA).unwrap());
static TLS_NEG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"\[\w*{ABBA}\w*\]")).unwrap());

fn validate(s: &&str) -> bool {
    TLS_REGEX.is_match(s).unwrap() && !TLS_NEG_REGEX.is_match(s).unwrap()
}

fn validate2(s: &&str) -> bool {
    let chars = s.chars().collect_vec();
    let mut i = 0;
    let mut aba_seqs = FxHashSet::default();
    let mut bracket_positions = vec![];
    while i + 2 < chars.len() {
        if chars[i] == '[' {
            let mut j = i + 1;
            while j < chars.len() && chars[j] != ']' {
                j += 1;
            }
            bracket_positions.push((i, j));
            i = j + 1;
        }
        if chars[i] == chars[i + 2] && chars[i + 1] != chars[i] {
            aba_seqs.insert((chars[i], chars[i + 1]));
        }
        i += 1;
    }
    for (i, j) in bracket_positions {
        let contents = &chars[i + 1..j];
        if contents
            .windows(3)
            .any(|cs| cs[2] == cs[0] && cs[0] != cs[1] && aba_seqs.contains(&(cs[1], cs[0])))
        {
            return true;
        }
    }
    false
}

pub fn part1(input: &str) -> impl ToString {
    input.lines().filter(validate).count()
}

pub fn part2(input: &str) -> impl ToString {
    input.lines().filter(validate2).count()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2016;
    const DAY: u8 = 7;
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
