use atoi::FromRadix10;
use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
    let s = input
        .bytes()
        .filter(|c| !c.is_ascii_whitespace())
        .collect_vec();
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'(' {
            let end = {
                let mut j = i + 1;
                while s[j] != b')' {
                    j += 1;
                }
                j + 1
            };
            let (k, n) = s[i + 1..end - 1]
                .split(|&c| c == b'x')
                .map(|x| usize::from_radix_10(x).0)
                .collect_tuple()
                .unwrap();

            count += k * n - 1;
            i = end + k - 1;
        }
        count += 1;
        i += 1;
    }

    count
}

fn calc_part2(s: &[u8]) -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'(' {
            let end = {
                let mut j = i + 1;
                while s[j] != b')' {
                    j += 1;
                }
                j + 1
            };
            let (k, n) = s[i + 1..end - 1]
                .split(|&c| c == b'x')
                .map(|x| usize::from_radix_10(x).0)
                .collect_tuple()
                .unwrap();

            count += n * calc_part2(&s[end..end + k]) - 1;
            i = end + k - 1;
        }
        count += 1;
        i += 1;
    }

    count
}

pub fn part2(input: &str) -> impl ToString {
    let s = input
        .bytes()
        .filter(|c| !c.is_ascii_whitespace())
        .collect_vec();
    calc_part2(&s)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2016;
    const DAY: u8 = 9;
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
