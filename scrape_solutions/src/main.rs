use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use clap::Parser;
use scraper::{Html, Selector};

const MY_COOKIE: &str = include_str!("../../cookie.txt");

async fn fetch_day_page(year: u16, day: u8) -> Result<String, reqwest::Error> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    reqwest::Client::new()
        .get(url)
        .header(reqwest::header::COOKIE, MY_COOKIE)
        .send()
        .await?
        .text()
        .await
}

fn get_level_solution(page: &str, level: u8) -> String {
    let document = Html::parse_document(page);
    let selector = Selector::parse(&format!("main > p:nth-of-type({level}) > code")).unwrap();
    document
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .to_string()
}

fn save_solution(year: u16, day: u8, level: u8, answer: &str) {
    let fname = util::solution_path(year, day, level);
    fs::create_dir_all(fname.parent().unwrap()).unwrap();
    let f = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&fname)
        .unwrap();
    let mut buf = BufWriter::new(f);
    write!(&mut buf, "{answer}").unwrap();
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    pub(crate) year: u16,
    pub(crate) day: u8,

    pub(crate) level: Option<u8>,
}

#[tokio::main]
async fn main() {
    let Cli { year, day, level } = Cli::parse();
    let page = fetch_day_page(year, day).await.unwrap();
    if let Some(level) = level {
        let solution = get_level_solution(&page, level);
        save_solution(year, day, level, &solution);
    } else {
        for level in [1, 2] {
            let solution = get_level_solution(&page, level);
            save_solution(year, day, level, &solution);
        }
    }
}
