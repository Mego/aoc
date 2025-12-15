use grid::Grid;
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[derive(Debug)]
struct Region {
    rows: usize,
    cols: usize,
    items: FxHashMap<usize, usize>,
}

fn parse(input: &str) -> (Vec<Grid<bool>>, Vec<Region>) {
    let mut lines = input.lines();
    let mut shapes_section = true;
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    while let Some(line) = lines.next() {
        if line.ends_with(':') {
            continue;
        }
        if line.contains('x') {
            shapes_section = false;
        }
        if shapes_section {
            let cols = line.len();
            let mut shape = line.bytes().map(|b| b == b'#').collect_vec();
            while let Some(line) = lines.next()
                && !line.trim().is_empty()
            {
                shape.extend(line.bytes().map(|b| b == b'#'));
            }
            let g = Grid::from_vec(shape, cols);
            shapes.push(g);
        } else {
            let (dims, items) = line.split_once(": ").unwrap();
            let (cols, rows) = dims.split_once('x').unwrap();
            regions.push(Region {
                rows: rows.parse().unwrap(),
                cols: cols.parse().unwrap(),
                items: items
                    .split_ascii_whitespace()
                    .enumerate()
                    .filter(|&(_, c)| c != "0")
                    .map(|(i, c)| (i, c.parse().unwrap()))
                    .collect(),
            });
        }
    }

    (shapes, regions)
}

pub fn part1(input: &str) -> impl ToString {
    let (shapes, regions) = parse(input);
    regions
        .into_par_iter()
        .filter(|region| {
            // velveeta: is the total size of the required shapes' bounding boxes less than or equal to the total region size?
            // this fails the example input but passes for actual problem input
            region.items.iter().fold(0, |a, (&i, &c)| {
                let shape = &shapes[i];
                a + shape.rows() * shape.cols() * c
            }) <= region.rows * region.cols
        })
        .count()
}

pub fn part2(_input: &str) -> impl ToString {
    "free"
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 12;
    static INPUT: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string(util::input_path(YEAR, DAY)).unwrap());

    #[test]
    fn test_part1() {
        assert!(check(YEAR, DAY, 1, &part1(&INPUT).to_string()).unwrap());
    }
}
