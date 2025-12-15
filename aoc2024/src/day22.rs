use itertools::Itertools;
use rayon::prelude::*;

const fn step_secret(secret: u32) -> u32 {
    let mut res = ((secret * 64) ^ secret) & 0xFFFFFF;
    res ^= res / 32;
    res &= 0xFFFFFF;
    res ^= res * 2048;
    res &= 0xFFFFFF;
    res
}

pub fn part1(input: &str) -> impl ToString {
    input.lines().fold(0, |sum, l| {
        let mut secret = l.parse().unwrap();
        for _ in 0..2000 {
            secret = step_secret(secret);
        }
        sum + secret as u64
    })
}

pub fn part2(input: &str) -> impl ToString {
    let mut initial_prices = vec![];
    let deltas = input
        .lines()
        .map(|l| {
            let mut secret = l.parse().unwrap();
            let mut result = vec![];
            for i in 0..2000 {
                secret = step_secret(secret);
                if i == 0 {
                    initial_prices.push((secret % 10) as i32);
                }
                result.push(secret % 10);
            }
            result
                .windows(2)
                .map(|x| {
                    let a = x[0] as i32;
                    let b = x[1] as i32;
                    b - a
                })
                .collect_vec()
        })
        .collect_vec();

    deltas
        .iter()
        .flat_map(|ds| ds.windows(4))
        .unique()
        .par_bridge()
        .map(|w| {
            let mut profit = 0;
            for (buyer_deltas, &init_price) in deltas.iter().zip(initial_prices.iter()) {
                let mut pos = 0;
                while pos < buyer_deltas.len() - 4 && buyer_deltas[pos..pos + 4] != *w {
                    pos += 1;
                }
                if pos < buyer_deltas.len() - 4 {
                    profit += buyer_deltas[..pos + 4]
                        .iter()
                        .fold(init_price, |a, v| a + v);
                }
            }
            profit
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 22;
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
