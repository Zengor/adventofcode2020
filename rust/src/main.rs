use aoc2020::*;

fn main() {
    let input = include_str!("../../input/01-1.txt");
    println!("part 1 {:?}", day01::part1(input));
    println!("part 1 comb {:?}", day01::part1_combinations(input));
    println!("part 1 regular loop {:?}", day01::part1_regular_loop(input));
    println!("part 2 comb {:?}", day01::part2_combinations(input));
}
