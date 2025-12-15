use itertools::Itertools;

fn is_invalid_p1(input: u64) -> bool {
    let n = input.to_string();
    if n.len().is_multiple_of(2) {
        let mid = n.len() / 2;
        n[..mid] == n[mid..]
    } else {
        false
    }
}

fn is_invalid_p2(input: u64) -> bool {
    let n = input.to_string();
    for i in 1..=(n.len() / 2) {
        let repeats = n.len() / i;
        if n[..i].repeat(repeats) == n {
            return true;
        }
    }
    false
}

fn parse(input: &str) -> impl Iterator<Item = u64> {
    input.split(',').flat_map(|range| {
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

pub fn part1(input: &str) -> impl ToString {
    parse(input.trim())
        .filter(|&x| is_invalid_p1(x))
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl ToString {
    parse(input.trim())
        .filter(|&x| is_invalid_p2(x))
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 2;
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
