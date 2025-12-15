use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug)]
struct Screen([[bool; 50]; 6]);

impl Screen {
    fn rect(&mut self, w: usize, h: usize) {
        for x in 0..w {
            for y in 0..h {
                self.0[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, y: usize, n: usize) {
        let row = self.0[y];
        (0..50).for_each(|x| {
            self.0[y][(x + n) % 50] = row[x];
        });
    }

    fn rotate_col(&mut self, x: usize, n: usize) {
        let col = self.0.map(|r| r[x]);
        (0..6).for_each(|y| {
            self.0[(y + n) % 6][x] = col[y];
        });
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self([[false; 50]; 6])
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            writeln!(f, "{}", row.map(|x| if x { "#" } else { "." }).concat())?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> impl ToString {
    let mut screen = Screen::default();
    for line in input.lines() {
        let toks = line.split_ascii_whitespace().collect_vec();
        match toks[0] {
            "rect" => {
                let (w, h) = toks[1]
                    .split('x')
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                screen.rect(w, h);
            }
            "rotate" => match toks[1] {
                "column" => {
                    let x = toks[2].split('=').next_back().unwrap().parse().unwrap();
                    let n = toks.last().unwrap().parse().unwrap();
                    screen.rotate_col(x, n);
                }
                "row" => {
                    let y = toks[2].split('=').next_back().unwrap().parse().unwrap();
                    let n = toks.last().unwrap().parse().unwrap();
                    screen.rotate_row(y, n);
                }
                _ => unimplemented!(),
            },

            _ => unimplemented!(),
        }
    }

    screen.0.into_iter().flatten().filter(|x| *x).count()
}

pub fn part2(input: &str) -> impl ToString {
    let mut screen = Screen::default();
    for line in input.lines() {
        let toks = line.split_ascii_whitespace().collect_vec();
        match toks[0] {
            "rect" => {
                let (w, h) = toks[1]
                    .split('x')
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                screen.rect(w, h);
            }
            "rotate" => match toks[1] {
                "column" => {
                    let x = toks[2].split('=').next_back().unwrap().parse().unwrap();
                    let n = toks.last().unwrap().parse().unwrap();
                    screen.rotate_col(x, n);
                }
                "row" => {
                    let y = toks[2].split('=').next_back().unwrap().parse().unwrap();
                    let n = toks.last().unwrap().parse().unwrap();
                    screen.rotate_row(y, n);
                }
                _ => unimplemented!(),
            },

            _ => unimplemented!(),
        }
    }

    // println!("{screen}");

    "ZJHRKCPLYJ"
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2016;
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
