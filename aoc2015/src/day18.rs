use std::fmt::Debug;

use itertools::Itertools;

const GRID_SIZE: usize = 100;

struct Grid([bool; GRID_SIZE * GRID_SIZE]);

impl Grid {
    const fn index(x: usize, y: usize) -> usize {
        y * GRID_SIZE + x
    }

    fn neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx != 0 || dy != 0)
                    && let Some(x2) = x.checked_add_signed(dx)
                    && let Some(y2) = y.checked_add_signed(dy)
                    && y2 < GRID_SIZE
                    && x2 < GRID_SIZE
                    && self.0[Self::index(x2, y2)]
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn next_state_for_square(&self, x: usize, y: usize) -> bool {
        let curr = self.0[Self::index(x, y)];
        match self.neighbor_count(x, y) {
            2 => curr,
            3 => true,
            _ => false,
        }
    }

    fn next_state_for_square2(&self, x: usize, y: usize) -> bool {
        if (x == 0 || x == GRID_SIZE - 1) && (y == 0 || y == GRID_SIZE - 1) {
            return true;
        }
        let curr = self.0[Self::index(x, y)];
        match self.neighbor_count(x, y) {
            2 => curr,
            3 => true,
            _ => false,
        }
    }

    fn next_state(self) -> Self {
        let mut next = [false; GRID_SIZE * GRID_SIZE];
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                next[Self::index(x, y)] = self.next_state_for_square(x, y);
            }
        }
        Self(next)
    }

    fn next_state2(self) -> Self {
        let mut next = [false; GRID_SIZE * GRID_SIZE];
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                next[Self::index(x, y)] = self.next_state_for_square2(x, y);
            }
        }
        Self(next)
    }

    fn on_count(&self) -> usize {
        self.0.iter().filter(|&&x| x).count()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\n{}\n",
            &self
                .0
                .chunks_exact(GRID_SIZE)
                .map(|l| l.iter().map(|&b| if b { "#" } else { "." }).join(""))
                .join("\n"),
        )
    }
}

pub fn part1(input: &str) -> impl ToString {
    const N: usize = 100;
    let mut g = Grid(
        input
            .lines()
            .flat_map(|l| l.bytes().map(|b| b == b'#'))
            .collect_array()
            .unwrap(),
    );
    for _ in 0..N {
        g = g.next_state();
    }
    g.on_count()
}

pub fn part2(input: &str) -> impl ToString {
    const N: usize = 100;
    let mut g = Grid(
        input
            .lines()
            .flat_map(|l| l.bytes().map(|b| b == b'#'))
            .collect_array()
            .unwrap(),
    );
    g.0[0] = true;
    g.0[GRID_SIZE - 1] = true;
    g.0[GRID_SIZE * GRID_SIZE - 1] = true;
    g.0[GRID_SIZE * (GRID_SIZE - 1)] = true;
    for _ in 0..N {
        g = g.next_state2();
    }
    g.on_count()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 18;
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
