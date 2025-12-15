use petgraph::prelude::*;
use rustc_hash::FxHashMap;

fn parse(input: &str) -> (DiGraph<&str, ()>, FxHashMap<&str, NodeIndex>) {
    let mut g = DiGraph::new();
    let mut node_idxs = FxHashMap::default();
    let nodes: FxHashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (from, tos) = line.split_once(':').unwrap();
            let from = from.trim();
            (
                from,
                tos.trim()
                    .split_ascii_whitespace()
                    .map(|to| to.trim())
                    .collect(),
            )
        })
        .collect();
    nodes.iter().for_each(|(&from, tos)| {
        tos.iter().for_each(|to| {
            let from_idx = *node_idxs.entry(from).or_insert_with(|| g.add_node(from));
            let to_idx = *node_idxs.entry(to).or_insert_with(|| g.add_node(*to));
            g.add_edge(from_idx, to_idx, ());
        });
    });

    (g, node_idxs)
}

// pathfinding's count_paths fn is much faster than petgraph's simple_paths fn, but petgraph's graphs are more convenient to build
fn count_paths(g: &Graph<&str, ()>, from: NodeIndex, to: NodeIndex) -> usize {
    pathfinding::prelude::count_paths(from, |&n| g.neighbors(n), |&n| n == to)
}

pub fn part1(input: &str) -> impl ToString {
    let (g, nodes) = parse(input);
    count_paths(&g, nodes["you"], nodes["out"])
}

pub fn part2(input: &str) -> impl ToString {
    let (g, nodes) = parse(input);
    let svr = nodes["svr"];
    let out = nodes["out"];
    let dac = nodes["dac"];
    let fft = nodes["fft"];
    (count_paths(&g, svr, dac) * count_paths(&g, dac, fft) * count_paths(&g, fft, out))
        + (count_paths(&g, svr, fft) * count_paths(&g, fft, dac) * count_paths(&g, dac, out))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 11;
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
