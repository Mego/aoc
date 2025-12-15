use itertools::{Itertools, max};

fn is_triangle(a: u16, b: u16, c: u16) -> bool {
    a + b + c > max([a, b, c]).unwrap() * 2
}

pub fn part1(input: &str) -> impl ToString {
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

pub fn part2(input: &str) -> impl ToString {
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

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2016;
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
