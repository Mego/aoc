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

const YEAR: u16 = 2025;
const DAY: u8 = 1;
const LEVEL: u8 = 2;

#[tokio::main]
async fn main() {
    use aoc25::*;
    use util::{fetch_input, submit_answer};
    let input = &fetch_input(YEAR, DAY).await;
    let answer = day01::part2(input);
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
