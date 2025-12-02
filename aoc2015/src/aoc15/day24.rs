use itertools::Itertools;

pub fn part1(input: String) -> u64 {
    let packages: Vec<u16> = input.lines().map(|l| l.parse().unwrap()).collect();
    let goal: u16 = packages.iter().copied().sum::<u16>() / 3;
    (0..packages.len())
        .filter_map(|k| {
            packages
                .iter()
                .combinations(k)
                .filter_map(|c| {
                    (c.iter().copied().sum::<u16>() == goal)
                        .then_some(c.iter().map(|x| **x as u64).product::<u64>())
                })
                .min()
        })
        .next()
        .unwrap()
}
pub fn part2(input: String) -> u64 {
    let packages: Vec<u16> = input.lines().map(|l| l.parse().unwrap()).collect();
    let goal: u16 = packages.iter().copied().sum::<u16>() / 4;
    (0..packages.len())
        .filter_map(|k| {
            packages
                .iter()
                .combinations(k)
                .filter_map(|c| {
                    (c.iter().copied().sum::<u16>() == goal)
                        .then_some(c.iter().map(|x| **x as u64).product::<u64>())
                })
                .min()
        })
        .next()
        .unwrap()
}
