use divisors_fixed::Divisors;

fn sigma(n: u64) -> u64 {
    n.divisors_unordered().into_iter().sum()
}

pub fn part1(input: &str) -> impl ToString {
    let input = input.parse().unwrap();
    (1..).find(|&x| 10 * sigma(x) >= input).unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    let input = input.parse().unwrap();
    (1..)
        .find(|&x| {
            let d: u64 = x
                .divisors_unordered()
                .iter()
                .filter(|&&d| x / d <= 50)
                .sum();
            11 * d >= input
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 20;
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
