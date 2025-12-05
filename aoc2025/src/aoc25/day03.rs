use itertools::Itertools;
use rayon::prelude::*;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .combinations(2)
                .map(|c| c.into_iter().join("").parse::<u64>().unwrap())
                .max()
                .unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
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
        .sum()
}
