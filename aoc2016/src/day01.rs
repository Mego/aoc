use rustc_hash::FxHashSet;

pub fn part1(input: &str) -> impl ToString {
    let (x, y, _, _) = input.split(", ").fold((0, 0, 0, -1), |(x, y, fx, fy), d| {
        let (fx, fy) = match &d[0..1] {
            "R" => (-fy, fx),
            "L" => (fy, -fx),
            _ => unimplemented!(),
        };
        let mag: i32 = d[1..].parse().unwrap();
        (x + fx * mag, y + fy * mag, fx, fy)
    });
    x.unsigned_abs() + y.unsigned_abs()
}

pub fn part2(input: &str) -> impl ToString {
    let mut visited = FxHashSet::default();
    visited.insert((0, 0));
    let (mut x, mut y, mut fx, mut fy) = (0i32, 0i32, 0, -1);
    for d in input.split(", ") {
        (fx, fy) = match &d[0..1] {
            "R" => (-fy, fx),
            "L" => (fy, -fx),
            _ => unimplemented!(),
        };
        let mag: u32 = d[1..].parse().unwrap();
        for _ in 0..mag {
            (x, y) = (x + fx, y + fy);
            if !visited.insert((x, y)) {
                return x.unsigned_abs() + y.unsigned_abs();
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2016;
    const DAY: u8 = 1;
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
