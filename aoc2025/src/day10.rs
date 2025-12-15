use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone)]
struct Machine {
    goal_lights: Vec<bool>,
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    goal_joltage: Vec<usize>,
}

impl Machine {
    fn press_button_lights(&mut self, idx: usize) {
        self.buttons[idx].iter().for_each(|&i| {
            self.lights[i] = !self.lights[i];
        });
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect_vec();
            let goal_lights = parts[0]
                .trim_matches(['[', ']'])
                .bytes()
                .map(|b| b == b'#')
                .collect_vec();
            let goal_joltage = parts
                .last()
                .unwrap()
                .trim_matches(['{', '}'])
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_vec();
            let buttons = parts[1..(parts.len() - 1)]
                .iter()
                .map(|button| {
                    button
                        .trim_matches(['(', ')'])
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect_vec()
                })
                .collect_vec();
            Machine {
                lights: vec![false; goal_lights.len()],
                goal_lights,
                buttons,
                goal_joltage,
            }
        })
        .collect_vec()
}

pub fn part1(input: &str) -> impl ToString {
    let machines = parse(input);
    machines
        .into_par_iter()
        .map(|machine| {
            (1..)
                .find(|&n| {
                    (0..machine.buttons.len()).combinations(n).any(|b| {
                        let mut machine = machine.clone();
                        for btn in b {
                            machine.press_button_lights(btn);
                        }
                        machine.lights == machine.goal_lights
                    })
                })
                .unwrap()
        })
        .sum::<usize>()
}

fn solve_joltage(machine: Machine) -> usize {
    use highs::{RowProblem, Sense};
    let mut pb = RowProblem::default();
    let vars = machine
        .buttons
        .iter()
        .map(|btn| {
            pb.add_integer_column(
                1.0,
                0.0..(btn.iter().map(|&i| machine.goal_joltage[i]).max().unwrap() as f64),
            )
        })
        .collect_vec();
    machine
        .goal_joltage
        .iter()
        .enumerate()
        .for_each(|(i, &goal)| {
            let goal = goal as f64;
            pb.add_row(
                goal..=goal,
                machine
                    .buttons
                    .iter()
                    .enumerate()
                    .map(|(k, btn)| (vars[k], if btn.contains(&i) { 1.0 } else { 0.0 })),
            );
        });
    pb.optimise(Sense::Minimise).solve().objective_value() as usize
}

pub fn part2(input: &str) -> impl ToString {
    let machines = parse(input);
    machines.into_iter().map(solve_joltage).sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2025;
    const DAY: u8 = 10;
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
