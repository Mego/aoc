use rustc_hash::FxHashSet;

use grid::Grid;
use itertools::Itertools;
use util::{coordinate::Coordinate, direction::Direction};

fn step(
    grid: &Grid<u8>,
    cur_pos: Coordinate,
    cur_dir: Direction,
) -> (Direction, Option<Coordinate>) {
    if let Some(next_pos) = cur_dir.try_move_dir_on_grid(cur_pos, grid) {
        if grid[next_pos] == b'#' {
            let new_dir = cur_dir.cw();
            (new_dir, Some(cur_pos))
        } else {
            (cur_dir, Some(next_pos))
        }
    } else {
        (cur_dir, None)
    }
}

fn simulate(g: &Grid<u8>, mut cur_pos: Coordinate, mut cur_dir: Direction) -> (bool, usize) {
    let mut visited = FxHashSet::default();
    let mut loops = true;
    while visited.insert((cur_pos, cur_dir)) {
        let next = step(g, cur_pos, cur_dir);
        if let Some(next_pos) = next.1 {
            (cur_dir, cur_pos) = (next.0, next_pos);
        } else {
            loops = false;
            break;
        }
    }
    (
        loops,
        visited
            .into_iter()
            .map(|(p, _)| p)
            .collect::<FxHashSet<_>>()
            .len(),
    )
}

pub fn part1(input: &str) -> impl ToString {
    let grid = Grid::from_vec(
        input.lines().flat_map(|l| l.bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let cur_pos = grid
        .indexed_iter()
        .find(|(_, c)| **c == b'^')
        .unwrap()
        .0
        .into();

    simulate(&grid, cur_pos, Direction::Up).1
}

pub fn part2(input: &str) -> impl ToString {
    let grid = Grid::from_vec(
        input.lines().flat_map(|l| l.bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let init_pos = grid
        .indexed_iter()
        .find_map(|(i, c)| (*c == b'^').then_some(i))
        .unwrap();
    let cur_dir = Direction::Up;
    let cur_pos = init_pos.into();
    let cols = grid.cols();

    grid.indexed_iter()
        .filter(|(pos, c)| {
            if **c == b'#' || **c == b'^' {
                return false;
            }
            let mut grid = Grid::from_vec(grid.iter().copied().collect_vec(), cols);
            *grid.get_mut(pos.0, pos.1).unwrap() = b'#';
            simulate(&grid, cur_pos, cur_dir).0
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 6;
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
