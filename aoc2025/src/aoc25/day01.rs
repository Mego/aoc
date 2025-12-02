fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let mut val: i32 = line[1..].parse().unwrap();
            if line.starts_with('L') {
                val = -val;
            }
            val
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let mut dial: i32 = 50;
    let turns = parse(input);
    let mut counter = 0;
    for turn in turns {
        dial += turn;
        if dial % 100 == 0 {
            counter += 1;
        }
    }
    counter
}

pub fn part2(input: &str) -> u64 {
    let mut dial: i32 = 50;
    let turns = parse(input);
    let mut counter = 0;
    for turn in turns {
        let n = turn.abs() / 100;
        let m = turn % 100;
        counter += n as u64;
        if dial != 0 && !(1..=99).contains(&(dial + m)) {
            counter += 1;
        }
        let prev_dial = dial;
        dial += m;
        dial = dial.rem_euclid(100);
    }
    counter
}
