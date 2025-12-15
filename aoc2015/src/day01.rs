pub fn part1(input: &str) -> impl ToString {
    input
        .bytes()
        .fold(0i64, |a, c| if c == b'(' { a + 1 } else { a - 1 })
}

pub fn part2(input: &str) -> impl ToString {
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

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
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
