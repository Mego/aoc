use rustc_hash::{FxBuildHasher, FxHashSet};

pub fn part1(input: String) -> u64 {
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
        .len() as u64
}
pub fn part2(input: String) -> u64 {
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
    visited.len() as u64
}
