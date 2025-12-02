pub fn part1(input: String) -> u64 {
    for i in 1.. {
        let digest = format!("{:x?}", md5::compute(format!("{input}{i}")));
        if digest.starts_with("00000") {
            return i;
        }
    }
    unreachable!()
}
pub fn part2(input: String) -> u64 {
    for i in 1.. {
        let digest = format!("{:x?}", md5::compute(format!("{input}{i}")));
        if digest.starts_with("000000") {
            return i;
        }
    }
    unreachable!()
}
