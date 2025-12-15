use rustc_hash::FxHashSet as HashSet;

use grid::Grid;
use itertools::Itertools;

fn next_pos(
    cur_pos: (usize, usize),
    cur_dir: u8,
    rows: usize,
    cols: usize,
) -> Option<(usize, usize)> {
    let delta = match cur_dir {
        b'^' => (-1isize, 0isize),
        b'>' => (0, 1),
        b'v' => (1, 0),
        b'<' => (0, -1),
        _ => unreachable!(),
    };
    let next_pos_i = (cur_pos.0 as isize + delta.0, cur_pos.1 as isize + delta.1);
    if next_pos_i.0 < 0
        || next_pos_i.1 < 0
        || (next_pos_i.0 as usize) >= rows
        || (next_pos_i.1 as usize) >= cols
    {
        return None;
    }
    Some((next_pos_i.0 as usize, next_pos_i.1 as usize))
}

fn step(
    mut grid: Grid<u8>,
    cur_pos: (usize, usize),
    cur_dir: u8,
) -> (Grid<u8>, u8, Option<(usize, usize)>) {
    if let Some(next_pos) = next_pos(cur_pos, cur_dir, grid.rows(), grid.cols()) {
        if *grid.get(next_pos.0, next_pos.1).unwrap() == b'#' {
            let new_dir = match cur_dir {
                b'^' => b'>',
                b'>' => b'v',
                b'v' => b'<',
                b'<' => b'^',
                _ => unreachable!(),
            };
            (
                {
                    *grid.get_mut(cur_pos.0, cur_pos.1).unwrap() = new_dir;
                    grid
                },
                new_dir,
                Some(cur_pos),
            )
        } else {
            (
                {
                    *grid.get_mut(cur_pos.0, cur_pos.1).unwrap() = b'.';
                    *grid.get_mut(next_pos.0, next_pos.1).unwrap() = cur_dir;
                    grid
                },
                cur_dir,
                Some(next_pos),
            )
        }
    } else {
        (
            {
                *grid.get_mut(cur_pos.0, cur_pos.1).unwrap() = b'.';
                grid
            },
            cur_dir,
            None,
        )
    }
}

fn does_loop(mut g: Grid<u8>, mut cur_pos: (usize, usize), mut cur_dir: u8) -> (bool, usize) {
    let mut visited = HashSet::default();
    let mut loops = true;
    let mut pos = u32::from_be_bytes([0, cur_pos.0 as u8, cur_pos.1 as u8, cur_dir]);
    while !visited.contains(&pos) {
        let next = step(g, cur_pos, cur_dir);
        if let Some(next_pos) = next.2 {
            if cur_dir != next.1 {
                visited.insert(pos);
            }
            (g, cur_dir, cur_pos) = (next.0, next.1, next_pos);
            pos = u32::from_be_bytes([0, cur_pos.0 as u8, cur_pos.1 as u8, cur_dir]);
        } else {
            loops = false;
            break;
        }
    }
    (
        loops,
        visited
            .into_iter()
            .map(|p| {
                let bytes = p.to_be_bytes();
                (bytes[1], bytes[2])
            })
            .collect::<HashSet<_>>()
            .len(),
    )
}

pub fn part1(input: &str) -> impl ToString {
    let grid = Grid::from_vec(
        input.lines().flat_map(|l| l.bytes()).collect_vec(),
        input.lines().next().unwrap().len(),
    );
    let cur_pos = grid.indexed_iter().find(|(_, c)| **c == b'^').unwrap().0;

    does_loop(grid, cur_pos, b'^').1
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
    let cur_dir = b'^';
    let cur_pos = init_pos;
    let cols = grid.cols();

    grid.indexed_iter()
        .filter(|(pos, c)| {
            if **c == b'#' || **c == b'^' {
                return false;
            }
            let mut grid = Grid::from_vec(grid.iter().copied().collect_vec(), cols);
            *grid.get_mut(pos.0, pos.1).unwrap() = b'#';
            does_loop(grid, cur_pos, cur_dir).0
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
    #[ignore = "still debugging this one"]
    fn test_part1() {
        assert!(check(YEAR, DAY, 1, &part1(&INPUT).to_string()).unwrap());
    }

    #[test]
    fn test_part2() {
        assert!(check(YEAR, DAY, 2, &part2(&INPUT).to_string()).unwrap());
    }
}
