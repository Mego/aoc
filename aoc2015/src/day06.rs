use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashSet};

struct Lights {
    lights: FxHashSet<(u16, u16)>,
}

impl Lights {
    fn turn_on(&mut self, light: (u16, u16)) {
        self.lights.insert(light);
    }

    fn turn_off(&mut self, light: (u16, u16)) {
        self.lights.remove(&light);
    }

    fn toggle(&mut self, light: (u16, u16)) {
        if !self.lights.insert(light) {
            self.lights.remove(&light);
        }
    }

    fn len(&self) -> u64 {
        self.lights.len() as u64
    }
}

impl Default for Lights {
    fn default() -> Self {
        Self {
            lights: FxHashSet::with_capacity_and_hasher(1000 * 1000, FxBuildHasher),
        }
    }
}

pub fn part1(input: &str) -> impl ToString {
    let mut lights = Lights::default();
    for line in input.lines() {
        if line.starts_with("turn on") {
            let bounds: [(u16, u16); 2] = line["turn on ".len()..]
                .split(" through ")
                .map(|b| {
                    b.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_array()
                .unwrap();
            for i in bounds[0].0..=bounds[1].0 {
                for j in bounds[0].1..=bounds[1].1 {
                    lights.turn_on((i, j));
                }
            }
        } else if line.starts_with("turn off") {
            let bounds: [(u16, u16); 2] = line["turn off ".len()..]
                .split(" through ")
                .map(|b| {
                    b.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_array()
                .unwrap();
            for i in bounds[0].0..=bounds[1].0 {
                for j in bounds[0].1..=bounds[1].1 {
                    lights.turn_off((i, j));
                }
            }
        } else if line.starts_with("toggle") {
            let bounds: [(u16, u16); 2] = line["toggle ".len()..]
                .split(" through ")
                .map(|b| {
                    b.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_array()
                .unwrap();
            for i in bounds[0].0..=bounds[1].0 {
                for j in bounds[0].1..=bounds[1].1 {
                    lights.toggle((i, j));
                }
            }
        } else {
            unreachable!("{line}");
        }
    }
    lights.len()
}

macro_rules! saturating_sub_assign {
    ($x:expr, $y:expr) => {
        $x = $x.saturating_sub($y)
    };
}

struct Lights2 {
    lights: [u8; 1000 * 1000],
}

impl Lights2 {
    const fn get_index(light: (u16, u16)) -> usize {
        (light.0 as usize * 1000) + light.1 as usize
    }

    fn turn_on(&mut self, light: (u16, u16)) {
        self.lights[Self::get_index(light)] += 1;
    }

    fn turn_off(&mut self, light: (u16, u16)) {
        saturating_sub_assign!(self.lights[Self::get_index(light)], 1);
    }

    fn toggle(&mut self, light: (u16, u16)) {
        self.lights[Self::get_index(light)] += 2;
    }

    fn total(&self) -> u64 {
        self.lights.iter().fold(0u64, |a, &b| a + (b as u64))
    }
}

impl Default for Lights2 {
    fn default() -> Self {
        Self {
            lights: [0; 1000 * 1000],
        }
    }
}

pub fn part2(input: &str) -> impl ToString {
    let mut lights = Lights2::default();
    for line in input.lines() {
        if line.starts_with("turn on") {
            let bounds: [(u16, u16); 2] = line["turn on ".len()..]
                .split(" through ")
                .map(|b| {
                    b.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_array()
                .unwrap();
            for i in bounds[0].0..=bounds[1].0 {
                for j in bounds[0].1..=bounds[1].1 {
                    lights.turn_on((i, j));
                }
            }
        } else if line.starts_with("turn off") {
            let bounds: [(u16, u16); 2] = line["turn off ".len()..]
                .split(" through ")
                .map(|b| {
                    b.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_array()
                .unwrap();
            for i in bounds[0].0..=bounds[1].0 {
                for j in bounds[0].1..=bounds[1].1 {
                    lights.turn_off((i, j));
                }
            }
        } else if line.starts_with("toggle") {
            let bounds: [(u16, u16); 2] = line["toggle ".len()..]
                .split(" through ")
                .map(|b| {
                    b.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_array()
                .unwrap();
            for i in bounds[0].0..=bounds[1].0 {
                for j in bounds[0].1..=bounds[1].1 {
                    lights.toggle((i, j));
                }
            }
        } else {
            unreachable!("{line}");
        }
    }
    lights.total()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
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
