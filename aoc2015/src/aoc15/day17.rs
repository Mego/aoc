use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn part1(input: String) -> usize {
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
pub fn part2(input: String) -> usize {
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
