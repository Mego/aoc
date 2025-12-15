fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let mut val: i32 = line[1..].parse().unwrap();
            if line.starts_with('L') {
                val = -val;
            }
            val
        })
        .collect()
}

pub fn part1(input: &str) -> impl ToString {
    let turns = parse(input);
    let mut dial = 50;
    let mut counter = 0;
    for turn in turns {
        dial += turn;
        if dial % 100 == 0 {
            counter += 1;
        }
    }
    counter
}

pub fn part2(input: &str) -> impl ToString {
    let turns = parse(input);
    let mut dial = 50;
    let mut counter = 0;
    for turn in turns {
        let n = turn.abs() / 100;
        let m = turn % 100;
        counter += n as u64;
        if dial != 0 && !(1..=99).contains(&(dial + m)) {
            counter += 1;
        }
        dial += m;
        dial = dial.rem_euclid(100);
    }
    counter
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 1;
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
