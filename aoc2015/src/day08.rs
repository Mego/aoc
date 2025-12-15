use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
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
        a + (line.len() - counter + 2)
    })
}

pub fn part2(input: &str) -> impl ToString {
    input.lines().fold(0, |a, line| {
        a + ((line.bytes().fold(2, |a, b| {
            a + match b {
                b'\\' | b'"' => 2,
                _ => 1,
            }
        })) - line.len())
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 8;
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
