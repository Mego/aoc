mod aoc25;
mod math;
mod util;

#[cfg(test)]
use aoc25::*;
use itertools::Itertools;
#[cfg(test)]
use util::{fetch_input, submit};

#[cfg(test)]
use paste::paste;

const fn day_part_fn(day: u8, level: u8) -> impl Fn(&str) -> String {
    use aoc25::*;
    match (day, level) {
        (1, 1) => |input: &str| (day01::part1)(input).to_string(),
        (1, 2) => |input: &str| (day01::part2)(input).to_string(),
        (2, 1) => |input: &str| (day02::part1)(input).to_string(),
        (2, 2) => |input: &str| (day02::part2)(input).to_string(),
        (3, 1) => |input: &str| (day03::part1)(input).to_string(),
        (3, 2) => |input: &str| (day03::part2)(input).to_string(),
        (4, 1) => |input: &str| (day04::part1)(input).to_string(),
        (4, 2) => |input: &str| (day04::part2)(input).to_string(),
        (5, 1) => |input: &str| (day05::part1)(input).to_string(),
        (5, 2) => |input: &str| (day05::part2)(input).to_string(),
        (6, 1) => |input: &str| (day06::part1)(input).to_string(),
        (6, 2) => |input: &str| (day06::part2)(input).to_string(),
        (7, 1) => |input: &str| (day07::part1)(input).to_string(),
        (7, 2) => |input: &str| (day07::part2)(input).to_string(),
        (8, 1) => |input: &str| (day08::part1)(input).to_string(),
        (8, 2) => |input: &str| (day08::part2)(input).to_string(),
        (9, 1) => |input: &str| (day09::part1)(input).to_string(),
        (9, 2) => |input: &str| (day09::part2)(input).to_string(),
        (10, 1) => |input: &str| (day10::part1)(input).to_string(),
        (10, 2) => |input: &str| (day10::part2)(input).to_string(),
        (11, 1) => |input: &str| (day11::part1)(input).to_string(),
        (11, 2) => |input: &str| (day11::part2)(input).to_string(),
        (12, 1) => |input: &str| (day12::part1)(input).to_string(),
        (12, 2) => |input: &str| (day12::part2)(input).to_string(),
        _ => unimplemented!(),
    }
}

const YEAR: u16 = 2025;
const DAY: u8 = 6;
const LEVEL: u8 = 2;

#[tokio::main]
async fn main() {
    use aoc25::*;
    use util::{fetch_input, submit_answer};
    let input = &fetch_input(YEAR, DAY).await;
    let f = day_part_fn(DAY, LEVEL);
    let answer = f(input);
    println!("{answer}");
    let result = submit_answer(YEAR, DAY, &LEVEL.to_string(), &answer.to_string()).await;
    println!("{result}");
}

#[cfg(test)]
macro_rules! day {
    ( $day:ident ) => {
        stringify!($day)[3..].parse::<u8>().unwrap()
    };
}

#[cfg(test)]
macro_rules! run_day_test {
    ( $day:ident ) => {
        paste! {
            #[test]
            fn [<test_ $day>]() {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let day = day!($day);
                let input = runtime.block_on(fetch_input(YEAR, day)).trim().to_string();
                {
                    let res = $day::part1(&input);
                    let output = submit(YEAR, day, 1, format!("{res}"));
                    println!("{day}::part1 {}", res);
                    assert!(output.starts_with("right"));
                }
                {
                    let res = $day::part2(&input);
                    let output = submit(YEAR, day, 2, format!("{res}"));
                    println!("{day}::part2 {}", res);
                    assert!(output.starts_with("right"));
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    run_day_test!(day01);
    run_day_test!(day02);
    run_day_test!(day03);
    run_day_test!(day04);
    run_day_test!(day05);
    run_day_test!(day06);
    run_day_test!(day07);
    run_day_test!(day08);
    run_day_test!(day09);
    run_day_test!(day10);
    run_day_test!(day11);
    run_day_test!(day12);
}
