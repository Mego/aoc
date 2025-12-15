use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[cached]
fn update_stone(stone: u64, count: u8) -> u64 {
    if count == 0 {
        return 1;
    }
    if stone == 0 {
        return update_stone(1, count - 1);
    } else {
        let digits = stone.ilog10() + 1;
        if digits.is_multiple_of(2) {
            return update_stone(stone / 10u64.pow(digits / 2), count - 1)
                + update_stone(stone % 10u64.pow(digits / 2), count - 1);
        } else {
            return update_stone(stone * 2024, count - 1);
        }
    }
}

pub fn part1(input: &str) -> impl ToString {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    stones
        .into_par_iter()
        .map(|stone| update_stone(stone, 25))
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl ToString {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    stones
        .into_par_iter()
        .map(|stone| update_stone(stone, 75))
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
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
