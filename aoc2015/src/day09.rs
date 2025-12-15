use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1(input: &str) -> impl ToString {
    let mut nodes = FxHashSet::default();
    let mut edges = FxHashMap::default();
    for line in input.lines() {
        let (from, r) = line.split_once(" to ").unwrap();
        let (to, cost) = r.split_once(" = ").unwrap();
        let cost: u64 = cost.parse().unwrap();
        nodes.insert(from);
        nodes.insert(to);
        edges.insert((from, to), cost);
        edges.insert((to, from), cost);
    }
    let n = nodes.len();
    nodes
        .into_iter()
        .permutations(n)
        .map(|p| p.windows(2).map(|w| edges[&(w[0], w[1])]).sum::<u64>())
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    let mut nodes = FxHashSet::default();
    let mut edges = FxHashMap::default();
    for line in input.lines() {
        let (from, r) = line.split_once(" to ").unwrap();
        let (to, cost) = r.split_once(" = ").unwrap();
        let cost: u64 = cost.parse().unwrap();
        nodes.insert(from);
        nodes.insert(to);
        edges.insert((from, to), cost);
        edges.insert((to, from), cost);
    }
    let n = nodes.len();
    nodes
        .into_iter()
        .permutations(n)
        .map(|p| p.windows(2).map(|w| edges[&(w[0], w[1])]).sum::<u64>())
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
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
