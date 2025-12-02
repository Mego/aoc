use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use indexmap::IndexSet;
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::{FxHashSet, FxHasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = FxHasher::default();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FloorInventory {
    chips: FxHashSet<usize>,
    gens: FxHashSet<usize>,
}

#[derive(Debug, Clone, Eq)]
struct State(pub Vec<FloorInventory>, pub usize);

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let c: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .map(|(fl, inv)| (fl, inv.chips.len()))
            .collect();
        c.hash(state);
        let g: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .map(|(fl, inv)| (fl, inv.gens.len()))
            .collect();
        g.hash(state);
        self.1.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1 && (self.0 == other.0 || calculate_hash(self) == calculate_hash(other))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in [3, 2, 1, 0] {
            writeln!(
                f,
                "F{}: {:?}",
                i + 1,
                self.0[i]
                    .chips
                    .iter()
                    .copied()
                    .sorted()
                    .map(|s| format!("{}M", s))
                    .chain(
                        self.0[i]
                            .gens
                            .iter()
                            .copied()
                            .sorted()
                            .map(|s| format!("{}G", s))
                    )
                    .collect_vec()
            )?;
        }
        writeln!(f, "elevator: {}", self.1 + 1)?;
        Ok(())
    }
}

fn move_to_floor(state: State, target_floor: usize, chips: &[usize], gens: &[usize]) -> State {
    let mut inv = state.0;
    let elev = state.1;
    for &c in chips {
        inv[elev].chips.take(&c).unwrap();
        inv[target_floor].chips.insert(c);
    }
    for &g in gens {
        inv[elev].gens.take(&g).unwrap();
        inv[target_floor].gens.insert(g);
    }
    State(inv, target_floor)
}

fn next_states(state: &State) -> FxHashSet<State> {
    let inv = &state.0;
    let elev = state.1;
    let mut other_floors = match elev {
        0 => vec![1],
        1 => vec![0, 2],
        2 => vec![1, 3],
        3 => vec![2],
        _ => unreachable!(),
    };
    if (0..elev).all(|fl| inv[fl].chips.is_empty() && inv[fl].gens.is_empty()) {
        other_floors.retain(|&x| x > elev);
    }
    let mut res = FxHashSet::default();

    // move a single chip
    res.extend(inv[elev].chips.iter().filter_map(|&c| {
        // move to floor with matching generator, if not current floor
        let gen_floor = inv.iter().position(|fl| fl.gens.contains(&c)).unwrap();
        (gen_floor != elev && other_floors.contains(&gen_floor))
            .then(|| move_to_floor(state.clone(), gen_floor, &[c], &[]))
    }));
    res.extend(inv[elev].chips.iter().flat_map(|&c| {
        // move to floor with no generators, if not current floor
        let no_gen_floors = inv
            .iter()
            .positions(|fl| fl.gens.is_empty())
            .filter(|fl| other_floors.contains(fl));
        no_gen_floors.map(move |fl| move_to_floor(state.clone(), fl, &[c], &[]))
    }));

    // move two chips
    res.extend(
        inv[elev]
            .chips
            .iter()
            .copied()
            .combinations(2)
            .flat_map(|cs| {
                other_floors
                    .iter()
                    .filter(|&&fl| {
                        inv[fl].gens.is_empty() || cs.iter().all(|c| inv[fl].chips.contains(c))
                    })
                    .map(|&fl| move_to_floor(state.clone(), fl, &cs, &[]))
                    .collect_vec()
            }),
    );

    // move a single generator
    res.extend(inv[elev].gens.iter().filter_map(|&g| {
        // move to floor with matching chip and no other chips, if not current floor
        let chip_floor = inv.iter().position(|fl| fl.chips.contains(&g)).unwrap();
        (chip_floor != elev
            && other_floors.contains(&chip_floor)
            && inv[chip_floor].chips.len() == 1)
            .then(|| move_to_floor(state.clone(), chip_floor, &[], &[g]))
    }));
    res.extend(inv[elev].gens.iter().flat_map(|&g| {
        // move to floor with no chips, if not current floor
        let no_chip_floors = inv
            .iter()
            .positions(|fl| fl.chips.is_empty())
            .filter(|fl| other_floors.contains(fl));
        no_chip_floors.map(move |fl| move_to_floor(state.clone(), fl, &[], &[g]))
    }));

    // move two generators
    res.extend(
        inv[elev]
            .gens
            .iter()
            .copied()
            .combinations(2)
            .flat_map(|gs| {
                let gs_set = FxHashSet::from_iter(gs.iter().copied());
                other_floors
                    .iter()
                    .filter(|&&fl| inv[fl].chips.is_empty() || inv[fl].chips == gs_set)
                    .map(|&fl| move_to_floor(state.clone(), fl, &[], &gs))
                    .collect_vec()
            }),
    );

    // move a chip and a generator
    res.extend(
        // move matching chip and generator to any other floor
        inv[elev]
            .chips
            .intersection(&inv[elev].gens)
            .flat_map(|&c| {
                other_floors
                    .iter()
                    .map(move |&fl| move_to_floor(state.clone(), fl, &[c], &[c]))
            }),
    );

    res
}

fn parse(input: &str) -> Vec<FloorInventory> {
    let mut elems = IndexSet::new();
    input
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect_vec();
            let chips = parts
                .iter()
                .positions(|p| p.starts_with("microchip"))
                .map(|mp| {
                    let elem = parts[mp - 1].split('-').next().unwrap();
                    elems.get_index_of(elem).unwrap_or_else(|| {
                        elems.insert(elem);
                        elems.len() - 1
                    })
                })
                .collect();
            let gens = parts
                .iter()
                .positions(|p| p.starts_with("generator"))
                .map(|mp| {
                    let elem = parts[mp - 1];
                    elems.get_index_of(elem).unwrap_or_else(|| {
                        elems.insert(elem);
                        elems.len() - 1
                    })
                })
                .collect();
            FloorInventory { chips, gens }
        })
        .collect_vec()
}

pub fn part1(input: String) -> usize {
    let floors = parse(&input);
    let total_chips: usize = floors.iter().map(|inv| inv.chips.len()).sum();
    let total_gens: usize = floors.iter().map(|inv| inv.gens.len()).sum();
    let is_solved = |state: &State| {
        let inv = &state.0;
        (0..=2).all(|i| inv[i].chips.is_empty() && inv[i].gens.is_empty())
            && inv[3].chips.len() == total_chips
            && inv[3].gens.len() == total_gens
    };
    let mut states: FxHashSet<State> = FxHashSet::default();
    let mut count = 0;
    let init = State(floors, 0);
    states.insert(init);
    while !states.iter().any(is_solved) {
        states = states
            .into_par_iter()
            .flat_map(|s| next_states(&s).into_iter().collect_vec())
            .collect();
        count += 1;
    }

    count
}
pub fn part2(input: String) -> u64 {
    todo!()
}
