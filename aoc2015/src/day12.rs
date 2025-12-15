use itertools::Itertools;
use serde_json::{Value, json};

fn json_sum(v: &Value) -> i64 {
    match v {
        Value::Array(a) => a.iter().map(json_sum).sum(),
        Value::Number(x) => x.as_i64().unwrap(),
        Value::Object(x) => x.values().map(json_sum).sum(),
        _ => 0,
    }
}

fn json_sum2(v: &Value) -> i64 {
    match v {
        Value::Array(a) => a.iter().map(json_sum2).sum(),
        Value::Number(x) => x.as_i64().unwrap(),
        Value::Object(x) => {
            if !x.values().contains(&json!("red")) {
                x.values().map(json_sum2).sum()
            } else {
                0
            }
        }
        _ => 0,
    }
}

pub fn part1(input: &str) -> impl ToString {
    let v: Value = serde_json::from_str(input).unwrap();
    json_sum(&v)
}

pub fn part2(input: &str) -> impl ToString {
    let v: Value = serde_json::from_str(input).unwrap();
    json_sum2(&v)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 12;
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
