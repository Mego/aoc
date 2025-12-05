use itertools::Itertools;

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ids) = input.split("\n\n").collect_tuple().unwrap();
    (
        ranges
            .lines()
            .map(|line| {
                let (a, b) = line
                    .split('-')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                (a, b)
            })
            .collect_vec(),
        ids.lines().map(|line| line.parse().unwrap()).collect_vec(),
    )
}

fn parse2(input: &str) -> Vec<(u64, u64)> {
    let ranges = input.split("\n\n").next().unwrap();

    ranges
        .lines()
        .map(|line| {
            line.split('-')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .sorted_unstable()
        .collect_vec()
}

pub fn part1(input: &str) -> usize {
    let (ranges, ids) = parse(input);
    ids.into_iter()
        .filter(|&id| ranges.iter().any(|&(a, b)| a <= id && id <= b))
        .count()
}

pub fn part2(input: &str) -> u64 {
    let mut ranges = parse2(input);
    let mut ranges_iter = ranges.into_iter();
    let mut new_ranges = vec![ranges_iter.next().unwrap()];
    for (mut c, mut d) in ranges_iter {
        let (a, b) = *new_ranges.last().unwrap();
        if c <= b + 1 {
            new_ranges.pop();
            c = a.min(c);
            d = b.max(d);
        }
        new_ranges.push((c, d));
    }
    new_ranges
        .into_iter()
        .fold(0, |acc, (a, b)| acc + (b - a + 1))
}
