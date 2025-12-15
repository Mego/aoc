use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let avail = lines
        .next()
        .unwrap()
        .split(", ")
        .map(String::from)
        .collect_vec();
    lines.next().unwrap();
    let goals = lines.map(String::from).collect_vec();
    (avail, goals)
}

pub fn part1(input: &str) -> impl ToString {
    let (avail, goals) = parse(input);
    goals
        .into_par_iter()
        .filter(|g| can_make_string(g.clone(), avail.clone()) > 0)
        .count()
}

#[cached]
fn can_make_string(goal: String, avail: Vec<String>) -> u64 {
    avail
        .iter()
        .filter_map(|s| {
            if goal == *s {
                return Some(1);
            }
            goal.starts_with(s)
                .then(|| can_make_string(goal[s.len()..].to_owned(), avail.clone()))
        })
        .sum()
}

pub fn part2(input: &str) -> impl ToString {
    let (avail, goals) = parse(input);
    goals
        .into_par_iter()
        .map(|g| can_make_string(g, avail.clone()))
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 19;
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
