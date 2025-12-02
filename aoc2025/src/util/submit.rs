use std::{
    collections::HashMap,
    fs::{self, File, read_to_string},
    io::{BufWriter, Write},
    path::Path,
    sync::LazyLock,
};

use reqwest::header::COOKIE;

const MY_COOKIE: &str = include_str!("../../cookie.txt");

pub async fn fetch_input(year: u16, day: u8) -> String {
    let fname = Path::new("inputs")
        .join(format!("{year}"))
        .join(format!("day{day}.txt"));
    fs::create_dir_all(fname.parent().unwrap()).unwrap();
    if let Ok(contents) = read_to_string(&fname) {
        return contents;
    }
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let data = reqwest::Client::new()
        .get(url)
        .header(COOKIE, MY_COOKIE)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let f = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&fname)
        .unwrap();
    let mut buf = BufWriter::new(f);
    write!(&mut buf, "{}", data).unwrap();
    data
}

pub async fn submit_answer(year: u16, day: u8, level: &str, answer: &str) -> String {
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let form = HashMap::from([("level", level), ("answer", answer)]);
    let resp = reqwest::Client::new()
        .post(url)
        .header(COOKIE, MY_COOKIE)
        .form(&form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let start_idx = resp.find("<article><p>").unwrap() + "<article><p>".len();
    let end_idx = resp.find("</p></article>").unwrap();
    resp[start_idx..end_idx].trim().to_owned()
}

static CORRECT_ANSWERS: LazyLock<HashMap<(u8, u8), u64>> =
    LazyLock::new(|| [((1, 1), 964), ((1, 2), 5872)].into_iter().collect());

static CORRECT_STR_ANSWERS: LazyLock<HashMap<(u8, u8), &str>> =
    LazyLock::new(|| [].into_iter().collect());

pub fn submit_int(_year: u16, day: u8, level: u8, answer: u64) -> String {
    if let Some(&val) = CORRECT_ANSWERS.get(&(day, level)) {
        if answer == val {
            return format!("right ({answer})");
        }
        return format!("wrong (expected {val}, received {answer})");
    }
    "unsolved".to_owned()
}

pub fn submit_str(_year: u16, day: u8, level: u8, answer: String) -> String {
    if let Some(&val) = CORRECT_STR_ANSWERS.get(&(day, level)) {
        if answer == val {
            return format!("right ({answer})");
        }
        return format!("wrong (expected {val}, received {answer})");
    }
    "unsolved".to_owned()
}

pub fn submit(year: u16, day: u8, level: u8, answer: String) -> String {
    if day == 25 && level == 2 {
        return "right (free)".to_owned();
    }
    if CORRECT_STR_ANSWERS.contains_key(&(day, level)) {
        return submit_str(year, day, level, answer);
    }
    submit_int(year, day, level, answer.parse::<u64>().unwrap())
}
