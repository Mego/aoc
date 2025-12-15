use itertools::Itertools;
use rayon::prelude::*;

pub fn part1(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| {
            line.chars()
                .combinations(2)
                .map(|c| c.into_iter().join("").parse::<u64>().unwrap())
                .max()
                .unwrap()
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl ToString {
    let lines = input.lines().collect_vec();
    let n = lines[0].len() - 12;
    lines
        .into_par_iter()
        .map(|line| {
            (0..12)
                .fold((0, 0), |(a, b), j| {
                    let (i, x) = line[b..=n + j]
                        .bytes()
                        .enumerate()
                        .max_by_key(|&(idx, x)| (x, -(idx as isize)))
                        .unwrap();
                    (a * 10 + (x - b'0') as u64, b + i + 1)
                })
                .0
        })
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 3;
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
