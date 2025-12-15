use grid::Grid;
use itertools::Itertools;

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, usize) {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut height = 0;

    input.split("\n\n").for_each(|d| {
        let first_line = d.lines().next().unwrap();
        let is_lock = first_line.bytes().all(|b| b == b'#');
        let mut d2 = Grid::from_vec(
            d.lines().flat_map(|l| l.bytes()).collect_vec(),
            first_line.len(),
        );
        d2.transpose();
        height = d2.rows();
        let nums = d2
            .iter_rows()
            .map(|r| r.filter(|&&b| b == b'#').count() - 1)
            .collect_vec();
        if is_lock {
            locks.push(nums);
        } else {
            keys.push(nums);
        }
    });

    (locks, keys, height)
}

pub fn part1(input: &str) -> impl ToString {
    let (locks, keys, height) = parse(input);
    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(l, k)| l.iter().zip(k).all(|(a, b)| a + b <= height))
        .count()
}

pub fn part2(_input: &str) -> impl ToString {
    "free"
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 25;
    static INPUT: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string(util::input_path(YEAR, DAY)).unwrap());

    #[test]
    fn test_part1() {
        assert!(check(YEAR, DAY, 1, &part1(&INPUT).to_string()).unwrap());
    }
}
