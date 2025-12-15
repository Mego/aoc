use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use exhaust::Exhaust;
use itertools::Itertools;
use pathfinding::prelude::astar_bag;
use petgraph::{algo::dijkstra, prelude::*};

use util::{coordinate::Coordinate, direction::Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node(Coordinate, Direction);

impl<T> From<(T, Direction)> for Node
where
    T: Into<Coordinate>,
{
    fn from(value: (T, Direction)) -> Self {
        Self(value.0.into(), value.1)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

fn parse(input: &str) -> (Graph<Node, usize>, NodeIndex, Vec<NodeIndex>) {
    let grid = grid::Grid::from_vec(
        input.lines().flat_map(|l| l.trim().bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let mut g = Graph::new();
    let start: Coordinate = grid
        .indexed_iter()
        .find(|&(_, &c)| c == b'S')
        .unwrap()
        .0
        .into();
    let end: Coordinate = grid
        .indexed_iter()
        .find(|&(_, &c)| c == b'E')
        .unwrap()
        .0
        .into();

    let mut node_indexes: HashMap<Node, NodeIndex> = HashMap::new();
    Direction::exhaust().for_each(|dir| {
        let idx1 = g.add_node((start, dir).into());
        let idx2 = g.add_node((end, dir).into());
        node_indexes.insert((start, dir).into(), idx1);
        node_indexes.insert((end, dir).into(), idx2);
    });
    grid.indexed_iter().for_each(|(pos, &c)| {
        if c != b'#' {
            for dir in Direction::exhaust() {
                let coord = (pos, dir).into();
                let idx = *node_indexes
                    .entry(coord)
                    .or_insert_with(|| g.add_node(coord));
                let next_pos: Coordinate = dir.move_dir(pos.into()).into();
                if grid[next_pos] != b'#' {
                    let coord2 = (next_pos, dir).into();
                    let idx2 = *node_indexes
                        .entry(coord2)
                        .or_insert_with(|| g.add_node(coord2));
                    g.add_edge(idx, idx2, 1);
                }
            }
        }
    });
    node_indexes.iter().for_each(|(&node, &idx)| {
        let idx2 = node_indexes[&(node.0, node.1.cw()).into()];
        g.add_edge(idx, idx2, 1000);
        let idx3 = node_indexes[&(node.0, node.1.cw().opposite()).into()];
        g.add_edge(idx, idx3, 1000);
    });

    let start_node = node_indexes[&(start, Direction::Right).into()];
    let end_nodes = node_indexes
        .iter()
        .filter_map(|(&k, &v)| (k.0 == end).then_some(v))
        .collect_vec();
    (g, start_node, end_nodes)
}

pub fn part1(input: &str) -> impl ToString {
    let (g, start, ends) = parse(input);
    let sol = dijkstra(&g, start, None, |edge| *edge.weight());
    ends.into_iter()
        .filter_map(|end| sol.get(&end).copied())
        .min()
        .unwrap() as u64
}

pub fn part2(input: &str) -> impl ToString {
    let (g, start, ends) = parse(input);
    let res: HashSet<_> = astar_bag(
        &start,
        |&idx| {
            let edges = g.edges(idx);
            edges.map(|edge| (edge.target(), *edge.weight()))
        },
        |&idx| {
            let node = g[idx];
            ends.iter()
                .map(|&end_idx| {
                    let end = g[end_idx];
                    node.0.x.abs_diff(end.0.x) + node.0.y.abs_diff(end.0.y)
                })
                .min()
                .unwrap()
        },
        |idx| ends.contains(idx),
    )
    .unwrap()
    .0
    .flat_map(|p| p.into_iter().map(|idx| g[idx].0))
    .collect();
    res.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 16;
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
