use std::{fmt::Display, sync::LazyLock};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
            Operator::Cat => format!("{a}{b}").parse().unwrap(),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Mul => "*",
                Operator::Cat => "||",
            }
        )
    }
}

static LINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+): ((?:\d+(?:\s+)?)+)").unwrap());

pub fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let mut captures_iter = LINE_REGEX.captures_iter(l);
            let capture = captures_iter.next().unwrap();
            let goal = capture.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let nums = capture
                .get(2)
                .unwrap()
                .as_str()
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            (goal, nums)
        })
        .collect_vec()
}

pub fn part1(input: &str) -> impl ToString {
    let v = parse(input);
    v.into_iter()
        .map(|(goal, nums)| {
            let mut acc_vals = Vec::new();
            for i in 1..nums.len() {
                if i == 1 {
                    [Operator::Add, Operator::Mul].into_iter().for_each(|op| {
                        let val = op.apply(nums[0], nums[1]);
                        if i == nums.len() - 1 {
                            if val == goal {
                                acc_vals.push(val);
                            }
                        } else if val <= goal {
                            acc_vals.push(val);
                        }
                    });
                } else {
                    acc_vals = acc_vals
                        .into_iter()
                        .flat_map(|acc| {
                            let mut res = Vec::new();
                            for op in [Operator::Add, Operator::Mul] {
                                let val = op.apply(acc, nums[i]);
                                if i == nums.len() - 1 {
                                    if val == goal {
                                        res.push(val);
                                    }
                                } else if val <= goal {
                                    res.push(val);
                                }
                            }
                            res
                        })
                        .collect_vec()
                }
            }
            if !acc_vals.is_empty() {
                return goal as usize;
            }
            0
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl ToString {
    let v = parse(input);
    v.into_iter()
        .map(|(goal, nums)| {
            let mut acc_vals = Vec::new();
            for i in 1..nums.len() {
                if i == 1 {
                    [Operator::Add, Operator::Mul, Operator::Cat]
                        .into_iter()
                        .for_each(|op| {
                            let val = op.apply(nums[0], nums[1]);
                            if i == nums.len() - 1 {
                                if val == goal {
                                    acc_vals.push(val);
                                }
                            } else if val <= goal {
                                acc_vals.push(val);
                            }
                        });
                } else {
                    acc_vals = acc_vals
                        .into_iter()
                        .flat_map(|acc| {
                            let mut res = Vec::new();
                            for op in [Operator::Add, Operator::Mul, Operator::Cat] {
                                let val = op.apply(acc, nums[i]);
                                if i == nums.len() - 1 {
                                    if val == goal {
                                        res.push(val);
                                    }
                                } else if val <= goal {
                                    res.push(val);
                                }
                            }
                            res
                        })
                        .collect_vec()
                }
            }
            if !acc_vals.is_empty() {
                return goal as usize;
            }
            0
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 7;
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
