use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashMap;
use std::hash::Hash;

fn count_paths_cached<T: Clone + Eq + Hash, IN: IntoIterator<Item = T>>(
    start_pos: T,
    successors: &impl Fn(&T) -> IN,
    success: &impl Fn(&T) -> bool,
    cache: &mut FxHashMap<T, usize>,
) -> usize {
    if let Some(&n) = cache.get(&start_pos) {
        return n;
    }

    let count = if success(&start_pos) {
        1
    } else {
        successors(&start_pos)
            .into_iter()
            .map(|p| count_paths_cached(p, successors, success, cache))
            .sum()
    };

    cache.insert(start_pos, count);

    count
}

pub fn count_paths<T: Clone + Eq + Hash, IN: IntoIterator<Item = T>>(
    start_pos: T,
    successors: impl Fn(&T) -> IN,
    success: impl Fn(&T) -> bool,
) -> usize {
    let mut cache = FxHashMap::default();
    count_paths_cached(start_pos, &successors, &success, &mut cache)
}

pub fn multi_count_paths<T: Clone + Eq + Hash, IN: IntoIterator<Item = T>>(
    start_pos: T,
    successors: impl Fn(&T) -> IN,
    goals: &[T],
) -> usize {
    goals.iter().fold(0, |_, goal| {
        count_paths(start_pos.clone(), |p| successors(p), |p| p == goal)
    })
}

pub fn par_multi_count_paths<T: Clone + Eq + Hash + Send + Sync, IN: IntoIterator<Item = T>>(
    start_pos: T,
    successors: impl Fn(&T) -> IN + Sync,
    goals: impl IntoParallelIterator<Item = T>,
) -> usize {
    goals
        .into_par_iter()
        .fold(
            || 0,
            |_, goal| count_paths(start_pos.clone(), |p| successors(p), |p| *p == goal),
        )
        .sum()
}
