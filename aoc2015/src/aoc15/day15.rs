use std::{iter, str::FromStr};

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl FromStr for Ingredient {
    type Err = <i64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect_vec();
        Ok(Self {
            capacity: parts[2].trim_end_matches(',').parse()?,
            durability: parts[4].trim_end_matches(',').parse()?,
            flavor: parts[6].trim_end_matches(',').parse()?,
            texture: parts[8].trim_end_matches(',').parse()?,
            calories: parts[10].parse()?,
        })
    }
}

fn partition_func(n: u64, k: usize, l: usize) -> Box<dyn Iterator<Item = Vec<u64>> + Send> {
    match k {
        0 => Box::new(iter::empty()),
        1 => {
            if n as usize >= l {
                Box::new([vec![n]].into_iter())
            } else {
                Box::new(iter::empty())
            }
        }
        _ => Box::new((l..(n as usize)).flat_map(move |i| {
            partition_func(n - i as u64, k - 1, i)
                .map(move |p| [i as u64].into_iter().chain(p).collect_vec())
        })),
    }
}

pub fn part1(input: String) -> u64 {
    let ingredients: Vec<Ingredient> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();
    const MAX_INGREDIENTS: u64 = 100;

    partition_func(MAX_INGREDIENTS, ingredients.len(), 1)
        .par_bridge()
        .flat_map(|p| p.iter().copied().permutations(p.len()).collect_vec())
        .map(|p| {
            let totals = p
                .into_iter()
                .zip_eq(&ingredients)
                .fold((0, 0, 0, 0), |a, (n, i)| {
                    (
                        a.0 + i.capacity * n as i64,
                        a.1 + i.durability * n as i64,
                        a.2 + i.flavor * n as i64,
                        a.3 + i.texture * n as i64,
                    )
                });
            (totals.0.max(0) as u64)
                * (totals.1.max(0) as u64)
                * (totals.2.max(0) as u64)
                * (totals.3.max(0) as u64)
        })
        .max()
        .unwrap()
}
pub fn part2(input: String) -> u64 {
    let ingredients: Vec<Ingredient> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();
    const MAX_INGREDIENTS: u64 = 100;
    const CALORIES: u64 = 500;

    partition_func(MAX_INGREDIENTS, ingredients.len(), 1)
        .par_bridge()
        .flat_map(|p| p.iter().copied().permutations(p.len()).collect_vec())
        .filter_map(|p| {
            let totals = p
                .into_iter()
                .zip_eq(&ingredients)
                .fold((0, 0, 0, 0, 0), |a, (n, i)| {
                    (
                        a.0 + i.capacity * n as i64,
                        a.1 + i.durability * n as i64,
                        a.2 + i.flavor * n as i64,
                        a.3 + i.texture * n as i64,
                        a.4 + i.calories * n as i64,
                    )
                });
            ((totals.4 as u64) == CALORIES).then_some(
                (totals.0.max(0) as u64)
                    * (totals.1.max(0) as u64)
                    * (totals.2.max(0) as u64)
                    * (totals.3.max(0) as u64),
            )
        })
        .max()
        .unwrap()
}
