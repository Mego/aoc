use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
    input.lines().fold(0, |a, line| {
        let dims: [u64; 3] = line
            .split('x')
            .map(|s| s.parse().unwrap())
            .sorted()
            .collect_array()
            .unwrap();
        let sides = [dims[0] * dims[1], dims[0] * dims[2], dims[1] * dims[2]];
        a + 2 * (sides[0] + sides[1] + sides[2]) + sides[0]
    })
}

pub fn part2(input: &str) -> impl ToString {
    input.lines().fold(0, |a, line| {
        let dims: [u64; 3] = line
            .split('x')
            .map(|s| s.parse().unwrap())
            .sorted()
            .collect_array()
            .unwrap();
        a + 2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2]
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 2;
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
