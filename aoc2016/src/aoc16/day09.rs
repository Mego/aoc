use atoi::FromRadix10;
use itertools::Itertools;

pub fn part1(input: String) -> usize {
    let s = input
        .bytes()
        .filter(|c| !c.is_ascii_whitespace())
        .collect_vec();
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'(' {
            let end = {
                let mut j = i + 1;
                while s[j] != b')' {
                    j += 1;
                }
                j + 1
            };
            let (k, n) = s[i + 1..end - 1]
                .split(|&c| c == b'x')
                .map(|x| usize::from_radix_10(x).0)
                .collect_tuple()
                .unwrap();

            count += k * n - 1;
            i = end + k - 1;
        }
        count += 1;
        i += 1;
    }

    count
}

fn calc_part2(s: &[u8]) -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'(' {
            let end = {
                let mut j = i + 1;
                while s[j] != b')' {
                    j += 1;
                }
                j + 1
            };
            let (k, n) = s[i + 1..end - 1]
                .split(|&c| c == b'x')
                .map(|x| usize::from_radix_10(x).0)
                .collect_tuple()
                .unwrap();

            count += n * calc_part2(&s[end..end + k]) - 1;
            i = end + k - 1;
        }
        count += 1;
        i += 1;
    }

    count
}

pub fn part2(input: String) -> usize {
    let s = input
        .bytes()
        .filter(|c| !c.is_ascii_whitespace())
        .collect_vec();
    calc_part2(&s)
}
