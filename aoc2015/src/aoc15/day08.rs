use itertools::Itertools;

pub fn part1(input: String) -> u64 {
    input.lines().fold(0, |a, line| {
        let mut counter = 0;
        let mut i = 0;
        let bytes = line.bytes().collect_vec();
        while i < bytes.len() {
            counter += 1;
            i += if bytes[i] == b'\\' && i + 1 < bytes.len() {
                match bytes[i + 1] {
                    b'"' | b'\\' => 2,
                    b'x' => 4,
                    _ => 1,
                }
            } else {
                1
            };
        }
        a + (line.len() - counter + 2) as u64
    })
}
pub fn part2(input: String) -> u64 {
    input.lines().fold(0, |a, line| {
        a + ((line.bytes().fold(2, |a, b| {
            a + match b {
                b'\\' | b'"' => 2,
                _ => 1,
            }
        })) - line.len()) as u64
    })
}
