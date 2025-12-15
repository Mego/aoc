#[derive(Debug, Clone, Copy)]
enum Instruction {
    Do,
    Dont,
    Mul(u64, u64),
}

peg::parser! {
    grammar day3parser() for str {
        rule do_rule() -> Instruction
            = "do()" { Instruction::Do }

        rule dont() -> Instruction
            = "don't()" { Instruction::Dont }

        rule number() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("u64")) }

        rule mul() -> Instruction
            = "mul(" a:number() "," b:number() ")" { Instruction::Mul(a, b) }

        rule token() -> Instruction
            = do_rule() / dont() / mul()

        rule junk() -> &'input str
            = !token() s:$([c if c.is_ascii_whitespace() || c.is_ascii_graphic()]) { s }

        #[no_eof]
        pub rule day3() -> Vec<Instruction>
            = junk()* l:(token() ** (junk()*)) junk()* { l }
    }
}

pub fn part1(input: &str) -> impl ToString {
    let tokens = day3parser::day3(input).unwrap();
    tokens.into_iter().fold(0, |acc, token| {
        acc + match token {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        }
    })
}

pub fn part2(input: &str) -> impl ToString {
    let tokens = day3parser::day3(input).unwrap();
    tokens
        .into_iter()
        .fold((0, true), |(acc, enabled), token| match token {
            Instruction::Do => (acc, true),
            Instruction::Dont => (acc, false),
            Instruction::Mul(a, b) => (
                acc + if enabled { a * b } else { Default::default() },
                enabled,
            ),
        })
        .0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 3;
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
