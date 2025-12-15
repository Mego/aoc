use std::{convert::Infallible, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Default)]
struct Computer {
    a: u64,
    b: u64,
    pc: usize,
}

impl Computer {
    fn execute_one(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::HLF(register) => {
                let reg = match register {
                    Register::A => &mut self.a,
                    Register::B => &mut self.b,
                };
                *reg /= 2;
                self.pc += 1;
            }
            Instruction::TPL(register) => {
                let reg = match register {
                    Register::A => &mut self.a,
                    Register::B => &mut self.b,
                };
                *reg *= 3;
                self.pc += 1;
            }
            Instruction::INC(register) => {
                let reg = match register {
                    Register::A => &mut self.a,
                    Register::B => &mut self.b,
                };
                *reg += 1;
                self.pc += 1;
            }
            Instruction::JMP(offset) => {
                self.pc = self.pc.saturating_add_signed(*offset);
            }
            Instruction::JIE(register, offset) => {
                let reg = match register {
                    Register::A => self.a,
                    Register::B => self.b,
                };
                if reg % 2 == 0 {
                    self.pc = self.pc.saturating_add_signed(*offset);
                } else {
                    self.pc += 1;
                }
            }
            Instruction::JIO(register, offset) => {
                let reg = match register {
                    Register::A => self.a,
                    Register::B => self.b,
                };
                if reg == 1 {
                    self.pc = self.pc.saturating_add_signed(*offset);
                } else {
                    self.pc += 1;
                }
            }
        }
    }
    fn execute_all(&mut self, instructions: &[Instruction]) {
        while let Some(instruction) = instructions.get(self.pc) {
            self.execute_one(instruction);
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
enum Instruction {
    HLF(Register),
    TPL(Register),
    INC(Register),
    JMP(isize),
    JIE(Register, isize),
    JIO(Register, isize),
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[..3] {
            "hlf" => Self::HLF(if s.ends_with('a') {
                Register::A
            } else {
                Register::B
            }),
            "tpl" => Self::TPL(if s.ends_with('a') {
                Register::A
            } else {
                Register::B
            }),
            "inc" => Self::INC(if s.ends_with('a') {
                Register::A
            } else {
                Register::B
            }),
            "jmp" => Self::JMP(s[4..].parse().unwrap()),
            "jie" => {
                let (reg, off) = s[4..].split(", ").collect_tuple().unwrap();
                Self::JIE(
                    if reg == "a" { Register::A } else { Register::B },
                    off.parse().unwrap(),
                )
            }
            "jio" => {
                let (reg, off) = s[4..].split(", ").collect_tuple().unwrap();
                Self::JIO(
                    if reg == "a" { Register::A } else { Register::B },
                    off.parse().unwrap(),
                )
            }
            _ => unimplemented!(),
        })
    }
}

pub fn part1(input: &str) -> impl ToString {
    let mut computer = Computer::default();
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect_vec();
    computer.execute_all(&instructions);
    computer.b
}

pub fn part2(input: &str) -> impl ToString {
    let mut computer = Computer {
        a: 1,
        ..Default::default()
    };
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect_vec();
    computer.execute_all(&instructions);
    computer.b
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 23;
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
