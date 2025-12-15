use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
    let packages: Vec<u16> = input.lines().map(|l| l.parse().unwrap()).collect();
    let goal: u16 = packages.iter().copied().sum::<u16>() / 3;
    (0..packages.len())
        .filter_map(|k| {
            packages
                .iter()
                .combinations(k)
                .filter_map(|c| {
                    (c.iter().copied().sum::<u16>() == goal)
                        .then_some(c.iter().map(|x| **x as u64).product::<u64>())
                })
                .min()
        })
        .next()
        .unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    let packages: Vec<u16> = input.lines().map(|l| l.parse().unwrap()).collect();
    let goal: u16 = packages.iter().copied().sum::<u16>() / 4;
    (0..packages.len())
        .filter_map(|k| {
            packages
                .iter()
                .combinations(k)
                .filter_map(|c| {
                    (c.iter().copied().sum::<u16>() == goal)
                        .then_some(c.iter().map(|x| **x as u64).product::<u64>())
                })
                .min()
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 24;
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
