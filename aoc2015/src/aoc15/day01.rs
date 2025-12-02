pub fn part1(input: String) -> i64 {
    input
        .bytes()
        .fold(0i64, |a, c| if c == b'(' { a + 1 } else { a - 1 })
}
pub fn part2(input: String) -> u64 {
    let mut floor = 0i64;
    for (i, c) in input.bytes().enumerate() {
        if c == b'(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor < 0 {
            return (i + 1) as u64;
        }
    }
    unreachable!()
}
