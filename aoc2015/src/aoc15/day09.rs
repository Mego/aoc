use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1(input: String) -> u64 {
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
pub fn part2(input: String) -> u64 {
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
