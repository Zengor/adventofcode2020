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
    bench!(g, "part 1 lux", part1_lux, &input);
    bench!(g, "part 2 combinations", part2_combinations, &input);
    bench!(g, "part 2 jake", part2_jake, &input);
    bench!(g, "part 2 lux", part2_lux, &input);
}

fn day02(c: &mut Criterion) {
    use aoc2020::day02::*;

    let input = include_str!("../../input/02-1.txt");
    let mut g = c.benchmark_group("Day 2");
    // TODO compare regex and straight parsing
    bench!(g, "part 1", part1, &input);
    bench!(g, "part 2", part2, &input);
}

#[allow(dead_code)]
fn day03(c: &mut Criterion) {
    use aoc2020::day03::*;

    let input = include_str!("../../input/03-1.txt");
    let mut g = c.benchmark_group("Day 3");
    // TODO compare regex and straight parsing
    // write method that only goes through the array once
    // also iterator that generates all position values instead
    // of calculating and tracking manually
    // another one:
    // calculate the number of "steps" (x * y) and just move that many times
    // instead of doing the modulo math
    bench!(g, "part 1", part1, &input);
    bench!(g, "part 2", part2, &input);
}

// TODO: day 4
// try regex parsing
// refactor
//
// day 5
// write version that goes through the list linearly to find a hole

fn day08(c: &mut Criterion) {
    use aoc2020::day08::*;

    let input = include_str!("../../input/08-1.txt");
    let mut g = c.benchmark_group("Day 8");
    // TODO compare regex and straight parsing
    // write method that only goes through the array once
    // also iterator that generates all position values instead
    // of calculating and tracking manually
    // another one:
    // calculate the number of "steps" (x * y) and just move that many times
    // instead of doing the modulo math
    bench!(g, "part 1", part1, &input);
    //bench!(g, "part 1 bitvec", part1_bitvec, &input);
    bench!(g, "part 2", part2, &input);
    bench!(g, "part 2 bitvec", part2_bitvec, &input);
}
fn day09(c: &mut Criterion) {
    use aoc2020::day09::*;

    let input = include_str!("../../input/09-1.txt");
    let mut g = c.benchmark_group("Day 9");
    bench!(g, "part 1", part1, &input);
    bench!(g, "part 2", part2, &input);
}

fn day13(c: &mut Criterion) {
    use aoc2020::day13::*;
    let input = include_str!("../../input/09-1.txt");
    let mut g = c.benchmark_group("Day 13");
    bench!(g, "part 1", part1, &input);
    bench!(g, "part 2", part2, &input);
}

criterion_group!(benches, day01, day02, day08, day09, day13);
criterion_main!(benches);
