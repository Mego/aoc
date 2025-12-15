use itertools::Itertools;

fn parse_line(line: &str) -> (u64, u64, u64) {
    let pieces = line.split_ascii_whitespace().collect_vec();
    (
        // pieces[0],
        pieces[3].parse().unwrap(),
        pieces[6].parse().unwrap(),
        pieces[pieces.len() - 2].parse().unwrap(),
    )
}

pub fn part1(input: &str) -> impl ToString {
    const SECONDS: u64 = 2503;
    input
        .lines()
        .map(parse_line)
        .map(|(v, t, r)| {
            let full_cycles = SECONDS / (t + r);
            let partial_cycle = SECONDS % (t + r);
            v * t * full_cycles + v * partial_cycle.min(t)
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    const SECONDS: u64 = 2503;
    let stats = input.lines().map(parse_line).collect_vec();
    let mut positions = [0].repeat(stats.len());
    let mut points = [0].repeat(stats.len());
    for i in 0..SECONDS {
        for (j, &(v, t, r)) in stats.iter().enumerate() {
            if i % (t + r) < t {
                positions[j] += v;
            }
        }
        let max = *positions.iter().max().unwrap();
        for (j, v) in points.iter_mut().enumerate() {
            if positions[j] == max {
                *v += 1;
            }
        }
    }
    points.into_iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 14;
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
