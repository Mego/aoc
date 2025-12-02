use itertools::{Itertools, max};

fn is_triangle(a: u16, b: u16, c: u16) -> bool {
    a + b + c > max([a, b, c]).unwrap() * 2
}

pub fn part1(input: String) -> usize {
    input
        .lines()
        .filter(|l| {
            let (a, b, c) = l
                .split_ascii_whitespace()
                .map(|x| x.parse::<u16>().unwrap())
                .collect_tuple()
                .unwrap();
            is_triangle(a, b, c)
        })
        .count()
}
pub fn part2(input: String) -> usize {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.split_ascii_whitespace()
                .enumerate()
                .map(move |(col, x)| ((row / 3, col), x.parse::<u16>().unwrap()))
        })
        .into_group_map()
        .values()
        .filter(|v| is_triangle(v[0], v[1], v[2]))
        .count()
}
