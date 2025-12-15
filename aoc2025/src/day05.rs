use itertools::Itertools;

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ids) = input.split("\n\n").collect_tuple().unwrap();
    (
        ranges
            .lines()
            .map(|line| {
                let (a, b) = line
                    .split('-')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                (a, b)
            })
            .collect_vec(),
        ids.lines().map(|line| line.parse().unwrap()).collect_vec(),
    )
}

fn parse2(input: &str) -> Vec<(u64, u64)> {
    let ranges = input.split("\n\n").next().unwrap();

    ranges
        .lines()
        .map(|line| {
            line.split('-')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .sorted_unstable()
        .collect_vec()
}

pub fn part1(input: &str) -> impl ToString {
    let (ranges, ids) = parse(input);
    ids.into_iter()
        .filter(|&id| ranges.iter().any(|&(a, b)| a <= id && id <= b))
        .count()
}

pub fn part2(input: &str) -> impl ToString {
    let ranges = parse2(input);
    let mut ranges_iter = ranges.into_iter();
    let mut new_ranges = vec![ranges_iter.next().unwrap()];
    for (mut c, mut d) in ranges_iter {
        let (a, b) = *new_ranges.last().unwrap();
        if c <= b + 1 {
            new_ranges.pop();
            c = a.min(c);
            d = b.max(d);
        }
        new_ranges.push((c, d));
    }
    new_ranges
        .into_iter()
        .fold(0, |acc, (a, b)| acc + (b - a + 1))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 5;
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
