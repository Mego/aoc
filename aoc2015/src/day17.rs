use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn part1(input: &str) -> impl ToString {
    const N: usize = 150;
    let sizes = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();
    (0..sizes.len())
        .flat_map(|k| sizes.iter().combinations(k))
        .par_bridge()
        .filter(|c| c.iter().copied().sum::<usize>() == N)
        .count()
}

pub fn part2(input: &str) -> impl ToString {
    const N: usize = 150;
    let sizes = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();
    (0..sizes.len())
        .find_map(|k| {
            let combs = sizes.iter().combinations(k);
            let res = combs
                .filter(|c| c.iter().copied().sum::<usize>() == N)
                .count();
            (res > 0).then_some(res)
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 17;
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
