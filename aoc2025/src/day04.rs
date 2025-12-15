use grid::Grid;
use itertools::Itertools;

use util::gridtools::adj_squares8;

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

pub fn part1(input: &str) -> impl ToString {
    let g = parse(input);
    g.indexed_iter()
        .filter(|&(i, &x)| x && adj_squares8(&g, i).iter().filter(|&x| x).count() < 4)
        .count()
}

pub fn part2(input: &str) -> impl ToString {
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

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 4;
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
