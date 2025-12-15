use itertools::Itertools;

pub fn part1(input: &str) -> impl ToString {
    (0..)
        .filter_map(|n| {
            let hash = format!("{:x?}", md5::compute(format!("{input}{n}")));
            hash.starts_with("00000").then_some(hash[5..6].to_string())
        })
        .take(8)
        .join("")
}

pub fn part2(input: &str) -> impl ToString {
    let mut pw = [const { None }; 8];
    let mut i = 0;
    while pw.iter().any(|v| v.is_none()) {
        let hash = format!("{:x?}", md5::compute(format!("{input}{i}")));
        if hash.starts_with("00000")
            && let Ok(idx) = hash[5..6].parse::<usize>()
            && idx < 8
            && pw[idx].is_none()
        {
            pw[idx] = Some(hash[6..7].to_string());
        }
        i += 1;
    }
    pw.into_iter().flatten().join("")
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2016;
    const DAY: u8 = 5;
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
