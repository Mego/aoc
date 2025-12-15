use itertools::Itertools;
use rayon::prelude::*;

fn parse(input: &str) -> impl Iterator<Item = (usize, usize)> {
    input.lines().map(|line| {
        let (b, a) = line
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        (a, b)
    })
}

pub fn part1(input: &str) -> impl ToString {
    parse(input)
        .combinations(2)
        .map(|xs| (xs[0].0.abs_diff(xs[1].0) + 1) * (xs[0].1.abs_diff(xs[1].1) + 1))
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    let reds = parse(input).collect_vec();
    reds.iter()
        .combinations(2)
        .par_bridge()
        .filter(|xs| {
            let a = xs[0];
            let b = xs[1];
            let xmin = a.0.min(b.0);
            let xmax = a.0.max(b.0);
            let ymin = a.1.min(b.1);
            let ymax = a.1.max(b.1);
            reds.iter().circular_tuple_windows().all(|(p1, p2)| {
                let pxmin = p1.0.min(p2.0);
                let pxmax = p1.0.max(p2.0);
                let pymin = p1.1.min(p2.1);
                let pymax = p1.1.max(p2.1);
                !(xmin < pxmax && xmax > pxmin && ymin < pymax && ymax > pymin)
            })
        })
        .map(|xs| (xs[0].0.abs_diff(xs[1].0) + 1) * (xs[0].1.abs_diff(xs[1].1) + 1))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 9;
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
