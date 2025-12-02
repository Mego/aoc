use itertools::Itertools;

pub fn part1(input: String) -> u64 {
    input.lines().fold(0, |a, line| {
        let dims: [u64; 3] = line
            .split('x')
            .map(|s| s.parse().unwrap())
            .sorted()
            .collect_array()
            .unwrap();
        let sides = [dims[0] * dims[1], dims[0] * dims[2], dims[1] * dims[2]];
        a + 2 * (sides[0] + sides[1] + sides[2]) + sides[0]
    })
}
pub fn part2(input: String) -> u64 {
    input.lines().fold(0, |a, line| {
        let dims: [u64; 3] = line
            .split('x')
            .map(|s| s.parse().unwrap())
            .sorted()
            .collect_array()
            .unwrap();
        a + 2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2]
    })
}
