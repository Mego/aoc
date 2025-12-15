use itertools::Itertools;

use util::math::Matrix;

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    goal: (usize, usize),
}

fn parse(input: String) -> Machine {
    let mut lines = input.lines();
    let a = lines.next().unwrap()[10..]
        .split(", ")
        .map(|s| s[2..].parse().unwrap())
        .collect_tuple()
        .unwrap();
    let b = lines.next().unwrap()[10..]
        .split(", ")
        .map(|s| s[2..].parse().unwrap())
        .collect_tuple()
        .unwrap();
    let goal = lines.next().unwrap()[7..]
        .split(", ")
        .map(|s| s[2..].parse().unwrap())
        .collect_tuple()
        .unwrap();
    Machine { a, b, goal }
}

pub fn part1(input: &str) -> impl ToString {
    let machines = input
        .split("\n\n")
        .map(str::to_string)
        .map(parse)
        .collect_vec();
    machines
        .into_iter()
        .map(|machine| {
            let m = Matrix::new(machine.a.0, machine.b.0, machine.a.1, machine.b.1);
            let solution = m.solve(machine.goal.0, machine.goal.1);
            solution.map(|(a, b)| a * 3 + b).unwrap_or_default()
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl ToString {
    let machines = input
        .split("\n\n")
        .map(str::to_string)
        .map(|s| {
            let mut m = parse(s);
            m.goal.0 += 10000000000000;
            m.goal.1 += 10000000000000;
            m
        })
        .collect_vec();
    machines
        .into_iter()
        .map(|machine| {
            let m = Matrix::new(machine.a.0, machine.b.0, machine.a.1, machine.b.1);
            let solution = m.solve(machine.goal.0, machine.goal.1);
            solution.map(|(a, b)| a * 3 + b).unwrap_or_default()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 13;
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
