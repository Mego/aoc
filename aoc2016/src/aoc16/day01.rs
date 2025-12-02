use rustc_hash::FxHashSet;

pub fn part1(input: String) -> u32 {
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
pub fn part2(input: String) -> u32 {
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
