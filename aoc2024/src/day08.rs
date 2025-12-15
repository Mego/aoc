use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;

fn antinode_locs(
    a: (usize, usize),
    b: (usize, usize),
    rows: usize,
    cols: usize,
    max_mul: usize,
) -> Vec<(usize, usize)> {
    let mut locs = Vec::new();

    [1, -1].into_iter().for_each(|d| {
        let mut mul = 1;
        while mul <= max_mul {
            let diff = (
                ((a.0 as isize) - (b.0 as isize)) * d * mul as isize,
                ((a.1 as isize) - (b.1 as isize)) * d * mul as isize,
            );
            let mut c = ((a.0 as isize) + diff.0, (a.1 as isize) + diff.1);
            if (c.0 as usize, c.1 as usize) == b {
                c = ((b.0 as isize) + diff.0, (b.1 as isize) + diff.1);
            }
            if c.0 >= 0 && (c.0 as usize) < rows && c.1 >= 0 && (c.1 as usize) < cols {
                locs.push((c.0 as usize, c.1 as usize));
                mul += 1;
            } else {
                break;
            }
        }
    });

    locs
}

pub fn part1(input: &str) -> impl ToString {
    let g = {
        Grid::from_vec(
            input.lines().flat_map(|l| l.chars()).collect_vec(),
            input.find("\n").unwrap(),
        )
    };
    let antenna_locs = g
        .indexed_iter()
        .filter_map(|(i, c)| c.is_ascii_alphanumeric().then_some(i))
        .collect_vec();

    let rows = g.rows();
    let cols = g.cols();
    let antinode_locs = antenna_locs
        .iter()
        .cartesian_product(antenna_locs.iter())
        .filter(|&(&a, &b)| a != b && *g.get(a.0, a.1).unwrap() == *g.get(b.0, b.1).unwrap())
        .flat_map(|(&a, &b)| antinode_locs(a, b, rows, cols, 1))
        .collect::<HashSet<_>>();
    antinode_locs.len()
}

pub fn part2(input: &str) -> impl ToString {
    let g = {
        Grid::from_vec(
            input.lines().flat_map(|l| l.chars()).collect_vec(),
            input.find("\n").unwrap(),
        )
    };
    let antenna_locs = g
        .indexed_iter()
        .filter_map(|(i, c)| c.is_ascii_alphanumeric().then_some(i))
        .collect_vec();

    let rows = g.rows();
    let cols = g.cols();
    let mut antinode_locs = antenna_locs
        .iter()
        .cartesian_product(antenna_locs.iter())
        .filter(|&(&a, &b)| a != b && *g.get(a.0, a.1).unwrap() == *g.get(b.0, b.1).unwrap())
        .flat_map(|(&a, &b)| antinode_locs(a, b, rows, cols, 50))
        .collect::<HashSet<_>>();
    antenna_locs.iter().copied().for_each(|loc| {
        antinode_locs.insert(loc);
    });
    antinode_locs.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
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
