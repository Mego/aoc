use itertools::Itertools;

pub fn part1(input: String) -> String {
    (0..)
        .filter_map(|n| {
            let hash = format!("{:x?}", md5::compute(format!("{input}{n}")));
            hash.starts_with("00000").then_some(hash[5..6].to_string())
        })
        .take(8)
        .join("")
}
pub fn part2(input: String) -> String {
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
