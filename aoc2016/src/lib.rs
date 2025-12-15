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

pub mod day13;

pub mod day14;

pub mod day15;

pub mod day16;

pub mod day17;

pub mod day18;

pub mod day19;

pub mod day20;

pub mod day21;

pub mod day22;

pub mod day23;

pub mod day24;

pub mod day25;

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

        (13, 1) => |input: &str| (day13::part1)(input).to_string(),
        (13, 2) => |input: &str| (day13::part2)(input).to_string(),

        (14, 1) => |input: &str| (day14::part1)(input).to_string(),
        (14, 2) => |input: &str| (day14::part2)(input).to_string(),

        (15, 1) => |input: &str| (day15::part1)(input).to_string(),
        (15, 2) => |input: &str| (day15::part2)(input).to_string(),

        (16, 1) => |input: &str| (day16::part1)(input).to_string(),
        (16, 2) => |input: &str| (day16::part2)(input).to_string(),

        (17, 1) => |input: &str| (day17::part1)(input).to_string(),
        (17, 2) => |input: &str| (day17::part2)(input).to_string(),

        (18, 1) => |input: &str| (day18::part1)(input).to_string(),
        (18, 2) => |input: &str| (day18::part2)(input).to_string(),

        (19, 1) => |input: &str| (day19::part1)(input).to_string(),
        (19, 2) => |input: &str| (day19::part2)(input).to_string(),

        (20, 1) => |input: &str| (day20::part1)(input).to_string(),
        (20, 2) => |input: &str| (day20::part2)(input).to_string(),

        (21, 1) => |input: &str| (day21::part1)(input).to_string(),
        (21, 2) => |input: &str| (day21::part2)(input).to_string(),

        (22, 1) => |input: &str| (day22::part1)(input).to_string(),
        (22, 2) => |input: &str| (day22::part2)(input).to_string(),

        (23, 1) => |input: &str| (day23::part1)(input).to_string(),
        (23, 2) => |input: &str| (day23::part2)(input).to_string(),

        (24, 1) => |input: &str| (day24::part1)(input).to_string(),
        (24, 2) => |input: &str| (day24::part2)(input).to_string(),

        (25, 1) => |input: &str| (day25::part1)(input).to_string(),
        (25, 2) => |input: &str| (day25::part2)(input).to_string(),

        _ => unimplemented!(),
    }
}
