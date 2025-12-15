use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
    let lines = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    lines
        .iter()
        .filter(|line| {
            (line.iter().is_sorted() || line.iter().rev().is_sorted())
                && line
                    .windows(2)
                    .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
        })
        .count()
}

pub fn part2(input: &str) -> impl ToString {
    let lines = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    lines
        .iter()
        .filter(|line| {
            line.iter().combinations(line.len() - 1).any(|c| {
                (c.iter().is_sorted() || c.iter().rev().is_sorted())
                    && c.windows(2)
                        .all(|w| w[0].abs_diff(*w[1]) >= 1 && w[0].abs_diff(*w[1]) <= 3)
            })
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
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
