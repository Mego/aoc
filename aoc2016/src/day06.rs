use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
    let lines = input.lines().collect_vec();
    (0..(lines[0].len()))
        .map(|i| {
            lines
                .iter()
                .map(|l| l.chars().nth(i).unwrap())
                .counts()
                .into_iter()
                .max_by_key(|(_, c)| *c)
                .unwrap()
                .0
        })
        .join("")
}

pub fn part2(input: &str) -> impl ToString {
    let lines = input.lines().collect_vec();
    (0..(lines[0].len()))
        .map(|i| {
            lines
                .iter()
                .map(|l| l.chars().nth(i).unwrap())
                .counts()
                .into_iter()
                .min_by_key(|(_, c)| *c)
                .unwrap()
                .0
        })
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2016;
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
