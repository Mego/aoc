use itertools::Itertools;

pub fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap()
        })
        .unzip()
}

pub fn part1(input: &str) -> impl ToString {
    let (mut a, mut b) = parse(input);
    a.sort();
    b.sort();
    a.iter().zip(b).map(|(&a, b)| a.abs_diff(b)).sum::<u64>()
}

pub fn part2(input: &str) -> impl ToString {
    let (a, b) = parse(input);
    a.iter()
        .map(|x| b.iter().filter(|&y| x == y).count() * (*x as usize))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 1;
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
