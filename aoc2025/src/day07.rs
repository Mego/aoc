use grid::Grid;
use itertools::Itertools;
use rustc_hash::FxHashSet;

use util::{
    coordinate::Coordinate, direction::Direction, gridtools::IsValidIndex,
    pathfinding::par_multi_count_paths,
};

fn parse(input: &str) -> (Grid<u8>, Coordinate) {
    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(i, line)| line.bytes().position(|c| c == b'S').map(|j| (i, j)))
        .unwrap();
    let cols = input.lines().next().unwrap().len();
    (
        Grid::from_vec(input.lines().flat_map(|line| line.bytes()).collect(), cols),
        start_pos.into(),
    )
}

pub fn part1(input: &str) -> impl ToString {
    let (g, start_pos) = parse(input);
    let mut positions = FxHashSet::from_iter([start_pos]);
    let mut splitters_hit = FxHashSet::default();
    while !positions.is_empty() {
        positions = positions
            .into_iter()
            .flat_map(|p| {
                (if g[p] == b'^' {
                    splitters_hit.insert(p);
                    vec![Direction::Left.move_dir(p), Direction::Right.move_dir(p)]
                } else {
                    vec![Direction::Down.move_dir(p)]
                })
                .into_iter()
                .filter(|&np| g.is_valid_index(np))
                .map(|np| np.into())
            })
            .collect();
    }
    splitters_hit.len()
}

pub fn part2(input: &str) -> impl ToString {
    let (g, start_pos) = parse(input);
    let last_row_x = g.rows() - 1;
    par_multi_count_paths(
        start_pos,
        |&p| {
            (if g[p] == b'^' {
                vec![Direction::Left.move_dir(p), Direction::Right.move_dir(p)]
            } else {
                vec![Direction::Down.move_dir(p)]
            })
            .into_iter()
            .filter(|&np| g.is_valid_index(np))
            .map(|np| np.into())
        },
        (0..g.cols())
            .map(|y| Coordinate::from((last_row_x, y)))
            .collect_vec(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 7;
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
