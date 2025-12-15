use clap::Parser;

mod cli;
use cli::Cli;

fn run_year_day_part(year: u16, day: u8, level: u8, input: &str) -> String {
    match year {
        2015 => aoc2015::day_part_fn(day, level)(input),

        2016 => aoc2016::day_part_fn(day, level)(input),

        2024 => aoc2024::day_part_fn(day, level)(input),

        2025 => aoc2025::day_part_fn(day, level)(input),

        _ => unimplemented!("year {year} has not been generated"),
    }
}

async fn run(year: u16, day: u8, level: u8) {
    let input = util::fetch_input(year, day).await;
    let answer = run_year_day_part(year, day, level, &input);
    let resp = util::submit_answer(year, day, level, &answer).await;
    println!("{resp}");
}

#[tokio::main]
pub async fn main() {
    let Cli {
        year,
        day,
        level,
        fetch_input,
    } = Cli::parse();
    if fetch_input {
        util::fetch_input(year, day).await;
    } else {
        run(year, day, level).await;
    }
}
