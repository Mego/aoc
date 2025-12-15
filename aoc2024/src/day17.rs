use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Computer {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub ip: usize,
    pub program: Vec<u8>,
    pub output: Vec<u8>,
}

impl Computer {
    pub fn new<T: Into<Vec<u8>>>(a: usize, b: usize, c: usize, program: T) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            program: program.into(),
            output: vec![],
        }
    }

    fn combo(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unimplemented!(),
        }
    }

    pub fn step(&mut self) {
        if self.ip >= self.program.len() - 1 {
            return;
        }
        let instruction = self.program[self.ip].into();
        let operand = self.program[self.ip + 1];
        let mut increment_ip = true;
        match instruction {
            Instruction::Adv => {
                self.a >>= self.combo(operand);
            }
            Instruction::Bxl => {
                self.b ^= operand as usize;
            }
            Instruction::Bst => {
                self.b = self.combo(operand) % 8;
            }
            Instruction::Jnz => {
                if self.a != 0 {
                    increment_ip = false;
                    self.ip = operand as usize;
                }
            }
            Instruction::Bxz => {
                self.b ^= self.c;
            }
            Instruction::Out => {
                self.output.push((self.combo(operand) % 8) as u8);
            }
            Instruction::Bdv => {
                self.b = self.a >> self.combo(operand);
            }
            Instruction::Cdv => {
                self.c = self.a >> self.combo(operand);
            }
        }
        if increment_ip {
            self.ip += 2;
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.program.len() - 1 {
            // dbg!(&self);
            self.step();
        }
    }

    pub fn try_run(&mut self) -> Result<(), &str> {
        let mut seen_states = HashSet::new();
        while self.ip < self.program.len() - 1 {
            if !seen_states.insert((self.a, self.b, self.c, self.ip)) {
                return Err("repeated state");
            }
            if !self.program.starts_with(&self.output) {
                return Err("wrong output");
            }
            // dbg!(&self);
            self.step();
        }
        Ok(())
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.program.chunks(2) {
            let operand = match (chunk[0], chunk[1]) {
                (0 | 2 | 5 | 6 | 7, 4..=6) => {
                    format!("{}", "ABC".chars().nth((chunk[1] - 4) as usize).unwrap())
                }
                _ => format!("{}", chunk[1]),
            };
            writeln!(f, "{:?} {operand}", Instruction::from(chunk[0]))?;
        }
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxz,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxz,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("{value} too large for instruction"),
        }
    }
}

fn parse(input: &str) -> Computer {
    let mut lines = input.lines();
    let a = lines.next().unwrap()[11..].trim().parse().unwrap();
    let b = lines.next().unwrap()[11..].trim().parse().unwrap();
    let c = lines.next().unwrap()[11..].trim().parse().unwrap();
    lines.next().unwrap();
    let program = lines.next().unwrap()[8..]
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect_vec();
    Computer::new(a, b, c, program)
}

pub fn part1(input: &str) -> impl ToString {
    let mut c = parse(input);
    c.run();
    c.output.into_iter().map(|n| format!("{n}")).join(",")
}

pub fn part2(input: &str) -> impl ToString {
    let mut c_orig = parse(input);
    let mut candidate = 8usize.pow(15);
    for n in 0..=14 {
        let to_match = &c_orig.program[if n == 14 { 0.. } else { 15 - n.. }];
        // dbg!(to_match);
        loop {
            // dbg!(candidate);
            let mut c = c_orig.clone();
            c.a = candidate;
            for _ in 0..(8 * 16) {
                c.step();
            }
            if c.output.len() != 16 {
                continue;
            }
            if c.output.ends_with(to_match) {
                break;
            }
            candidate += 8usize.pow(14 - n as u32);
        }
    }
    c_orig.a = candidate;
    c_orig.try_run().unwrap();
    candidate as u64
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 17;
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
