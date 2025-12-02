use divisors_fixed::Divisors;

fn sigma(n: u64) -> u64 {
    n.divisors_unordered().into_iter().sum()
}

pub fn part1(input: String) -> u64 {
    let input = input.parse().unwrap();
    (1..).find(|&x| 10 * sigma(x) >= input).unwrap()
}
pub fn part2(input: String) -> u64 {
    let input = input.parse().unwrap();
    (1..)
        .find(|&x| {
            let d: u64 = x
                .divisors_unordered()
                .iter()
                .filter(|&&d| x / d <= 50)
                .sum();
            11 * d >= input
        })
        .unwrap()
}
