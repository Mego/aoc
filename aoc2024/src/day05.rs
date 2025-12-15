use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

fn validate(update: &[u64], rules: &HashSet<(u64, u64)>) -> bool {
    let mut combs = update.iter().combinations(2);
    combs.all(|c| {
        let &a = c[0];
        let &b = c[1];
        !rules.contains(&(b, a))
    })
}

struct ParseResult(Vec<Vec<u64>>, Vec<Vec<u64>>, HashSet<(u64, u64)>);

fn parse(input: &str) -> ParseResult {
    let mut rules = HashSet::new();
    let mut valid_updates = vec![];
    let mut invalid_updates = vec![];
    let mut parsing_rules = true;
    for line in input.lines() {
        if line.trim().is_empty() {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            rules.insert(
                line.trim()
                    .split("|")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            );
        } else {
            let update = line
                .trim()
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            if validate(&update, &rules) {
                valid_updates.push(update);
            } else {
                invalid_updates.push(update);
            }
        }
    }
    ParseResult(valid_updates, invalid_updates, rules)
}

pub fn part1(input: &str) -> impl ToString {
    let ParseResult(valid, _, _) = parse(input.trim());
    let res: u64 = valid.iter().map(|update| update[update.len() / 2]).sum();
    res
}

pub fn part2(input: &str) -> impl ToString {
    let ParseResult(_, invalid, rules) = parse(input.trim());
    let res: u64 = invalid
        .into_iter()
        .map(|update| {
            let len = update.len();
            update
                .into_iter()
                .sorted_by(|&a, &b| {
                    if rules.contains(&(a, b)) {
                        return Ordering::Less;
                    } else if rules.contains(&(b, a)) {
                        return Ordering::Greater;
                    }
                    Ordering::Equal
                })
                .nth(len / 2)
                .unwrap()
        })
        .sum();
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 5;
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
