use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

fn parse_part1(input: &str) -> Vec<(Vec<u64>, Operation)> {
    let num_problems = input
        .lines()
        .last()
        .unwrap()
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .count();
    let mut problems = vec![(vec![], Operation::Add); num_problems];
    for line in input.lines() {
        let parts = line.split_ascii_whitespace();
        let is_ops = line.contains(['*', '+']);
        for (i, part) in parts.enumerate() {
            if is_ops {
                problems[i].1 = match part {
                    "*" => Operation::Mul,
                    "+" => Operation::Add,
                    _ => unimplemented!(),
                }
            } else {
                problems[i].0.push(part.parse().unwrap());
            }
        }
    }
    problems
}

pub fn part1(input: &str) -> u64 {
    parse_part1(input)
        .into_iter()
        .map(|(numbers, op)| {
            numbers
                .into_iter()
                .reduce(|a, x| match op {
                    Operation::Add => a + x,
                    Operation::Mul => a * x,
                })
                .unwrap()
        })
        .sum()
}

fn parse_part2(input: &str) -> Vec<(Vec<u64>, Operation)> {
    let num_problems = input
        .lines()
        .last()
        .unwrap()
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .count();
    let mut problems = vec![(vec![], Operation::Add); num_problems];
    let mut lines = input.lines().collect_vec();
    let ops_line = lines.pop().unwrap();
    let lines = lines
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut problem_idx = 0;
    for i in 1..=lines[0].len() {
        let num = lines
            .iter()
            .map(|line| line[line.len() - i])
            .join("")
            .trim()
            .to_string();
        if num.is_empty() {
            problem_idx += 1;
            continue;
        }
        problems[problem_idx].0.push(num.parse().unwrap());
    }
    problems
        .iter_mut()
        .zip(ops_line.split_ascii_whitespace().rev())
        .for_each(|(problem, op)| {
            problem.1 = match op {
                "*" => Operation::Mul,
                "+" => Operation::Add,
                _ => unimplemented!(),
            };
        });

    problems
}

pub fn part2(input: &str) -> u64 {
    parse_part2(input)
        .into_iter()
        .map(|(numbers, op)| {
            numbers
                .into_iter()
                .reduce(|a, x| match op {
                    Operation::Add => a + x,
                    Operation::Mul => a * x,
                })
                .unwrap()
        })
        .sum()
}
