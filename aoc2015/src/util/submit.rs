use std::{
    collections::HashMap,
    fs::{File, read_to_string},
    io::{BufWriter, Write},
    sync::LazyLock,
};

use reqwest::header::COOKIE;

const MY_COOKIE: &str = include_str!("../../cookie.txt");

pub async fn fetch_input(year: u16, day: u8) -> String {
    let fname = format!("inputs/{}/day{}.txt", year, day);
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

static CORRECT_ANSWERS: LazyLock<HashMap<(u8, u8), u64>> = LazyLock::new(|| {
    [
        ((1, 1), 74),
        ((1, 2), 1795),
        ((2, 1), 1598415),
        ((2, 2), 3812909),
        ((3, 1), 2592),
        ((3, 2), 2360),
        ((4, 1), 346386),
        ((4, 2), 9958218),
        ((5, 1), 258),
        ((5, 2), 53),
        ((6, 1), 543903),
        ((6, 2), 14687245),
        ((7, 1), 16076),
        ((7, 2), 2797),
        ((8, 1), 1342),
        ((8, 2), 2074),
        ((9, 1), 117),
        ((9, 2), 909),
        ((10, 1), 329356),
        ((10, 2), 4666278),
        ((12, 1), 156366),
        ((12, 2), 96852),
        ((13, 1), 618),
        ((13, 2), 601),
        ((14, 1), 2696),
        ((14, 2), 1084),
        ((15, 1), 13882464),
        ((15, 2), 11171160),
        ((16, 1), 40),
        ((16, 2), 241),
        ((17, 1), 1638),
        ((17, 2), 17),
        ((18, 1), 1061),
        ((18, 2), 1006),
        ((19, 1), 518),
        ((19, 2), 200),
        ((20, 1), 665280),
        ((20, 2), 705600),
        ((21, 1), 78),
        ((21, 2), 148),
        ((22, 1), 1269),
        ((22, 2), 1309),
        ((23, 1), 307),
        ((23, 2), 160),
        ((24, 1), 10723906903),
        ((24, 2), 74850409),
        ((25, 1), 19980801),
    ]
    .into_iter()
    .collect()
});

static CORRECT_STR_ANSWERS: LazyLock<HashMap<(u8, u8), &str>> = LazyLock::new(|| {
    [((11, 1), "hepxxyzz"), ((11, 2), "heqaabcc")]
        .into_iter()
        .collect()
});

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
