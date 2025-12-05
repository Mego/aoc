use grid::Grid;
use itertools::Itertools;

use crate::util::gridtools::adj_squares8;

fn parse(input: &str) -> Grid<bool> {
    let cols = input.find("\n").unwrap();
    Grid::from_vec(
        input
            .lines()
            .flat_map(|line| line.bytes().map(|b| b == b'@'))
            .collect(),
        cols,
    )
}

pub fn part1(input: &str) -> usize {
    let g = parse(input);
    g.indexed_iter()
        .filter(|&(i, &x)| x && adj_squares8(&g, i).iter().filter(|&x| x).count() < 4)
        .count()
}

pub fn part2(input: &str) -> u64 {
    let mut g = parse(input);
    let mut count = 0;
    loop {
        let mut removed = false;
        for i in g
            .indexed_iter()
            .filter(|&(i, &x)| x && adj_squares8(&g, i).iter().filter(|&x| x).count() < 4)
            .map(|(i, _)| i)
            .collect_vec()
        {
            g[i] = false;
            removed = true;
            count += 1;
        }
        if !removed {
            break;
        }
    }
    count
}
