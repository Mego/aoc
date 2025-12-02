use fancy_regex::{Captures, Regex};
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

fn apply_replacement(input: &str, from: &str, to: &str) -> impl Iterator<Item = String> {
    input
        .match_indices(from)
        .map(move |(i, _)| format!("{}{}{}", &input[..i], to, &input[i + from.len()..]))
}

pub fn part1(input: String) -> usize {
    let mut lines = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect_vec();
    let molecule = lines.pop().unwrap();
    let replacements = lines
        .into_iter()
        .map(|line| line.split(" => ").collect_tuple::<(&str, &str)>().unwrap())
        .collect_vec();
    let possibilities = FxHashSet::from_iter([molecule.to_owned()]);
    // let mut new_possibilities = possibilities.clone();
    let next_possibilities: FxHashSet<_> = possibilities
        .into_par_iter()
        .flat_map(|p| {
            let p = &p;
            replacements
                .par_iter()
                .flat_map(|(from, to)| apply_replacement(p, from, to).collect_vec())
                .collect::<Vec<_>>()
        })
        .collect();
    // new_possibilities = next_possibilities
    //     .difference(&possibilities)
    //     .cloned()
    //     .collect();
    // possibilities = next_possibilities;
    next_possibilities.len()
}
pub fn part2(input: String) -> usize {
    let mut lines = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect_vec();
    let target = "e";
    let mut molecule: String = lines.pop().unwrap().chars().rev().collect();
    let replacements: FxHashMap<_, _> = lines
        .into_iter()
        .map(|line| {
            let (a, b) = line
                .split(" => ")
                .map(|s| s.chars().rev().collect::<String>())
                .collect_tuple()
                .unwrap();
            (b, a)
        })
        .collect();
    let pat = Regex::new(&replacements.keys().join("|")).unwrap();
    let mut i = 0;
    while molecule != target {
        molecule = pat
            .replacen(&molecule, 1, |m: &Captures<'_>| {
                replacements[m.get(0).unwrap().as_str()].clone()
            })
            .to_string();
        i += 1;
        println!("{i}");
    }
    i
}
