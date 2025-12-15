pub mod day01;

pub mod day02;

pub mod day03;

pub mod day04;

pub mod day05;

pub mod day06;

pub mod day07;

pub mod day08;

pub mod day09;

pub mod day10;

pub mod day11;

pub mod day12;

pub const fn day_part_fn(day: u8, level: u8) -> impl Fn(&str) -> String {
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
