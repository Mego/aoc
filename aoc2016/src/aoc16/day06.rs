use itertools::Itertools;

pub fn part1(input: String) -> String {
    let lines = input.lines().collect_vec();
    (0..(lines[0].len()))
        .map(|i| {
            lines
                .iter()
                .map(|l| l.chars().nth(i).unwrap())
                .counts()
                .into_iter()
                .max_by_key(|(_, c)| *c)
                .unwrap()
                .0
        })
        .join("")
}
pub fn part2(input: String) -> String {
    let lines = input.lines().collect_vec();
    (0..(lines[0].len()))
        .map(|i| {
            lines
                .iter()
                .map(|l| l.chars().nth(i).unwrap())
                .counts()
                .into_iter()
                .min_by_key(|(_, c)| *c)
                .unwrap()
                .0
        })
        .join("")
}
