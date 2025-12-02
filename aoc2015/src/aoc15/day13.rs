use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

fn parse_line<'a>(
    line: &'a str,
    people: &mut FxHashSet<&'a str>,
    weights: &mut FxHashMap<(&'a str, &'a str), i64>,
) {
    let pieces = line.split_ascii_whitespace().collect_vec();
    let from = pieces[0];
    let to = pieces.last().unwrap().trim_end_matches('.');
    people.insert(from);
    people.insert(to);
    weights.insert(
        (from, to),
        pieces[3].parse::<i64>().unwrap() * if pieces[2] == "gain" { 1 } else { -1 },
    );
}

pub fn part1(input: String) -> i64 {
    let mut people = FxHashSet::default();
    let mut weights = FxHashMap::default();
    for line in input.lines() {
        parse_line(line, &mut people, &mut weights);
    }
    let n = people.len();
    people
        .into_iter()
        .permutations(n)
        .map(|p| {
            let mut sum = 0;
            for (i, &person) in p.iter().enumerate() {
                let left = if i == 0 { *p.last().unwrap() } else { p[i - 1] };
                let right = if i + 1 == n { p[0] } else { p[i + 1] };
                sum += weights[&(person, left)] + weights[&(person, right)];
            }
            sum
        })
        .max()
        .unwrap()
}
pub fn part2(input: String) -> i64 {
    let mut people = FxHashSet::default();
    let mut weights = FxHashMap::default();
    for line in input.lines() {
        parse_line(line, &mut people, &mut weights);
    }
    people.insert("me");
    for &person in people.iter() {
        weights.insert(("me", person), 0);
        weights.insert((person, "me"), 0);
    }
    let n = people.len();
    people
        .into_iter()
        .permutations(n)
        .map(|p| {
            let mut sum = 0;
            for (i, &person) in p.iter().enumerate() {
                let left = if i == 0 { *p.last().unwrap() } else { p[i - 1] };
                let right = if i + 1 == n { p[0] } else { p[i + 1] };
                sum += weights[&(person, left)] + weights[&(person, right)];
            }
            sum
        })
        .max()
        .unwrap()
}
