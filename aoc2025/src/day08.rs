use itertools::Itertools;
use petgraph::prelude::*;
use rustc_hash::FxHashMap;

fn parse(input: &str) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

pub fn part1(input: &str) -> impl ToString {
    const NUM_CONNECTIONS: usize = 1000;

    let nodes = parse(input);
    let mut node_map = FxHashMap::default();
    let mut g = UnGraph::new_undirected();
    for node in nodes {
        node_map.insert(g.add_node(node), node);
    }
    node_map
        .keys()
        .combinations(2)
        .sorted_unstable_by_key(|idxs| {
            let a = node_map[idxs[0]];
            let b = node_map[idxs[1]];
            a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)
        })
        .take(NUM_CONNECTIONS)
        .for_each(|idxs| {
            g.add_edge(*idxs[0], *idxs[1], ());
        });

    use pathfinding::prelude::connected_components;
    connected_components(&node_map.keys().copied().collect_vec(), |&idx| {
        g.neighbors(idx)
    })
    .into_iter()
    .map(|cc| -(cc.len() as isize))
    .sorted_unstable()
    .take(3)
    .map(|x| -x as usize)
    .product::<usize>()
}

pub fn part2(input: &str) -> impl ToString {
    let nodes = parse(input);
    let mut node_map = FxHashMap::default();
    let mut g = UnGraph::new_undirected();
    for node in nodes {
        node_map.insert(g.add_node(node), node);
    }
    let mut cnx_iter = node_map
        .keys()
        .combinations(2)
        .sorted_unstable_by_key(|idxs| {
            let a = node_map[idxs[0]];
            let b = node_map[idxs[1]];
            a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)
        });

    use petgraph::algo::connected_components;
    let mut last_cnx_x_prod = None;
    while connected_components(&g) > 1 {
        let cnx = cnx_iter.next().unwrap();
        g.add_edge(*cnx[0], *cnx[1], ());
        last_cnx_x_prod = Some(node_map[cnx[0]].0 * node_map[cnx[1]].0);
    }
    last_cnx_x_prod.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 8;
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
