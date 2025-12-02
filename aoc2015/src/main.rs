use aoc2015::{
    aoc15::*,
    util::{fetch_input, submit_answer},
};

#[cfg(test)]
use paste::paste;

const fn day_part_fn(day: u8, level: u8) -> impl Fn(String) -> String {
    match (day, level) {
        (1, 1) => |input: String| (day01::part1)(input).to_string(),
        (1, 2) => |input: String| (day01::part2)(input).to_string(),
        (2, 1) => |input: String| (day02::part1)(input).to_string(),
        (2, 2) => |input: String| (day02::part2)(input).to_string(),
        (3, 1) => |input: String| (day03::part1)(input).to_string(),
        (3, 2) => |input: String| (day03::part2)(input).to_string(),
        (4, 1) => |input: String| (day04::part1)(input).to_string(),
        (4, 2) => |input: String| (day04::part2)(input).to_string(),
        (5, 1) => |input: String| (day05::part1)(input).to_string(),
        (5, 2) => |input: String| (day05::part2)(input).to_string(),
        (6, 1) => |input: String| (day06::part1)(input).to_string(),
        (6, 2) => |input: String| (day06::part2)(input).to_string(),
        (7, 1) => |input: String| (day07::part1)(input).to_string(),
        (7, 2) => |input: String| (day07::part2)(input).to_string(),
        (8, 1) => |input: String| (day08::part1)(input).to_string(),
        (8, 2) => |input: String| (day08::part2)(input).to_string(),
        (9, 1) => |input: String| (day09::part1)(input).to_string(),
        (9, 2) => |input: String| (day09::part2)(input).to_string(),
        (10, 1) => |input: String| (day10::part1)(input).to_string(),
        (10, 2) => |input: String| (day10::part2)(input).to_string(),
        (11, 1) => |input: String| (day11::part1)(input).to_string(),
        (11, 2) => |input: String| (day11::part2)(input).to_string(),
        (12, 1) => |input: String| (day12::part1)(input).to_string(),
        (12, 2) => |input: String| (day12::part2)(input).to_string(),
        (13, 1) => |input: String| (day13::part1)(input).to_string(),
        (13, 2) => |input: String| (day13::part2)(input).to_string(),
        (14, 1) => |input: String| (day14::part1)(input).to_string(),
        (14, 2) => |input: String| (day14::part2)(input).to_string(),
        (15, 1) => |input: String| (day15::part1)(input).to_string(),
        (15, 2) => |input: String| (day15::part2)(input).to_string(),
        (16, 1) => |input: String| (day16::part1)(input).to_string(),
        (16, 2) => |input: String| (day16::part2)(input).to_string(),
        (17, 1) => |input: String| (day17::part1)(input).to_string(),
        (17, 2) => |input: String| (day17::part2)(input).to_string(),
        (18, 1) => |input: String| (day18::part1)(input).to_string(),
        (18, 2) => |input: String| (day18::part2)(input).to_string(),
        (19, 1) => |input: String| (day19::part1)(input).to_string(),
        (19, 2) => |input: String| (day19::part2)(input).to_string(),
        (20, 1) => |input: String| (day20::part1)(input).to_string(),
        (20, 2) => |input: String| (day20::part2)(input).to_string(),
        (21, 1) => |input: String| (day21::part1)(input).to_string(),
        (21, 2) => |input: String| (day21::part2)(input).to_string(),
        (22, 1) => |input: String| (day22::part1)(input).to_string(),
        (22, 2) => |input: String| (day22::part2)(input).to_string(),
        (23, 1) => |input: String| (day23::part1)(input).to_string(),
        (23, 2) => |input: String| (day23::part2)(input).to_string(),
        (24, 1) => |input: String| (day24::part1)(input).to_string(),
        (24, 2) => |input: String| (day24::part2)(input).to_string(),
        (25, 1) => |input: String| (day25::part1)(input).to_string(),
        (25, 2) => |input: String| (day25::part2)(input).to_string(),
        _ => unimplemented!(),
    }
}

#[tokio::main]
async fn main() {
    const YEAR: u16 = 2015;
    const DAY: u8 = 25;
    const LEVEL: u8 = 1;

    let f = day_part_fn(DAY, LEVEL);

    let input = fetch_input(YEAR, DAY).await.trim().to_string();

    let answer = f(input);
    println!("{answer}");

    let result = submit_answer(YEAR, DAY, &LEVEL.to_string(), &answer).await;
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
                let input = runtime.block_on(fetch_input(2015, day)).trim().to_string();
                {
                    let res = $day::part1(input.clone());
                    let output = aoc2015::util::submit::submit(2015, day, 1, format!("{res}"));
                    println!("{day}::part1 {}", res);
                    assert!(output.starts_with("right"));
                }
                {
                    let res = $day::part2(input);
                    let output = aoc2015::util::submit::submit(2015, day, 2, format!("{res}"));
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
    run_day_test!(day13);
    run_day_test!(day14);
    run_day_test!(day15);
    run_day_test!(day16);
    run_day_test!(day17);
    run_day_test!(day18);
    run_day_test!(day19);
    run_day_test!(day20);
    run_day_test!(day21);
    run_day_test!(day22);
    run_day_test!(day23);
    run_day_test!(day24);
    run_day_test!(day25);
}
