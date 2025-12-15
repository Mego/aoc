pub fn part1(input: &str) -> impl ToString {
    for i in 1.. {
        let digest = format!("{:x?}", md5::compute(format!("{input}{i}")));
        if digest.starts_with("00000") {
            return i;
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> impl ToString {
    for i in 1.. {
        let digest = format!("{:x?}", md5::compute(format!("{input}{i}")));
        if digest.starts_with("000000") {
            return i;
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
    const DAY: u8 = 4;
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
