use itertools::Itertools;

fn next_str(s: &str) -> String {
    let mut res = s.bytes().rev().collect_vec();
    for c in res.iter_mut() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            break;
        }
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

fn check(s: &str) -> bool {
    let mut straight = false;
    let mut pair = 0;
    let bytes = s.bytes().collect_vec();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b'i' || c == b'o' || c == b'l' {
            return false;
        }
        if i + 2 < bytes.len() && bytes[i + 1] == c + 1 && bytes[i + 2] == bytes[i + 1] + 1 {
            straight = true;
        }
        if i + 1 < bytes.len() && bytes[i + 1] == c && (i == 0 || bytes[i - 1] != c) {
            pair += 1;
        }
        if straight && pair >= 2 {
            return !s.contains(['i', 'o', 'l']);
        }
    }
    straight && pair >= 2
}

pub fn part1(input: &str) -> impl ToString {
    let mut next = next_str(input);
    while !check(&next) {
        next = next_str(&next);
    }
    next
}

pub fn part2(input: &str) -> impl ToString {
    let mut next = next_str(input);
    while !check(&next) {
        next = next_str(&next);
    }
    next = next_str(&next);
    while !check(&next) {
        next = next_str(&next);
    }
    next
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 11;
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
