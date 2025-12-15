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

pub fn part1(input: &str) -> impl ToString {
    let mut n = input.to_string();
    for _ in 0..40 {
        n = look_and_say(&n);
    }
    n.len()
}

pub fn part2(input: &str) -> impl ToString {
    let mut n = input.to_string();
    for _ in 0..50 {
        n = look_and_say(&n);
    }
    n.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 10;
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
