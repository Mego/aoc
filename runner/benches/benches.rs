use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use util::fetch_input;

macro_rules! bench_part {
    ( $c:expr, $input:expr, $year:ident, $day:ident, $part:ident ) => {
        $c.bench_function(
            &format!("{} - {}", stringify!($day), stringify!($part)),
            |b| {
                b.iter_batched(
                    || $input.clone(),
                    |i| $year::$day::$part(i),
                    BatchSize::LargeInput,
                );
            },
        );
    };
}

macro_rules! day {
    ( $day:ident ) => {
        stringify!($day)[3..].parse::<u8>().unwrap()
    };
}

macro_rules! year {
    ($year:ident) => {
        stringify!($year)[3..].parse::<u16>().unwrap()
    };
}

macro_rules! create_benches {
    ( $benches:ident, $year:ident, $( $day:ident ),* ) => {
        $(
            paste::paste! {
                fn [<$year _ $day>](c: &mut Criterion) {
                    let day = day!($day);
                    let year = year!($year);
                    let input = &Runtime::new().unwrap().block_on(fetch_input(year, day));

                    bench_part!(c, &input, $year, $day, part1);
                    bench_part!(c, &input, $year, $day, part2);
                }
            }
        )*
        paste::paste! {
            criterion_group!{
                name = $benches;
                config = Criterion::default();
                targets = $( [< $year _ $day >], )*
            }
        }
    };
}

create_benches!(
    benches_2015,
    aoc2015,
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
    day24,
    day25
);

create_benches!(
    benches_2016,
    aoc2016,
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
    day24,
    day25
);

create_benches!(
    benches_2024,
    aoc2024,
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
    day24,
    day25
);

create_benches!(
    benches_2025,
    aoc2025,
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12
);

criterion_main!(benches_2015, benches_2016, benches_2024, benches_2025);
