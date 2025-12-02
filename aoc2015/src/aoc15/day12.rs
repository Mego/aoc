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

pub fn part1(input: String) -> i64 {
    let v: Value = serde_json::from_str(&input).unwrap();
    json_sum(&v)
}
pub fn part2(input: String) -> i64 {
    let v: Value = serde_json::from_str(&input).unwrap();
    json_sum2(&v)
}
