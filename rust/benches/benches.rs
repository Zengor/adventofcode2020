use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench {
    ($group:ident, $name:expr, $func:expr, $input:expr) => {
        $group.bench_function($name, |b| b.iter(|| $func(black_box($input))));
    };
}

fn day01(c: &mut Criterion) {
    use aoc2020::day01::*;

    let input = include_str!("../../input/01-1.txt");
    let mut g = c.benchmark_group("Day 1");

    bench!(g, "part 1 hashset", part1, &input);
    bench!(g, "part 1 combinations", part1_combinations, &input);
    bench!(g, "part 1 regular loop", part1_regular_loop, &input);
    bench!(g, "part 1 jake", part1_jake, &input);
    bench!(g, "part 2 combinations", part2_combinations, &input);
    bench!(g, "part 2 jake", part2_jake, &input);
}

criterion_group!(benches, day01);
criterion_main!(benches);
