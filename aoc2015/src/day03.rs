use rustc_hash::{FxBuildHasher, FxHashSet};

pub fn part1(input: &str) -> impl ToString {
    input
        .bytes()
        .fold(
            ((0i64, 0i64), {
                let mut visited =
                    FxHashSet::with_capacity_and_hasher(input.len() + 1, FxBuildHasher);
                visited.insert((0, 0));
                visited
            }),
            |(mut curr, mut visited), c| {
                curr.0 += match c {
                    b'>' => 1,
                    b'<' => -1,
                    _ => 0,
                };
                curr.1 += match c {
                    b'v' => 1,
                    b'^' => -1,
                    _ => 0,
                };
                visited.insert(curr);

                (curr, visited)
            },
        )
        .1
        .len()
}

pub fn part2(input: &str) -> impl ToString {
    let mut visited = FxHashSet::with_capacity_and_hasher(input.len() + 1, FxBuildHasher);
    let mut currs = [(0i64, 0i64); 2];
    visited.insert((0, 0));
    for (i, c) in input.bytes().enumerate() {
        let curr = &mut currs[i & 1];
        curr.0 += match c {
            b'>' => 1,
            b'<' => -1,
            _ => 0,
        };
        curr.1 += match c {
            b'v' => 1,
            b'^' => -1,
            _ => 0,
        };
        visited.insert(*curr);
    }
    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 3;
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
