use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Stats {
    hp: u8,
    damage: u8,
    armor: u8,
}

impl Stats {
    fn does_win(mut self, mut other: Stats) -> bool {
        while self.hp > 0 && other.hp > 0 {
            other.hp = other
                .hp
                .saturating_sub(self.damage.saturating_sub(other.armor).max(1));
            if other.hp == 0 {
                return true;
            }
            self.hp = self
                .hp
                .saturating_sub(other.damage.saturating_sub(self.armor).max(1));
        }
        self.hp > 0
    }
}

const SHOP: [&[(u16, u8, u8)]; 3] = [
    &[(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)],
    &[
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ],
    &[
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ],
];

pub fn part1(input: &str) -> impl ToString {
    let boss = {
        let (hp, damage, armor) = input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u8>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        Stats { hp, damage, armor }
    };
    SHOP[0]
        .iter()
        .cartesian_product(SHOP[1])
        .cartesian_product(SHOP[2].iter().combinations(2))
        .filter_map(|((w, a), rs)| {
            let cost = w.0 + a.0 + rs[0].0 + rs[1].0;
            let damage = w.1 + a.1 + rs[0].1 + rs[1].1;
            let armor = w.2 + a.2 + rs[0].2 + rs[1].2;
            let me = Stats {
                hp: 100,
                armor,
                damage,
            };
            me.does_win(boss).then_some(cost)
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    let boss = {
        let (hp, damage, armor) = input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u8>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        Stats { hp, damage, armor }
    };
    SHOP[0]
        .iter()
        .cartesian_product(SHOP[1])
        .cartesian_product(SHOP[2].iter().combinations(2))
        .filter_map(|((w, a), rs)| {
            let cost = w.0 + a.0 + rs[0].0 + rs[1].0;
            let damage = w.1 + a.1 + rs[0].1 + rs[1].1;
            let armor = w.2 + a.2 + rs[0].2 + rs[1].2;
            let me = Stats {
                hp: 100,
                armor,
                damage,
            };
            (!me.does_win(boss)).then_some(cost)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 21;
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
